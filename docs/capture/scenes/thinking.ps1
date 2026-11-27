# Insights, the Mind Graph and the thinking tools (shots/thinking.json). Client area 1440x900,
# demo profile (Epistemology & Mind active, AI and embeddings on the stub; demo/seed.py gives
# Sokrates of Alopece a few beliefs with their history, as the stub cannot form any).
#
# Order: the Mind Graph and every Insights tab on the demo posts; then the tutorial "Map an
# argument" is played through (the atomic note, the claim with its 65% credence, the argument
# map, the counterpoint and the tensions found (this build does not pair the two; the whole
# stream's pairs are shown), the resolution and its calibration); then nine more
# credences are set and resolved for the full Calibration tab; then the Beliefs tabs; last an
# empty network ("Empty Lab", made if missing and left in place) for the empty states.

# --- helpers ----------------------------------------------------------------------------------

$script:Dbg = Join-Path ([System.IO.Path]::GetTempPath()) 'simulacra-docs-thinking-debug'
$script:TkGeom = Join-Path ([System.IO.Path]::GetTempPath()) 'simulacra-docs-thinking-geom.py'
$script:Feed = 241, 80, 519, 820            # the Timeline's stream column (x, y, w, h)
$script:Insp = 761, 38, 679, 862            # the Thread Inspector
$script:Claim = 'Predicting the next word is not the same as understanding it. A mirror reproduces the sun without feeling its warmth. #Epistemology'
$script:Counter = 'Predicting the next word well is understanding it. #Epistemology'
$script:Atomic = 'This changes everything. Studies show 70% of notes are never reread, and most links rot within a single year.'
$script:Authors = '^(Arya Wiratama|Sokrates of Alop|Simone Weil|Baruch de Spinoza|Senja Prakasa)'
# Insights tabs: twelve equal slots across the tab row (y 140 with the page at its top).
$script:Tabs = @{ Overview = 303; Network = 390; Sets = 462; Symmetry = 540; Categories = 632; Embedding = 741
    Activity = 843; Calibration = 930; Tensions = 1020; Timeline = 1125; Diversity = 1257; Readability = 1374 }

Set-Content -Path $script:TkGeom -Encoding utf8 -Value @'
"""Geometry read from a capture. Prints numbers, one result per line.
box <png> <px> <py> [tol]: the card around (px, py), a point in its empty padding.
bbox <png> <x0> <y0> <x1> <y1> [tol]: the bounding box of what differs from the region's edge colour.
bottom <png> <x0> <y0> <x1> <y1>: the row under the last ink in a region.
colour <png> <x0> <y0> <x1> <y1> <r> <g> <b> [minarea]: filled blobs of one colour, "x y area", largest first.
dots <png> <x0> <y0> <x1> <y1> [minarea]: filled blobs of any strong colour (graph nodes), largest first.
dense <png> <x0> <y0> <x1> <y1> [radius]: the dot with the most dots around it.
peak <png> <x0> <y0> <x1> <y1>: the centre of the strongest purple patch (a heatmap's busiest cell).
"""
import sys
from collections import deque
import numpy as np
from PIL import Image

a = np.asarray(Image.open(sys.argv[2]).convert("RGB"), dtype=np.int16)
H, W = a.shape[:2]

def differs(p, q, tol):
    return np.abs(p - q).max() > tol

def box(px, py, tol=5):
    bg = a[py, px]
    top = py
    while top > 0 and not differs(a[top - 1, px], bg, tol):
        top -= 1
    bottom = py
    while bottom < H - 1 and not differs(a[bottom + 1, px], bg, tol):
        bottom += 1
    left, right = px, px
    for row in {min(top + 6, bottom), min(top + 9, bottom), max(bottom - 9, top), max(bottom - 6, top)}:
        x = px
        while x > 0 and not differs(a[row, x - 1], bg, tol):
            x -= 1
        left = min(left, x)
        x = px
        while x < W - 1 and not differs(a[row, x + 1], bg, tol):
            x += 1
        right = max(right, x)
    print(left - 1, top - 1, right - left + 3, bottom - top + 3)

def bbox(x0, y0, x1, y1, tol=10):
    r = a[y0:y1, x0:x1]
    edge = np.concatenate([r[0], r[-1], r[:, 0], r[:, -1]])
    bg = np.median(edge, axis=0)
    mask = np.abs(r - bg).max(axis=2) > tol
    ys, xs = np.nonzero(mask)
    if not len(xs):
        print(x0, y0, x1 - x0, y1 - y0)
        return
    print(x0 + xs.min(), y0 + ys.min(), xs.max() - xs.min() + 1, ys.max() - ys.min() + 1)

def bottom(x0, y0, x1, y1):
    r = a[y0:y1, x0:x1]
    bg = np.median(r.reshape(-1, 3), axis=0)
    rows = np.where((np.abs(r - bg).max(axis=2) > 24).sum(axis=1) >= 2)[0]
    print(y0 + int(rows[-1]) + 1 if rows.size else y0)

def erode(m):
    e = m.copy()
    e[1:, :] &= m[:-1, :]
    e[:-1, :] &= m[1:, :]
    e[:, 1:] &= m[:, :-1]
    e[:, :-1] &= m[:, 1:]
    return e

def components(mask):
    seen = np.zeros_like(mask, dtype=bool)
    out = []
    for y, x in zip(*np.nonzero(mask)):
        if seen[y, x]:
            continue
        q = deque([(y, x)])
        seen[y, x] = True
        pts = []
        while q:
            cy, cx = q.popleft()
            pts.append((cy, cx))
            for ny, nx in ((cy + 1, cx), (cy - 1, cx), (cy, cx + 1), (cy, cx - 1)):
                if 0 <= ny < mask.shape[0] and 0 <= nx < mask.shape[1] and mask[ny, nx] and not seen[ny, nx]:
                    seen[ny, nx] = True
                    q.append((ny, nx))
        p = np.array(pts)
        out.append((int(p[:, 1].mean()), int(p[:, 0].mean()), len(pts)))
    return out

def blobs(mask, x0, y0, minarea):
    found = [(x + x0, y + y0, n) for x, y, n in components(erode(mask)) if n >= minarea]
    return sorted(found, key=lambda c: -c[2])

def colour(x0, y0, x1, y1, r, g, b, minarea=12):
    reg = a[y0:y1, x0:x1]
    for c in blobs(np.abs(reg - np.array([r, g, b])).max(axis=2) <= 14, x0, y0, minarea):
        print(*c)

def strong(x0, y0, x1, y1):
    reg = a[y0:y1, x0:x1]
    return ((reg.max(axis=2) - reg.min(axis=2)) > 50) & (reg.max(axis=2) > 100)

def dots(x0, y0, x1, y1, minarea=6):
    for c in blobs(strong(x0, y0, x1, y1), x0, y0, minarea):
        print(*c)

def dense(x0, y0, x1, y1, radius=70):
    pts = blobs(strong(x0, y0, x1, y1), x0, y0, 4)
    best = max(pts, key=lambda p: sum(1 for q in pts if (p[0] - q[0]) ** 2 + (p[1] - q[1]) ** 2 <= radius * radius))
    print(best[0], best[1])

def peak(x0, y0, x1, y1):
    reg = a[y0:y1, x0:x1]
    r, g, b = reg[..., 0], reg[..., 1], reg[..., 2]
    score = np.where((b > g + 40) & (r > g), r + b, 0)
    m = score >= score.max() - 10
    c = sorted(components(m), key=lambda c: -c[2])[0]
    print(c[0] + x0, c[1] + y0)

def popover(px, py, left, top, right, bottom, *out):
    """The card around (px, py), cut from this same capture with extra margins into argv[-1]."""
    import io, contextlib
    buf = io.StringIO()
    with contextlib.redirect_stdout(buf):
        box(px, py)
    x, y, w, h = (int(v) for v in buf.getvalue().split())
    x0, y0 = max(0, x - left), max(0, y - top)
    x1, y1 = min(W, x + w + right), min(H, y + h + bottom)
    Image.open(sys.argv[2]).crop((x0, y0, x1, y1)).save(OUT)
    print(x0, y0, x1 - x0, y1 - y0)

mode = sys.argv[1]
OUT = sys.argv[-1] if mode == "popover" else None
n = [int(v) for v in (sys.argv[3:-1] if mode == "popover" else sys.argv[3:])]
{"popover": popover, "box": box, "bbox": bbox, "bottom": bottom, "colour": colour, "dots": dots, "dense": dense, "peak": peak}[mode](*n)
'@

function Park { Hover 900 20 300 }

function Save-Temp {
    $p = Join-Path ([System.IO.Path]::GetTempPath()) "sim-thinking-$PID.png"
    Save-SimShot -Path $p | Out-Null
    $p
}

function Dbg([string]$Name) {
    <# A whole-window capture outside raw/, with its OCR lines, kept when a step goes wrong. #>
    Save-SimShot -Path (Join-Path $script:Dbg "$Name.png") | Out-Null
    Get-SimText | ForEach-Object { "{0,5} {1,5} {2,4} {3,3}  {4}" -f $_.x, $_.y, $_.w, $_.h, $_.text } | Set-Content -Encoding utf8 (Join-Path $script:Dbg "$Name.txt")
}

function Geo([string]$Mode, [int[]]$Numbers) {
    <# Runs one geometry query on a fresh capture; returns its output lines (always an array,
       so (Geo ...)[0] is the first line, not the first character of a lone line). #>
    $out = @(python $script:TkGeom $Mode (Save-Temp) @Numbers)
    return , $out
}

function Nums([string]$Line) { [int[]]($Line -split ' ') }

function Find-One([string]$Text, [int[]]$Region) {
    $hit = @(Find-SimText $Text -Region $Region)
    if (-not $hit) { Dbg ("missing-" + ($Text -replace '[^\w]', '_')); throw "'$Text' not found" }
    $hit[0]
}

function Wait-Text([string]$Text, [int[]]$Region, [int]$Seconds = 25, [switch]$Gone) {
    $deadline = (Get-Date).AddSeconds($Seconds)
    while ((Get-Date) -lt $deadline) {
        $hit = @(Find-SimText $Text -Region $Region)
        if ($Gone -and -not $hit) { return }
        if (-not $Gone -and $hit) { return $hit[0] }
        Start-Sleep -Milliseconds 600
    }
    if ($Gone) { throw "'$Text' still showing after $Seconds s" } else { throw "'$Text' did not appear within $Seconds s" }
}

function ClickWord([string]$Word, [int[]]$Region, [int]$Nth = 0, [int]$WaitMs = 700, [switch]$Case) {
    <# Clicks $Word inside the OCR line that holds it (OCR merges a row of chips or buttons).
       -Case matches the letter case too (a button, not a sentence that mentions it). #>
    $hits = @(Find-SimText $Word -Region $Region | Where-Object { -not $Case -or $_.text -clike "*$Word*" })
    if ($hits.Count -le $Nth) { Dbg ("word-" + ($Word -replace '[^\w]', '_')); throw "Word '$Word' not found on screen" }
    $h = $hits[$Nth]
    $i = if ($Case) { $h.text.IndexOf($Word) } else { $h.text.ToLower().IndexOf($Word.ToLower()) }
    $x = $h.x + [int]($h.w * ($i + $Word.Length / 2) / $h.text.Length)
    ClickAt $x ($h.y + [int]($h.h / 2)) $WaitMs
}

function Get-Box([int]$X, [int]$Y) { Nums (Geo 'box' @($X, $Y))[0] }

function Shot-Popover([string]$Id, [int]$X, [int]$Y, [int[]]$Margins) {
    <# The card around (X, Y) plus margins (left, top, right, bottom), cut from one capture: a
       popover can close between two captures when the stream refreshes. #>
    $path = Join-Path $script:RawDir "$Id.png"
    $box = python $script:TkGeom popover (Save-Temp) $X $Y $Margins[0] $Margins[1] $Margins[2] $Margins[3] $path
    Write-Host "  shot $Id (popover $box)" -ForegroundColor DarkGray
    Nums $box
}

function Shot-Box([string]$Id, [int[]]$Box, [int]$Pad = 10) {
    Shot $Id -Crop ([Math]::Max(0, $Box[0] - $Pad)), ([Math]::Max(38, $Box[1] - $Pad)), ($Box[2] + 2 * $Pad), ($Box[3] + 2 * $Pad)
}

function Get-Bottom([int]$X0, [int]$Y0, [int]$X1, [int]$Y1) { [int](Geo 'bottom' @($X0, $Y0, $X1, $Y1))[0] }

function Write-Text([string]$Text, [int]$WaitMs = 1400) {
    <# Types into a fresh editor: the first key alone (an empty editor drops keys while it swaps
       its placeholder out), then the rest. #>
    TypeText $Text.Substring(0, 1) 450
    if ($Text.Length -gt 1) { TypeText $Text.Substring(1) $WaitMs }
}

function Reset-Thinking {
    <# Closes menus, popovers and a composer left open; back to the Timeline. #>
    Key '{ESC}' 250; Key '{ESC}' 250
    $lines = @(Get-SimText)
    $c = @($lines | Where-Object { $_.text -match '^Cancel$' -and $_.x -ge 431 -and $_.x -le 631 })
    if ($c) { ClickAt ($c[0].x + 18) ($c[0].y + 5) 800 }
    $s = @($lines | Where-Object { $_.text -match '^Close$' -and $_.x -ge 600 -and $_.x -le 1000 })
    if ($s) { ClickAt ($s[0].x + 16) ($s[0].y + 6) 800 }
    Key '^1' 700
}

# --- the stream --------------------------------------------------------------------------------

function Show-StreamTop {
    Key '^2' 700; Key '^1' 900
    for ($i = 0; $i -lt 4; $i++) { Scroll 500 450 40 }
    Start-Sleep -Milliseconds 400
}

function Post-Inline([string]$Text) {
    <# Publishes a thought from the stream's inline composer; it lands at the top of the stream.
       Checks that it did (input can be lost if the window loses focus) and tries once more. #>
    $probe = $Text.Substring(0, [Math]::Min(40, $Text.IndexOf(' ', 30)))
    for ($try = 0; $try -lt 2; $try++) {
        Reset-Thinking
        Show-StreamTop
        ClickAt 520 112 400
        Key '^a' 150; Key '{BACKSPACE}' 300
        Write-Text $Text 600
        Key '^{ENTER}' 2500
        Show-StreamTop
        if (@(Find-SimText $probe -Region 296, 150, 464, 400)) { return }
    }
    Dbg 'post-inline'
    throw "the post '$probe' was not published"
}

function Find-Card([string]$Text) {
    <# A stream card near the top by the opening words of its text: the text line and the author
       line above it (the card's "…" button sits on the author row at x 733). #>
    $lines = @(Get-SimText | Where-Object { $_.x -ge 241 -and $_.x -lt 760 -and $_.y -ge 80 })
    $hit = $lines | Where-Object { $_.text -like "*$Text*" -and $_.x -ge 296 -and $_.x -le 320 } | Select-Object -First 1
    if (-not $hit) { Dbg 'card'; throw "post '$Text' is not in view" }
    $author = $lines | Where-Object { $_.x -ge 296 -and $_.x -le 320 -and $_.text -match $script:Authors -and $_.y -lt $hit.y } | Sort-Object y | Select-Object -Last 1
    $next = $lines | Where-Object { $_.x -ge 296 -and $_.x -le 320 -and $_.text -match $script:Authors -and $_.y -gt $hit.y } | Sort-Object y | Select-Object -First 1
    [pscustomobject]@{ h = $hit; a = $author; bottom = if ($next) { $next.y - 22 } else { 900 } }
}

function Select-Other([object]$Card) {
    <# Opens the thread of the card below $Card in the Thread Inspector. While the inspector shows
       the same post as the stream, its credence popover opens twice (stream and inspector) and a
       click on either closes both without saving: so the inspector is moved to another post. #>
    ClickAt 500 ($Card.bottom + 50) 1200
}

function Get-Chip([object]$Card) {
    <# The centre of a card's credence chip: right of the action row's icons (OCR rarely reads
       its accent-coloured label). #>
    $chip = @(Find-SimText '65%' -Region 600, ($Card.bottom - 40), 160, 30) | Select-Object -Last 1
    if ($chip) { return [pscustomobject]@{ x = $chip.x + 10; y = $chip.y + 6 } }
    [pscustomobject]@{ x = 628; y = $Card.bottom - 26 }
}

function Open-CardMenu([object]$Card) {
    ClickAt 733 ($Card.a.y + 6) 900
    Hover 290 ($Card.a.y + 6) 300
    if (-not @(Find-SimText 'Copy' -Region 500, 0, 260, 900)) { Dbg 'card-menu'; throw 'the post menu did not open' }
}

function Menu-Row([string]$Label, [switch]$Hover, [int]$WaitMs = 900) {
    <# A row of an open stream-card menu (x 517-743). #>
    $m = Find-One $Label @(500, 0, 260, 900)
    if ($Hover) { Hover ($m.x + 40) ($m.y + 6) $WaitMs } else { ClickAt ($m.x + 40) ($m.y + 6) $WaitMs }
    $m
}

function Actions-Item([string]$Card, [string]$Label, [int]$WaitMs = 1500) {
    <# Opens a card's "…" › Actions › $Label. #>
    Reset-Thinking
    Show-StreamTop
    $c = Find-Card $Card
    Open-CardMenu $c
    $hit = Open-Actions $Label
    ClickAt ($hit.x + 40) ($hit.y + 6) $WaitMs
}

function Open-Actions([string]$Label) {
    <# With a post menu open: opens its Actions submenu (hover, then a click if the hover went
       unnoticed) and returns the line of $Label in it. #>
    $a = Menu-Row 'Actions' -Hover -WaitMs 1100
    for ($i = 0; $i -lt 3; $i++) {
        $hit = @(Find-SimText $Label -Region 745, 0, 400, 900) | Select-Object -First 1
        if ($hit) { return $hit }
        Hover ($a.x + 10) ($a.y + 6) 300
        ClickAt ($a.x + 40) ($a.y + 6) 1100
    }
    Dbg 'actions-submenu'
    throw "'$Label' is not in the Actions submenu"
}

# --- Insights -----------------------------------------------------------------------------------

function Page-Scroll([int]$Notches) {
    <# Scrolls the Insights page from its left margin, off every chart that zooms. #>
    Scroll 249 500 $Notches
}

function Page-Wheel([int]$Delta) {
    <# A wheel turn of $Delta units (120 = one notch; negative = down) over the page margin. #>
    Move-SimMouse 249 500 -WaitMs 60
    Send-SimWheel ($Delta)
    Start-Sleep -Milliseconds 450
}

function Open-Insights([string]$Tab) {
    <# Insights on $Tab with the page at its top; the first time in a run, the period is set to
       All time (the demo posts span four months; the filter is remembered from then on). #>
    Key '{ESC}' 200
    Key '^7' 1500
    Page-Scroll 40
    if (-not $script:AllTime -and $Tab -notin 'Calibration', 'Tensions') {
        ClickAt $script:Tabs['Overview'] 140 1600
        Set-AllTime
        $script:AllTime = $true
    }
    ClickAt $script:Tabs[$Tab] 140 1600
    Page-Scroll 40
    Wait-Updated
}

function Wait-Updated {
    Start-Sleep -Milliseconds 400
    try { Wait-Text 'Updating' -Region 250, 50, 800, 70 -Seconds 30 -Gone } catch { }
    Start-Sleep -Milliseconds 500
}

function Set-AllTime {
    ClickWord 'All time' @(250, 180, 600, 40) -WaitMs 1200
    Wait-Updated
}

function Show-Line([string]$Text, [int]$Top = 60, [int[]]$Region = @(244, 38, 1196, 862)) {
    <# Scrolls the page from its top until a line holding $Text sits near $Top (or the page
       ends); returns it. A small turn measures how far a wheel unit moves the page. #>
    Page-Scroll 40
    $h = @(Find-SimText $Text -Region $Region) | Select-Object -First 1
    if (-not $h) { Dbg ("line-" + ($Text -replace '[^\w]', '_')); throw "'$Text' not found on the page" }
    if ($h.y -le $Top + 20) { return $h }
    Page-Wheel -30
    $h2 = @(Find-SimText $Text -Region $Region) | Select-Object -First 1
    if (-not $h2 -or $h2.y -ge $h.y) { return $h }
    $perUnit = ($h.y - $h2.y) / 30.0
    $rest = [int](($h2.y - $Top) / $perUnit)
    if ($rest -gt 0) { Page-Wheel (-$rest) }
    Start-Sleep -Milliseconds 300
    $h3 = @(Find-SimText $Text -Region $Region) | Select-Object -First 1
    if (-not $h3) { Dbg ("line-" + ($Text -replace '[^\w]', '_')); throw "'$Text' scrolled out of view" }
    $h3
}

function Page-End {
    Page-Scroll -40
    Start-Sleep -Milliseconds 1200
}

function Shot-Body([string]$Id, [int]$Top) {
    <# The tab's body from $Top down to where its content ends. #>
    Park
    $Top = [Math]::Max(38, $Top)
    $bottom = Get-Bottom 250 ($Top + 4) 1430 899
    Shot $Id -Crop 244, $Top, 1192, ([Math]::Min(900, $bottom + 14) - $Top)
}

function Shot-Card([string]$Id, [int]$X, [int]$Y, [int]$Pad = 8) {
    <# The card around a point in its padding. #>
    Park
    Shot-Box $Id (Get-Box $X $Y) $Pad
}

# --- the scene ----------------------------------------------------------------------------------

Step 'welcome' -Always {
    New-Item -ItemType Directory -Force -Path $script:Dbg | Out-Null
    $script:AllTime = $false
    if (-not (Get-NetTCPConnection -LocalPort 8765 -State Listen -ErrorAction SilentlyContinue)) {
        Start-Process node -ArgumentList (Join-Path $PSScriptRoot '..\stub-ai.mjs') -WindowStyle Hidden
        Start-Sleep -Seconds 2
    }
    Skip-Welcome
}

Step 'graph' {
    Reset-Thinking
    Key '^5' 2500
    Park
    Shot 'graph-overview'
    Shot 'graph-header' -Crop 244, 48, 1192, 68
    Shot 'graph-legend' -Crop 257, 123, 1166, 43
    # The seed thought: hover the thought dots (largest first) until its label shows.
    $found = $null
    foreach ($line in (Geo 'colour' @(270, 168, 1410, 814, 167, 139, 250, 12))) {
        $d = Nums $line
        Hover $d[0] $d[1] 500
        if (@(Find-SimText 'Exploring how' -Region ($d[0] - 200), $d[1], 400, 60)) { $found = $d; break }
    }
    if (-not $found) { Dbg 'graph-seed'; throw 'the seed thought was not found on the canvas' }
    ClickAt $found[0] $found[1] 1200
    Park
    Shot 'graph-selected' -Crop 268, 165, 1144, 651
    Shot 'graph-details-bar' -Crop 256, 815, 1168, 69
}

Step 'graph mini' {
    Reset-Thinking
    ClickAt 1166 18 1200
    Park
    if (-not @(Find-SimText 'MIND GRAPH' -Region 1121, 90, 319, 40)) { Dbg 'graph-mini'; ClickAt 1166 18 900; throw 'the right sidebar did not show its Mind Graph' }
    Shot 'graph-mini' -Crop 1121, 96, 319, 238
    ClickAt 1166 18 1000
}

Step 'overview' {
    Open-Insights 'Overview'
    Park
    Shot 'insights-overview'
    Shot 'insights-header-tabs' -Crop 244, 50, 1192, 112
    Shot 'insights-filter-bar' -Crop 250, 168, 1180, 104
}

Step 'network' {
    Open-Insights 'Network'
    # The tiles, then the card with its controls and canvas filling the window.
    $g = Show-Line 'Measure' 90
    Park
    Shot 'insights-network'
    # The largest node and its tooltip.
    $leg = Find-One 'Community 1' @(250, 300, 600, 600)
    $dot = Nums (Geo 'dots' @(272, ($g.y + 40), 1408, ($leg.y - 24), 20))[0]
    Hover $dot[0] $dot[1] 1500
    $t = @(Find-SimText 'Degree' -Region ($dot[0] - 360), ($dot[1] - 300), 720, 600 | Where-Object { $_.text -match 'Degree \d' })
    if (-not $t) { Dbg 'network-tooltip'; throw 'the node tooltip did not show' }
    # The tooltip card from a point in its left padding, joined with the node under it.
    $b = Get-Box ($t[0].x - 5) ($t[0].y + 6)
    $x0 = [Math]::Min($b[0], $dot[0] - 18); $y0 = [Math]::Min($b[1], $dot[1] - 18)
    $x1 = [Math]::Max($b[0] + $b[2], $dot[0] + 18); $y1 = [Math]::Max($b[1] + $b[3], $dot[1] + 18)
    Shot 'insights-network-tooltip' -Crop ($x0 - 10), ($y0 - 10), ($x1 - $x0 + 20), ($y1 - $y0 + 20)
    Park
    # The ranking under the canvas: pick its first row.
    Page-End
    $r = Find-One 'Ranked by' @(250, 38, 1180, 862)
    ClickAt 330 ($r.y + 32) 1200
    Park
    Page-End
    $r = Find-One 'Ranked by' @(250, 38, 1180, 862)
    $bottom = Get-Bottom 262 ($r.y + 10) 1416 899
    Shot 'insights-network-picked' -Crop 258, ($r.y - 14), 1164, ([Math]::Min(899, $bottom + 14) - $r.y + 14)
}

function Set-Chip([string]$Label) {
    <# Toggles a set chip of the Sets card (the page may jump after each change). #>
    $t = Find-One 'Sets of posts' @(250, 38, 600, 862)
    ClickWord $Label @(290, ($t.y + 60), 1130, 30) -WaitMs 1500
    Wait-Updated
}

Step 'sets' {
    Open-Insights 'Sets'
    # Three concepts that overlap: #epistemology, #mind and #language (not #attention).
    Set-Chip '#attention'
    Set-Chip '#language'
    $t = Show-Line 'Sets of posts' 56
    # The lens of the top circle (first set) and the lower-left one (third set).
    Hover 803 ($t.y + 234) 1500
    $b = Get-Box 262 ($t.y + 8)
    Shot-Box 'insights-sets-venn' $b 6
    Park
    $j = Find-One 'Similarity of sets' @(250, 38, 600, 862)
    Shot-Card 'insights-sets-jaccard' 262 ($j.y + 8) 6
    # A fourth set switches to the UpSet matrix.
    Set-Chip '#spinoza'
    $t = Show-Line 'Sets of posts' 56
    Shot-Card 'insights-sets-upset' 262 ($t.y + 8) 6
}

Step 'symmetry' {
    Open-Insights 'Symmetry'
    Page-End
    $t = Find-One 'Thread symmetry' @(250, 38, 600, 862)
    Shot-Body 'insights-symmetry' ($t.y - 24)
}

Step 'categories' {
    Open-Insights 'Categories'
    Page-End
    $t = Find-One 'Category of relations' @(250, 38, 600, 862)
    Shot-Body 'insights-categories' ($t.y - 24)
}

Step 'embedding' {
    Open-Insights 'Embedding'
    $t = Show-Line 'Map of meaning' 52
    Park
    $c = Find-One 'Colour by' @(250, 38, 600, 862)
    $p = Nums (Geo 'dense' @(275, ($c.y + 40), 1405, 860, 70))[0]
    Hover $p[0] $p[1] 1500
    if (-not @(Find-SimText 'Nearest in meaning')) { Hover ($p[0] + 1) ($p[1] + 1) 1500 }
    if (-not @(Find-SimText 'Nearest in meaning')) { throw 'the point tooltip did not show' }
    Shot-Box 'insights-embedding' (Get-Box 262 ($t.y + 8)) 6
    Park
}

Step 'activity' {
    Open-Insights 'Activity'
    $p = Show-Line 'Posts over time' 52
    Park
    Shot 'insights-activity'
    $r = Find-One 'Weekly rhythm' @(250, 38, 600, 862)
    $box = Get-Box 262 ($r.y + 8)
    $a = Find-One 'Agent traits' @(840, 38, 600, 862)
    Shot-Card 'insights-radar' 853 ($a.y + 8) 6
    $c = Nums (Geo 'peak' @(($box[0] + 20), ($r.y + 30), ($box[0] + $box[2] - 20), ($box[1] + $box[3] - 60)))[0]
    Hover $c[0] $c[1] 1500
    Shot-Box 'insights-rhythm' $box 6
    Park
}

Step 'timeline' {
    Open-Insights 'Timeline'
    $t = Show-Line 'Concepts over time' 52
    Park
    Shot 'insights-timeline' -Crop 244, ($t.y - 24), 1192, (900 - $t.y + 24)
    Page-End
    $w = Find-One 'Where each concept' @(250, 38, 600, 862)
    ClickAt 330 ($w.y + 34) 1500
    Park
    Page-End
    # The Concept card sits beside the list, its top level with the list's.
    $w = Find-One 'Where each concept' @(250, 38, 600, 862)
    Shot-Card 'insights-timeline-detail' 853 ($w.y + 8) 6
}

Step 'diversity' {
    Open-Insights 'Diversity'
    Page-End
    $e = Find-One 'Equally active' @(250, 38, 600, 862)
    $b = Get-Box 262 ($e.y + 4)
    Shot-Body 'insights-diversity' ($b[1] - 8)
}

Step 'readability' {
    Open-Insights 'Readability'
    $m = Show-Line 'Median reading ease' 100
    $b = Get-Box 262 ($m.y + 4)
    Park
    $top = [Math]::Max(38, $b[1] - 8)
    # Down to the bottom of the Median ease per period card (the lists below are cut off).
    $e = @(Find-SimText 'Higher bars' -Region 250, 300, 600, 600) | Select-Object -First 1
    $bottom = if ($e) { [Math]::Min(900, $e.y + 30) } else { 900 }
    Shot 'insights-readability' -Crop 244, $top, 1192, ($bottom - $top)
}

# --- the thinking tools: the tutorial "Map an argument" ----------------------------------------

function Credence-Presets {
    <# The open credence popover in the stream: its CHANCE IT IS TRUE label; the presets 5% to
       95% sit under it at fixed offsets. #>
    $l = @(Find-SimText 'CHANCE IT IS' -Region $script:Feed)
    if (-not $l) { Dbg 'credence-popover'; throw 'the credence popover is not open' }
    $l[0]
}

function Click-Preset([int]$Percent) {
    $l = Credence-Presets
    $i = @(5, 20, 35, 50, 65, 80, 95).IndexOf($Percent)
    ClickAt ($l.x + 17 + 43 * $i) ($l.y + 27) 900
}

Step 'atomic' {
    Reset-Thinking
    Key '^n' 1200
    Write-Text $script:Atomic 2500
    Start-Sleep -Milliseconds 1500
    Park
    $h = Find-One '@architect' @(431, 200, 578, 400)
    $m = Find-One 'Atomicity' @(431, 200, 578, 500)
    # Down to the atomic row: the readability meter wraps onto the row below it.
    Shot 'thinking-atomic-hint' -Crop 445, ($h.y - 18), 560, ($m.y + 21 - $h.y + 18)
    Key '^{ENTER}' 2500
}

Step 'atomic sheet' {
    Reset-Thinking
    Show-StreamTop
    $c = Find-Card 'This changes everything'
    Open-CardMenu $c
    $hit = Open-Actions 'Check atomicity'
    ClickAt ($hit.x + 40) ($hit.y + 6) 1800
    Park
    # The sheet's edges: its header row (left, right, top) and its footer row (bottom), each
    # walked from a point in their empty padding.
    $t = Find-One 'Atomicity' @(440, 60, 560, 120)
    $h = Get-Box ($t.x - 20) ($t.y + 4)
    $c = @(Find-SimText 'Close' -Region 440, 500, 560, 400 | Where-Object { $_.text -match '^Close' }) | Select-Object -First 1
    if (-not $c) { throw 'the sheet footer was not found' }
    $f = Get-Box ($h[0] + 12) ($c.y + 6)
    Shot 'thinking-atomic-sheet' -Crop $h[0], $h[1], $h[2], ($f[1] + $f[3] - $h[1])
    # Escape does not close this sheet: its Close button does.
    ClickAt ($c.x + 16) ($c.y + 6) 800
}

Step 'claim' {
    Post-Inline $script:Claim
    Show-StreamTop
    $c = Find-Card 'Predicting the next word is not'
    Park
    Mark
    Open-CardMenu $c
    Shot 'thinking-credence-menu' -Auto -AutoPad 8 -Within 500, 38, 760, 900
    $menu = $script:LastBox
    Mark 200
    Menu-Row 'Actions' -Hover -WaitMs 1300 | Out-Null
    $sub = Nums (python $script:AutoCrop $script:MarkPath (Save-Temp) (Join-Path ([System.IO.Path]::GetTempPath()) "sim-thinking-sub-$PID.png") 0 745 0 1120 900)
    $x0 = [Math]::Min($menu[0], $sub[0]) - 6; $y0 = [Math]::Max(38, [Math]::Min($menu[1], $sub[1]) - 6)
    $x1 = [Math]::Max($menu[0] + $menu[2], $sub[0] + $sub[2]) + 6; $y1 = [Math]::Min(900, [Math]::Max($menu[1] + $menu[3], $sub[1] + $sub[3]) + 6)
    Shot 'thinking-actions-menu' -Crop $x0, $y0, ($x1 - $x0), ($y1 - $y0)
    Key '{ESC}' 300; Key '{ESC}' 500
    # Set a credence: 65%, in a week.
    $c = Find-Card 'Predicting the next word is not'
    Select-Other $c
    $c = Find-Card 'Predicting the next word is not'
    Open-CardMenu $c
    Menu-Row 'credence' -WaitMs 1200 | Out-Null
    Click-Preset 65
    $w = Find-One 'In a week' $script:Feed
    ClickAt ($w.x + 20) ($w.y + 6) 1000
    Park
    $l = Credence-Presets
    $rc = @(Find-SimText 'Remove credence' -Region $script:Feed) | Select-Object -First 1
    # The popover and, above its top right corner, the chip it hangs from; its bottom edge
    # is hard to see against the card under it, so it is cut under Remove credence.
    $box = Shot-Popover 'thinking-credence-popover-open' ($l.x - 6) ($l.y + 4) @(1, 34, 1, 1)
    if ($rc) {
        $png = Join-Path $script:RawDir 'thinking-credence-popover-open.png'
        python -c "import sys; from PIL import Image; im = Image.open(sys.argv[1]); h = int(sys.argv[2]); im.crop((0, 0, im.width, min(im.height, h))).save(sys.argv[1])" $png ($rc.y + 24 - $box[1])
    }
    # Closed again, the chip on the card and its tooltip.
    Key '{ESC}' 600
    if (@(Find-SimText 'CHANCE IT IS' -Region $script:Feed)) { ClickAt 1000 20 600 }
    $c = Find-Card 'Predicting the next word is not'
    $chip = Get-Chip $c
    $tip = @()
    for ($i = 0; $i -lt 3 -and -not $tip; $i++) {
        Park
        Hover ($chip.x - 2) ($chip.y - 1) 300
        Hover $chip.x $chip.y 2000
        $tip = @(Find-SimText 'that this is true' -Region 241, 38, 1199, 862)
    }
    if (-not $tip) { Dbg 'credence-tip'; throw 'the chip tooltip did not show' }
    $t = $tip[0]
    # The whole card (the tooltip covers part of its text), down to its action row.
    $y0 = [Math]::Min($c.a.y - 16, $t.y - 12); $y1 = [Math]::Max($chip.y + 18, $t.y + $t.h + 12)
    $x1 = [Math]::Max(760, $t.x + $t.w + 14)
    Shot 'thinking-credence-chip' -Crop 292, $y0, ($x1 - 292), ($y1 - $y0)
    Park
}

Step 'calibration empty' {
    Reset-Thinking
    Palette 'Insights: calibration' 2200
    Page-Scroll 40
    Shot-Body 'insights-calibration-empty' 160
}

Step 'persona beliefs' {
    Reset-Thinking
    Key '^4' 1300
    Click 'Arya Wiratama' -Region 241, 100, 243, 300 -WaitMs 1000
    ClickAt 995 71 1200
    Park
    $bottom = Get-Bottom 610 90 1320 899
    Shot 'beliefs-persona' -Crop 606, 44, 718, ([Math]::Min(899, $bottom + 16) - 44)
}

function Insp-Find([string]$Text, [switch]$Case) {
    <# A line in the Thread Inspector; scrolls the pane down a little at a time until it shows. #>
    for ($i = 0; $i -lt 6; $i++) {
        $hit = @(Find-SimText $Text -Region $script:Insp | Where-Object { -not $Case -or $_.text -clike "*$Text*" }) | Sort-Object y | Select-Object -First 1
        if ($hit) { return $hit }
        Scroll 1100 700 -2
    }
    Dbg ("insp-" + ($Text -replace '[^\w]', '_'))
    throw "'$Text' not found in the Thread Inspector"
}

function Insp-Top { Scroll 1100 500 20 }

function Show-ArgumentView {
    <# The Thread Inspector's View menu › Show the argument map (a first click can go unheard
       while the thread is still opening). #>
    for ($i = 0; $i -lt 3; $i++) {
        ClickAt 1392 60 1000
        $m = @(Find-SimText 'argument map' -Region 1100, 60, 340, 500) | Select-Object -First 1
        if ($m) { ClickAt ($m.x + 40) ($m.y + 6) 1500; return }
        Key '{ESC}' 300
    }
    Dbg 'view-menu'
    throw 'the View menu did not open'
}

function Add-Statement([string]$Text, [string]$Role) {
    <# Add statement, write it, pick its role, Save; the editor then edits the new statement.
       OCR cannot read the field or the role chips (text inside a border), so they are placed
       from the card's title: the field and Save one row under it, the chips a row lower, in
       the order Claim, Ground, Warrant, Backing, Qualifier, Objection. #>
    Insp-Top
    for ($i = 0; $i -lt 3; $i++) {
        ClickWord 'Add statement' $script:Insp -Case -WaitMs 1200
        if (@(Find-SimText 'New statement' -Region $script:Insp)) { break }
    }
    $t = Insp-Find 'New statement' -Case
    ClickAt ($t.x + 200) ($t.y + 40) 400
    Write-Text $Text 600
    $dx = @{ Claim = 6; Ground = 61; Warrant = 121; Backing = 182; Qualifier = 244; Objection = 311 }[$Role]
    ClickAt ($t.x + $dx) ($t.y + 78) 700
    ClickAt 1394 ($t.y + 39) 1300
}

function Link-To([string]$Kind, [string]$Target) {
    <# With a statement in the editor: Supports… or Attacks…, then the target's chip on the canvas. #>
    # The editor's button row: OCR reads 'Attacks… Remove' but not the green 'Supports…'
    # (always the row's first button, at x 830).
    $b = $null
    for ($i = 0; $i -lt 5 -and -not $b; $i++) {
        $b = @(Find-SimText 'Remove' -Region 761, 38, 679, 840 | Where-Object { $_.text -clike '*Remove*' }) | Sort-Object y | Select-Object -Last 1
        if (-not $b) { Scroll 1100 700 -2 }
    }
    if (-not $b) { Dbg 'link-buttons'; throw 'the editor buttons were not found' }
    if ($Kind -eq 'Supports') { ClickAt 830 ($b.y + 6) 900 }
    else { ClickWord 'Attacks' @(761, ($b.y - 6), 679, 24) -Case -WaitMs 900 }
    Insp-Top
    $t = Insp-Find $Target
    ClickAt ($t.x + 20) ($t.y + 6) 1200
}

Step 'argument' {
    Reset-Thinking
    Show-StreamTop
    $c = Find-Card 'Predicting the next word is not'
    ClickAt ($c.h.x + 60) ($c.h.y + 6) 1500
    Show-ArgumentView
    Park
    $e = Find-One 'no argument map yet' $script:Insp
    $bottom = Get-Bottom 766 $e.y 1434 ($e.y + 200)
    Shot 'thinking-argument-empty' -Crop 761, 38, 679, ($bottom + 24 - 38)
    Add-Statement 'Predicting words is not understanding them' 'Claim'
    Add-Statement 'A mirror reproduces light without feeling warmth' 'Ground'
    Link-To 'Supports' 'Predicting words'
    Add-Statement 'Understanding may just be very good prediction' 'Objection'
    Link-To 'Attacks' 'Predicting words'
    Add-Statement 'A model of words is not a model of the world' 'Ground'
    Link-To 'Attacks' 'Understanding may'
    # The finished map, the editor closed (its × at the right of its title row).
    $e = Insp-Find 'Edit statement' -Case
    ClickAt 1401 ($e.y + 5) 900
    Insp-Top
    Park
    $bottom = Get-Bottom 766 82 1434 846
    Shot 'thinking-argument-map' -Crop 761, 38, 679, ([Math]::Min(850, $bottom + 16) - 38)
    # The objection in the editor.
    $o = Insp-Find 'Understanding may'
    ClickAt ($o.x + 20) ($o.y + 6) 1200
    Park
    $e = Insp-Find 'Edit statement' -Case
    Shot-Card 'thinking-argument-editor' 781 ($e.y + 4) 4
    ClickAt 1401 ($e.y + 5) 900
    # The statement list at the bottom of the pane.
    Scroll 1100 700 -10
    Park
    $l = Insp-Find 'STATEMENTS' -Case
    $bottom = Get-Bottom 766 $l.y 1434 846
    Shot 'thinking-argument-list' -Crop 768, ($l.y - 12), 666, ($bottom + 10 - $l.y + 12)
}

function Confirm-Spend {
    <# Answers "Send this to the AI?" with Send, ticking "Don't ask again this session" first. #>
    $d = @(Find-SimText 'ask again this session')
    if (-not $d) { return }
    ClickAt ($d[0].x - 14) ($d[0].y + 7) 400
    Click 'Send' -Region ($d[0].x + 200), ($d[0].y + 20), 300, 80 -At ($d[0].x + 348), ($d[0].y + 42) -WaitMs 900
}

Step 'tensions' {
    Post-Inline $script:Counter
    Actions-Item 'Predicting the next word is not' 'Find tensions' 2500
    Confirm-Spend
    try { Wait-Text 'Looking for tensions' -Region 250, 38, 1180, 400 -Seconds 40 -Gone } catch { }
    # The finder pairs nothing with the claim: it counts negations, and the claim's "without"
    # cancels its "not". Drop the focus (the chip's ×) to scan the whole stream.
    $a = Find-One 'Around' @(250, 150, 1180, 120)
    $x = if ($a.text -match '[×xX]\s*$') { $a.x + $a.w - 4 } else { $a.x + $a.w + 8 }
    ClickAt $x ($a.y + 6) 1500
    Confirm-Spend
    try { Wait-Text 'Looking for tensions' -Region 250, 38, 1180, 400 -Seconds 40 -Gone } catch { }
    Start-Sleep -Milliseconds 1200
    Page-Scroll 40
    Shot-Body 'insights-tensions' 160
    $r = @(Find-SimText 'Reply to reconcile' -Region 250, 150, 1180, 750) | Sort-Object y | Select-Object -First 1
    if (-not $r) { Dbg 'tension-card'; throw 'no tension card was found' }
    Shot-Card 'insights-tension-card' 262 ($r.y + 6) 6
}

Step 'resolve' {
    Reset-Thinking
    Show-StreamTop
    $c = Find-Card 'Predicting the next word is not'
    Select-Other $c
    $c = Find-Card 'Predicting the next word is not'
    $chip = Get-Chip $c
    ClickAt $chip.x $chip.y 1200
    $n = Find-One 'What happened' $script:Feed
    ClickAt ($n.x + 60) ($n.y + 6) 400
    Write-Text 'The thread convinced me' 600
    ClickWord 'True' @(241, ($n.y + 16), 519, 60) -WaitMs 1200
    Park
    if (-not @(Find-SimText 'Said 65' -Region $script:Feed)) {
        Key '{ESC}' 500
        ClickAt $chip.x $chip.y 1200
        Park
    }
    $s = Find-One 'Said 65' $script:Feed
    Shot-Popover 'thinking-credence-popover-resolved' ($s.x - 6) ($s.y + 4) @(1, 1, 1, 1) | Out-Null
    Key '{ESC}' 500
}

Step 'tutorial calibration' {
    Reset-Thinking
    Palette 'Insights: calibration' 2200
    Page-Scroll 40
    # The tiles and the two charts, down to the (now empty) Open predictions card.
    Park
    $o = Find-One 'Open predictions' @(250, 300, 600, 600)
    Shot 'thinking-tutorial-calibration' -Crop 244, 160, 1192, ($o.y - 24 - 160)
}

# --- more predictions, beliefs and the empty network ----------------------------------------------

# Nine more predictions for the full Calibration tab: text, chance, and how each resolves
# (T true, F false, A ambiguous, '' left open).
$script:Bets = @(
    @('Prediction: someone at the seminar defends Descartes on purpose. #Predictions', 5, 'F', 'someone at the seminar'),
    @('Prediction: the library gets the Curley translation back before the seminar. #Predictions', 20, 'F', 'library gets the Curley'),
    @('Prediction: Sokrates will grant that know-how is a kind of knowing. #Predictions', 35, 'T', 'Sokrates will grant'),
    @('Prediction: the group splits evenly on whether attention can be trained. #Predictions', 50, 'A', 'group splits evenly'),
    @('Prediction: I finish the essay draft on attention by Friday. #Predictions', 65, 'F', 'finish the essay draft'),
    @('Prediction: the seminar room is free on Monday morning. #Predictions', 80, 'T', 'seminar room is free'),
    @('Prediction: at least five people come to the next reading. #Predictions', 80, 'F', 'at least five people'),
    @('Prediction: I reread Ethics II p7 before the seminar. #Predictions', 95, 'T', 'reread Ethics II p7'),
    @('Prediction: the reading list goes out to the group this week. #Predictions', 95, '', 'reading list goes out')
)

Step 'bets' {
    foreach ($b in $script:Bets) {
        Post-Inline $b[0]
        Show-StreamTop
        $c = Find-Card $b[3]
        Select-Other $c
        $c = Find-Card $b[3]
        Open-CardMenu $c
        Menu-Row 'credence' -WaitMs 1200 | Out-Null
        Click-Preset $b[1]
        Key '{ESC}' 500
        if (@(Find-SimText 'CHANCE IT IS' -Region $script:Feed)) { ClickAt 1000 20 500 }
    }
}

Step 'resolve bets' {
    # Calibration's open predictions: resolve each bet from its row (the rows reflow after each).
    Reset-Thinking
    Palette 'Insights: calibration' 2200
    foreach ($b in $script:Bets) {
        if (-not $b[2]) { continue }
        $word = @{ T = 'True'; F = 'False'; A = 'Ambiguous' }[$b[2]]
        Page-End
        $row = @(Find-SimText $b[3] -Region 250, 38, 1180, 862) | Select-Object -First 1
        if (-not $row) { Dbg 'resolve-row'; throw "the open prediction '$($b[3])' was not found" }
        ClickWord $word @(900, ($row.y - 8), 530, 30) -WaitMs 1200
    }
    # The page's end: the six tiles down to the Open predictions card (the tabs scroll away).
    Page-End
    $r = Find-One 'Resolved' @(250, 38, 600, 400)
    $b = Get-Box 262 ($r.y + 4)
    Shot-Body 'insights-calibration' ($b[1] - 8)
}

Step 'beliefs' {
    Reset-Thinking
    Show-StreamTop
    # A thread open in the inspector, so Read the open thread is available.
    ClickAt 500 230 1200
    Key '^3' 1300
    Click 'Sokrates' -Region 241, 100, 243, 300 -WaitMs 1000
    ClickAt 995 71 1500
    Park
    Shot 'beliefs-tab' -Crop 606, 44, 718, 856
    # Edit the belief about reflections (it has a history): its edit button (second of the four
    # on the date line, at the right of the row), then + once.
    $r = Find-One 'A reflection can reproduce' @(606, 90, 718, 810)
    for ($i = 0; $i -lt 3; $i++) {
        ClickAt 1213 ($r.y + 58) 1000
        if (@(Find-SimText 'Cancel' -Region 622, $r.y, 680, 200)) { break }
    }
    Park
    # The stepper '− 0.60 +' (OCR reads it as one line, the + as its last character).
    $v = @(Find-SimText '0.60' -Region 622, ($r.y + 60), 300, 200) | Select-Object -First 1
    if (-not $v) { Dbg 'belief-stepper'; throw 'the credence stepper was not found' }
    $px = if ($v.text -match '\+\s*$') { $v.x + $v.w - 4 } else { $v.x + $v.w + 22 }
    ClickAt $px ($v.y + 6) 900
    Park
    Shot-Card 'beliefs-row-editing' 627 ($r.y + 4) 6
    # Save sits just left of Cancel (OCR misses its white-on-purple label).
    $c = @(Find-SimText 'Cancel' -Region 622, $r.y, 680, 260) | Select-Object -First 1
    if ($c) { ClickAt ($c.x - 65) ($c.y + 6) 1200 } else { ClickWord 'Save' @(622, $r.y, 680, 260) -WaitMs 1200 }
    Park
    # The revision log button (the fourth icon on the date line): try the likely rows until the
    # log opens.
    $r = Find-One 'A reflection can reproduce' @(606, 90, 718, 810)
    foreach ($dy in 58, 52, 64, 70) {
        ClickAt 1277 ($r.y + $dy) 1000
        if (@(Find-SimText 'REVISION LOG' -Region 606, $r.y, 718, 400)) { break }
    }
    Park
    Shot-Card 'beliefs-history' 627 ($r.y + 4) 6
    # Simone Weil has formed no belief.
    Click 'Simone Weil' -Region 241, 100, 243, 300 -WaitMs 1000
    ClickAt 995 71 1500
    Park
    $bottom = Get-Bottom 610 90 1320 899
    Shot 'beliefs-empty' -Crop 606, 44, 718, ([Math]::Min(899, $bottom + 16) - 44)
}

Step 'empty network' {
    Reset-Thinking
    ClickAt 96 64 800
    $e = @(Find-SimText 'Empty Lab' -Region 0, 70, 400, 500)
    if ($e) { ClickAt ($e[0].x + 30) ($e[0].y + 6) 1200 }
    else {
        Key '{ESC}' 400
        Palette 'create network' 900
        TypeText 'Empty Lab' 400
        $c = Find-One 'Cancel' @(300, 80, 900, 400)
        if ($c.text -match 'Create') { ClickWord 'Create' @($c.x, ($c.y - 2), ($c.w + 4), 20) -WaitMs 1200 }
        else { ClickAt ($c.x + $c.w + 40) ($c.y + 6) 1200 }
    }
    if (-not @(Find-SimText 'Empty Lab' -Region 0, 44, 241, 40)) { Dbg 'empty-network'; throw 'the Empty Lab network is not active' }
    Key '^5' 2000
    Park
    Shot-Card 'graph-empty' 262 150 6
    Open-Insights 'Overview'
    Park
    $bottom = Get-Bottom 256 280 1424 899
    Shot 'insights-empty' -Crop 250, 168, 1180, ([Math]::Min(899, $bottom + 24) - 168)
    ClickAt 96 64 800
    Click 'Epistemology' -Region 0, 70, 360, 300 -WaitMs 900
}
