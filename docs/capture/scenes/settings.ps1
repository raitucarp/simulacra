# Settings, Appearance and Data (shots/settings.json). Client area 1440x900, demo profile (AI on
# the stub as a Custom Endpoint). The Settings dialog sits at x 180-1259, y 70-829: the category
# rail on the left (x 181-400), the content column on the right (x 402-1258) with its header
# (y 71-153), its scrolling body (y 154-774) and its footer.
#
# Order: General (window, rail, menu), search, Reading & Writing (and a changed row), AI & Models
# (provider, requests, a test credential added and removed again, unsaved provider changes thrown
# away), the other categories, Privacy & Data (and a backup), Code, Advanced, Symbols and
# Licences, then Appearance (a Nord copy is edited, then deleted, and Simulacra Midnight comes
# back), the scheduler, the title-bar tooltip, the compact layout, and the Data sheets (export,
# an encrypted snapshot saved to a temp folder and verified, a vault document's history).
#
# Tall categories are captured as one image: the body is scrolled a notch at a time and the
# captures are stitched where they overlap (see $script:SetGeom).
#
# Icon sets are not downloaded here (a download fetches a package from the npm registry), so the
# Symbols and Licences shots show the state before any download.

$script:Tmp = if ($env:SIM_CAPTURE_SCRATCH) { $env:SIM_CAPTURE_SCRATCH } else { [System.IO.Path]::GetTempPath() }
$script:Dbg = Join-Path $script:Tmp 'simulacra-docs-settings-debug'
$script:SetGeom = Join-Path $script:Tmp 'simulacra-docs-settings-geom.py'
# The snapshot file dialog saves and opens only this temp folder.
$script:SnapDir = Join-Path $script:Tmp 'simulacra-docs-settings-snap'
$script:Snap = Join-Path $script:SnapDir 'simulacra-demo.simsnap'
# A test passphrase for the throw-away snapshot (six words read Strong on the meter).
$script:Pass = 'lantern orbit pickle granite meadow quiver'
# Rail rows (x 290) by category.
$script:Rail = @{
    General = 140; Appearance = 176; Writing = 212; AI = 248; Embeddings = 284; Images = 320; Agents = 356
    Learning = 392; Import = 428; Privacy = 464; Notifications = 500; Shortcuts = 536; Code = 572
    Symbols = 608; Licences = 644; Advanced = 680
}
# Toasts stack in the top-right corner (over the dialog's header).
$script:ToastArea = 1040, 34, 1440, 230

New-Item -ItemType Directory -Force -Path $script:Tmp | Out-Null
Set-Content -Path $script:SetGeom -Encoding utf8 -Value @'
"""Capture geometry for the Settings scene.

same <a.png> <b.png>: prints 1 when the content bodies (x 402-1258, y 154-774) are identical.
stitch <out.png> <img...>: the content column of the first capture (header and body) followed by
    the body rows each later capture adds, matched where consecutive captures overlap; the
    scrollbar strip is painted over and trailing empty rows are trimmed (16 px kept).
trim <png> <pad>: cuts empty rows off the bottom of a capture, keeping <pad> px.
"""
import sys

import numpy as np
from PIL import Image

X0, X1 = 402, 1259            # content column (exclusive end)
HY0, CY0, CY1 = 71, 154, 775  # header top, body top, body end (exclusive)
BAR = 1244                    # the scrollbar strip starts here


def load(path):
    return np.asarray(Image.open(path).convert("RGB"), dtype=np.uint8)


def body(a):
    return a[CY0:CY1, X0:X1]


def same(p, q):
    print(1 if np.array_equal(body(load(p))[:, : BAR - X0], body(load(q))[:, : BAR - X0]) else 0)


def offset(a, b):
    """Rows b moved up relative to a (0 when nothing moved, -1 when they do not overlap)."""
    ka = [r.tobytes() for r in a[:, 4 : BAR - X0]]
    kb = [r.tobytes() for r in b[:, 4 : BAR - X0]]
    n = len(ka)
    best, best_d = 0.0, -1
    for d in range(0, n - 40):
        m = sum(1 for r in range(n - d) if kb[r] == ka[r + d])
        ratio = m / (n - d)
        if ratio == 1.0:
            return d
        if ratio > best:
            best, best_d = ratio, d
    return best_d if best > 0.98 else -1


def trim_rows(img, pad):
    w = min(img.shape[1], BAR - X0)
    a = img[:, :w].astype(np.int16)
    bg = a[-1, 2]
    rows = np.where((np.abs(a - bg).max(axis=2) > 12).sum(axis=1) >= 2)[0]
    end = int(rows[-1]) + 1 + pad if rows.size else img.shape[0]
    return img[: min(end, img.shape[0])]


def stitch(out, paths):
    imgs = [load(p) for p in paths]
    header = imgs[0][HY0:CY0, X0:X1]
    strip = body(imgs[0]).copy()
    prev = body(imgs[0])
    for img in imgs[1:]:
        cur = body(img)
        d = offset(prev, cur)
        if d == 0:
            break
        if d < 0:
            raise SystemExit("no overlap between consecutive captures")
        strip = np.vstack([strip, cur[cur.shape[0] - d :]])
        prev = cur
    strip = strip.copy()
    # Paint the scrollbar strip with the background of the same row.
    strip[:, BAR - X0 :] = strip[:, 6:7]
    strip = trim_rows(strip, 16)
    Image.fromarray(np.vstack([header, strip])).save(out)
    print(strip.shape[0] + header.shape[0])


def trim(path, pad):
    Image.fromarray(trim_rows(load(path), pad)).save(path)


def box(path, px, py):
    """The panel around a point in its empty padding: x y w h, border included. Thin separator
    lines (1-2 px) inside the panel are crossed; a row with nothing in it reaches the border."""
    a = load(path).astype(np.int16)
    h, w = a.shape[:2]
    bg = a[py, px]
    mask = np.abs(a - bg).max(axis=2) > 5

    def walk(y, step):
        while 0 < y + step < h - 1:
            if not mask[y + step, px]:
                y += step
            elif not mask[y + 2 * step, px]:
                y += 2 * step
            elif 0 < y + 3 * step < h and not mask[y + 3 * step, px]:
                y += 3 * step
            else:
                break
        return y + step  # the border row

    top, bottom = walk(py, -1), walk(py, 1)
    left, right = px, px
    for r in range(top + 4, bottom - 3):
        row = mask[r]
        hits = np.where(row[px + 1 :])[0]
        right = max(right, px + 1 + int(hits[0]) if hits.size else w - 1)
        hits = np.where(row[:px])[0]
        left = min(left, int(hits[-1]) if hits.size else 0)
    print(left, top, right - left + 1, bottom - top + 1)


cmd = sys.argv[1]
if cmd == "same":
    same(sys.argv[2], sys.argv[3])
elif cmd == "stitch":
    stitch(sys.argv[2], sys.argv[3:])
elif cmd == "trim":
    trim(sys.argv[2], int(sys.argv[3]))
elif cmd == "box":
    box(sys.argv[2], int(sys.argv[3]), int(sys.argv[4]))
'@

# --- helpers ------------------------------------------------------------------------------------

function Park { Hover 700 880 300 }

function Dbg([string]$Name) {
    <# A whole-window capture outside raw/, with its OCR lines, kept when a step goes wrong. #>
    New-Item -ItemType Directory -Force -Path $script:Dbg | Out-Null
    Save-SimShot -Path (Join-Path $script:Dbg "$Name.png") | Out-Null
    Get-SimText | ForEach-Object { "{0,5} {1,5} {2,4} {3,3}  {4}" -f $_.x, $_.y, $_.w, $_.h, $_.text } | Set-Content -Encoding utf8 (Join-Path $script:Dbg "$Name.txt")
}

function Temp-Shot([string]$Name) {
    $p = Join-Path $script:Tmp "sim-settings-$PID-$Name.png"
    Save-SimShot -Path $p | Out-Null
    $p
}

function Find-One([string]$Text, [int[]]$Region) {
    $hit = @(Find-SimText $Text -Region $Region)
    if (-not $hit) { Dbg ("missing-" + ($Text -replace '[^\w]', '_')); throw "'$Text' not found" }
    $hit[0]
}

function Find-Label([string]$Label, [int[]]$Region = @(402, 150, 857, 628)) {
    <# A section label (upper case) of the content column, matched case-sensitively at line start. #>
    @(Find-SimText $Label -Region $Region | Where-Object { $_.text -cmatch ('^\W{0,2}' + [regex]::Escape($Label)) -and $_.x -lt 440 })
}

function Label-Y([string]$Label) {
    $h = @(Find-Label $Label)
    if (-not $h) { Dbg ("label-" + ($Label -replace '[^\w]', '_')); throw "label '$Label' not found" }
    $h[0].y
}

function ClickWord([string]$Word, [int[]]$Region, [int]$Nth = 0, [int]$WaitMs = 700) {
    <# Clicks $Word inside the OCR line that holds it (OCR merges a row of chips or buttons). #>
    $hits = @(Find-SimText $Word -Region $Region)
    if ($hits.Count -le $Nth) { Dbg ("word-" + ($Word -replace '[^\w]', '_')); throw "Word '$Word' not found on screen" }
    $h = $hits[$Nth]
    $i = $h.text.ToLower().IndexOf($Word.ToLower())
    $x = $h.x + [int]($h.w * ($i + $Word.Length / 2) / $h.text.Length)
    ClickAt $x ($h.y + [int]($h.h / 2)) $WaitMs
}

function Wait-Text([string]$Text, [int[]]$Region, [int]$Seconds = 20, [switch]$Gone) {
    $deadline = (Get-Date).AddSeconds($Seconds)
    while ((Get-Date) -lt $deadline) {
        $hit = @(Find-SimText $Text -Region $Region)
        if ($Gone -and -not $hit) { return }
        if (-not $Gone -and $hit) { return $hit[0] }
        Start-Sleep -Milliseconds 400
    }
    Dbg ("wait-" + ($Text -replace '[^\w]', '_'))
    if ($Gone) { throw "'$Text' still showing after $Seconds s" } else { throw "'$Text' did not appear within $Seconds s" }
}

function Wait-NoToast {
    <# Toasts stack top right, over the dialog header: wait until none shows. #>
    $deadline = (Get-Date).AddSeconds(10)
    while ((Get-Date) -lt $deadline) {
        $t = @(Get-SimText | Where-Object { $_.x -ge 1040 -and $_.y -ge 40 -and $_.y -le 200 -and $_.text.Length -gt 3 })
        if (-not $t) { return }
        Start-Sleep -Milliseconds 500
    }
}

function ScrollPx([int]$X, [int]$Y, [int]$Delta) {
    <# A wheel turn of $Delta units (120 = one notch, about 260 px of the Settings body). #>
    Move-SimMouse $X $Y -WaitMs 60
    Send-SimWheel ($Delta)
    Start-Sleep -Milliseconds 500
}

function Body-Top { Scroll 900 500 25; Start-Sleep -Milliseconds 300 }
function Body-End { Scroll 900 500 -25; Start-Sleep -Milliseconds 300 }

function Bring([string]$Text, [int]$TargetY = 172, [switch]$Label, [int[]]$Region = @(402, 150, 857, 628), [int]$Max = 14) {
    <# Scrolls the Settings body from its top until $Text shows, then puts it near $TargetY. #>
    $find = { if ($Label) { @(Find-Label $Text) } else { @(Find-SimText $Text -Region $Region) } }
    Body-Top
    for ($i = 0; $i -lt $Max; $i++) {
        $hit = & $find
        if ($hit) {
            $d = $hit[0].y - $TargetY
            if ([Math]::Abs($d) -gt 6) { ScrollPx 900 500 ([int](-$d * 120 / 260)); Start-Sleep -Milliseconds 300 }
            $hit = & $find
            if ($hit) { return $hit[0] }
        }
        Scroll 900 500 -1
        Start-Sleep -Milliseconds 300
    }
    Dbg ("bring-" + ($Text -replace '[^\w]', '_'))
    throw "could not bring '$Text' into view"
}

function Open-Settings([string]$Cat) {
    if (-not @(Find-SimText 'Search settings')) { Key '{ESC}' 200; Key '^,' 1400 }
    if (-not @(Find-SimText 'Search settings')) { Dbg 'settings-closed'; throw 'Settings did not open' }
    ClickAt 290 $script:Rail[$Cat] 1100
}

function Leave-Settings {
    <# Closes Settings when it is open (steps outside Settings start with this). #>
    if (@(Find-SimText 'Search settings')) { Close-Settings }
}

function Close-Settings {
    <# Esc can stop at a focused field: the dialog's X closes it for sure. #>
    Key '{ESC}' 500
    if (@(Find-SimText 'Search settings')) { ClickAt 1226 112 800 }
    if (@(Find-SimText 'Search settings')) { throw 'Settings would not close' }
}

function Shot-Pane([string]$Id) {
    <# The content column, header and whole body, stitched from captures a notch apart. #>
    Body-Top
    Park
    $files = @(Temp-Shot "pane-0")
    for ($i = 1; $i -lt 12; $i++) {
        Scroll 900 500 -1
        Park
        $p = Temp-Shot "pane-$i"
        if ((python $script:SetGeom same $files[-1] $p) -eq '1') { break }
        $files += $p
    }
    $out = Join-Path $script:RawDir "$Id.png"
    $h = python $script:SetGeom stitch $out @files
    if ($LASTEXITCODE -ne 0) { throw "stitching $Id failed" }
    Write-Host "  shot $Id (stitched $($files.Count), $h px)" -ForegroundColor DarkGray
    Get-ChildItem $script:Tmp -Filter "sim-settings-$PID-pane-*.png" | Remove-Item -ErrorAction SilentlyContinue
    Body-Top
}

function Shot-Rows([string]$Id, [int]$Y0, [int]$Y1, [int]$X0 = 409, [int]$X1 = 1251) {
    <# A slice of the content column (x $X0-$X1, y $Y0-$Y1). #>
    Shot $Id -Crop $X0, $Y0, ($X1 - $X0), ($Y1 - $Y0)
}

function Shot-Trimmed([string]$Id, [int[]]$Crop, [int]$Pad = 12) {
    Shot $Id -Crop $Crop
    python $script:SetGeom trim (Join-Path $script:RawDir "$Id.png") $Pad
}

function Shot-Sheet([string]$Id) {
    <# A sheet opened since the last Mark, cropped to its own edges (the scrim diff runs a few px
       wide): the rough box from the diff, then the panel around a point in its left padding. #>
    Shot 'tmp-settings-sheet' -Auto -AutoPad 0
    Remove-Item (Join-Path $script:RawDir 'tmp-settings-sheet.png') -ErrorAction SilentlyContinue
    $r = $script:LastBox
    $b = [int[]]((python $script:SetGeom box (Temp-Shot 'sheet') ($r[0] + 10) ($r[1] + [int]($r[3] / 2))) -split ' ')
    if ($b[2] -lt 200 -or $b[3] -lt 100) { Dbg "sheet-$Id"; throw "the sheet edges of $Id were not found ($b)" }
    Shot $Id -Crop $b
    $script:LastBox = $b
}

function Send-FileDialog([string]$Path) {
    <# Puts a full path into the file name box of the dialog Simulacra just opened, and confirms. #>
    Send-SimFileDialog $Path
    $proc = Get-Process simulacra | Select-Object -First 1
    if ([SimWin]::Windows($proc.Id) | Where-Object { [SimWin]::Class($_) -eq '#32770' }) { throw 'the file dialog is still open' }
}

# --- General, search ----------------------------------------------------------------------------

Step 'welcome' -Always {
    New-Item -ItemType Directory -Force -Path $script:Dbg | Out-Null
    Skip-Welcome
}

Step 'general' {
    Open-Settings 'General'
    Park
    Shot 'settings-window'
    Shot 'settings-rail' -Crop 180, 70, 222, 760
    Shot-Pane 'settings-general'
    # The Startup screen menu (its button is the second row of STARTUP).
    $s = Find-One 'Startup screen' @(420, 560, 300, 120)
    Mark
    ClickAt 1184 ($s.y + 14) 900
    Park
    Shot 'tmp-settings-menu' -Auto -AutoPad 10 -Within 1040, ($s.y - 6), 1259, ($s.y + 170)
    Remove-Item (Join-Path $script:RawDir 'tmp-settings-menu.png') -ErrorAction SilentlyContinue
    # The menu and the button it hangs from.
    $mb = $script:LastBox
    $y0 = $s.y - 8
    Shot 'settings-control-menu' -Crop ($mb[0] - 14), $y0, (1250 - ($mb[0] - 14)), ($mb[1] + $mb[3] + 4 - $y0)
    Key '{ESC}' 500
    if (@(Find-SimText 'Knowledge Vault' -Region 1080, ($s.y + 30), 180, 140)) { Key '{ESC}' 500 }
}

Step 'search' {
    Open-Settings 'General'
    ClickAt 300 95 400
    TypeText 'key' 1000
    Park
    Shot 'settings-search'
    Key '^a' 150; Key '{BACKSPACE}' 600
    Close-Settings
}

# --- Reading & Writing --------------------------------------------------------------------------

Step 'reading writing' {
    Open-Settings 'Writing'
    Shot-Pane 'settings-reading-writing'
    # Autosave 30 s -> 45 s (three clicks of +), its reset arrow, then the arrow puts it back.
    $a = Bring 'Autosave drafts' 260
    ClickAt 1216 ($a.y + 10) 400; ClickAt 1216 ($a.y + 10) 400; ClickAt 1216 ($a.y + 10) 500
    Park
    if (-not @(Find-SimText '45 s' -Region 1100, ($a.y - 10), 150, 40)) { Dbg 'autosave'; throw 'autosave did not reach 45 s' }
    Shot 'settings-row-modified' -Crop 418, ($a.y - 15), 825, 58
    ClickAt 1081 ($a.y + 14) 700
    Park
    if (-not @(Find-SimText '30' -Region 1100, ($a.y - 10), 150, 40)) { Dbg 'autosave-reset'; throw 'autosave was not reset to 30 s' }
    Close-Settings
}

# --- AI & Models --------------------------------------------------------------------------------

Step 'ai' {
    Open-Settings 'AI'
    Body-Top
    Park
    $p = Find-One 'Presets' @(402, 400, 300, 300)
    Shot 'settings-ai-provider' -Crop 402, 71, 857, ($p.y + 28 - 71)
    Body-End
    Park
    $r = Label-Y 'REQUESTS'
    Shot-Trimmed 'settings-ai-requests-budget' @(409, ($r - 14), 842, (775 - ($r - 14))) 10
}

Step 'credentials' {
    # A test credential (a fake key) in this throw-away profile, removed again below.
    Open-Settings 'AI'
    $c = Bring 'CREDENTIALS' 200 -Label
    Click 'Add credential' -Region 1100, ($c.y - 10), 159, 30 -At 1182, ($c.y + 5) -WaitMs 1000
    # The form's Name label (OCR misses the Service label above the chips).
    $n = @(Find-SimText 'Name' -Region 420, $c.y, 200, 400 | Where-Object { $_.text -cmatch '^Name$' })[0]
    if (-not $n) { Dbg 'credential-form'; throw 'the credential form did not open' }
    ClickWord 'OpenA' -Region 420, ($n.y - 40), 820, 30 -WaitMs 600
    ClickAt 830 ($n.y + 32) 300
    TypeText 'Personal OpenAI key' 500
    Park
    # Save credential (white on violet) escapes OCR: it sits right of Cancel.
    $cn = Find-One 'Cancel' @(1000, ($c.y + 250), 259, 300)
    Shot 'settings-credentials-editor' -Crop 409, ($c.y - 14), 842, ($cn.y + 30 - ($c.y - 14))
    $k = Find-One 'API key' @(420, ($n.y + 40), 200, 80)
    ClickAt 830 ($k.y + 32) 300
    TypeText 'test-0000' 400
    ClickAt ($cn.x + 113) ($cn.y + 6) 1200
    Wait-NoToast
    $c = Bring 'CREDENTIALS' 200 -Label
    Park
    if (-not @(Find-SimText 'Personal Open' -Region 420, $c.y, 400, 300)) { Dbg 'credential-list'; throw 'the test credential was not saved' }
    $r = Label-Y 'REQUESTS'
    Shot 'settings-credentials-list' -Crop 409, ($c.y - 14), 842, ($r - 16 - ($c.y - 14))
    # Remove it again (the bin icon of its row).
    $row = Find-One 'Personal Open' @(420, $c.y, 400, 300)
    ClickAt 1219 ($row.y + 10) 1000
    if (@(Find-SimText 'Personal Open' -Region 420, $c.y, 400, 300)) { Dbg 'credential-remove'; throw 'the test credential was not removed' }
    Wait-NoToast
    Close-Settings
}

Step 'unsaved' {
    Open-Settings 'AI'
    Body-Top
    Click 'OpenAI' -Region 700, 195, 260, 40 -At 840, 216 -WaitMs 900
    Park
    if (-not @(Find-SimText 'Unsaved' -Region 402, 776, 400, 50)) { Dbg 'unsaved'; throw 'no unsaved marker' }
    Shot 'settings-footer-unsaved' -Crop 402, 776, 857, 53
    # Closing throws the provider change away: the stub engine stays active.
    Close-Settings
    Open-Settings 'AI'
    Body-Top
    if (-not @(Find-SimText 'Active: Custom' -Region 1000, 160, 259, 40)) { Dbg 'provider-kept'; throw 'the provider change was not discarded' }
    Close-Settings
}

# --- the other categories -----------------------------------------------------------------------

Step 'panes' {
    foreach ($p in @(
            @('Embeddings', 'settings-embeddings'), @('Images', 'settings-images'), @('Agents', 'settings-agents'),
            @('Learning', 'settings-learning'), @('Import', 'settings-import'), @('Notifications', 'settings-notifications'))) {
        Open-Settings $p[0]
        Shot-Pane $p[1]
    }
    Close-Settings
}

Step 'privacy' {
    Open-Settings 'Privacy'
    Shot-Pane 'settings-privacy-data'
    Body-Top
    Park
    $s = Label-Y 'STORAGE'
    $m = Label-Y 'MAINTENANCE'
    Shot 'data-storage-card' -Crop 409, ($s - 14), 842, ($m - 16 - ($s - 14))
    Body-End
    Park
    $t = Find-One 'Telemetry' @(420, 300, 300, 470)
    Shot 'settings-control-locked' -Crop 418, ($t.y - 15), 825, 58
    # A backup of the throw-away profile, and its toast.
    Body-Top
    $b = Find-One 'Back up now' @(1080, 300, 179, 200)
    Park
    Mark
    ClickAt ($b.x + 40) ($b.y + 6) 300
    Hover 700 880 100
    Shot 'data-backup-toast' -Auto -AutoPad 8 -Within $script:ToastArea -SettleMs 600
    Wait-NoToast
    Close-Settings
}

Step 'code' {
    Open-Settings 'Code'
    $r = Find-One 'Refresh' @(1100, 380, 159, 100)
    ClickAt ($r.x + 20) ($r.y + 6) 1500
    Wait-Text 'Checked' -Region 420, 380, 400, 100 -Seconds 30 | Out-Null
    Start-Sleep -Seconds 1
    Shot-Pane 'settings-code'
    Close-Settings
}

Step 'advanced' {
    Open-Settings 'Advanced'
    $r = Find-One 'Reset all' @(1000, 300, 259, 200)
    ClickAt ($r.x + 30) ($r.y + 6) 700
    if (-not @(Find-SimText 'Click again' -Region 1000, 300, 259, 200)) { Dbg 'reset-armed'; throw 'the reset did not arm' }
    Shot-Pane 'settings-advanced'
    # Closing Settings disarms it; the second click never happens.
    Close-Settings
}

Step 'symbols' {
    Open-Settings 'Symbols'
    Body-Top
    Park
    $s10 = Find-One 'set 10' @(420, 600, 300, 175)
    Shot 'appearance-icon-sets' -Crop 402, 71, 857, ($s10.y + 40 - 71)
    Shot 'appearance-icon-download' -Crop 418, ($s10.y - 14), 825, 52
    Open-Settings 'Licences'
    Park
    $n = Find-One 'No icon set' @(420, 150, 500, 200)
    Shot 'appearance-licences' -Crop 402, 71, 857, ($n.y + 30 - 71)
    Close-Settings
}

# --- Appearance ---------------------------------------------------------------------------------

Step 'appearance' {
    Open-Settings 'Appearance'
    Body-Top
    Park
    Shot 'appearance-pane'
    $rc = Label-Y 'READING COMFORT'
    $am = Label-Y 'APPEARANCE MODE'
    $tg = Label-Y 'THEME GALLERY'
    Shot-Rows 'appearance-reading-comfort' ($rc - 14) ($am - 16)
    Shot-Rows 'appearance-mode' ($am - 14) ($tg - 16)
    # The gallery: its heading near the top, two rows of cards under it.
    $g = Bring 'THEME GALLERY' 172 -Label
    Park
    $bt = Label-Y 'BUNDLED THEMES'
    # Two rows of cards (the cards share each row, 197 x 165 px at this width).
    Shot-Rows 'appearance-gallery' ($g.y - 14) ($bt + 375)
    $mid = @(Find-SimText 'Simulacra Midnight' -Region 402, ($bt + 20), 200, 400)[0]
    if (-not $mid) { Dbg 'midnight-card'; throw 'the Simulacra Midnight card was not found' }
    Shot 'appearance-theme-card' -Crop 410, ($mid.y - 133), 211, 179
    # CUSTOM THEME on a bundled theme.
    $c = Bring 'CUSTOM THEME' 172 -Label
    Park
    $ty = Label-Y 'TYPOGRAPHY & LAYOUT'
    Shot-Rows 'appearance-editor-locked' ($c.y - 14) ($ty - 16)
    # The end of the pane: TYPOGRAPHY & LAYOUT and IMPORT & EXPORT.
    Body-End
    Park
    $ty = Label-Y 'TYPOGRAPHY & LAYOUT'
    $ie = Label-Y 'IMPORT & EXPORT'
    Shot-Rows 'appearance-typography' ($ty - 14) ($ie - 16)
    Shot-Trimmed 'appearance-transfer' @(409, ($ie - 14), 842, (775 - ($ie - 14))) 10
}

function Restore-Midnight {
    <# Deletes a custom theme being edited and makes Simulacra Midnight the theme again. #>
    Open-Settings 'Appearance'
    $c = Bring 'CUSTOM THEME' 172 -Label
    if (@(Find-SimText 'Editing' -Region 420, $c.y, 400, 80)) {
        $x = Find-One 'Delete' @(1100, ($c.y + 20), 159, 60)
        ClickAt ($x.x + 15) ($x.y + 6) 1200
        Wait-NoToast
    }
    $m = Bring 'Simulacra Midnight' 450 -Region 402, 150, 600, 628
    ClickAt ($m.x + 40) ($m.y - 60) 1500
    Body-Top
    if (-not @(Find-SimText 'Simulacra Midnight' -Region 1000, 440, 259, 40)) { Dbg 'theme-back'; throw 'Simulacra Midnight is not the dark theme again' }
    Close-Settings
}

Step 'custom theme' {
    try {
        Open-Settings 'Appearance'
        # Nord, then a copy of it to edit.
        $n = Bring 'Nord' 520 -Region 402, 150, 760, 628
        ClickAt ($n.x + 40) ($n.y - 60) 1500
        $c = Bring 'CUSTOM THEME' 172 -Label
        $d = Find-One 'Duplicate' @(900, ($c.y + 20), 359, 60)
        ClickAt ($d.x + 20) ($d.y + 6) 1500
        Wait-NoToast
        $c = Bring 'CUSTOM THEME' 172 -Label
        Park
        if (-not @(Find-SimText 'Editing Nord' -Region 420, $c.y, 400, 80)) { Dbg 'nord-copy'; throw 'the Nord copy is not being edited' }
        $ty = Label-Y 'TYPOGRAPHY & LAYOUT'
        Shot-Rows 'appearance-editor-custom' ($c.y - 14) ($ty - 16)
        # A dark muted text (#4c566a, close to the background): the muted-text pairs fail the
        # contrast check. (A pale accent does not: the label on it turns dark by itself.)
        # The colour grid has two even columns: Muted text heads the right one's third row, its
        # swatch 21 px left of the label.
        $mt = Find-One 'Muted text' @(720, $c.y, 400, 300)
        $sx = $mt.x - 21
        ClickAt $sx ($mt.y + 6) 1200
        Hover ($sx + 160) ($mt.y + 45) 600
        $hex = @(Find-SimText '#' -Region ($sx + 20), ($mt.y + 300), 280, 120)
        if (-not $hex) { Dbg 'picker'; throw 'the picker hex field was not found' }
        ClickAt ($hex[0].x + 60) ($hex[0].y + 6) 300
        Key '^a' 150
        TypeText '#4c566a' 300
        Key '{ENTER}' 900
        # Enter commits the colour; a click on the header closes the picker if it is still open
        # (Esc would close Settings when it is not).
        if (@(Find-SimText 'HSLA')) { ClickAt 700 128 600 }
        $c = Bring 'CUSTOM THEME' 172 -Label
        Park
        $w = Label-Y 'CONTRAST (WCAG'
        $ty = Label-Y 'TYPOGRAPHY & LAYOUT'
        if (-not @(Find-SimText 'Fail' -Region 1100, $w, 159, ($ty - $w))) { Dbg 'contrast'; throw 'no Fail badge' }
        Shot 'appearance-contrast' -Crop 418, ($w - 12), 825, ($ty - 18 - ($w - 12))
    } finally {
        # Delete the copy and go back to Simulacra Midnight, whatever happened above.
        Restore-Midnight
    }
}

Step 'scheduler' {
    Open-Settings 'Agents'
    Body-Top
    # The pane's subtitle also starts with "Autonomous posting": take the row below AUTONOMY.
    $a = Find-One 'Autonomous posting' @(420, 195, 400, 50)
    ClickAt 1211 ($a.y + 8) 2500
    Park
    # The queue line ends the status block; its pill (OCR garbles it) is 68 px above.
    $q = Find-One 'generating' @(420, ($a.y + 80), 400, 120)
    Shot 'settings-agents-scheduler-status' -Crop 418, ($q.y - 82), 825, 104
    ClickAt 1211 ($a.y + 8) 900
    if (-not @(Find-SimText 'Scheduler Off' -Region 420, ($a.y + 40), 300, 40)) { Dbg 'scheduler-off'; throw 'autonomous posting did not switch off again' }
    Close-Settings
}

Step 'title bar toggle' {
    Leave-Settings
    Key '{ESC}' 300
    Key '^1' 700
    Park
    Mark
    Hover 1268 18 1500
    Shot 'tmp-settings-tooltip' -Auto -AutoPad 6 -Within 1100, 0, 1440, 120
    Remove-Item (Join-Path $script:RawDir 'tmp-settings-tooltip.png') -ErrorAction SilentlyContinue
    # The tooltip and the moon button above it.
    $tb = $script:LastBox
    $x0 = [Math]::Min($tb[0], 1240) - 4
    $x1 = [Math]::Max($tb[0] + $tb[2] - 4, 1296)
    Shot 'appearance-title-bar-toggle' -Crop $x0, 0, ($x1 - $x0), ($tb[1] + $tb[3] + 6)
    Park
}

Step 'compact' {
    Leave-Settings
    Set-SimulacraSize -Width (860 + 16) -Height (900 + 8) -X 40 -Y 30
    $script:W = 860
    Start-Sleep -Milliseconds 900
    try {
        Key '{ESC}' 200
        Key '^,' 1400
        ClickAt 52 96 900
        Hover 430 880 300
        Shot 'settings-compact'
        Close-Settings
    } finally {
        Set-SimulacraSize -Width (1440 + 16) -Height (900 + 8) -X 40 -Y 30
        $script:W = 1440
        Start-Sleep -Milliseconds 900
    }
}

# --- Data ---------------------------------------------------------------------------------------

Step 'export' {
    Leave-Settings
    Key '{ESC}' 300
    Key '^1' 700
    Park
    Mark
    Palette 'export bundle' 1200
    Park
    Shot-Sheet 'data-export-sheet'
    $b = $script:LastBox
    $w = Find-One 'WHAT TO EXPORT' @($b[0], $b[1], $b[2], 200)
    Click 'Date range' -Region ($b[0] + 300), ($w.y + 20), ($b[2] - 300), 40 -At ($b[0] + 400), ($w.y + 36) -WaitMs 800
    Park
    $al = Find-One 'ALSO INCLUDE' @($b[0], $w.y, $b[2], 300)
    Shot 'data-export-date-range' -Crop $b[0], ($w.y - 12), $b[2], ($al.y - 10 - ($w.y - 12))
    ClickWord 'Everything' -Region $b[0], ($w.y + 20), $b[2], 40 -WaitMs 800
    $al = Find-One 'ALSO INCLUDE' @($b[0], $w.y, $b[2], 300)
    ClickWord 'private jo' -Region $b[0], $al.y, $b[2], 100 -WaitMs 800
    Park
    # Case-sensitive: the journal warning also says "will be written".
    $wb = @(Find-SimText 'WILL BE WRITTEN' -Region $b[0], $al.y, $b[2], 300 | Where-Object { $_.text -cmatch 'WILL BE WRITTEN' })[0]
    if (-not $wb) { Dbg 'export-written'; throw 'WILL BE WRITTEN not found' }
    Shot 'data-export-journal-warning' -Crop $b[0], ($al.y - 12), $b[2], ($wb.y - 10 - ($al.y - 12))
    Key '{ESC}' 700
    if (@(Find-SimText 'WILL BE WRITTEN')) { Click 'Cancel' -WaitMs 700 }
}

Step 'snapshot' {
    Leave-Settings
    New-Item -ItemType Directory -Force -Path $script:SnapDir | Out-Null
    Remove-Item $script:Snap -ErrorAction SilentlyContinue
    Key '{ESC}' 300
    Park
    Mark
    Palette 'encrypted snapshot' 1200
    TypeText $script:Pass 600
    $t = Find-One 'Type it again'
    ClickAt 700 ($t.y + 30) 400
    TypeText $script:Pass 900
    Park
    if (-not @(Find-SimText 'Strong')) { Dbg 'snapshot-meter'; throw 'the meter does not read Strong' }
    Shot-Sheet 'data-snapshot-sheet'
    $b = $script:LastBox
    # Create snapshot… is the last button of the sheet; save into the temp folder.
    ClickAt ($b[0] + $b[2] - 75) ($b[1] + $b[3] - 21) 600
    Send-FileDialog $script:Snap
    Wait-Text 'snapshot saved' -Seconds 30 | Out-Null
    if (-not (Test-Path $script:Snap)) { throw 'the snapshot file was not written' }
    Wait-NoToast
}

Step 'restore' {
    Leave-Settings
    if (-not (Test-Path $script:Snap)) { throw 'no snapshot to restore (run the snapshot step)' }
    Key '{ESC}' 300
    Park
    Mark
    Palette 'restore snapshot' 1200
    $f = Find-One 'SNAPSHOT FILE'
    ClickAt 480 ($f.y + 32) 600
    Send-FileDialog $script:Snap
    Park
    if (-not @(Find-SimText 'simsnap')) { Dbg 'restore-file'; throw 'the snapshot file name is not shown' }
    Shot-Sheet 'data-restore-choose'
    $p = Find-One 'Passphrase of the snapshot'
    ClickAt 700 ($p.y + 30) 400
    TypeText $script:Pass 700
    $v = Find-One 'Verifying decrypts'
    ClickAt 981 ($v.y + 41) 1500
    Wait-Text 'Verified' -Seconds 30 | Out-Null
    Park
    Shot-Sheet 'data-restore-verified'
    # Cancel (never Restore and restart); Esc does not close the sheet at this point.
    $b = $script:LastBox
    $cn = Find-One 'Cancel' @($b[0], ($b[1] + $b[3] - 50), ($b[2] - 200), 50)
    ClickAt ($cn.x + 20) ($cn.y + 6) 900
    if (@(Find-SimText 'Restore and restart')) { Dbg 'restore-open'; throw 'the restore sheet is still open' }
    Remove-Item $script:Snap -ErrorAction SilentlyContinue
}

Step 'history' {
    Leave-Settings
    # The Tractatus edited twice (its title, then a sentence added to its text).
    Key '{ESC}' 200
    Key '^2' 1200
    for ($i = 0; $i -lt 3; $i++) {
        $bk = @(Find-SimText 'Back' -Region 241, 80, 500, 60)
        if (-not $bk) { break }
        ClickAt ($bk[0].x + 12) ($bk[0].y + 6) 900
    }
    ClickAt 695 300 1200
    # Edit follows "Continue reading (35%)" (OCR sometimes misses it).
    Click 'Edit' -Region 400, 280, 200, 60 -At 482, 310 -WaitMs 1200
    if (-not @(Find-SimText 'Edit document')) { Dbg 'vault-edit'; throw 'the edit form did not open' }
    ClickAt 1150 226 300
    Key '{END}' 150
    TypeText ' (Ogden translation)' 400
    Scroll 1330 600 -10
    Click 'Save' -Region 1100, 780, 200, 120 -At 1188, 872 -WaitMs 1500
    # Edit follows "Continue reading (35%)" (OCR sometimes misses it).
    Click 'Edit' -Region 400, 280, 200, 60 -At 482, 310 -WaitMs 1200
    if (-not @(Find-SimText 'Edit document')) { Dbg 'vault-edit'; throw 'the edit form did not open' }
    # A new first paragraph, so the word diff shows it at the top.
    ClickAt 840 700 300
    Key '^{HOME}' 200
    TypeText 'Whereof one cannot speak, thereof one must be silent.' 300
    Key '{ENTER}' 150; Key '{ENTER}' 300
    Scroll 1330 600 -10
    Click 'Save' -Region 1100, 780, 200, 120 -At 1188, 872 -WaitMs 1500
    Wait-NoToast
    Park
    Mark
    $h = Find-One 'History' @(241, 280, 900, 60)
    ClickAt ($h.x + 20) ($h.y + 6) 1200
    $o = Find-One 'Original' @(410, 150, 270, 300)
    ClickAt ($o.x + 40) ($o.y + 6) 1000
    Hover 1300 880 300
    Shot-Sheet 'data-history-sheet'
    Key '{ESC}' 600
    Key '^1' 700
}
