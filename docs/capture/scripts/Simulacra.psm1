# Helpers that drive a running Simulacra window for documentation captures:
# start it on a throw-away profile, size the window, send keys, and save PNG captures.

Add-Type -AssemblyName System.Drawing
Add-Type -AssemblyName System.Windows.Forms

if (-not ([System.Management.Automation.PSTypeName]'SimWin').Type) {
Add-Type -TypeDefinition @'
using System;
using System.Text;
using System.Runtime.InteropServices;

public static class SimWin {
    public delegate bool EnumProc(IntPtr hWnd, IntPtr lParam);
    [DllImport("user32.dll")] public static extern bool EnumWindows(EnumProc f, IntPtr l);
    [DllImport("user32.dll")] public static extern uint GetWindowThreadProcessId(IntPtr h, out uint pid);
    [DllImport("user32.dll")] public static extern bool IsWindowVisible(IntPtr h);
    [DllImport("user32.dll", CharSet = CharSet.Unicode)] public static extern int GetClassName(IntPtr h, StringBuilder s, int n);
    [DllImport("user32.dll", CharSet = CharSet.Unicode)] public static extern int GetWindowText(IntPtr h, StringBuilder s, int n);
    [DllImport("user32.dll")] public static extern bool SetForegroundWindow(IntPtr h);
    [DllImport("user32.dll")] public static extern bool ShowWindow(IntPtr h, int cmd);
    [DllImport("user32.dll")] public static extern bool SetWindowPos(IntPtr h, IntPtr after, int x, int y, int cx, int cy, uint flags);
    [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr h, out RECT r);
    [DllImport("user32.dll")] public static extern bool GetClientRect(IntPtr h, out RECT r);
    [DllImport("user32.dll")] public static extern bool ClientToScreen(IntPtr h, ref POINT p);
    [DllImport("user32.dll")] public static extern bool PrintWindow(IntPtr h, IntPtr hdc, uint flags);
    [DllImport("user32.dll")] public static extern uint GetDpiForWindow(IntPtr h);
    [DllImport("user32.dll")] public static extern bool SetProcessDpiAwarenessContext(IntPtr v);
    [DllImport("dwmapi.dll")] public static extern int DwmGetWindowAttribute(IntPtr h, int attr, out RECT r, int size);
    [DllImport("user32.dll")] public static extern bool SetCursorPos(int x, int y);
    [DllImport("user32.dll")] public static extern IntPtr GetForegroundWindow();
    [DllImport("user32.dll")] public static extern void keybd_event(byte vk, byte scan, uint flags, UIntPtr extra);
    [DllImport("user32.dll")] public static extern bool AttachThreadInput(uint a, uint b, bool attach);
    [DllImport("user32.dll")] public static extern bool BringWindowToTop(IntPtr h);
    [DllImport("kernel32.dll")] public static extern uint GetCurrentThreadId();
    [DllImport("user32.dll")] public static extern bool IsIconic(IntPtr h);
    [DllImport("user32.dll")] public static extern void mouse_event(uint flags, int dx, int dy, int data, UIntPtr extra);

    [StructLayout(LayoutKind.Sequential)] public struct RECT { public int Left, Top, Right, Bottom; }
    [StructLayout(LayoutKind.Sequential)] public struct POINT { public int X, Y; }

    public static IntPtr[] Windows(int pid) {
        var list = new System.Collections.Generic.List<IntPtr>();
        EnumWindows((h, l) => {
            uint p; GetWindowThreadProcessId(h, out p);
            if (p == pid && IsWindowVisible(h)) list.Add(h);
            return true;
        }, IntPtr.Zero);
        return list.ToArray();
    }
    public static string Title(IntPtr h) { var s = new StringBuilder(512); GetWindowText(h, s, 512); return s.ToString(); }
    public static string Class(IntPtr h) { var s = new StringBuilder(256); GetClassName(h, s, 256); return s.ToString(); }
}
'@
# Per-monitor DPI awareness so window coordinates are physical pixels.
[SimWin]::SetProcessDpiAwarenessContext([IntPtr]-4) | Out-Null
}
if (-not ([System.Management.Automation.PSTypeName]'SimPost').Type) {
    Add-Type -Path (Join-Path $PSScriptRoot 'SimPost.cs')
}

$script:Exe = Join-Path $PSScriptRoot '..\..\..\target\debug\simulacra.exe'

# Background mode (the default): the window is parked off-screen below the primary monitor and
# never takes the foreground; clicks, keys and the wheel are posted to it (SimPost.cs) and
# captures read the window itself (PrintWindow), so the computer stays usable during a run.
# Foreground mode drives the real pointer and keyboard on a topmost window.
$script:Background = $true
$script:Pointer = @(0, 0)

function Set-SimBackground([bool]$On) { $script:Background = $On }
function Test-SimBackground { $script:Background }

function Get-SimParking {
    <# Where the window waits in background mode: just below the primary monitor, off every
       screen, so it keeps the primary monitor's scale. #>
    $primary = [System.Windows.Forms.Screen]::PrimaryScreen.Bounds
    @(($primary.X + 40), ($primary.Bottom + 120))
}

function Start-Simulacra {
    param([Parameter(Mandatory)][string]$ProfileDir, [string]$Exe = $script:Exe)
    New-Item -ItemType Directory -Force -Path $ProfileDir | Out-Null
    $env:SIMULACRA_DB_PATH = Join-Path $ProfileDir 'simulacra.db'
    $proc = Start-Process -FilePath (Resolve-Path $Exe) -WorkingDirectory $ProfileDir -PassThru -WindowStyle Normal `
        -RedirectStandardError (Join-Path $ProfileDir 'stderr.log') -RedirectStandardOutput (Join-Path $ProfileDir 'stdout.log')
    $proc
}

function Get-SimulacraWindow {
    $proc = Get-Process -Name simulacra -ErrorAction SilentlyContinue | Select-Object -First 1
    if (-not $proc) { return $null }
    $wins = [SimWin]::Windows($proc.Id)
    # The main window is the largest visible top-level window of the process.
    $best = $null; $bestArea = 0
    foreach ($h in $wins) {
        $r = New-Object SimWin+RECT; [SimWin]::GetWindowRect($h, [ref]$r) | Out-Null
        $area = ($r.Right - $r.Left) * ($r.Bottom - $r.Top)
        if ($area -gt $bestArea) { $best = $h; $bestArea = $area }
    }
    $best
}

$script:Placement = @(1456, 908, 40, 30)

function Move-SimWindow([IntPtr]$H, [int]$Width, [int]$Height, [int]$X, [int]$Y) {
    <# Places the window: topmost on screen in foreground mode; parked off-screen, at the
       bottom of the z-order and not activated, in background mode. #>
    if ($script:Background) {
        $park = Get-SimParking
        [SimWin]::ShowWindow($H, 4) | Out-Null   # SW_SHOWNOACTIVATE: restores without activating
        [SimWin]::SetWindowPos($H, [IntPtr]1, $park[0], $park[1], $Width, $Height, 0x0050) | Out-Null   # HWND_BOTTOM, NOACTIVATE | SHOWWINDOW
    } else {
        [SimWin]::ShowWindow($H, 9) | Out-Null   # SW_RESTORE
        # Topmost while captures run, so no other window covers a click target.
        [SimWin]::SetWindowPos($H, [IntPtr]-1, $X, $Y, $Width, $Height, 0x0040) | Out-Null
    }
}

function Restore-Simulacra {
    <# Puts a minimised window back where Set-SimulacraSize last placed it. #>
    $h = Get-SimulacraWindow
    if ($h -and [SimWin]::IsIconic($h)) {
        Write-Warning 'Simulacra was minimised; restoring it'
        $p = $script:Placement
        Move-SimWindow $h $p[0] $p[1] $p[2] $p[3]
        Start-Sleep -Milliseconds 600
        if ($script:Background) { [SimPost]::Activate($h) }
    }
}

function Set-SimulacraSize {
    param([int]$Width = 1440, [int]$Height = 900, [int]$X = 40, [int]$Y = 30)
    $script:Placement = @($Width, $Height, $X, $Y)
    $h = Get-SimulacraWindow
    Move-SimWindow $h $Width $Height $X $Y
    if ($script:Background) { Start-Sleep -Milliseconds 300; [SimPost]::Activate($h) }
}

function Give-ForegroundBack([IntPtr]$To) {
    <# Hands the foreground back to the window that had it before Simulacra opened (Simulacra
       takes it once, at start-up). The same unlock as Focus-Simulacra: an unused F24 tap and the
       foreground thread's input queue. #>
    if ($To -eq [IntPtr]::Zero) { return }
    for ($try = 0; $try -lt 4; $try++) {
        $fg = [SimWin]::GetForegroundWindow()
        if ($fg -eq $To) { return }
        [SimWin]::keybd_event(0x87, 0, 0, [UIntPtr]::Zero); [SimWin]::keybd_event(0x87, 0, 2, [UIntPtr]::Zero)
        $fgPid = 0; $fgThread = [SimWin]::GetWindowThreadProcessId($fg, [ref]$fgPid)
        $me = [SimWin]::GetCurrentThreadId()
        [SimWin]::AttachThreadInput($me, $fgThread, $true) | Out-Null
        [SimWin]::SetForegroundWindow($To) | Out-Null
        [SimWin]::AttachThreadInput($me, $fgThread, $false) | Out-Null
        Start-Sleep -Milliseconds 150
    }
}

function Hide-SimulacraLaunch([IntPtr]$UserWindow) {
    <# Background mode, while Simulacra starts: parks each window it shows (splash, then the
       maximised main window) and gives the foreground back. Returns true when it moved one. #>
    if (-not $script:Background) { return $false }
    $proc = Get-Process -Name simulacra -ErrorAction SilentlyContinue | Select-Object -First 1
    if (-not $proc) { return $false }
    $park = Get-SimParking; $moved = $false
    foreach ($h in [SimWin]::Windows($proc.Id)) {
        $r = New-Object SimWin+RECT; [SimWin]::GetWindowRect($h, [ref]$r) | Out-Null
        if ($r.Top -ge $park[1] - 20) { continue }
        $w = $r.Right - $r.Left; $hh = $r.Bottom - $r.Top
        $zoomed = $w -gt 1600
        if ($zoomed) { $w = 1456; $hh = 908 }
        Move-SimWindow $h $w $hh 0 0
        $moved = $true
    }
    if ($moved -or ([SimWin]::GetForegroundWindow() -ne $UserWindow)) {
        $fgPid = 0; [void][SimWin]::GetWindowThreadProcessId([SimWin]::GetForegroundWindow(), [ref]$fgPid)
        if ($fgPid -eq $proc.Id) { Give-ForegroundBack $UserWindow }
    }
    $moved
}

function Focus-Simulacra {
    <# Gives the window the keyboard. Foreground mode brings it to the front (Windows refuses
       focus changes from background processes, so a tapped, unused F24 key and a borrowed input
       queue unlock it); background mode only tells it that it is active. #>
    Restore-Simulacra
    $h = Get-SimulacraWindow
    if (-not $h) { throw 'No Simulacra window' }
    if ($script:Background) {
        # Someone clicked the window and then left it: it now believes it is inactive.
        if ([SimWin]::GetForegroundWindow() -eq $h) { $script:WasForeground = $true }
        elseif ($script:WasForeground) { $script:WasForeground = $false; [SimPost]::Activate($h) }
        return
    }
    for ($try = 0; $try -lt 4; $try++) {
        if ([SimWin]::GetForegroundWindow() -eq $h) { return }
        [SimWin]::keybd_event(0x87, 0, 0, [UIntPtr]::Zero); [SimWin]::keybd_event(0x87, 0, 2, [UIntPtr]::Zero)   # F24: harmless
        $fg = [SimWin]::GetForegroundWindow(); $fgPid = 0; $targetPid = 0
        $fgThread = [SimWin]::GetWindowThreadProcessId($fg, [ref]$fgPid)
        $me = [SimWin]::GetCurrentThreadId()
        [SimWin]::AttachThreadInput($me, $fgThread, $true) | Out-Null
        [SimWin]::BringWindowToTop($h) | Out-Null
        [SimWin]::SetForegroundWindow($h) | Out-Null
        [SimWin]::AttachThreadInput($me, $fgThread, $false) | Out-Null
        Start-Sleep -Milliseconds 200
    }
    if ([SimWin]::GetForegroundWindow() -ne $h) { throw 'Simulacra could not take the keyboard focus; not sending input' }
}

# SendKeys names → virtual keys; the second value marks extended keys (arrows and the block above).
$script:KeyNames = @{
    ENTER = 0x0D, $false; ESC = 0x1B, $false; ESCAPE = 0x1B, $false; TAB = 0x09, $false
    BACKSPACE = 0x08, $false; BS = 0x08, $false; BKSP = 0x08, $false; DEL = 0x2E, $true; DELETE = 0x2E, $true
    INS = 0x2D, $true; INSERT = 0x2D, $true; HOME = 0x24, $true; END = 0x23, $true; PGUP = 0x21, $true; PGDN = 0x22, $true
    UP = 0x26, $true; DOWN = 0x28, $true; LEFT = 0x25, $true; RIGHT = 0x27, $true; SPACE = 0x20, $false
}
1..24 | ForEach-Object { $script:KeyNames["F$_"] = (0x6F + $_), $false }

function Send-PostedKeys([IntPtr]$H, [string]$Keys) {
    <# The SendKeys syntax the scenes use, posted: ^ Ctrl, + Shift, % Alt before a key; {NAME}
       or {NAME n} for named keys; {c} for a literal c; ~ for Enter; other characters typed. #>
    $mods = 0; $text = [System.Text.StringBuilder]::new(); $i = 0
    $flush = { if ($text.Length) { [SimPost]::Text($H, $text.ToString()); [void]$text.Clear() } }
    while ($i -lt $Keys.Length) {
        $c = $Keys[$i]
        if ('^+%'.Contains($c)) {
            & $flush
            $mods = $mods -bor @{ '^' = [SimPost]::Ctrl; '+' = [SimPost]::Shift; '%' = [SimPost]::Alt }["$c"]
            $i++; continue
        }
        $name = $null; $count = 1
        if ($c -eq '{') {
            $close = if ($Keys.Length -gt $i + 2 -and $Keys[$i + 1] -eq '}' -and $Keys[$i + 2] -eq '}') { $i + 2 } else { $Keys.IndexOf('}', $i + 1) }
            if ($close -lt 0) { throw "unclosed { in keys '$Keys'" }
            $name = $Keys.Substring($i + 1, $close - $i - 1)
            if ($name -match '^(.+?) (\d+)$') { $name = $Matches[1]; $count = [int]$Matches[2] }
            $i = $close + 1
        } elseif ($c -eq '~') { $name = 'ENTER'; $i++ }
        elseif ($c -eq '(' -or $c -eq ')') { throw "SendKeys groups are not supported in background mode: '$Keys'" }
        else { $name = "$c"; $i++ }
        $named = if ($name.Length -gt 1) { $script:KeyNames[$name.ToUpperInvariant()] } else { $null }
        if ($name.Length -gt 1 -and -not $named) { throw "unknown key {$name} in '$Keys'" }
        if ($named) {
            & $flush
            [SimPost]::Key($H, [uint32]$named[0], $mods, $named[1], $count)
        } elseif ($mods) {
            $scan = [SimPost]::ScanOf($name[0])
            if ($scan -eq -1) { throw "no key for '$name' on this keyboard layout" }
            [SimPost]::Key($H, [uint32]($scan -band 0xFF), ($mods -bor (($scan -shr 8) -band 1)), $false, $count)
        } else {
            [void]$text.Append($name * $count)
        }
        $mods = 0
    }
    & $flush
}

function Send-SimKeys {
    param([Parameter(Mandatory)][string]$Keys, [int]$WaitMs = 400)
    Focus-Simulacra
    if ($script:Background) { Send-PostedKeys (Get-SimulacraWindow) $Keys }
    else { [System.Windows.Forms.SendKeys]::SendWait($Keys) }
    Start-Sleep -Milliseconds $WaitMs
}

function Save-SimShot {
    <# Saves the window's client area (no OS frame) as PNG. Crop is x,y,w,h in client pixels. #>
    param([Parameter(Mandatory)][string]$Path, [int[]]$Crop, [switch]$Screen)
    Restore-Simulacra
    $h = Get-SimulacraWindow
    $cr = New-Object SimWin+RECT; [SimWin]::GetClientRect($h, [ref]$cr) | Out-Null
    $pt = New-Object SimWin+POINT; [SimWin]::ClientToScreen($h, [ref]$pt) | Out-Null
    $w = $cr.Right; $hgt = $cr.Bottom
    $bmp = New-Object System.Drawing.Bitmap $w, $hgt
    $g = [System.Drawing.Graphics]::FromImage($bmp)
    if ($Screen -and -not $script:Background) {
        Focus-Simulacra
        $g.CopyFromScreen($pt.X, $pt.Y, 0, 0, (New-Object System.Drawing.Size $w, $hgt))
    } else {
        $wr = New-Object SimWin+RECT; [SimWin]::GetWindowRect($h, [ref]$wr) | Out-Null
        $full = New-Object System.Drawing.Bitmap ($wr.Right - $wr.Left), ($wr.Bottom - $wr.Top)
        $fg = [System.Drawing.Graphics]::FromImage($full)
        $hdc = $fg.GetHdc()
        [SimWin]::PrintWindow($h, $hdc, 2) | Out-Null   # PW_RENDERFULLCONTENT
        $fg.ReleaseHdc($hdc); $fg.Dispose()
        $g.DrawImage($full, (New-Object System.Drawing.Rectangle 0, 0, $w, $hgt), ($pt.X - $wr.Left), ($pt.Y - $wr.Top), $w, $hgt, [System.Drawing.GraphicsUnit]::Pixel)
        $full.Dispose()
    }
    $g.Dispose()
    if ($Crop) {
        $rect = New-Object System.Drawing.Rectangle $Crop[0], $Crop[1], $Crop[2], $Crop[3]
        $part = $bmp.Clone($rect, $bmp.PixelFormat); $bmp.Dispose(); $bmp = $part
    }
    New-Item -ItemType Directory -Force -Path (Split-Path $Path) | Out-Null
    $bmp.Save($Path, [System.Drawing.Imaging.ImageFormat]::Png)
    $bmp.Dispose()
    Get-Item $Path
}

function ConvertTo-ScreenPoint {
    param([int]$X, [int]$Y)
    $h = Get-SimulacraWindow
    $pt = New-Object SimWin+POINT; $pt.X = $X; $pt.Y = $Y
    [SimWin]::ClientToScreen($h, [ref]$pt) | Out-Null
    $pt
}

function Move-SimMouse {
    <# Moves the pointer to client coordinates (hover). #>
    param([int]$X, [int]$Y, [int]$WaitMs = 300)
    Focus-Simulacra
    $script:Pointer = @($X, $Y)
    if ($script:Background) {
        $h = Get-SimulacraWindow
        [SimPost]::Move($h, $X - 3, $Y - 3, $false)
        Start-Sleep -Milliseconds 40
        [SimPost]::Move($h, $X, $Y, $false)
    } else {
        $pt = ConvertTo-ScreenPoint $X $Y
        [SimWin]::SetCursorPos($pt.X - 3, $pt.Y - 3) | Out-Null
        Start-Sleep -Milliseconds 40
        [SimWin]::SetCursorPos($pt.X, $pt.Y) | Out-Null
    }
    Start-Sleep -Milliseconds $WaitMs
}

function Invoke-SimClick {
    <# Clicks at client coordinates. #>
    param([int]$X, [int]$Y, [switch]$Right, [switch]$Double, [int]$WaitMs = 450)
    Move-SimMouse $X $Y -WaitMs 60
    $times = if ($Double) { 2 } else { 1 }
    $h = Get-SimulacraWindow
    for ($i = 0; $i -lt $times; $i++) {
        if ($script:Background) {
            [SimPost]::Button($h, $X, $Y, $Right, $true); Start-Sleep -Milliseconds 30
            [SimPost]::Button($h, $X, $Y, $Right, $false); Start-Sleep -Milliseconds 60
        } else {
            $down = if ($Right) { 0x0008 } else { 0x0002 }
            $up = if ($Right) { 0x0010 } else { 0x0004 }
            [SimWin]::mouse_event($down, 0, 0, 0, [UIntPtr]::Zero); Start-Sleep -Milliseconds 30
            [SimWin]::mouse_event($up, 0, 0, 0, [UIntPtr]::Zero); Start-Sleep -Milliseconds 60
        }
    }
    Start-Sleep -Milliseconds $WaitMs
}

function Send-SimWheel {
    <# A wheel turn of $Delta units (120 = one notch; negative scrolls down) where the pointer is. #>
    param([int]$Delta)
    if ($script:Background) {
        $pt = ConvertTo-ScreenPoint $script:Pointer[0] $script:Pointer[1]
        [SimPost]::Wheel((Get-SimulacraWindow), $pt.X, $pt.Y, $Delta)
    } else {
        [SimWin]::mouse_event(0x0800, 0, 0, $Delta, [UIntPtr]::Zero)
    }
}

function Invoke-SimScroll {
    param([int]$X, [int]$Y, [int]$Notches = -3, [int]$WaitMs = 350)
    Move-SimMouse $X $Y -WaitMs 60
    Send-SimWheel (120 * $Notches)
    Start-Sleep -Milliseconds $WaitMs
}

function Invoke-SimDrag {
    <# Drags with the left button held through client points (x, y pairs), $StepMs apart. #>
    param([Parameter(Mandatory)][int[][]]$Points, [int]$StepMs = 12)
    Focus-Simulacra
    $last = $Points[$Points.Count - 1]
    $script:Pointer = @($last[0], $last[1])
    if ($script:Background) {
        [SimPost]::Drag((Get-SimulacraWindow), [int[]]($Points | ForEach-Object { $_[0] }), [int[]]($Points | ForEach-Object { $_[1] }), $StepMs)
    } else {
        $p0 = ConvertTo-ScreenPoint $Points[0][0] $Points[0][1]
        [SimWin]::SetCursorPos($p0.X, $p0.Y) | Out-Null
        Start-Sleep -Milliseconds 60
        [SimWin]::mouse_event(0x0002, 0, 0, 0, [UIntPtr]::Zero)
        Start-Sleep -Milliseconds 40
        foreach ($pt in $Points) {
            $s = ConvertTo-ScreenPoint $pt[0] $pt[1]
            [SimWin]::SetCursorPos($s.X, $s.Y) | Out-Null
            Start-Sleep -Milliseconds $StepMs
        }
        [SimWin]::mouse_event(0x0004, 0, 0, 0, [UIntPtr]::Zero)
    }
    Start-Sleep -Milliseconds 120
}

function Send-SimFileDialog {
    <# Fills the file name box of the file dialog Simulacra just opened with a full path and
       confirms it. Background mode does it by message, so the dialog never needs the keyboard. #>
    param([Parameter(Mandatory)][string]$Path)
    $proc = Get-Process simulacra | Select-Object -First 1
    $dlg = $null
    for ($i = 0; $i -lt 40 -and -not $dlg; $i++) {
        $dlg = [SimWin]::Windows($proc.Id) | Where-Object { [SimWin]::Class($_) -eq '#32770' } | Select-Object -First 1
        if (-not $dlg) { Start-Sleep -Milliseconds 250 }
    }
    if (-not $dlg) { throw 'no file dialog opened' }
    Start-Sleep -Milliseconds 700
    if ($script:Background) {
        if (-not [SimPost]::FileDialog($dlg, $Path)) { throw 'the file dialog has no file name box' }
    } else {
        [SimWin]::SetForegroundWindow($dlg) | Out-Null
        Start-Sleep -Milliseconds 300
        [System.Windows.Forms.SendKeys]::SendWait('%n')   # the File name box
        Start-Sleep -Milliseconds 200
        [System.Windows.Forms.SendKeys]::SendWait(($Path -replace '([+^%~(){}\[\]])', '{$1}'))
        Start-Sleep -Milliseconds 300
        [System.Windows.Forms.SendKeys]::SendWait('{ENTER}')
    }
    Start-Sleep -Milliseconds 1500
}

$script:ClipboardHelper = Join-Path $PSScriptRoot 'clipboard-files.ps1'

function Set-SimClipboardFile {
    <# Puts one file on the clipboard the way Explorer's Copy does, keeping what was there: a
       hidden helper saves the clipboard first and puts it back on Restore-SimClipboard. #>
    param([Parameter(Mandatory)][string]$Path)
    Restore-SimClipboard
    $script:ClipboardFlag = Join-Path ([System.IO.Path]::GetTempPath()) "sim-clip-$PID-$(Get-Random).flag"
    $ready = "$($script:ClipboardFlag).ready"
    $script:ClipboardProc = Start-Process powershell.exe -PassThru -WindowStyle Hidden -ArgumentList '-NoProfile', '-STA',
        '-ExecutionPolicy', 'Bypass', '-File', $script:ClipboardHelper, '-File', $Path, '-Flag', $script:ClipboardFlag
    for ($i = 0; $i -lt 50 -and -not (Test-Path $ready); $i++) { Start-Sleep -Milliseconds 100 }
    Remove-Item $ready -ErrorAction SilentlyContinue
}

function Restore-SimClipboard {
    <# Puts back the clipboard Set-SimClipboardFile replaced (after the paste). #>
    if (-not $script:ClipboardProc) { return }
    New-Item -ItemType File -Force -Path $script:ClipboardFlag | Out-Null
    if (-not $script:ClipboardProc.WaitForExit(5000)) { Stop-Process -Id $script:ClipboardProc.Id -Force -ErrorAction SilentlyContinue }
    Remove-Item $script:ClipboardFlag -ErrorAction SilentlyContinue
    $script:ClipboardProc = $null
}

$script:OcrScript = Join-Path $PSScriptRoot 'ocr.ps1'

function Get-SimText {
    <# OCR of the window: lines with client-pixel boxes. Scale 2 reads small text better. #>
    param([int]$Scale = 2)
    $tmp = Join-Path ([System.IO.Path]::GetTempPath()) "sim-ocr-$PID.png"
    Save-SimShot -Path $tmp | Out-Null
    if ($Scale -ne 1) {
        $img = [System.Drawing.Image]::FromFile($tmp)
        $big = New-Object System.Drawing.Bitmap ($img.Width * $Scale), ($img.Height * $Scale)
        $g = [System.Drawing.Graphics]::FromImage($big)
        $g.InterpolationMode = [System.Drawing.Drawing2D.InterpolationMode]::HighQualityBicubic
        $g.DrawImage($img, 0, 0, $big.Width, $big.Height); $g.Dispose(); $img.Dispose()
        $tmp2 = $tmp -replace '\.png$', '-x.png'
        $big.Save($tmp2, [System.Drawing.Imaging.ImageFormat]::Png); $big.Dispose()
        $tmp = $tmp2
    }
    $lines = & powershell.exe -NoProfile -ExecutionPolicy Bypass -File $script:OcrScript -Path $tmp
    $lines | Where-Object { $_ -match '^\{' } | ForEach-Object {
        $o = $_ | ConvertFrom-Json
        [pscustomobject]@{ text = $o.text; x = [int]($o.x / $Scale); y = [int]($o.y / $Scale); w = [int]($o.w / $Scale); h = [int]($o.h / $Scale) }
    }
}

function Find-SimText {
    <# Lines whose text contains $Text (case-insensitive), optionally inside a client region x,y,w,h. #>
    param([Parameter(Mandatory)][string]$Text, [int[]]$Region, [int]$Scale = 2)
    Get-SimText -Scale $Scale | Where-Object {
        $_.text -like "*$Text*" -and (-not $Region -or ($_.x -ge $Region[0] -and $_.y -ge $Region[1] -and $_.x -le ($Region[0] + $Region[2]) -and $_.y -le ($Region[1] + $Region[3])))
    }
}

function Invoke-SimClickText {
    <# Clicks the centre of the $Nth (0-based) OCR line containing $Text. Throws when absent. #>
    param([Parameter(Mandatory)][string]$Text, [int]$Nth = 0, [int[]]$Region, [int]$DX = 0, [int]$DY = 0, [switch]$Right, [switch]$Double, [int]$WaitMs = 500)
    $hits = @(Find-SimText -Text $Text -Region $Region)
    if ($hits.Count -le $Nth) { throw "Text '$Text' not found on screen (found $($hits.Count))" }
    $hit = $hits[$Nth]
    Invoke-SimClick ([int]($hit.x + $hit.w / 2) + $DX) ([int]($hit.y + $hit.h / 2) + $DY) -Right:$Right -Double:$Double -WaitMs $WaitMs
    $hit
}

Export-ModuleMember -Function Restore-Simulacra, Start-Simulacra, Get-SimulacraWindow, Set-SimulacraSize, Focus-Simulacra, Send-SimKeys, Save-SimShot, ConvertTo-ScreenPoint, Move-SimMouse, Invoke-SimClick, Invoke-SimScroll, Get-SimText, Find-SimText, Invoke-SimClickText,
    Set-SimBackground, Test-SimBackground, Hide-SimulacraLaunch, Send-SimWheel, Invoke-SimDrag, Send-SimFileDialog, Set-SimClipboardFile, Restore-SimClipboard
