# The post menu and its tools, and the timeline views (shots/post-tools.json). Client area
# 1440x900, demo profile. Most shots act on the seed post of Epistemology & Mind ("Exploring how
# language constructs internal semiotics", by Arya Wiratama, below "Show more thoughts"); the
# steps are ordered so that what changes it (a retraction, a series, planting it in the garden)
# comes after the shots that need it plain. Most post tools sit in the … menu's submenus
# (Actions ›, Organize ›, Think it over ›), reached with Open-Path / Path-Item; views are picked
# from the View ▾ menu (Pick-View).

# --- helpers (the stream geometry ones are the same as in posts.ps1) ---------------------------

$script:Dialog = 425, 0, 1015, 900
$script:Sheet = 405, 0, 1035, 900
$script:Seed = 'Exploring how language constructs'
$script:Authors = '^(Arya Wiratama|Sokrates of Alop|Simone Weil|Baruch de Spinoza|Senja Prakasa)'
$script:Geom = Join-Path ([System.IO.Path]::GetTempPath()) 'simulacra-docs-geom.py'
$script:ToastArea = 1040, 34, 1440, 112
# Card and icon geometry read from a capture: separators between stream cards, and the x centres
# of the icons on one row (the action bar, the composer toolbar).
Set-Content -Path $script:Geom -Encoding utf8 -Value @'
import sys
import numpy as np
from PIL import Image

def card(a, x0, x1, y):
    xs = [x0 + 7, x0 + 60, (x0 + x1) // 3, (x0 + x1) // 2, 2 * (x0 + x1) // 3, x1 - 60, x1 - 8]
    cols = a[:, xs].reshape(-1, 3)
    vals, counts = np.unique(cols, axis=0, return_counts=True)
    bg = vals[counts.argmax()]
    seps = []
    for r in range(1, a.shape[0] - 1):
        row = a[r, xs]
        if np.abs(row - row[0]).max() > 3 or np.abs(row[0] - bg).max() <= 6:
            continue
        up, dn = a[r - 1, xs], a[r + 1, xs]
        if (np.abs(up - row).max(axis=1) > 6).sum() >= 6 and (np.abs(dn - row).max(axis=1) > 6).sum() >= 6:
            seps.append(r)
    return max([s for s in seps if s < y], default=0), min([s for s in seps if s > y], default=a.shape[0])

def runs(a, x0, x1, y0, y1, minw):
    # Ink runs grouped when the gap is under 9 px (an icon and its count, the two halves of </>);
    # a group counts when it is at least minw wide, centred on its first run if that is wide.
    a = a[y0:y1, x0:x1]
    bg = np.median(a.reshape(-1, 3), axis=0)
    ink = (np.abs(a - bg).max(axis=2) > 40).any(axis=0)
    spans, start = [], None
    for i, v in enumerate(list(ink) + [False]):
        if v and start is None:
            start = i
        elif not v and start is not None:
            spans.append((start, i))
            start = None
    groups = []
    for s, e in spans:
        if groups and s - groups[-1][-1][1] < 9:
            groups[-1].append((s, e))
        else:
            groups.append([(s, e)])
    out = []
    for g in groups:
        (s0, e0), end = g[0], g[-1][1]
        if end - s0 < minw:
            continue
        out.append(x0 + ((s0 + e0) // 2 if e0 - s0 >= minw else (s0 + end) // 2))
    return out

def fills(a, x0, x1, y, minw):
    # Solid blocks on one pixel row (a hidden spoiler): runs of one colour that is not the background.
    row = a[y, x0:x1]
    bg = row[0]   # x0 sits in the card's padding
    out, start = [], None
    for i in range(len(row) + 1):
        same = i < len(row) and start is not None and np.abs(row[i] - row[start]).max() <= 4
        if same:
            continue
        if start is not None and i - start >= minw and np.abs(row[start] - bg).max() > 12:
            out.append(x0 + (start + i) // 2)
        start = i if i < len(row) else None
    return out

def bar(a, y0, y1):
    # The composer's attach bar: five icons centred near x 509, 537, 565, 593 and 621.
    want = [509, 537, 565, 593, 621]
    hits = []
    for y in range(y0, y1, 2):
        got = runs(a, 495, 640, y - 7, y + 7, 9)
        if len(got) == 5 and all(abs(g - w) <= 5 for g, w in zip(got, want)):
            hits.append(y)
        elif hits:
            break
    return [(hits[0] + hits[-1]) // 2] if hits else []

a = np.asarray(Image.open(sys.argv[2]).convert("RGB"), dtype=np.int16)
n = [int(v) for v in sys.argv[3:]]
mode = {"card": card, "runs": runs, "fills": fills, "bar": bar}[sys.argv[1]]
print(*mode(a, *n))
'@

function Park { Hover 900 20 300 }

function Save-Temp {
    $p = Join-Path ([System.IO.Path]::GetTempPath()) "sim-posts-$PID.png"
    Save-SimShot -Path $p | Out-Null
    $p
}

function Get-CardBox([int]$Y, [int]$X0 = 241, [int]$X1 = 760) {
    <# Top and bottom separator rows of the stream card that contains row $Y. #>
    [int[]]((python $script:Geom card (Save-Temp) $X0 $X1 $Y) -split ' ')
}

function Get-Runs([int]$Y0, [int]$Y1, [int]$X0 = 296, [int]$X1 = 760, [int]$MinW = 11) {
    <# x centres of the ink runs at least $MinW wide in a strip: icons on a row, spoiler blocks. #>
    $r = python $script:Geom runs (Save-Temp) $X0 $X1 $Y0 $Y1 $MinW
    if (-not $r) { return @() }
    [int[]]($r -split ' ')
}

function Get-ChangedBox([int[]]$Within) {
    <# What changed since the last Mark, as x,y,w,h (no file is kept). #>
    $after = Save-Temp
    $out = Join-Path ([System.IO.Path]::GetTempPath()) "sim-posts-box-$PID.png"
    $cropArgs = @($script:AutoCrop, $script:MarkPath, $after, $out, 0)
    if ($Within) { $cropArgs += $Within }
    [int[]]((python @cropArgs) -split ' ')
}

function Shot-Card([string]$Id, [int]$Y, [int]$Above = 0, [int]$Below = 0, [int]$Left = 242) {
    <# The stream card containing row $Y, optionally with a sliver of its neighbours. #>
    Park
    $b = Get-CardBox $Y
    $top = [Math]::Max(38, $b[0] + 1 - $Above)
    Shot $Id -Crop $Left, $top, (759 - $Left), ([Math]::Min(900, $b[1] + $Below) - $top)
}

function Write-Text([string]$Text, [int]$WaitMs = 1400) {
    <# Types into a fresh editor: the first key alone (an empty editor drops keys while it
       swaps its placeholder out), then the rest. #>
    if (-not $Text) { return }
    TypeText $Text.Substring(0, 1) 450
    if ($Text.Length -gt 1) { TypeText $Text.Substring(1) $WaitMs }
}

function Find-One([string]$Text, [int[]]$Region) {
    $hits = @(Find-SimText -Text $Text -Region $Region)
    if (-not $hits) {
        if ($env:SIM_DEBUG) { Save-SimShot -Path (Join-Path $env:SIM_DEBUG "fail-$(Get-Date -Format HHmmss)-$($Text -replace '[^\w]', '_').png") | Out-Null }
        throw "'$Text' is not on screen"
    }
    $hits[0]
}

function Focus-Editor {
    <# Puts the caret at the end of the first line of the composer's first post. #>
    $a = @(Find-SimText '@architect' -Region 431, 0, 578, 900) | Select-Object -First 1
    if (-not $a) { throw 'composer editor not found' }
    ClickAt 975 ($a.y + 40) 300
}

function Submit-Composer([int]$WaitMs = 2000) { Focus-Editor; Key '^{ENTER}' $WaitMs }

function Reset-State {
    <# Closes menus, sheets and a composer left open by a failed step; back to the Timeline.
       A composer has Cancel top left; a sheet has it bottom right. #>
    Key '{ESC}' 250; Key '{ESC}' 250
    for ($i = 0; $i -lt 3; $i++) {
        $c = @(Get-SimText | Where-Object { $_.text -match '^Cancel$' -and (($_.x -ge 431 -and $_.x -le 531) -or ($_.x -ge 760 -and $_.x -le 1030)) })
        if (-not $c) { break }
        ClickAt ($c[0].x + 18) ($c[0].y + 5) 800
    }
    Key '^1' 700
}

function Open-Composer([switch]$Style) {
    <# Ctrl+N. The Tint and Symbols rows sit behind the footer's palette button (Tint and symbols),
       which the app remembers between compositions: -Style unfolds them, otherwise they are folded
       again if a step left them open. #>
    Key '^n' 1100
    if ($Style -or $script:StyleOpen) { Set-Style ([bool]$Style) }
}

function Set-Style([bool]$Open) {
    $on = [bool]@(Find-SimText 'Symbols' -Region 431, 0, 578, 900)
    if ($on -ne $Open) { $f = Get-Footer; ClickAt $f.style $f.y 800 }
    $script:StyleOpen = $Open
}

function Fold-Style {
    <# Leaves the Tint and Symbols rows folded for the compositions of later steps. #>
    if (-not $script:StyleOpen) { return }
    Open-Composer
    Close-Composer
}

function Close-Composer { Click 'Cancel' -Region 431, 0, 200, 900 -WaitMs 800 }

function Get-Footer {
    <# The composer footer: its row and the x of its icon buttons, measured from Schedule…: preview
       (eye), the five formats, Tint and symbols (palette), Schedule…, then lock, feeling, Free
       compose (list-tree) and To-do syntax (checklist), 32 px apart. #>
    $s = @(Find-SimText 'Schedule' -Region 431, 0, 578, 900) | Sort-Object y | Select-Object -Last 1
    if (-not $s) { throw 'composer footer not found' }
    $y = $s.y + 5
    $right = $s.x + $s.w
    [pscustomobject]@{ y = $y; eye = 459; style = $s.x - 29; schedule = $s.x + 20; lock = $right + 29
        feeling = $right + 61; free = $right + 93; todo = $right + 125 }
}

function Shot-Dialog([string]$Id) {
    <# The composer dialog, from its header (Cancel) to the bottom of its footer. #>
    Park
    $c = @(Find-SimText 'Cancel' -Region 431, 0, 120, 900) | Sort-Object y | Select-Object -First 1
    if (-not $c) { throw 'composer header not found' }
    $f = Get-Footer
    $top = $c.y - 17
    Shot $Id -Crop 430, $top, 580, ($f.y + 24 - $top)
}

function Show-StreamTop {
    <# Back to the stream's first post: a quick visit to another place lets the Timeline pick up
       posts written while it was scrolled down, then the (possibly long) stream is scrolled up. #>
    Key '^2' 700; Key '^1' 900
    for ($i = 0; $i -lt 8; $i++) { Scroll 500 450 40 }
    Start-Sleep -Milliseconds 400
}

function Find-Post([string]$Text, [int]$Top = 150, [int]$Bottom = 620, [switch]$Own) {
    <# Scrolls the stream until a line containing $Text sits between $Top and $Bottom, loading
       more posts on the way. Returns its OCR line. -Own takes only a post's own text column
       (x 300-312), not the same words inside a quote card. #>
    Show-StreamTop
    $lastY = -1; $dir = 0; $flips = 0
    for ($i = 0; $i -lt 45; $i++) {
        $lines = @(Get-SimText | Where-Object { $_.x -ge 241 -and $_.x -lt 760 -and $_.y -ge 80 })
        $hit = $lines | Where-Object { $_.text -like "*$Text*" -and (-not $Own -or ($_.x -ge 300 -and $_.x -le 312)) } | Select-Object -First 1
        if ($hit) {
            # A band narrower than a wheel notch can be overshot both ways: after two turns, a
            # little either side of it will do.
            if ($flips -ge 2 -and $hit.y -ge ($Top - 40) -and $hit.y -le ($Bottom + 80)) { return $hit }
            if ($hit.y -lt $Top) {
                if ($dir -gt 0 -and $hit.y -eq $lastY) { return $hit }   # a notch up did not move it: the top of the stream
                if ($dir -lt 0) { $flips++ }
                $dir = 1; $lastY = $hit.y; Scroll 500 450 1; Start-Sleep -Milliseconds 300; continue
            }
            if ($hit.y -gt $Bottom) { if ($dir -gt 0) { $flips++ }; $dir = -1; $lastY = -1; Scroll 500 450 -1; Start-Sleep -Milliseconds 300; continue }
            return $hit
        }
        $more = $lines | Where-Object { $_.text -like '*Show more*' } | Select-Object -First 1
        if ($more) { ClickAt ($more.x + [int]($more.w / 2)) ($more.y + 6) 1400; continue }
        Scroll 500 450 -3
        Start-Sleep -Milliseconds 300
    }
    if ($env:SIM_DEBUG) { Save-SimShot -Path (Join-Path $env:SIM_DEBUG "fail-$(Get-Date -Format HHmmss)-post.png") | Out-Null }
    throw "post '$Text' not found in the stream"
}

function Get-ActionIcons([int]$CardBottom) {
    <# Heart, reply, repost, quote, references, share, clap, react: x centres and the row. #>
    $y = $CardBottom - 24
    $x = @(Get-Runs ($y - 8) ($y + 8) 300 760 9)
    if ($x.Count -lt 8) { throw "action bar not found ($($x.Count) icons at y $y)" }
    [pscustomobject]@{ y = $y; heart = $x[0]; reply = $x[1]; repost = $x[2]; quote = $x[3]; refs = $x[4]; share = $x[5]; clap = $x[6]; react = $x[7] }
}


# --- helpers for the post menu, sheets, the Thread Inspector and the profile page --------------

$script:PtGeom = Join-Path ([System.IO.Path]::GetTempPath()) 'simulacra-docs-ptgeom.py'
Set-Content -Path $script:PtGeom -Encoding utf8 -Value @'
import sys
import numpy as np
from PIL import Image

def bottom(path, x0, y0, x1, y1):
    # The row under the last ink in a region.
    a = np.asarray(Image.open(path).convert("RGB"), dtype=np.int16)
    r = a[y0:y1, x0:x1]
    bg = np.median(r.reshape(-1, 3), axis=0)
    rows = np.where((np.abs(r - bg).max(axis=2) > 24).sum(axis=1) >= 2)[0]
    print(y0 + int(rows[-1]) + 1 if rows.size else y0)

def trim(path):
    # Cuts the dimmed window off the edges of a sheet capture: keeps the rows and columns where
    # the sheet's own background colour is common, plus its 1 px border. That colour is the
    # commonest one that is not near black (the scrim darkens the window behind to near black).
    im = Image.open(path).convert("RGB")
    a = np.asarray(im, dtype=np.int16)
    px = a.reshape(-1, 3)
    px = px[px.max(axis=1) >= 12]
    if not len(px):
        return
    vals, counts = np.unique(px, axis=0, return_counts=True)
    bg = vals[counts.argmax()]
    same = np.abs(a - bg).max(axis=2) <= 3
    rows = np.where(same.mean(axis=1) > 0.3)[0]
    if not rows.size:
        return
    h, w = same.shape
    y0, y1 = max(0, rows[0] - 2), min(h, rows[-1] + 3)
    cols = np.where(same[y0:y1].mean(axis=0) > 0.3)[0]
    if not cols.size:
        return
    x0, x1 = max(0, cols[0] - 2), min(w, cols[-1] + 3)
    if x0 > 120 or y0 > 120 or w - x1 > 200 or y1 - y0 < 80 or x1 - x0 < 120:
        return   # not a sheet's edges: leave it
    im.crop((x0, y0, x1, y1)).save(path)
    print(x0, y0, x1 - x0, y1 - y0)

def edge(path, x, y, dy):
    # The first row above (dy -1) or below (dy 1) row y in column x whose colour differs from
    # (x, y): a panel's border, scanned from inside its padding.
    a = np.asarray(Image.open(path).convert("RGB"), dtype=np.int16)
    bg = a[y, x]
    while 0 < y < a.shape[0] - 1:
        y += dy
        if np.abs(a[y, x] - bg).max() > 8:
            break
    print(y)

def same(path_a, path_b, x0, y0, x1, y1):
    # 1 when two captures match inside a region (nothing there is moving), else 0.
    a = np.asarray(Image.open(path_a).convert("RGB"), dtype=np.int16)[y0:y1, x0:x1]
    b = np.asarray(Image.open(path_b).convert("RGB"), dtype=np.int16)[y0:y1, x0:x1]
    print(1 if (np.abs(a - b).max(axis=2) > 12).sum() < 6 else 0)

mode = sys.argv[1]
if mode == "same":
    same(sys.argv[2], sys.argv[3], *(int(v) for v in sys.argv[4:8]))
elif mode == "edge":
    edge(sys.argv[2], *(int(v) for v in sys.argv[3:6]))
elif mode == "bottom":
    bottom(sys.argv[2], *(int(v) for v in sys.argv[3:7]))
else:
    trim(sys.argv[2])
'@

function Get-Bottom([int]$X0, [int]$Y0, [int]$X1, [int]$Y1) {
    <# The row under the last ink in a region: where its content ends. #>
    [int](python $script:PtGeom bottom (Save-Temp) $X0 $Y0 $X1 $Y1)
}

function Wait-Calm([int[]]$Area = @(1121, 90, 1440, 340), [int]$Seconds = 4) {
    <# Waits until the right sidebar's mini mind graph stops moving (it settles for a while after
       posts change): a moving area would read as part of a submenu that opened. #>
    $a = Join-Path ([System.IO.Path]::GetTempPath()) "sim-calm-a-$PID.png"
    $b = Join-Path ([System.IO.Path]::GetTempPath()) "sim-calm-b-$PID.png"
    Save-SimShot -Path $a | Out-Null
    $end = (Get-Date).AddSeconds($Seconds)
    while ((Get-Date) -lt $end) {
        Start-Sleep -Milliseconds 500
        Save-SimShot -Path $b | Out-Null
        if ((python $script:PtGeom same $a $b @Area) -eq '1') { return }
        Copy-Item $b $a -Force
    }
    if ($env:SIM_DEBUG) { Write-Host '  the mind graph did not settle' }
}

function Shot-Sheet([string]$Id, [int[]]$Within = $script:Sheet) {
    <# A sheet or dialog opened since the last Mark, cut to its own edges. #>
    Park
    Shot $Id -Auto -AutoPad 0 -Within $Within
    python $script:PtGeom trim (Join-Path $script:RawDir "$Id.png") | Out-Null
}

function Wait-Text([string]$Text, [int[]]$Region, [int]$Seconds = 25, [switch]$Gone) {
    <# Waits until $Text shows (returns its line) or, with -Gone, until it has left. #>
    $end = (Get-Date).AddSeconds($Seconds)
    while ((Get-Date) -lt $end) {
        $hit = @(Find-SimText -Text $Text -Region $Region)
        if ($Gone -and -not $hit) { return }
        if (-not $Gone -and $hit) { return $hit[0] }
        Start-Sleep -Milliseconds 600
    }
    if (-not $Gone) { throw "'$Text' did not appear" }
}

function Reset-Tools {
    <# Closes menus, the settings window, a sheet or composer left open, shows the right sidebar,
       and goes back to the Timeline. #>
    Key '{ESC}' 250; Key '{ESC}' 250
    $lines = @(Get-SimText)
    if ($lines | Where-Object { $_.text -like '*Changes are saved as you make them*' }) { ClickAt 1215 803 900; $lines = @(Get-SimText) }
    for ($i = 0; $i -lt 3; $i++) {
        $c = @($lines | Where-Object { $_.text -match '^Cancel$' -and $_.x -ge 431 -and $_.x -le 1035 })
        if (-not $c) { break }
        ClickAt ($c[0].x + 18) ($c[0].y + 5) 800
        $lines = @(Get-SimText)
    }
    # On the Timeline first: the right sidebar is shown or hidden per place.
    Key '^1' 700
    if (-not (Test-RightSidebar)) { Key '^%b' 900 }
}

function Test-RightSidebar {
    <# Whether the right sidebar shows: its 1 px border runs down x 1120 (toasts can cover its tab
       labels, so they are no test). #>
    $bmp = [System.Drawing.Bitmap]::FromFile((Save-Temp))
    try {
        $hits = @(300, 600, 860 | Where-Object { $c = $bmp.GetPixel(1120, $_); $d = $bmp.GetPixel(1122, $_); ($c.R - $d.R) -ge 10 -and $c.R -ge 22 -and $c.R -le 40 })
    } finally { $bmp.Dispose() }
    $hits.Count -ge 2
}

function Confirm-Spend {
    <# Answers "Send this to the AI?" with Send, ticking "Don't ask again this session" first. #>
    $d = @(Find-SimText 'ask again this session')
    if (-not $d) { return }
    ClickAt ($d[0].x - 14) ($d[0].y + 7) 400
    Click 'Send' -Region ($d[0].x + 200), ($d[0].y + 20), 300, 80 -At ($d[0].x + 348), ($d[0].y + 42) -WaitMs 900
}

function Open-Menu([int]$CardTop, [int]$X = 733) {
    <# Opens the … menu of the stream card whose top separator is at $CardTop; the pointer moves
       off it so no tooltip shows. #>
    ClickAt $X ($CardTop + 24) 900
    Hover 290 ($CardTop + 24) 300
}

function Open-PostMenu([string]$Text, [object]$Card) {
    <# Opens the … menu of the stream post with $Text (Mark taken just before) and checks that it
       opened and that the card under it is still that post: the For you stream can reorder
       itself a moment after a change. $Card, from an earlier Get-Card, saves a search when it
       is still in place. #>
    $probe = $Text.Substring(0, [Math]::Min(9, $Text.Length))
    for ($try = 0; $try -lt 3; $try++) {
        # Get-Card keeps the card's header (up to ~130 px above its words) clear of the inline
        # composer; menus and submenus shift up to stay in the window.
        if (-not $Card -or $try -gt 0) { $Card = Get-Card $Text }
        Park
        Mark
        Open-Menu $Card.top
        $lines = @(Get-SimText)
        $here = @($lines | Where-Object { $_.x -ge 296 -and $_.x -le 320 -and [Math]::Abs($_.y - $Card.h.y) -le 8 -and $_.text -like "*$probe*" })
        $open = @($lines | Where-Object { $_.x -ge 590 -and $_.x -le 760 -and $_.text -like '*Find similar*' })
        if ($here -and $open) { return $Card }
        if ($env:SIM_DEBUG) {
            Write-Host "  menu of '$Text': card top $($Card.top) line $($Card.h.x),$($Card.h.y) here $([bool]$here) open $([bool]$open)"
            Save-SimShot -Path (Join-Path $env:SIM_DEBUG "fail-$(Get-Date -Format HHmmss)-menu.png") | Out-Null
        }
        Key '{ESC}' 300; Key '{ESC}' 500
    }
    throw "the menu of '$Text' could not be opened"
}

# The labels of a stream card's … menu start at x 614-630 (the menu spans x ~600-745, right-aligned
# to the … button); its submenus open to the right of it.
$script:Menu = 590, 38, 170, 862

function Menu-Item([string]$Label, [int[]]$Region = $script:Menu, [switch]$Hover, [int]$WaitMs = 900, [switch]$Exact) {
    <# Clicks (or hovers) a menu row found by its label. -Exact takes only a line that ends with
       the label as a word (Edit, not the card's "edited" mark above the menu). #>
    if ($Exact) {
        $m = @(Find-SimText -Text $Label -Region $Region | Where-Object { $_.text -match ('(^|\W)' + [regex]::Escape($Label) + '$') } | Sort-Object y) | Select-Object -Last 1
        if (-not $m) { throw "menu row '$Label' is not on screen" }
    } else { $m = Find-One $Label $Region }
    if ($Hover) { Hover ($m.x + 40) ($m.y + 6) $WaitMs } else { ClickAt ($m.x + 40) ($m.y + 6) $WaitMs }
    $m
}

function Get-SideBox([int]$X0) {
    <# What changed right of $X0 since the last Mark, leaving out the right sidebar's mini mind
       graph (x 1121 on, above y 340), which keeps moving for a while after posts change. #>
    $boxes = @()
    if ($X0 -lt 1121) { $boxes += , (Get-ChangedBox @($X0, 0, 1121, 900)) }
    $boxes += , (Get-ChangedBox @([Math]::Max($X0, 1121), 340, 1440, 900))
    $boxes = @($boxes | Where-Object { -not ($_[2] -eq 1440 -and $_[3] -eq 900) })   # no change
    if (-not $boxes) { return [int[]]@(0, 0, 1440, 900) }
    $x0 = ($boxes | ForEach-Object { $_[0] } | Measure-Object -Minimum).Minimum
    $y0 = ($boxes | ForEach-Object { $_[1] } | Measure-Object -Minimum).Minimum
    $x1 = ($boxes | ForEach-Object { $_[0] + $_[2] } | Measure-Object -Maximum).Maximum
    $y1 = ($boxes | ForEach-Object { $_[1] + $_[3] } | Measure-Object -Maximum).Maximum
    [int[]]@($x0, $y0, ($x1 - $x0), ($y1 - $y0))
}

function Find-Label([string]$Label, [int[]]$Region) {
    <# A menu row by its label, or (OCR garbles a word now and then) by the label's last or
       first seven characters (its end first: "Add to " starts two rows). #>
    # A changed box starts at the first changed text column, and OCR puts a row's icon (read as a
    # letter) a pixel or two left of it: the region is widened a little.
    $lines = @(Get-SimText | Where-Object { $_.x -ge ($Region[0] - 16) -and $_.y -ge ($Region[1] - 8) -and $_.x -le ($Region[0] + $Region[2]) -and $_.y -le ($Region[1] + $Region[3]) })
    $keys = @($Label)
    if ($Label.Length -ge 12) { $keys += $Label.Substring($Label.Length - 7), $Label.Substring(0, 7) }
    foreach ($k in $keys) {
        $hit = @($lines | Where-Object { $_.text -like "*$k*" } | Sort-Object y) | Select-Object -First 1
        if ($hit) { return $hit }
    }
    $null
}

function Open-Path([string[]]$Path, [int[]]$Region = $script:Menu) {
    <# With a menu open: hovers each label of $Path in turn, each looked up in the submenu the one
       before opened (found as what changed, so the same words in the Thread Inspector beside it
       are never taken). Returns the hovered rows, the regions they were found in and the box
       (x,y,w,h) of the last submenu. The Organize ›, Think it over ›, Copy › … submenus. #>
    $rows = @(); $regions = @(); $box = $null; $r = $Region
    Copy-Item $script:MarkPath "$($script:MarkPath).keep" -Force -ErrorAction SilentlyContinue   # Path-Item puts it back
    for ($n = 0; $n -lt $Path.Count; $n++) {
        $label = $Path[$n]
        $m = Find-Label $label $r
        if (-not $m) { throw "'$label' is not on screen" }
        # A submenu opened when its box is submenu-sized and (before the last level) holds the
        # next label; otherwise the pointer leaves the row and comes back.
        for ($try = 0; $try -lt 3; $try++) {
            Wait-Calm
            Mark 100
            Hover ($m.x + 40) ($m.y + 6) 1100
            # A submenu opens to the right of its row, or to the left when the window has no room
            # (the Thread Inspector's menus).
            foreach ($side in @('right', 'left')) {
                $box = if ($side -eq 'right') { Get-SideBox ($m.x + $m.w + 8) } else { Get-ChangedBox @(0, 0, ($m.x - 25), 900) }
                $ok = $box[2] -lt 600 -and $box[3] -lt 860
                if ($ok -and $n -lt $Path.Count - 1) { $ok = [bool](Find-Label $Path[$n + 1] $box) }
                if ($env:SIM_DEBUG) { Write-Host "    $side box $($box -join ',') ok $ok" }
                if ($ok) { break }
            }
            if ($env:SIM_DEBUG) { Write-Host "  path '$label' at $($m.x),$($m.y): box $($box -join ',') ok $ok" }
            if ($ok) { break }
            if ($env:SIM_DEBUG -and $try -eq 2) {
                Save-SimShot -Path (Join-Path $env:SIM_DEBUG "fail-$(Get-Date -Format HHmmss)-open.png") | Out-Null
                Copy-Item $script:MarkPath (Join-Path $env:SIM_DEBUG "fail-$(Get-Date -Format HHmmss)-mark.png")
                Write-Host "    lines right of the row: $((@(Get-SimText | Where-Object { $_.x -gt ($m.x + $m.w) -and $_.x -lt ($m.x + 420) -and [Math]::Abs($_.y - $m.y) -lt 300 }) | ForEach-Object { "$($_.x),$($_.y) $($_.text)" }) -join ' | ')"
            }
            Hover ($m.x + 40) ($m.y - 30) 400
        }
        if (-not $ok) {
            if ($env:SIM_DEBUG) { Save-SimShot -Path (Join-Path $env:SIM_DEBUG "fail-$(Get-Date -Format HHmmss)-path.png") | Out-Null }
            throw "the $label submenu did not open"
        }
        $rows += $m; $regions += , $r
        $r = $box
    }
    [pscustomobject]@{ rows = $rows; regions = $regions; box = $box; m = $rows[-1] }
}

function Path-Item([string[]]$Path, [string]$Label, [int[]]$Region = $script:Menu, [int]$WaitMs = 1200) {
    <# Opens the submenus of $Path and clicks $Label in the last one. #>
    $s = Open-Path $Path $Region
    $b = $s.box
    $hit = Find-Label $Label $b
    if (-not $hit) {
        # Not where the change was seen: look right of the row, nearest to it first.
        $hit = @(Find-SimText -Text $Label -Region ($s.m.x + $s.m.w + 8), ($s.m.y - 450), 500, 900) | Sort-Object { [Math]::Abs($_.y - $s.m.y) } | Select-Object -First 1
        if (-not $hit) { throw "'$Label' is not in the $($Path -join ' › ') submenu" }
    }
    ClickAt ([int]($hit.x + [Math]::Min($hit.w / 2, 60))) ($hit.y + 6) $WaitMs
    # The Mark taken before the menu opened, so a sheet the item opens is what changed.
    if (Test-Path "$($script:MarkPath).keep") { Copy-Item "$($script:MarkPath).keep" $script:MarkPath -Force }
}

function Think-Item([string]$Label, [int]$WaitMs = 1200) { Path-Item @('Think it over') $Label -WaitMs $WaitMs }

function Organize-Item([string]$Label, [int]$WaitMs = 1200) { Path-Item @('Organize') $Label -WaitMs $WaitMs }

function Shot-Submenu([string]$Id, [string[]]$Path, [int[]]$Region = $script:Menu, [int]$Left = 597, [switch]$Chain) {
    <# With a menu open and no submenu showing: opens the submenus of $Path and keeps the last one
       with the menu it hangs from, around its row (a row above it). The row's hover tint is too
       faint to register as a change, so that menu's left edge is $Left for the top menu (a
       stream card's menu starts at x ~600) or the box of the submenu before. #>
    $s = Open-Path $Path $Region
    $m = $s.m
    $c = $s.box
    if ($env:SIM_DEBUG) { Save-SimShot -Path (Join-Path $env:SIM_DEBUG "submenu-$Id.png") | Out-Null }
    # The menu holding the hovered row: the submenu before (its text starts ~11 px inside its
    # border), or the top menu from $Left to a little past the row's label. -Chain keeps every
    # menu of the path from the top one (when the submenus between are one row).
    if ($Path.Count -gt 1) { $p = $s.regions[-1]; $pl = $p[0] - 12; $pr = $p[0] + $p[2] + 12 } else { $pl = $Left; $pr = $m.x + $m.w + 20 }
    if ($Chain) { $pl = $Left }
    $x0 = [Math]::Max(0, [Math]::Min($c[0] - 6, $pl)); $x1 = [Math]::Min(1440, [Math]::Max($c[0] + $c[2] + 6, $pr))
    # A row above the hovered one in the top menu; in a nested submenu only the hovered row, as
    # the window beside the submenu would show above it.
    $above = if ($Path.Count -gt 1) { 9 } else { 36 }
    $pad = if ($Path.Count -gt 1) { 3 } else { 6 }
    $y0 = [Math]::Max(0, [Math]::Min($c[1], $m.y - $above) - $pad); $y1 = [Math]::Min(900, [Math]::Max($c[1] + $c[3], $m.y + 24) + 4)
    Shot $Id -Crop $x0, $y0, ($x1 - $x0), ($y1 - $y0)
}

function Shot-Menu([string]$Id) {
    <# The … menu of a stream card, open with no submenu showing: from its Actions row to Delete. #>
    $a = Find-One 'Actions' $script:Menu
    $d = @(Find-SimText 'Delete' -Region $script:Menu | Where-Object { $_.text -match 'Delete$' } | Sort-Object y) | Select-Object -Last 1
    if (-not $d) { throw 'the Delete row of the menu is not on screen' }
    # Its 1 px border runs x 601-743; top and bottom are found from just above the Actions label
    # and just below the Delete one, clear of the rounded corners.
    $p = Save-Temp
    $top = [int](python $script:PtGeom edge $p 650 ($a.y - 5) -1)
    $bottom = [int](python $script:PtGeom edge $p 650 ($d.y + 17) 1)
    Shot $Id -Crop 601, $top, 143, ($bottom - $top + 1)
}

function Find-Seed { Find-Post $script:Seed -Own }

function Get-Card([string]$Text, [switch]$Any, [int]$Top = 300, [int]$Bottom = 620) {
    <# A post of the stream by its words: its line, and its card's top and bottom found from the
       author lines around it (the card above is the one whose header is nearest above the words;
       separators between cards are not reliable once a card is selected). -Any takes the words
       anywhere in a line, not only at the start of a post's text column. #>
    $h = if ($Any) { Find-Post $Text $Top $Bottom } else { Find-Post $Text $Top $Bottom -Own }
    $lines = @(Get-SimText | Where-Object { $_.x -ge 296 -and $_.x -le 320 -and $_.text -match $script:Authors })
    $hdr = @($lines | Where-Object { $_.y -lt $h.y -and $_.y -gt ($h.y - 420) } | Sort-Object y) | Select-Object -Last 1
    $next = @($lines | Where-Object { $_.y -gt $h.y } | Sort-Object y) | Select-Object -First 1
    $b = Get-CardBox $h.y
    $top = if ($hdr) { $hdr.y - 18 } else { $b[0] }
    $bottom = if ($next) { $next.y - 20 } else { $b[1] }
    [pscustomobject]@{ h = $h; top = $top; bottom = $bottom }
}

function Refresh-Card([object]$Card) {
    <# The same card after its height changed in place (its top stays). #>
    $next = @(Get-SimText | Where-Object { $_.x -ge 296 -and $_.x -le 320 -and $_.text -match $script:Authors -and $_.y -gt ($Card.top + 40) } | Sort-Object y) | Select-Object -First 1
    $bottom = if ($next) { $next.y - 20 } else { (Get-CardBox ($Card.top + 40))[1] }
    [pscustomobject]@{ h = $Card.h; top = $Card.top; bottom = $bottom }
}

function Shot-Post([string]$Id, [object]$Card) {
    <# One stream card, from Get-Card. #>
    Park
    Shot $Id -Crop 242, ($Card.top + 1), 517, ($Card.bottom - $Card.top - 1)
}

function Sheet-Click([string]$Label, [int]$WaitMs = 900, [int]$MinY = 0) {
    <# Clicks a label inside the open sheet: x 405-1035, below $MinY and above its Cancel button
       (OCR also reads the dimmed window around the sheet, and a click there closes it). #>
    $lines = @(Get-SimText)
    $cancel = @($lines | Where-Object { $_.text -match '^(Cancel|Close)\b' -and $_.x -ge 840 -and $_.x -le 1035 } | Sort-Object y)
    $maxY = if ($cancel) { $cancel[0].y - 6 } else { 900 }
    $hits = @($lines | Where-Object { $_.text -like "*$Label*" -and $_.x -ge 405 -and $_.x -le 1035 -and $_.y -ge $MinY -and $_.y -lt $maxY } | Sort-Object y)
    if (-not $hits) { throw "'$Label' is not in the sheet" }
    $hit = $hits[-1]
    ClickAt ([int]($hit.x + [Math]::Min($hit.w / 2, 120))) ($hit.y + 6) $WaitMs
    $hit
}

function Sheet-Button([string]$Label, [int]$WaitMs = 1500) {
    <# The sheet's action button at its bottom right (its title may hold the same word). OCR can
       read Cancel and the button as one line, so the word is found inside the line. #>
    $hits = @(Find-SimText -Text $Label -Region 700, 150, 335, 750) | Where-Object { $_.text -match ('(^|\W)' + [regex]::Escape($Label) + '$') } | Sort-Object y
    if (-not $hits) {
        # OCR can misread a label beside its icon (Fork, Start the series), even inside one line
        # with Cancel: the button starts right after Cancel.
        $c = @(Find-SimText -Text 'Cancel' -Region 700, 150, 335, 750 | Sort-Object y) | Select-Object -First 1
        if (-not $c) { throw "button '$Label' not found" }
        $at = $c.text.IndexOf('Cancel', [StringComparison]::OrdinalIgnoreCase)
        $x = if ($Label -eq 'Cancel') { [int]($c.x + $c.w * ($at + 3) / $c.text.Length) } else { [int]($c.x + $c.w * ($at + 6) / $c.text.Length) + 48 }
        ClickAt $x ($c.y + 6) $WaitMs
        return
    }
    $hit = $hits[0]   # dimmed text under the sheet comes later
    $at = $hit.text.LastIndexOf($Label, [StringComparison]::OrdinalIgnoreCase)
    ClickAt ([int]($hit.x + $hit.w * ($at + $Label.Length / 2) / $hit.text.Length)) ($hit.y + 6) $WaitMs
}

function Post-Inline([string]$Text, [int]$WaitMs = 2500) {
    <# Publishes a post from the stream's inline composer. #>
    Reset-Tools
    Show-StreamTop
    ClickAt 520 112 400
    Key '^a' 150; Key '{BACKSPACE}' 300
    Write-Text $Text 600
    Key '^{ENTER}' $WaitMs
}

function New-SymbolPost([string]$Text, [int[]]$Symbols) {
    <# A post with core symbols (x in the Symbols row: Task 507, Event 530, Note 553, Priority 576,
       Inspiration 599, Explore 622), behind the footer's Tint and symbols button. #>
    Open-Composer -Style
    Write-Text $Text
    $s = Find-One 'Symbols' (431, 0, 578, 900)
    foreach ($x in $Symbols) { ClickAt $x ($s.y + 6) 400 }
    Submit-Composer
}

function New-Checklist([string]$Title, [string[]]$Lines) {
    <# A post of to-do lines, typed so the composer's own list continuation is replaced. #>
    Reset-Tools
    Open-Composer
    Write-Text $Title 300
    foreach ($line in $Lines) {
        Key '{ENTER}' 300
        Key '+{HOME}' 150
        TypeText $line 500
        Key '{ESC}' 300
    }
    Start-Sleep -Milliseconds 1000
    Submit-Composer 2500
}

function Shot-Wide([string]$Id) {
    <# The Thread Inspector with the right sidebar hidden (x 761-1440), down to where its content ends. #>
    Park
    $bottom = Get-Bottom 766 82 1434 850
    Shot $Id -Crop 761, 38, 679, ([Math]::Min(850, $bottom + 18) - 38)
}

function Open-Wide([string]$Text) {
    <# Hides the right sidebar, so the inspector shows its title and offers all sixteen views, and opens
       the thread of the post with $Text. #>
    Reset-Tools
    if (Test-RightSidebar) { Key '^%b' 900 }
    if (Test-RightSidebar) { throw 'the right sidebar would not hide' }
    $h = Find-Post $Text -Own
    ClickAt ($h.x + 40) ($h.y + 6) 1600
}

# The View menu's entries in their order (a profile offers all but outline, argument map,
# what-ifs and lens fan, so the fallback below uses the nearest entry before that both offer).
$script:Views = 'Show as cards', 'Show as a rapid log', 'Show as raw text', 'Show as a calendar', 'Show the future log',
    'Show as a board', 'Garden', 'Series', 'Still true?', 'Glossary', 'Open questions', 'Changed my mind'

function Pick-View([string]$Entry, [int]$X, [int]$Y) {
    <# Opens the View ▾ button at $X,$Y and clicks the entry named $Entry in its menu ("Show as
       cards", "Show as outline", "Show as raw text", "Show as a rapid log", "Show as a calendar",
       "Show the future log", "Show as a board", "Garden", "What-ifs side by side", "Series",
       "Still true?", "Lens fan", "Glossary", "Open questions", "Changed my mind"). A first click
       can go unheard while a thread is still opening. #>
    for ($i = 0; $i -lt 3; $i++) {
        Park
        Mark 200
        ClickAt $X $Y 1000
        $b = Get-ChangedBox (241, 38, 1440, 900)
        # Widened a little: OCR puts an entry's icon (read as a letter) left of the changed box.
        $lines = @(Get-SimText | Where-Object { $_.x -ge ($b[0] - 16) -and $_.y -ge ($b[1] - 8) -and $_.x -le ($b[0] + $b[2]) -and $_.y -le ($b[1] + $b[3]) })
        # OCR can garble a word of an entry: fall back on its longer words ("questions").
        $keys = @($Entry) + @($Entry -split '\W+' | Where-Object { $_.Length -ge 5 })
        $hit = $null
        foreach ($k in $keys) {
            $hit = @($lines | Where-Object { $_.text -like "*$k*" } | Sort-Object y) | Select-Object -First 1
            if ($hit) { break }
        }
        if ($env:SIM_DEBUG -and -not $hit) { Write-Host "  view menu box $($b -join ','): $(($lines | ForEach-Object { $_.text }) -join ' | ')" }
        if ($hit) { ClickAt ($hit.x + 40) ($hit.y + 6) 1500; return }
        # OCR can drop a whole row ("Open questions"): it is the row under the entry before it in
        # the menu's order, when that one is read (rows are 28 px apart).
        $k = [array]::IndexOf($script:Views, $Entry)
        for ($j = $k - 1; $j -ge 0 -and -not $hit; $j--) {
            $prev = @($lines | Where-Object { $_.text -like "*$($script:Views[$j])*" }) | Select-Object -First 1
            if ($prev) { ClickAt ($prev.x + 40) ($prev.y + 6 + 28 * ($k - $j)) 1500; return }
        }
        Key '{ESC}' 400
    }
    if ($env:SIM_DEBUG) { Save-SimShot -Path (Join-Path $env:SIM_DEBUG "fail-$(Get-Date -Format HHmmss)-view.png") | Out-Null }
    throw "'$Entry' is not in the View menu"
}

function Thread-View([string]$Entry) {
    <# A view of the wide Thread Inspector (right sidebar hidden): its View ▾ button is at the
       right end of the header. #>
    Pick-View $Entry 1392 60
}

function Open-Profile {
    <# Arya Wiratama's page (My Personas, the first persona), Posts tab. #>
    Reset-Tools
    Key '^4' 1300
    if (Test-RightSidebar) { Key '^%b' 900 }   # the page is laid out without it
    Click 'Arya Wiratama' -Region 241, 100, 243, 120 -At 347, 142 -WaitMs 1000
    Click 'Posts' -Region 840, 50, 120, 40 -At 901, 71 -WaitMs 1600
}

function Profile-View([string]$Entry) {
    <# A view of the profile's Posts tab: its View ▾ button sits at the right end of the tab's
       toolbar, after Feelings only (OCR seldom reads its short dim label). #>
    Pick-View $Entry 1268 114
}

function Shot-Profile([string]$Id) {
    <# The list area of the Posts tab (x 622-1302), down to where its content ends. #>
    Park
    $bottom = Get-Bottom 624 132 1300 898
    Shot $Id -Crop 616, 128, 692, ([Math]::Min(900, $bottom + 14) - 128)
}

# --- the scene ---------------------------------------------------------------------------------

Step 'welcome' -Always { Skip-Welcome }

Step 'prepare' -Always {
    if (-not (Get-NetTCPConnection -LocalPort 8765 -State Listen -ErrorAction SilentlyContinue)) {
        Start-Process node -ArgumentList (Join-Path $PSScriptRoot '..\stub-ai.mjs') -WindowStyle Hidden
        Start-Sleep -Seconds 2
    }
}

Step 'menu' {
    # An own post's menu: Actions ›, Reply, Quote, Find similar, Copy ›, Learn ›, Set a credence…,
    # Edit, Bookmark, Organize ›, Think it over ›, Delete.
    Reset-Tools
    $s = Open-PostMenu $script:Seed
    Shot-Menu 'tools-post-menu'
    Shot-Submenu 'tools-actions-submenu' @('Actions')
    Key '{ESC}' 300; Key '{ESC}' 500
    $s = Open-PostMenu $script:Seed $s
    Shot-Submenu 'lifecycle-auto-delete-submenu' @('Organize', 'Delete automatically')
    Key '{ESC}' 300; Key '{ESC}' 500
}

Step 'still true' {
    Reset-Tools
    Shot-Post 'tools-still-true-strip' (Get-Card $script:Seed)
    Open-Profile
    Park
    Shot 'view-switcher-profile' -Crop 614, 96, 696, 34
    Profile-View 'Still true?'
    Shot-Profile 'view-still-true'
    Profile-View 'Show as cards'
}

Step 'bookmarks' {
    # The seed post is bookmarked from the palette; Sokrates' reply is bookmarked from its menu and
    # filed in a new collection (New collection… opens the Saved page with its name field).
    Reset-Tools
    $h = Find-Seed
    ClickAt ($h.x + 40) ($h.y + 6) 1500
    Key '^+p' 700; TypeText 'Bookmark this post' 700; Key '{ENTER}' 1500
    $t = Open-PostMenu 'Tell me, my friend'
    Menu-Item 'Bookmark' -WaitMs 1200 | Out-Null
    $t = Open-PostMenu 'Tell me, my friend'
    Path-Item @('Organize', 'Add to collection') 'New collection' -WaitMs 1800
    Write-Text 'Reading list' 600
    $c = Find-One 'Create' (241, 90, 1199, 80)
    ClickAt ($c.x + 20) ($c.y + 6) 1500
    $n = Find-One 'New collection name' (241, 90, 1199, 80)
    ClickAt ($n.x + 60) ($n.y + 6) 400
    Key '^a' 150
    Write-Text 'To revisit' 600
    ClickAt ($c.x + 20) ($c.y + 6) 1500
    Click 'Bookmarks' -Region 241, 180, 400, 40 -At 300, 200 -WaitMs 1200
    Start-Sleep -Seconds 4   # toasts fade
    Park
    Shot 'tools-saved-view'
    Reset-Tools
    $t = Open-PostMenu 'Tell me, my friend'
    Shot-Submenu 'tools-collection-submenu' @('Organize', 'Add to collection') -Chain   # an agent's post: Organize › holds only Add to collection
    Key '{ESC}' 300; Key '{ESC}' 300; Key '{ESC}' 500
}

Step 'inspector' {
    Open-Wide $script:Seed
    Park
    Shot 'view-switcher-thread' -Crop 761, 38, 679, 42
    Thread-View 'Show as raw text'
    Shot-Wide 'view-raw'
    Thread-View 'Show as cards'
    Reset-Tools
}

Step 'actions' {
    Reset-Tools
    $s = Open-PostMenu $script:Seed
    Path-Item @('Actions') 'Steelman' -WaitMs 1500
    Confirm-Spend
    Wait-Text 'Re-run' (300, 38, 840, 862) | Out-Null
    Park
    Shot-Sheet 'tools-actions-sheet' 280, 38, 1140, 880
    $box = $script:LastBox
    $right = $box[0] + $box[2]
    # The answer pane starts right of the rail (x ~ 552).
    Click 'Explain for' -Region 340, 38, 200, 862 -WaitMs 1500
    Confirm-Spend
    Wait-Text 'Re-run' (300, 38, 840, 862) | Out-Null
    Park
    $t = Find-One 'Explain for' (545, 38, 600, 300)
    $f = Find-One 'describe the reader' (545, 38, 600, 500)
    Shot 'tools-actions-choices' -Crop 553, ($t.y - 18), ($right - 553), ($f.y + 28 - ($t.y - 18))
    Click 'Search a concept' -Region 340, 38, 200, 862 -WaitMs 1500
    Confirm-Spend
    Wait-Text 'Re-run' (300, 38, 840, 862) | Out-Null
    Start-Sleep -Milliseconds 800
    Park
    $t = Find-One 'Search a concept' (545, 38, 600, 300)
    # Down to the buttons under the answer (Copy … Re-run), or to the sheet's edge.
    $r = @(Find-SimText 'Re-run' -Region 545, ($t.y + 40), 700, 850) | Sort-Object y | Select-Object -Last 1
    $bottom = if ($r) { [Math]::Min($box[1] + $box[3], $r.y + 30) } else { $box[1] + $box[3] }
    Shot 'tools-actions-concept' -Crop 553, ($t.y - 18), ($right - 553), ($bottom - ($t.y - 18))
    python $script:PtGeom trim (Join-Path $script:RawDir 'tools-actions-concept.png') | Out-Null   # not past the sheet's edge
    Key '{ESC}' 800
}

Step 'margin' {
    Reset-Tools
    $s = Open-PostMenu $script:Seed
    Think-Item 'Reply in the margin'
    Sheet-Click 'Is symbol prediction isomorphic' | Out-Null
    Sheet-Click 'What do you make of' 400 | Out-Null
    Write-Text 'Only if the model has a world to model.' 900
    Shot-Sheet 'tools-margin-sheet'
    Sheet-Button 'Cancel' 800
}

Step 'outline' {
    # Two replies: one under the seed post, one under Sokrates' reply, so the outline nests.
    Reset-Tools
    $s = Get-Card $script:Seed
    $ic = Get-ActionIcons $s.bottom
    ClickAt $ic.reply $ic.y 1200
    Write-Text 'A model that can be surprised is closer to understanding than one that only predicts.'
    Submit-Composer 2500
    $t = Get-Card 'Tell me, my friend'
    $ic = Get-ActionIcons $t.bottom
    ClickAt $ic.reply $ic.y 1200
    Write-Text 'A mirror is never surprised by the sun. That is the difference I mean.'
    Submit-Composer 2500
    Open-Wide $script:Seed
    Thread-View 'Show as outline'
    Shot-Wide 'view-outline'
    Thread-View 'Show as cards'
    Reset-Tools
}

Step 'fork' {
    Reset-Tools
    $s = Open-PostMenu $script:Seed
    Think-Item 'Fork a what-if'
    Write-Text 'What if meaning comes before symbols?'
    Sheet-Click 'Where does it lead' 400 | Out-Null
    Write-Text 'Then prediction would follow understanding, not build it.' 900
    Shot-Sheet 'tools-branch-sheet'
    Sheet-Button 'Fork' 2500
    Open-Wide $script:Seed
    Thread-View 'What-ifs side by side'
    Shot-Wide 'view-branches'
    Thread-View 'Show as cards'
    Reset-Tools
}

Step 'lens' {
    Reset-Tools
    $s = Open-PostMenu $script:Seed
    Think-Item 'Lens fan'
    Shot-Sheet 'tools-lens-sheet'
    Sheet-Button 'Ask' 1500
    Confirm-Spend
    Start-Sleep -Seconds 12
    Reset-Tools
    $s = Get-Card $script:Seed
    $l = Find-One 'Lens fan' (296, $s.top, 460, 200)
    Park
    Shot 'tools-lens-strip' -Crop 242, ($s.top + 1), 517, ($l.y + 24 - ($s.top + 1))
    Open-Wide $script:Seed
    Thread-View 'Lens fan'
    Shot-Wide 'view-lens-fan'
    Thread-View 'Show as cards'
    Reset-Tools
}

Step 'glossary' {
    Reset-Tools
    $s = Open-PostMenu $script:Seed
    Think-Item 'Define a term'
    Write-Text 'semiotics'
    Sheet-Click 'What it means' 400 | Out-Null
    Write-Text 'The study of signs and how they come to mean.' 900
    Shot-Sheet 'tools-glossary-sheet'
    Sheet-Button 'Define' 1500
    Post-Inline 'Semiotics keeps coming back in this network.'
    $g = Find-Post 'keeps coming back' -Own
    Park
    Mark
    Hover ($g.x + 30) ($g.y + 8) 1600
    $c = Get-ChangedBox (241, 38, 1121, 900)
    $x0 = [Math]::Min(296, $c[0] - 8); $x1 = [Math]::Max(756, $c[0] + $c[2] + 8)
    $y0 = [Math]::Min($g.y - 12, $c[1] - 8); $y1 = [Math]::Max($g.y + 28, $c[1] + $c[3] + 8)
    Shot 'tools-glossary-hover' -Crop $x0, $y0, ($x1 - $x0), ($y1 - $y0)
    Park
    # A second term, defined in the new post.
    Open-PostMenu 'keeps coming back' | Out-Null
    Think-Item 'Define a term'
    Write-Text 'isomorphism'
    Sheet-Click 'What it means' 400 | Out-Null
    Write-Text 'A structure-preserving match between two systems.' 900
    Sheet-Button 'Define' 1500
}

Step 'translation' {
    Reset-Tools
    $s = Open-PostMenu $script:Seed
    Think-Item 'Show in Esp'
    Confirm-Spend
    Start-Sleep -Seconds 2
    Wait-Text 'Translating' (241, 38, 520, 862) -Gone | Out-Null
    Start-Sleep -Seconds 1
    Shot-Post 'tools-translation-line' (Refresh-Card $s)
    Park
    Mark
    # OCR reads "Show original" and "Send words to review…" as one line: click its right end.
    $w = Find-One 'Send words' (296, $s.top, 460, 200)
    ClickAt ($w.x + $w.w - 40) ($w.y + 6) 1200
    # The words are buttons in rows, each read as "+ word" (a picked one shows a tick): click
    # the first, third and fifth word of the first row.
    $e = Find-One 'Pick the words' (405, 0, 630, 900)
    $row = @(Get-SimText | Where-Object { $_.x -ge 405 -and $_.x -le 1035 -and $_.y -gt ($e.y + 14) -and $_.text -match '^\W{0,2} ?\w+ \W{1,2} \w+' } | Sort-Object y) | Select-Object -First 1
    if (-not $row) { throw 'no word buttons found' }
    $tokens = $row.text -split ' '
    $words = @(for ($k = 0; $k -lt $tokens.Count; $k++) { if ($tokens[$k] -match '\w') { $k } })
    foreach ($i in 0, 2, 4) {
        if ($i -ge $words.Count) { break }
        $k = $words[$i]
        $before = ($tokens[0..$k] -join ' ').Length - $tokens[$k].Length / 2
        ClickAt ([int]($row.x + $row.w * $before / $row.text.Length)) ($row.y + 7) 500
    }
    Shot-Sheet 'tools-words-sheet'
    Sheet-Button 'Cancel' 800
    Open-Menu $s.top   # the post reads as its translation now, so it is not looked up by its words
    Think-Item 'Show original'
}

Step 'changed mind' {
    Post-Inline 'Symbol prediction models text, not the world the text is about. Understanding needs the second.'
    $p = Open-PostMenu 'Symbol prediction models text'
    Think-Item 'Changed my mind'
    Write-Text 'Symbol prediction mirrors how people model the world.'
    Sheet-Click 'Find one of your posts' 400 | Out-Null
    Write-Text 'semiotics' 1800
    Sheet-Click 'language constructs' 800 | Out-Null   # OCR tends to drop 'Exploring' after the quote icon
    Shot-Sheet 'tools-mind-sheet'
    Sheet-Button 'Mark' 1800
    Shot-Post 'tools-mind-banner' (Get-Card 'Symbol prediction models text')
}

Step 'questions' {
    Post-Inline 'What would show that a model understands the world, not just the text?'
    $q = Open-PostMenu 'What would show that a model'
    Think-Item 'Make it an open question'
    Shot-Post 'tools-open-question-chip' (Get-Card 'What would show that a model')
    Post-Inline 'Is meaning in the speaker, the listener or between them?'
    $q = Open-PostMenu 'Is meaning in the speaker'
    Think-Item 'Make it an open question'
    Park
    Mark
    Key '^+p' 700; TypeText 'Open questions' 700; Key '{ENTER}' 1500
    Shot-Sheet 'tools-open-questions-page'
    Key '{ESC}' 700
}

Step 'ask room' {
    Reset-Tools
    $q = Open-PostMenu 'What would show that a model'
    Think-Item 'Ask the room' 1500
    Confirm-Spend
    Start-Sleep -Seconds 15
    Open-Wide 'What would show that a model'
    $a = @(Find-SimText 'Accept' -Region 761, 38, 679, 812) | Where-Object { $_.text -match '^Accept$' } | Sort-Object y
    if (-not $a) { throw 'no answer to accept' }
    ClickAt ([int]($a[0].x + $a[0].w / 2)) ($a[0].y + 6) 1500
    Open-Wide 'What would show that a model'
    Park
    $r = @(Find-SimText 'Related thoughts' -Region 761, 38, 679, 812)
    if ($r) { Shot 'tools-ask-room-answers' -Crop 761, 38, 679, ($r[0].y - 16 - 38) } else { Shot-Wide 'tools-ask-room-answers' }
    Reset-Tools
}

Step 'series' {
    Reset-Tools
    $s = Open-PostMenu $script:Seed
    Path-Item @('Organize', 'Add to a series') 'New series'
    Write-Text 'Notes on meaning'
    $p = Find-One 'Parts planned' (405, 0, 630, 900)
    ClickAt 600 ($p.y + 28) 400
    Write-Text '10' 900
    Shot-Sheet 'tools-series-sheet'
    Sheet-Button 'Start the series' 1500
    foreach ($text in 'Symbol prediction models text', 'keeps coming back') {
        $p = Open-PostMenu $text
        Path-Item @('Organize', 'Add to a series') 'Notes on meaning' -WaitMs 1500
    }
}

Step 'audience' {
    Reset-Tools
    $s = $null
    foreach ($name in 'Sokrates of Alopece', 'Simone Weil') {
        $s = Open-PostMenu $script:Seed $s
        Path-Item @('Organize', 'Who can read this') $name
        Key '{ESC}' 300; Key '{ESC}' 300; Key '{ESC}' 500
    }
    $s = Open-PostMenu $script:Seed $s
    Shot-Submenu 'tools-audience-submenu' @('Organize', 'Who can read this')
    Key '{ESC}' 300; Key '{ESC}' 300; Key '{ESC}' 500
}

Step 'history' {
    # Two edits: words added, then a word replaced; the opening words stay so the post is found.
    Reset-Tools
    $s = Open-PostMenu $script:Seed
    Menu-Item 'Edit' -Exact -WaitMs 1400 | Out-Null
    Focus-Editor
    Key '^{HOME}' 200; Key '{RIGHT 66}' 300
    TypeText ' and shared' 1200
    Key '^{ENTER}' 2000
    $s = Open-PostMenu $script:Seed
    Menu-Item 'Edit' -Exact -WaitMs 1400 | Out-Null
    Focus-Editor
    Key '^{HOME}' 200; Key '{RIGHT 34}' 200; Key '+{RIGHT 8}' 200
    TypeText 'public' 1200
    Key '^{ENTER}' 2000
    # The history: … › Organize › History (2).
    $s = Open-PostMenu $script:Seed
    Organize-Item 'History' 1500
    Park
    Shot-Sheet 'tools-history-sheet' 241, 0, 1200, 900
    Key '{ESC}' 700
}

Step 'depth dial' {
    # After the edits, so the history shows them rather than the rendering this keeps as a version.
    Reset-Tools
    $h = Find-Seed
    ClickAt ($h.x + 40) ($h.y + 6) 1500
    $d = Find-One 'Simpler' (761, 38, 360, 862)
    ClickAt 812 ($d.y + 6) 900
    $a = Find-One 'render it' (761, 38, 360, 862)
    ClickAt ($a.x + 60) ($a.y + 6) 1200
    Confirm-Spend
    $r = Wait-Text 'Render again' (761, 38, 360, 862)
    Park
    $d = Find-One 'Simpler' (761, 38, 360, 862)
    Shot 'tools-depth-dial' -Crop 762, ($d.y - 52), 358, ($r.y + 26 - ($d.y - 52))
}

Step 'distill' {
    Reset-Tools
    $s = Open-PostMenu $script:Seed
    Think-Item 'Distill thread' 2500
    $s = Get-Card $script:Seed
    Wait-Text 'Distilled to' (296, $s.top, 460, 320) 12 | Out-Null
    $s = Get-Card $script:Seed
    $d = Find-One 'Distilled to' (296, $s.top, 460, 320)
    Park
    Shot 'tools-distill-chip' -Crop 242, ($s.top + 1), 517, ($d.y + 20 - ($s.top + 1))
}

Step 'retract' {
    Reset-Tools
    $s = Open-PostMenu $script:Seed
    Think-Item 'Retract'
    Write-Text 'Prediction and modelling are different tasks; the comparison does not hold.' 900
    Shot-Sheet 'tools-correction-sheet'
    Sheet-Button 'Retract' 2000
    Shot-Post 'tools-correction-banner' (Get-Card $script:Seed)
}

Step 'lifecycle' {
    Post-Inline 'A note that will not last.'
    $p = Open-PostMenu 'A note that will not last'
    $d = @(Find-SimText 'Delete' -Region $script:Menu | Where-Object { $_.text -match 'Delete$' } | Sort-Object y) | Select-Object -Last 1
    if (-not $d) { throw 'the Delete row is not in the menu' }
    ClickAt ($d.x + 30) ($d.y + 6) 1000
    if (-not @(Find-SimText 'Delete this thought')) { throw 'the delete confirmation did not open' }
    Shot-Sheet 'lifecycle-delete-dialog' 300, 0, 1140, 900
    Click 'Cancel' -Region 300, 0, 840, 600 -WaitMs 700
    $p = Open-PostMenu 'A note that will not last' $p
    Path-Item @('Organize', 'Delete automatically') 'In an hour' -WaitMs 1500
    $p = Get-Card 'A note that will not last'
    Park
    Shot 'lifecycle-countdown-chip' -Crop 242, ($p.top + 1), 517, 40
}

Step 'locked' {
    Reset-Tools
    Open-Composer
    Write-Text 'Notes for the essay, not ready to share.'
    $f = Get-Footer
    ClickAt $f.lock $f.y 900
    $pw = 'copper lantern violet harbor'
    $p = Find-One 'Password for this thread' (431, 0, 578, 900)
    ClickAt ($p.x + 100) ($p.y + 6) 300
    TypeText $pw 600
    $r = Find-One 'Type it again' (431, 0, 578, 900)
    ClickAt ($r.x + 100) ($r.y + 6) 300
    TypeText $pw 1200
    Submit-Composer 4000
    $c = Get-Card 'Locked post' -Any
    Shot-Post 'tools-locked-card' $c
    $h = $c.h
    Mark
    Click 'Unlock' -Region 296, ($h.y - 10), 460, 80 -WaitMs 1000
    Shot-Sheet 'tools-unlock-sheet'
    Sheet-Button 'Cancel' 700
}

Step 'log posts' {
    # Posts for the profile views: a task and a note (rapid log), a scheduled post and dated
    # to-dos (future log), and to-dos in every state (board).
    Reset-Tools
    New-SymbolPost 'Reread the chapter on signs' 507
    New-SymbolPost 'A sign is a habit, not a label' 553
    Open-Composer
    Write-Text 'Revisit the semiotics notes with fresh eyes.'
    Key '{ESC}' 600
    $f = Get-Footer
    ClickAt $f.schedule $f.y 900
    $pre = Find-One 'In an hour' (431, 0, 578, 900)
    ClickAt ($pre.x + 205) ($pre.y + 6) 900   # In a week
    Submit-Composer 2500
    New-Checklist 'Reading for the essay' '[ ] Reread Peirce ^fri', '[ ] Draft the essay ^+10d'
    New-Checklist 'Essay board' '[ ] Draft the outline ^fri !!', '[<] Email the reading group', '[x] Read chapter 2', '[~] Rewrite the intro'
    Fold-Style
}

Step 'garden' {
    Reset-Tools
    $s = Open-PostMenu $script:Seed
    Organize-Item 'Plant in the garden' 1500
    $g = Open-PostMenu 'keeps coming back'
    Organize-Item 'Plant in the garden' 1500
    # The planted post opens from the Garden view of the profile, in the Timeline's inspector.
    Open-Profile
    Profile-View 'Garden'
    Click 'Exploring how language' -Region 616, 128, 692, 772 -WaitMs 1800
    $a = Find-One 'Arya Wiratama' (761, 80, 360, 120)
    ClickAt 1094 ($a.y + 5) 900
    Hover 300 880 300
    # The inspector card's menu starts at x ~950.
    Shot-Submenu 'tools-garden-submenu' @('Organize', 'Garden') -Region 930, 38, 190, 862
    Path-Item @('Organize', 'Garden') 'Grow to Budding' -Region 930, 38, 190, 862 -WaitMs 1500
}

Step 'profile views' {
    Open-Profile
    Profile-View 'Show as a rapid log'
    Shot-Profile 'view-rapid-log'
    Profile-View 'Show as a calendar'
    Shot-Profile 'view-calendar'
    Profile-View 'Show the future log'
    Shot-Profile 'view-future'
    Profile-View 'Show as a board'
    Shot-Profile 'view-board'
    Profile-View 'Garden'
    Shot-Profile 'view-garden'
    Profile-View 'Series'
    Shot-Profile 'view-series'
    Profile-View 'Glossary'
    Shot-Profile 'view-glossary'
    Profile-View 'Open questions'
    Shot-Profile 'view-open-questions'
    Profile-View 'Changed my mind'
    Shot-Profile 'view-mind-changes'
    Profile-View 'Show as cards'
}

Step 'cadence' {
    Reset-Tools
    Click 'Cadence' -Region 1121, 45, 319, 45 -At 1206, 68 -WaitMs 900
    Park
    $c = Find-One 'THOUGHT CADENCE' (1121, 38, 319, 862)
    $m = Find-One 'More' (1300, $c.y, 140, 300)
    Shot 'view-cadence-heatmap' -Crop 1121, ($c.y - 20), 319, ($m.y + 26 - ($c.y - 20))
    # Today is in the last column; its rows run Sunday to Saturday, 12 px apart (Monday ~92 px
    # under the title).
    ClickAt 1422 ($c.y + 80 + 12 * [int](Get-Date).DayOfWeek) 1500
    Park
    $chip = Find-One 'Showing' (241, 38, 520, 120)
    $x0 = [Math]::Max(244, $chip.x - 60)   # not the column border at x 241
    Shot 'view-day-filter-chip' -Crop $x0, ($chip.y - 26), ($chip.x + $chip.w + 90 - $x0), 56
    ClickAt ($chip.x + $chip.w + 12) ($chip.y + 6) 900
}

function Set-FeedBudget([int]$Steps) {
    <# Settings > Reading & Writing > Daily feed budget: $Steps clicks on + (or - when negative),
       10 posts a day each, from No limit. #>
    Key '^,' 1200
    Click 'Reading & Writing' -Region 180, 120, 220, 600 -WaitMs 1000
    $r = $null
    for ($i = 0; $i -lt 6 -and -not $r; $i++) {
        $r = @(Find-SimText 'Daily feed budget' -Region 400, 38, 860, 800) | Select-Object -First 1
        if (-not $r) { Scroll 830 500 -5 }
    }
    if (-not $r) { throw 'Daily feed budget not found' }
    $x = if ($Steps -gt 0) { 1216 } else { 1108 }
    for ($i = 0; $i -lt [Math]::Abs($Steps); $i++) { ClickAt $x ($r.y + 21) 500 }
    Key '{ESC}' 900   # Settings takes the keyboard focus, so Escape closes it
}

Step 'finite feed' {
    Reset-Tools
    Set-FeedBudget 1
    Reset-Tools
    Show-StreamTop
    $e = $null
    for ($i = 0; $i -lt 40 -and -not $e; $i++) {
        $lines = @(Get-SimText | Where-Object { $_.x -ge 241 -and $_.x -lt 760 -and $_.y -ge 80 })
        $e = $lines | Where-Object { $_.text -like "*today's feed*" -or $_.text -like '*today*s feed*' } | Select-Object -First 1
        if ($e) { break }
        $more = $lines | Where-Object { $_.text -like '*Show more*' } | Select-Object -First 1
        if ($more) { ClickAt ($more.x + [int]($more.w / 2)) ($more.y + 6) 1400; continue }
        Scroll 500 450 -6
        Start-Sleep -Milliseconds 300
    }
    if (-not $e) { throw 'the closing card did not show' }
    if ($e.y -gt 600) { Scroll 500 450 -3; Start-Sleep -Milliseconds 500; $e = Find-One "today" (241, 80, 519, 820) }
    # The card has no separator of its own above it: its top is a little over its moon icon.
    Park
    $b = Get-CardBox ($e.y + 4)
    Shot 'view-feed-end-card' -Crop 242, ($e.y - 55), 517, ($b[1] - ($e.y - 55))
    Set-FeedBudget -1
}


