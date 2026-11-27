<#
.SYNOPSIS
    Captures the documentation screenshots: starts Simulacra on a fresh throw-away profile, sizes the
    window to a 1440x900 client area, and replays the scene scripts in scenes/*.ps1.

.EXAMPLE
    pwsh docs/capture/run.ps1                       # every scene, dark theme
    pwsh docs/capture/run.ps1 -Scenes vault,review  # only those scenes
    pwsh docs/capture/run.ps1 -Theme light          # light-theme twins
    python docs/capture/convert.py                  # then convert to WebP + manifest

.NOTES
    Windows only (PrintWindow, posted window messages, Windows OCR). Build the app first: `cargo build`.
    By default it runs in the background: Simulacra waits off-screen below the primary monitor,
    clicks and keys are posted to its window, and the computer stays usable (Simulacra takes the
    foreground once at start-up; the run hands it straight back). -Foreground drives the real
    pointer and keyboard on a topmost window instead: keep the mouse and keyboard still then.
#>
param(
    [string[]]$Scenes,
    [ValidateSet('dark', 'light')][string]$Theme = 'dark',
    [int]$Width = 1440,
    [int]$Height = 900,
    [string]$Exe,
    [string]$ProfileDir,
    [ValidateSet('fresh', 'demo')][string]$Profile = 'demo',
    [string]$SplashShot,
    [string[]]$Only,
    [switch]$KeepOpen,
    [switch]$Attach,
    [switch]$Foreground
)
$ErrorActionPreference = 'Stop'
# `pwsh -File run.ps1 -Scenes a,b` passes one string: split it.
if ($Scenes) { $Scenes = $Scenes -split ',' | ForEach-Object Trim }
if ($Only) { $Only = $Only -split ',' | ForEach-Object Trim }
Import-Module (Join-Path $PSScriptRoot 'scripts/Simulacra.psm1') -Force -WarningAction SilentlyContinue
Set-SimBackground (-not $Foreground)

$script:RawDir = Join-Path $PSScriptRoot "raw/$Theme"
$script:Theme = $Theme
$script:W = $Width
$script:H = $Height
$script:Failures = [System.Collections.Generic.List[string]]::new()
New-Item -ItemType Directory -Force -Path $script:RawDir | Out-Null

$script:MarkPath = Join-Path ([System.IO.Path]::GetTempPath()) "sim-mark-$PID.png"
$script:AutoCrop = Join-Path $PSScriptRoot 'autocrop.py'

function Mark {
    <# Remembers the window as it is now; a later `Shot -Auto` crops to what changed since. #>
    param([int]$SettleMs = 250)
    Start-Sleep -Milliseconds $SettleMs
    Save-SimShot -Path $script:MarkPath | Out-Null
}

function Shot {
    <#
      Saves capture `$Id`. Region/element shots crop with -Crop x,y,w,h (client pixels), around an
      OCR'd label with -Around 'Label' -Pad left,top,right,bottom, or with -Auto to what changed
      since the last `Mark` (a menu, popover, dialog or toast), padded by -AutoPad, optionally
      limited to -Within x0,y0,x1,y1.
    #>
    param([Parameter(Mandatory)][string]$Id, [int[]]$Crop, [string]$Around, [int[]]$Pad = @(12, 12, 12, 12), [int]$Nth = 0, [int]$SettleMs = 350,
        [switch]$Auto, [int]$AutoPad = 12, [int[]]$Within)
    Start-Sleep -Milliseconds $SettleMs
    if ($Auto) {
        $after = Join-Path ([System.IO.Path]::GetTempPath()) "sim-after-$PID.png"
        Save-SimShot -Path $after | Out-Null
        $path = Join-Path $script:RawDir "$Id.png"
        $cropArgs = @($script:AutoCrop, $script:MarkPath, $after, $path, $AutoPad)
        if ($Within) { $cropArgs += $Within }
        $box = python @cropArgs
        Write-Host "  shot $Id (auto $box)" -ForegroundColor DarkGray
        $script:LastBox = [int[]]($box -split ' ')
        return
    }
    if ($Around) {
        $hits = @(Find-SimText -Text $Around)
        if ($hits.Count -le $Nth) { $script:Failures.Add("$Id (anchor '$Around' not found)"); Write-Warning "anchor '$Around' not found for $Id"; return }
        $a = $hits[$Nth]
        $x = [Math]::Max(0, $a.x - $Pad[0]); $y = [Math]::Max(0, $a.y - $Pad[1])
        $Crop = @($x, $y, ($a.w + $Pad[0] + $Pad[2]), ($a.h + $Pad[1] + $Pad[3]))
    }
    if ($Crop) {
        $Crop[2] = [Math]::Min($Crop[2], $script:W - $Crop[0]); $Crop[3] = [Math]::Min($Crop[3], $script:H - $Crop[1])
    }
    $path = Join-Path $script:RawDir "$Id.png"
    Save-SimShot -Path $path -Crop $Crop | Out-Null
    Write-Host "  shot $Id" -ForegroundColor DarkGray
}

function Step {
    <# Runs a block; a failure is recorded and the run goes on with the next step. With -Only, only
       the named steps run (plus those marked -Always, which set up state the others need). #>
    param([Parameter(Mandatory)][string]$Name, [Parameter(Mandatory)][scriptblock]$Do, [switch]$Always)
    if ($Only -and -not $Always -and -not ($Only -contains $Name)) { return }
    try { & $Do } catch { $script:Failures.Add("$Name : $($_.Exception.Message)"); Write-Warning "$Name failed: $($_.Exception.Message)" }
}

function Key([string]$Keys, [int]$WaitMs = 450) { Send-SimKeys $Keys -WaitMs $WaitMs }
function TypeText([string]$Text, [int]$WaitMs = 300) {
    # SendKeys treats + ^ % ~ ( ) { } [ ] as syntax: brace them.
    $special = '+^%~(){}[]'
    $escaped = ($Text.ToCharArray() | ForEach-Object { if ($special.Contains($_)) { '{' + $_ + '}' } else { "$_" } }) -join ''
    Send-SimKeys $escaped -WaitMs $WaitMs
}
function Click([string]$Text, [int]$Nth = 0, [int[]]$Region, [int]$DX = 0, [int]$DY = 0, [int[]]$At, [switch]$Right, [switch]$Double, [int]$WaitMs = 550) {
    <# Clicks the label found by OCR; -At x,y is the fallback when OCR misses it (short words, dim text). #>
    try {
        Invoke-SimClickText -Text $Text -Nth $Nth -Region $Region -DX $DX -DY $DY -Right:$Right -Double:$Double -WaitMs $WaitMs | Out-Null
    } catch {
        if (-not $At) { throw }
        Invoke-SimClick $At[0] $At[1] -Right:$Right -Double:$Double -WaitMs $WaitMs
    }
}
function ClickAt([int]$X, [int]$Y, [switch]$Right, [int]$WaitMs = 550) { Invoke-SimClick $X $Y -Right:$Right -WaitMs $WaitMs }
function Hover([int]$X, [int]$Y, [int]$WaitMs = 700) { Move-SimMouse $X $Y -WaitMs $WaitMs }
function Scroll([int]$X, [int]$Y, [int]$Notches = -3) { Invoke-SimScroll $X $Y -Notches $Notches }
function Palette([string]$Query, [int]$WaitMs = 900) {
    Key '^k' 500; TypeText $Query 450; Key '{ENTER}' $WaitMs
}
function Skip-Welcome {
    <# Closes the first-run welcome if it is showing, and checks that it went. #>
    for ($i = 0; $i -lt 3; $i++) {
        if (-not @(Find-SimText 'Welcome to Simulacra')) { return }
        Click 'Skip for now' -WaitMs 900
    }
    if (@(Find-SimText 'Welcome to Simulacra')) { throw 'the welcome would not close' }
}

function Reset-View {
    <# Closes overlays and returns to the Timeline. #>
    Key '{ESC}' 200; Key '{ESC}' 200
    Key '^1' 600
}

if (-not $Attach) {
    # A new profile folder every run. `-Profile fresh` starts from the seeded first-run state;
    # `-Profile demo` starts from a copy of the demo database built by demo/build.ps1.
    if (-not $ProfileDir) { $ProfileDir = Join-Path ([System.IO.Path]::GetTempPath()) "simulacra-docs-$Profile-$Theme-$(Get-Date -Format yyyyMMddHHmmss)" }
    New-Item -ItemType Directory -Force -Path $ProfileDir | Out-Null
    if ($Profile -eq 'demo') {
        $demo = Join-Path $PSScriptRoot 'demo/demo.db'
        if (-not (Test-Path $demo)) { throw "No demo database: run demo/build.ps1 first ($demo)" }
        Copy-Item $demo (Join-Path $ProfileDir 'simulacra.db')
        $media = Join-Path $PSScriptRoot 'demo/media'
        if (Test-Path $media) { Copy-Item $media (Join-Path $ProfileDir 'media') -Recurse }
    }
    Get-Process simulacra -ErrorAction SilentlyContinue | Stop-Process -Force
    $userWindow = [SimWin]::GetForegroundWindow()
    $startArgs = @{ ProfileDir = $ProfileDir }
    if ($Exe) { $startArgs.Exe = $Exe }
    $proc = Start-Simulacra @startArgs
    $deadline = (Get-Date).AddSeconds(60)
    $splashSaved = -not $SplashShot
    while (-not (Get-SimulacraWindow) -or ([SimWin]::Class((Get-SimulacraWindow)) -ne 'Zed::Window')) {
        if ((Get-Date) -gt $deadline) { throw 'Simulacra did not open a window' }
        Start-Sleep -Milliseconds 50
    }
    [void](Hide-SimulacraLaunch $userWindow)
    if (-not $splashSaved) {
        # The splash is the only window for about a second: catch it while a stage label shows.
        Start-Sleep -Milliseconds 400
        Save-SimShot -Path (Join-Path $script:RawDir "$SplashShot.png") | Out-Null
        Write-Host "  shot $SplashShot" -ForegroundColor DarkGray
    }
    # The splash hands over to the maximised main window; in the background, park it at once.
    $settle = (Get-Date).AddSeconds(8)
    while ((Get-Date) -lt $settle) { [void](Hide-SimulacraLaunch $userWindow); Start-Sleep -Milliseconds 50 }
}
$h = Get-SimulacraWindow
if (-not (Test-SimBackground)) { [SimWin]::ShowWindow($h, 9) | Out-Null }
Start-Sleep -Milliseconds 300
$wr = New-Object SimWin+RECT; [SimWin]::GetWindowRect($h, [ref]$wr) | Out-Null
$cr = New-Object SimWin+RECT; [SimWin]::GetClientRect($h, [ref]$cr) | Out-Null
Set-SimulacraSize -Width ($Width + ($wr.Right - $wr.Left - $cr.Right)) -Height ($Height + ($wr.Bottom - $wr.Top - $cr.Bottom)) -X 40 -Y 30
Start-Sleep -Seconds 1

# Scenes for the fresh profile are named <area>.fresh.ps1; the rest run on the demo profile.
$order = 'shell', 'posts', 'post-tools', 'vault', 'agents', 'thinking', 'practice', 'settings'
$suffix = if ($Profile -eq 'fresh') { '.fresh' } else { '' }
$files = Get-ChildItem (Join-Path $PSScriptRoot 'scenes') -Filter '*.ps1' |
    Where-Object { ($_.BaseName -like '*.fresh') -eq ($Profile -eq 'fresh') } |
    Sort-Object { $i = $order.IndexOf(($_.BaseName -replace '\.fresh$', '')); if ($i -lt 0) { 99 } else { $i } }
if ($Scenes) { $files = $files | Where-Object { $Scenes -contains ($_.BaseName -replace '\.fresh$', '') } }
foreach ($file in $files) {
    Write-Host "scene $($file.BaseName)" -ForegroundColor Cyan
    . $file.FullName
}

Restore-SimClipboard
if (-not $KeepOpen -and $proc) { Stop-Process -Id $proc.Id -Force -ErrorAction SilentlyContinue }
if ($script:Failures.Count) {
    Write-Warning "$($script:Failures.Count) step(s) failed:"; $script:Failures | ForEach-Object { Write-Warning "  $_" }
}
