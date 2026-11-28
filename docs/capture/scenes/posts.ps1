# Composer, post cards, threads, links, reactions, media viewer and symbols (shots/posts.json).
# Client area 1440x900, demo profile. Most shots are made on posts this scene writes itself;
# the seed conversation (Arya, Sokrates, Spinoza) sits below "Show more thoughts" and is found
# by scrolling. The last steps follow the "Capture your thinking" tutorial, so the stream ends
# the way that tutorial's closing screenshot shows it.

# --- test files and helpers -------------------------------------------------------------------

$script:Media = Join-Path ([System.IO.Path]::GetTempPath()) 'simulacra-docs-media'
$script:Geom = Join-Path ([System.IO.Path]::GetTempPath()) 'simulacra-docs-geom.py'
$script:Url = 'https://plato.stanford.edu/entries/mental-representation/'
# The composer dialog's columns; sheets are a little wider.
$script:Dialog = 425, 0, 1015, 900
$script:Sheet = 405, 0, 1035, 900
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

function New-TestMedia {
    <# Images, a titled PDF and a short WAV: small local files to attach. #>
    New-Item -ItemType Directory -Force -Path $script:Media | Out-Null
    if (Test-Path (Join-Path $script:Media 'first-thoughts.wav')) { return }
    @'
import math, random, sys
from PIL import Image, ImageDraw, ImageFont
out = sys.argv[1]
random.seed(4)
try:
    font = ImageFont.truetype("C:/Windows/Fonts/segoeui.ttf", 34)
except OSError:
    font = ImageFont.load_default()

def tree(name, sky, ground, season, w=1200, h=800):
    im = Image.new("RGB", (w, h), sky); d = ImageDraw.Draw(im)
    for y in range(h):
        t = y / h
        d.line([(0, y), (w, y)], fill=tuple(int(sky[i] * (1 - t) + 64 * t) for i in range(3)))
    d.rectangle([0, int(h * 0.72), w, h], fill=ground)
    cx = w // 2
    d.polygon([(cx - 26, int(h * .74)), (cx + 26, int(h * .74)), (cx + 12, int(h * .42)), (cx - 12, int(h * .42))], fill=(78, 52, 36))
    for ang in (-70, -35, 0, 35, 70):
        a = math.radians(ang - 90)
        d.line([(cx, int(h * .46)), (cx + int(230 * math.cos(a)), int(h * .42) + int(210 * math.sin(a)))], fill=(70, 50, 40), width=10)
    if season != "winter":
        for _ in range(170):
            r = random.randint(30, 68); x = cx + random.randint(-260, 260); y = int(h * .28) + random.randint(-170, 120)
            fill = (40, random.randint(110, 170), 60) if season == "summer" else (random.randint(190, 235), random.randint(90, 150), 30)
            d.ellipse([x - r, y - r, x + r, y + r], fill=fill)
    else:
        for _ in range(300):
            x = random.randint(0, w); y = random.randint(0, h); d.ellipse([x - 3, y - 3, x + 3, y + 3], fill=(250, 250, 255))
    im.save(f"{out}/{name}.png")

tree("tree-summer", (110, 170, 230), (70, 140, 60), "summer")
tree("tree-autumn", (230, 190, 140), (150, 120, 60), "autumn")
tree("tree-winter", (150, 165, 190), (235, 238, 245), "winter")

im = Image.new("RGB", (1200, 800), (245, 245, 240)); d = ImageDraw.Draw(im)
boxes = {"Claim": (480, 80), "Reason A": (180, 340), "Reason B": (780, 340), "Objection": (480, 600)}
for k, (x, y) in boxes.items():
    d.rounded_rectangle([x, y, x + 240, y + 90], 16, outline=(40, 70, 160), width=5)
    d.text((x + 30, y + 22), k, fill=(30, 30, 60), font=font)
for p, q in (("Reason A", "Claim"), ("Reason B", "Claim"), ("Objection", "Reason B")):
    (x1, y1), (x2, y2) = boxes[p], boxes[q]
    d.line([(x1 + 120, y1), (x2 + 120, y2 + 90)], fill=(200, 60, 60), width=5)
im.save(f"{out}/whiteboard.png")

im = Image.new("RGB", (1200, 800), (30, 34, 44)); d = ImageDraw.Draw(im)
for i in range(12):
    d.line([(0, 60 * i), (1200, 60 * i + 200)], fill=(60 + i * 10, 90, 160), width=6)
d.text((60, 60), "Seminar 3: mental representation", fill=(240, 240, 240), font=font)
im.save(f"{out}/seminar-slide.png")

# A twelve-page PDF with a title and authors in its metadata.
objs = [None, None, "<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>"]
kids = []
for i in range(12):
    text = f"BT /F1 24 Tf 72 720 Td (Mental Representation in Practice) Tj 0 -40 Td /F1 14 Tf (Seminar paper, page {i + 1} of 12) Tj ET"
    objs.append(f"<< /Length {len(text)} >>\nstream\n{text}\nendstream")
    objs.append(f"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Resources << /Font << /F1 3 0 R >> >> /Contents {len(objs)} 0 R >>")
    kids.append(len(objs))
objs[0] = "<< /Type /Catalog /Pages 2 0 R >>"
objs[1] = f"<< /Type /Pages /Kids [{' '.join(f'{k} 0 R' for k in kids)}] /Count 12 >>"
objs.append("<< /Title (Mental Representation in Practice) /Author (Ada Reader, Ben Scholar) >>")
pdf = bytearray(b"%PDF-1.4\n"); offsets = []
for i, o in enumerate(objs, 1):
    offsets.append(len(pdf)); pdf += f"{i} 0 obj\n{o}\nendobj\n".encode("latin-1")
xref = len(pdf)
pdf += f"xref\n0 {len(objs) + 1}\n0000000000 65535 f \n".encode() + b"".join(f"{o:010d} 00000 n \n".encode() for o in offsets)
pdf += f"trailer\n<< /Size {len(objs) + 1} /Root 1 0 R /Info {len(objs)} 0 R >>\nstartxref\n{xref}\n%%EOF\n".encode()
open(f"{out}/paper.pdf", "wb").write(pdf)

# Twelve seconds of soft, slowly changing tones as a stand-in voice note.
import struct, wave
rate = 16000
with wave.open(f"{out}/first-thoughts.wav", "wb") as w:
    w.setnchannels(1); w.setsampwidth(2); w.setframerate(rate)
    frames = bytearray()
    for i in range(rate * 12):
        t = i / rate
        env = 0.5 + 0.5 * math.sin(2 * math.pi * 0.7 * t)
        v = 0.12 * env * math.sin(2 * math.pi * (180 + 40 * math.sin(2 * math.pi * 0.3 * t)) * t)
        frames += struct.pack("<h", int(v * 32767))
    w.writeframes(bytes(frames))
'@ | python - $script:Media
    if (-not (Test-Path (Join-Path $script:Media 'first-thoughts.wav'))) { throw 'test media could not be made (needs Python with Pillow)' }
}

function MediaPath([string]$Name) { Join-Path $script:Media $Name }

function Set-ClipboardFile([string]$Path) {
    <# Puts a file on the clipboard the way Explorer's Copy does; Restore-SimClipboard puts the
       previous clipboard back after the paste. #>
    Set-SimClipboardFile $Path
}

function Send-FileDialog([string]$Path) {
    <# Puts a full path into the file name box of the open dialog Simulacra just showed, and opens it. #>
    Send-SimFileDialog $Path
}

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

function Get-Fills([int]$Y, [int]$X0 = 296, [int]$X1 = 760, [int]$MinW = 40) {
    <# x centres of solid blocks on row $Y (hidden spoilers). #>
    $r = python $script:Geom fills (Save-Temp) $X0 $X1 $Y $MinW
    if (-not $r) { return @() }
    [int[]]($r -split ' ')
}

function Test-ToggleOn([int]$X, [int]$Y) {
    <# Whether the switch at x,y shows the accent fill (on) rather than grey (off). #>
    $bmp = [System.Drawing.Bitmap]::FromFile((Save-Temp))
    try { $c = $bmp.GetPixel($X, $Y) } finally { $bmp.Dispose() }
    ($c.B - $c.G) -gt 60
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

function Trim-Sheet([string]$Id, [string]$Anchor = 'Close') {
    <# Ends the last auto shot at its sheet's bottom edge, about 21 px under the middle of the
       footer button $Anchor (the auto box can take in a strip of the window under the sheet). #>
    $k = $script:LastBox
    $c = @(Find-SimText $Anchor -Region $k[0], $k[1], $k[2], $k[3]) | Sort-Object y | Select-Object -Last 1
    if (-not $c) { return }
    $bottom = $c.y + [int]($c.h / 2) + 21
    if ($bottom -lt ($k[1] + $k[3])) { Shot $Id -Crop $k[0], $k[1], $k[2], ($bottom - $k[1]) -SettleMs 50 }
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

function Get-AttachBar {
    <# The row of attach buttons under the first post (image 509, path 537, link 565, hashtag 593,
       ideas 621), found by its icons below the first post's handle. #>
    $a = @(Find-SimText '@architect' -Region 431, 0, 578, 900) | Select-Object -First 1
    if (-not $a) { throw 'composer editor not found' }
    $r = python $script:Geom bar (Save-Temp) ($a.y + 40) ([Math]::Min(890, $a.y + 600))
    if (-not $r) { throw 'attach bar not found' }
    [int]$r
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
    $lastY = -1
    for ($i = 0; $i -lt 45; $i++) {
        $lines = @(Get-SimText | Where-Object { $_.x -ge 241 -and $_.x -lt 760 -and $_.y -ge 80 })
        $hit = $lines | Where-Object { $_.text -like "*$Text*" -and (-not $Own -or ($_.x -ge 300 -and $_.x -le 312)) } | Select-Object -First 1
        if ($hit) {
            if ($hit.y -lt $Top -and $hit.y -ne $lastY) { $lastY = $hit.y; Scroll 500 450 1; Start-Sleep -Milliseconds 300; continue }
            if ($hit.y -lt $Top) { return $hit }   # already at the top of the stream
            if ($hit.y -gt $Bottom) { Scroll 500 450 -1; Start-Sleep -Milliseconds 300; continue }
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

function Draw-Stroke([int[][]]$Points) {
    <# Drags the mouse through client points with the left button held. #>
    Invoke-SimDrag -Points $Points -StepMs 12
}

function Get-Circle([int]$Cx, [int]$Cy, [int]$Radius) {
    $pts = @()
    for ($deg = 0; $deg -le 370; $deg += 8) { $rad = [Math]::PI * $deg / 180; $pts += , @([int]($Cx + $Radius * [Math]::Cos($rad)), [int]($Cy + $Radius * [Math]::Sin($rad))) }
    , $pts
}

function Get-Line([int]$X0, [int]$Y0, [int]$X1, [int]$Y1) {
    $pts = @(); $n = 16
    for ($i = 0; $i -le $n; $i++) { $pts += , @([int]($X0 + ($X1 - $X0) * $i / $n), [int]($Y0 + ($Y1 - $Y0) * $i / $n)) }
    , $pts
}

# --- the scene ---------------------------------------------------------------------------------

Step 'welcome' -Always { Skip-Welcome }

Step 'prepare' -Always {
    New-TestMedia
    if (-not (Get-NetTCPConnection -LocalPort 8765 -State Listen -ErrorAction SilentlyContinue)) {
        Start-Process node -ArgumentList (Join-Path $PSScriptRoot '..\stub-ai.mjs') -WindowStyle Hidden
        Start-Sleep -Seconds 2
    }
}

Step 'composer new' {
    Reset-State
    Open-Composer
    Park
    Shot 'composer-modal-new'
    Write-Text 'Hello'
    Park
    $f = Get-Footer
    Shot 'composer-footer-tools' -Crop 431, ($f.y - 22), 578, 45
    Close-Composer
}

Step 'composer inline' {
    Reset-State
    ClickAt 520 112 400
    Key '^a' 150; Key '{BACKSPACE}' 400
    Write-Text 'Reading the Tractatus tonight. **Limits** of language, limits of world. #Wittgenstein'
    Key '{ESC}' 300
    Park
    $b = Get-CardBox 112
    Shot 'composer-inline' -Crop 242, ($b[0] + 1), 517, ($b[1] - $b[0] - 1)
    # The toolbar row sits at the bottom of the composer: link, hashtag, preview, then the five
    # formats (Bold at x 406 ... Spoiler at x 534).
    $ty = $b[1] - 24
    Mark
    Hover 406 $ty 1700
    Shot 'composer-format-buttons' -Auto -AutoPad 6
    $t = $script:LastBox
    $x0 = [Math]::Min($t[0], 388) - 4; $x1 = [Math]::Max($t[0] + $t[2], 552) + 4
    $y0 = [Math]::Min($t[1], $ty - 18) - 4; $y1 = [Math]::Max($t[1] + $t[3], $ty + 18) + 4
    Shot 'composer-format-buttons' -Crop $x0, $y0, ($x1 - $x0), ($y1 - $y0)
    ClickAt 520 112 300
    Key '^a' 150; Key '{BACKSPACE}' 300
    Park
}

Step 'composer thread' {
    Reset-State
    Park
    Mark
    Open-Composer
    Write-Text 'Three things I keep forgetting about attention.'
    Click 'Add to thread' -Region 431, 0, 578, 900
    Write-Text 'It is a budget, not a tap.'
    Click 'Add to thread' -Region 431, 0, 578, 900
    Write-Text 'And it is spent before it is noticed.'
    Park
    Shot-Dialog 'composer-modal-thread'
    Close-Composer
}

Step 'composer split' {
    Reset-State
    Open-Composer
    Write-Text 'Attention is the rarest form of generosity. Every notification is a small loan against it, repaid with interest in lost focus. I used to think the cure was more discipline. Now I think it is fewer doors: close the tabs, silence the phone, and let one hard question stay open long enough to answer back. Then wait.'
    Park
    $a = Find-One '@architect' (431, 0, 578, 900)
    $n = @(Find-SimText 'characters over' -Region 431, 0, 578, 900) | Select-Object -Last 1
    if (-not $n) { $n = Find-One 'Split into thread' (431, 0, 578, 900) }
    Shot 'composer-split-into-thread' -Crop 431, ($a.y - 14), 578, ($n.y + $n.h + 14 - ($a.y - 14))
    Close-Composer
}

Step 'composer free' {
    Reset-State
    Park
    Mark
    Open-Composer
    $f = Get-Footer
    ClickAt $f.free $f.y 600
    Write-Text 'A reflection carries the shape of the sun, not its warmth.'
    Key '{ENTER}' 300
    TypeText '  Understanding needs a model that can be wrong, and a mirror cannot be wrong.' 350
    Key '{ENTER}' 300
    Key '+{HOME}' 150
    TypeText 'So the question is whether prediction can be wrong in the way a belief can. #Epistemology' 400
    Key '{ESC}' 1500
    Park
    Shot-Dialog 'composer-free-compose'
    Close-Composer
}

Step 'composer preview' {
    Reset-State
    Park
    Mark
    Open-Composer
    Write-Text '**Claim:** the answer is ||forty-two||'
    Key '{ENTER}' 200; Key '{ENTER}' 200
    TypeText '```python' 200; Key '{ENTER}' 200
    TypeText 'print(6 * 7)' 200; Key '{ENTER}' 200
    TypeText '```' 1500
    $f = Get-Footer
    ClickAt $f.eye $f.y 900
    Park
    Shot-Dialog 'composer-preview'
    Close-Composer
}

Step 'composer todo' {
    Reset-State
    Park
    Mark
    Open-Composer
    $f = Get-Footer
    ClickAt $f.todo $f.y 900
    Park
    Shot-Dialog 'composer-todo-help'
    $f = Get-Footer
    ClickAt $f.todo $f.y 700
    Close-Composer
    Open-Composer
    Write-Text '[ ] Reply to Spinoza ^t'
    Park
    Start-Sleep -Milliseconds 800
    $a = Find-One '@architect' (431, 0, 578, 900)
    $f = Get-Footer
    Shot 'composer-todo-suggest' -Crop 431, ($a.y - 14), 578, ($f.y - 24 - ($a.y - 14))
    Close-Composer
}

Step 'feeling' {
    Reset-State
    Park
    Mark
    Open-Composer
    $f = Get-Footer
    ClickAt $f.feeling $f.y 1000
    # The emoji search has the focus; the first result sits under it. Picking one moves the
    # fields down, so each is found again before it is clicked.
    foreach ($word in 'relieved', 'hot beverage') {
        $q = @(Find-SimText 'Search emoji' -Region 431, 0, 578, 900)
        if ($q) { ClickAt ($q[0].x + 120) ($q[0].y + 6) 300 }
        Key '^a' 150; TypeText $word 1200
        $q = Find-One $word (431, 0, 578, 900)
        ClickAt 464 ($q.y + 39) 900
        $q = Find-One $word (431, 0, 578, 900)
        ClickAt ($q.x + 160) ($q.y + 6) 300
        Key '^a' 150; Key '{BACKSPACE}' 900
    }
    $i = Find-One 'Intensity' (431, 0, 578, 900)
    ClickAt 970 ($i.y + 5) 500
    ClickAt 700 ($i.y + 40) 400   # the note field, under the Feeling row
    TypeText 'after a long read' 1200
    Park
    Shot-Dialog 'composer-feeling-bar'
    # Ctrl+Enter in the note posts the feeling; the Post button is the fallback.
    ClickAt 700 ($i.y + 40) 300
    Key '^{ENTER}' 1800
    if (@(Find-SimText 'Intensity' -Region 431, 0, 578, 900)) {
        $f = Get-Footer
        Click 'Post' -Region 930, ($f.y - 14), 80, 28 -At 971, $f.y -WaitMs 1800
    }
    $h = Find-Post 'after a long read'
    Shot-Card 'post-feeling-row' $h.y
}

Step 'seed thread' {
    Reset-State
    # Arya's seed post: its action bar, its References menu, the inspector, then an edit.
    $h = Find-Post 'Exploring how language constructs' -Own
    $b = Get-CardBox $h.y
    $ic = Get-ActionIcons $b[1]
    Park
    Shot 'post-action-bar' -Crop 296, ($ic.y - 17), 460, 34
    Mark
    ClickAt $ic.refs $ic.y 900
    Park
    $m = Get-ChangedBox ([Math]::Max(0, $ic.refs - 300)), ([Math]::Max(38, $ic.y - 420)), ([Math]::Min(1121, $ic.refs + 460)), ([Math]::Min(900, $ic.y + 420))
    $x0 = [Math]::Min($m[0], $ic.refs - 20) - 8; $x1 = [Math]::Max($m[0] + $m[2], $ic.refs + 20) + 8
    $y0 = [Math]::Min($m[1], $ic.y - 16) - 8; $y1 = [Math]::Max($m[1] + $m[3], $ic.y + 16) + 8
    Shot 'thread-references-menu' -Crop $x0, $y0, ($x1 - $x0), ($y1 - $y0)
    Key '{ESC}' 600
    ClickAt ($h.x + 40) ($h.y + 6) 1500
    Park
    Shot 'thread-inspector' -Crop 761, 38, 679, 862
    $r = Find-One 'Referred to by' (761, 38, 679, 862)
    $q = Find-One 'Quoted in' (761, 38, 679, 862)
    Shot 'thread-backlinks' -Crop 762, ($r.y - 12), 676, ($q.y + $q.h + 14 - ($r.y - 12))
    # Edit: a word added to the first sentence makes a patch version.
    Mark
    ClickAt 733 ($b[0] + 24) 800
    Hover 290 ($b[0] + 24) 300
    Click 'Edit' -Region 241, 0, 800, 900 -WaitMs 1200
    Key '^{HOME}' 200; Key '{RIGHT 75}' 300
    TypeText ', perhaps' 1500
    Park
    # The dialog, from its header to the footer with Save changes.
    Shot 'composer-edit-version' -Auto -AutoPad 0 -Within $script:Dialog
    Key '^{ENTER}' 1800
    $h = Find-Post 'Exploring how language constructs' -Own
    $b = Get-CardBox $h.y
    Park
    Shot 'post-author-row' -Crop 296, ($b[0] + 8), 460, 32
}

Step 'seed spinoza' {
    Reset-State
    # Spinoza's seed post: anatomy, reactions and claps, quote, embed.
    Key '^1' 700
    $h = Find-Post 'Substance is one and infinite' -Own
    Shot-Card 'post-card-anatomy' $h.y
    $b = Get-CardBox $h.y
    $ic = Get-ActionIcons $b[1]
    Mark
    ClickAt $ic.react $ic.y 1000
    Park
    $m = Get-ChangedBox ([Math]::Max(0, $ic.react - 120)), 38, ([Math]::Min(1121, $ic.react + 460)), 900
    if ($m[2] -ge 1400) { throw 'the emoji picker did not open' }
    $x0 = [Math]::Min($m[0], $ic.react - 18) - 8; $x1 = [Math]::Max($m[0] + $m[2], $ic.react + 18) + 8
    $y0 = [Math]::Min($m[1], $ic.y - 16) - 8; $y1 = [Math]::Max($m[1] + $m[3], $ic.y + 16) + 8
    Shot 'post-reaction-picker' -Crop $x0, $y0, ($x1 - $x0), ($y1 - $y0)
    foreach ($word in 'thinking', 'thumbs up') {
        if ($word -ne 'thinking') { ClickAt $ic.react $ic.y 1000 }
        $s = Find-One 'Search emoji'
        ClickAt ($s.x + 40) ($s.y + 6) 300
        TypeText $word 1200
        # The first result sits under the tab strip and the "Search results" heading.
        ClickAt ($s.x + 8) ($s.y + 108) 900
        Key '{ESC}' 400
        $b = Get-CardBox $h.y
        $ic = Get-ActionIcons $b[1]
    }
    for ($i = 0; $i -lt 3; $i++) { ClickAt $ic.clap $ic.y 350 }
    Park
    $b = Get-CardBox $h.y
    Shot 'post-reaction-bar' -Crop 242, ($h.y - 10), 517, ($b[1] - $h.y + 10)
    $ic = Get-ActionIcons $b[1]
    Park
    Mark
    ClickAt $ic.quote $ic.y 1200
    Write-Text 'The order of ideas and the order of things: same order, or same thing?'
    Park
    Shot-Dialog 'composer-modal-quote'
    Close-Composer
    ClickAt 733 ($b[0] + 24) 800
    Hover 290 ($b[0] + 24) 300
    $c = Find-One 'Copy' (590, 38, 200, 862)
    Hover ($c.x + 20) ($c.y + 6) 1000
    Click 'Copy embed' -Region 740, 38, 400, 862 -WaitMs 800
    Open-Composer
    Write-Text 'Compare with this:' 300
    Key '{ENTER}' 300
    Key '^v' 1500
    Key '^{ENTER}' 1800
    $h = Find-Post 'Compare with this'
    Shot-Card 'post-embed-card' $h.y
}


Step 'tint' {
    Reset-State
    Open-Composer -Style
    Write-Text 'A calm thought in blue'
    $t = Find-One 'Tint' (431, 0, 578, 900)
    ClickAt 545 ($t.y + 5) 600   # none 482, amber 503, rose 524, sky 545, mint 566, violet 587, slate 608
    Submit-Composer
    $h = Find-Post 'A calm thought in blue'
    Shot-Card 'post-tinted-card' $h.y -Above 36 -Below 36 -Left 241
    Fold-Style
}

Step 'gallery' {
    Reset-State
    Open-Composer
    Write-Text 'Three views of the same tree'
    ClickAt 537 (Get-AttachBar) 800
    foreach ($n in 'tree-summer', 'tree-autumn', 'tree-winter') {
        $p = Find-One 'Paste local image path' (431, 0, 578, 900)
        ClickAt ($p.x + 150) ($p.y + 8) 300
        TypeText (MediaPath "$n.png") 300
        Key '{ENTER}' 1600
    }
    $d = @(Find-SimText 'Describe this image' -Region 431, 0, 578, 900 | Sort-Object y)
    if ($d.Count -lt 3) { throw "image rows not found ($($d.Count))" }
    ClickAt ($d[1].x + 60) ($d[1].y + 6) 400
    TypeText 'The same tree in autumn, its leaves turning orange' 800
    Submit-Composer 3000
    $h = Find-Post 'Three views of the same tree'
    Shot-Card 'media-gallery' $h.y
    # The viewer: the first image, then the next one.
    ClickAt 400 ($h.y + 90) 1500
    Key '{RIGHT}' 1200
    Park
    Shot 'media-lightbox'
    Key '{ESC}' 800
}

Step 'attachments' {
    Reset-State
    Park
    Mark
    Open-Composer
    Write-Text 'Notes from the seminar'
    ClickAt 537 (Get-AttachBar) 800
    foreach ($n in 'whiteboard', 'seminar-slide') {
        $p = Find-One 'Paste local image path' (431, 0, 578, 900)
        ClickAt ($p.x + 150) ($p.y + 8) 300
        TypeText (MediaPath "$n.png") 300
        Key '{ENTER}' 1600
    }
    $p = Find-One 'Paste local image path' (431, 0, 578, 900)
    ClickAt 976 ($p.y + 8) 700   # close the path field
    $d = @(Find-SimText 'Describe this image' -Region 431, 0, 578, 900 | Sort-Object y)
    ClickAt ($d[0].x + 60) ($d[0].y + 6) 400
    TypeText 'Whiteboard with the argument map' 600
    Set-ClipboardFile (MediaPath 'paper.pdf')
    Focus-Editor
    Key '^v' 1600
    Restore-SimClipboard
    ClickAt 565 (Get-AttachBar) 800   # Attach link
    TypeText $script:Url 1500
    Park
    $box = Get-ChangedBox $script:Dialog
    $a = Find-One '@architect' (431, 0, 578, 900)
    $add = Find-One 'Add to thread' (431, 0, 578, 900)
    Shot 'composer-attachments' -Crop $box[0], ($a.y - 16), $box[2], ($add.y - 12 - ($a.y - 16))
    Close-Composer
}

Step 'file card' {
    Reset-State
    Open-Composer
    Write-Text 'The paper we discussed'
    Set-ClipboardFile (MediaPath 'paper.pdf')
    Focus-Editor
    Key '^v' 1600
    Restore-SimClipboard
    Submit-Composer 4000
    $h = Find-Post 'The paper we discussed'
    Start-Sleep -Seconds 2
    Shot-Card 'post-file-card' $h.y
}

Step 'link card' {
    Reset-State
    ClickAt 520 112 400
    Key '^a' 150; Key '{BACKSPACE}' 300
    Write-Text "Good overview $script:Url" 600
    Key '^{ENTER}' 2500
    for ($i = 0; $i -lt 12; $i++) {
        if (@(Find-SimText 'Stanford' -Region 241, 80, 519, 820)) { break }
        Start-Sleep -Milliseconds 1500
    }
    $h = Find-Post 'Good overview'
    Shot-Card 'post-link-card' $h.y
}

Step 'spoiler' {
    Reset-State
    ClickAt 520 112 400
    Key '^a' 150; Key '{BACKSPACE}' 300
    Write-Text 'The twist: ||the narrator is the mirror|| and ||the mirror is you||' 600
    Key '^{ENTER}' 2500
    $h = Find-Post 'The twist'
    # Reveal only the second spoiler: on one line it is the second solid block (checked, and
    # clicked again if the first click only focused the card); wrapped, the first of line two.
    $revealed = $false
    for ($try = 0; $try -lt 3 -and -not $revealed; $try++) {
        $blocks = @(Get-Fills ($h.y + 8))
        if ($blocks.Count -ge 2) { ClickAt $blocks[1] ($h.y + 8) 1000 }
        else {
            $next = @(Get-Fills ($h.y + 30))
            if ($next) { ClickAt $next[0] ($h.y + 30) 1000 }
        }
        Park
        $revealed = (@(Get-Fills ($h.y + 8)).Count -eq 1) -and -not @(Get-Fills ($h.y + 30))
    }
    if (-not $revealed) { throw 'second spoiler not revealed' }
    Park
    $b = Get-CardBox $h.y
    Shot 'post-spoiler' -Crop 296, ($h.y - 10), 460, ($b[1] - 42 - ($h.y - 10))
}

Step 'code' {
    Reset-State
    Key '^,' 1200
    Click 'Code' -Region 190, 120, 210, 600 -WaitMs 1000
    if (-not (Test-ToggleOn 1203 237)) { ClickAt 1211 237 700 }
    Click 'Refresh' -Region 1100, 380, 160, 80 -At 1198, 426 -WaitMs 3500
    ClickAt 1215 803 900
    Open-Composer
    Write-Text 'The sum of the first ten numbers:' 300
    Key '{ENTER}' 200; TypeText '```python' 200
    Key '{ENTER}' 200; TypeText 'print(sum(range(10)))' 200
    Key '{ENTER}' 200; TypeText '```' 1500
    Submit-Composer 2500
    $h = Find-Post 'The sum of the first ten'
    # The code card's header (python, Run, Copy code) is the next row under the text.
    Click 'Run' -Region 600, ($h.y + 14), 160, 50 -At 697, ($h.y + 36) -WaitMs 4000
    Park
    $b = Get-CardBox $h.y
    Shot 'post-code-block' -Crop 300, ($h.y + 18), 456, ($b[1] - 40 - ($h.y + 18))
}

Step 'lock' {
    Reset-State
    Park
    Mark
    Open-Composer
    Write-Text 'A thought for my eyes only'
    $f = Get-Footer
    ClickAt $f.lock $f.y 900
    $pw = 'copper lantern violet harbor'
    $p = Find-One 'Password for this thread' (431, 0, 578, 900)
    ClickAt ($p.x + 100) ($p.y + 6) 300
    TypeText $pw 600
    $r = Find-One 'Type it again' (431, 0, 578, 900)
    ClickAt ($r.x + 100) ($r.y + 6) 300
    TypeText $pw 1200
    Park
    Shot-Dialog 'composer-lock-row'
    Submit-Composer 4000
    $h = Find-Post 'Locked post'
    Shot-Card 'post-locked-card' $h.y
    Mark
    Click 'Unlock' -Region 296, ($h.y - 10), 460, 80 -WaitMs 1000
    Park
    Shot 'post-unlock-sheet' -Auto -AutoPad 0 -Within $script:Sheet
    Click 'Cancel' -Region 760, 80, 270, 400 -WaitMs 700
}

function New-SymbolPost([string]$Text, [int[]]$Symbols) {
    <# A post with core symbols (x of each in the Symbols row: Task 507, Event 530, Note 553,
       Priority 576, Inspiration 599, Explore 622). #>
    Open-Composer -Style
    Write-Text $Text
    $s = Find-One 'Symbols' (431, 0, 578, 900)
    foreach ($x in $Symbols) { ClickAt $x ($s.y + 6) 400 }
    Submit-Composer
}

Step 'symbols' {
    Reset-State
    Park
    Mark
    Open-Composer -Style
    Write-Text 'Draft the essay outline'
    $s = Find-One 'Symbols' (431, 0, 578, 900)
    ClickAt 507 ($s.y + 6) 500   # Task
    ClickAt 644 ($s.y + 6) 900   # Search symbols
    TypeText 'in' 1500
    Park
    $box = Get-ChangedBox $script:Dialog
    $f = Get-Footer
    $s = Find-One 'Symbols' (431, 0, 578, 900)
    Shot 'composer-symbol-picker' -Crop $box[0], ($s.y - 12), $box[2], ($f.y - 23 - ($s.y - 12))
    Submit-Composer
    New-SymbolPost 'Seminar at 4' 530, 576
    New-SymbolPost 'Attention is spent before it is noticed' 553
    $h = Find-Post 'Attention is spent before' 150 300
    $top = (Get-CardBox $h.y)[0]
    $t = Find-Post 'Draft the essay outline' 150 820
    $bottom = (Get-CardBox $t.y)[1]
    Park
    Shot 'post-symbol-face' -Crop 242, ($top + 1), 517, ($bottom - $top - 1)
    # The Entry submenu of the open task, under Organize ›; the task near the top of the stream
    # leaves the menus room above the window's bottom edge.
    $t = Find-Post 'Draft the essay outline' 150 330
    $b = Get-CardBox $t.y
    Hover 733 ($b[0] + 24) 600
    Mark
    ClickAt 733 ($b[0] + 24) 800
    $o = Find-One 'Organize' (560, 38, 240, 862)
    Hover ($o.x + 20) ($o.y + 6) 1000
    $e = Find-One 'Entry' (740, 38, 400, 862)
    Hover ($e.x + 20) ($e.y + 6) 1200
    # The ⋯ menu (from Actions down) with Organize › and Entry › open beside it: its top and left
    # from the Actions row, its right from the Entry submenu (about 100 px right of its text, as
    # its edge is too faint to read as a change), its bottom from the lowest entry.
    $top = Find-One 'Actions' (560, 38, 240, 862)
    $m = Get-ChangedBox (560, 38, 1440, 900)
    $lines = @(Get-SimText | Where-Object { $_.x -ge ($top.x - 20) })
    $low = @($lines | Where-Object { $_.text -match 'Delete$|Add to collection|Delegate' } | Sort-Object y)
    $md = @($lines | Where-Object { $_.text -like '*Mark done*' })
    $x0 = $top.x - 20; $y0 = [Math]::Max(38, $top.y - 18)
    $x1 = if ($md) { [Math]::Min(1440, $md[0].x + 106) } else { [Math]::Min(1440, $m[0] + $m[2] + 40) }
    $y1 = if ($low) { [Math]::Min(900, $low[-1].y + $low[-1].h + 16) } else { 900 }
    Shot 'post-entry-submenu' -Crop $x0, $y0, ($x1 - $x0), ($y1 - $y0)
    Key '{ESC}' 300; Key '{ESC}' 500
    # Four more open tasks for the monthly review.
    foreach ($task in 'Reread Ethics, part II', 'Write to the seminar group', 'Return the library books', 'Outline the talk on attention') {
        New-SymbolPost $task 507
    }
    Park
    Mark
    Palette 'Review open tasks' 1500
    Click 'Keep open' -WaitMs 1000
    Park
    Shot 'post-monthly-review' -Auto -AutoPad 0 -Within $script:Sheet
    Trim-Sheet 'post-monthly-review'
    Key '{ESC}' 700
    Park
    Mark
    Palette 'Symbol key' 1500
    Park
    Shot 'post-symbol-key' -Auto -AutoPad 0 -Within $script:Sheet
    Trim-Sheet 'post-symbol-key'
    Key '{ESC}' 700
    Fold-Style
}

Step 'agent post' {
    Reset-State
    Key '^3' 1200
    Click 'Spinoza' -Region 241, 80, 400, 500 -WaitMs 900
    Click 'Write a post' -WaitMs 1200
    $d = Find-One 'None Ethics' (241, 0, 1200, 900)
    ClickAt ($d.x + 53) ($d.y + 7) 700
    $c = Find-One 'Cancel' (($d.x - 20), $d.y, 500, 200)
    ClickAt ($c.x + 87) ($c.y + 5) 9000   # Write
    Key '{ESC}' 500
    Key '^1' 1200
    $h = Find-Post 'Sources'
    Shot-Card 'post-agent-chips' $h.y
    $b = Get-CardBox $h.y
    ClickAt 600 ($b[0] + 58) 1500
    Park
    $bh = Find-One 'Behind this post' (761, 38, 679, 862)
    $lv = Find-One 'Simpler' (761, 38, 679, 862)
    Shot 'thread-behind-this-post' -Crop 768, ($bh.y - 16), 664, ($lv.y - 14 - ($bh.y - 16))
}

Step 'capture' {
    Reset-State
    Park
    Mark
    Key '^+ ' 1000
    TypeText "Is a mirror's reflection a kind of understanding, or only a copy?" 500
    Key '{ENTER}' 1000
    TypeText $script:Url 500
    Key '{ENTER}' 1000
    TypeText "Is a mirror's reflection a kind of understanding, or only a copy?" 500
    Start-Sleep -Seconds 5   # the "Saved to your Inbox" toasts fade
    Park
    Shot 'composer-capture-box' -Auto -AutoPad 24 -Within 400, 38, 1040, 240
    Key '{ESC}' 700
    Palette 'Open Inbox' 1500
    for ($i = 0; $i -lt 10; $i++) {
        if (@(Find-SimText 'Stanford' -Region 241, 80, 519, 820)) { break }
        Start-Sleep -Milliseconds 1500
    }
    Scroll 500 450 20
    Park
    $n = Find-One 'kind of understanding' (241, 80, 519, 820)
    $b = Get-CardBox $n.y
    Shot 'composer-inbox-tab' -Crop 241, 38, 519, ($b[1] + 16 - 38)
    Click 'For you' -Region 241, 40, 300, 40 -At 280, 58
}

Step 'code confirm' {
    Reset-State
    $fence = '```'
    Set-Clipboard -Value "From the seminar handout:`n${fence}python`nprint('hello')`n${fence}"
    ClickAt 560 58 400   # an empty part of the stream's tab row: no text field has the focus
    Key '^v' 1500
    Palette 'Open Inbox' 1500
    $h = Find-Post 'seminar handout'
    Park
    Mark
    Click 'Run' -Region 600, ($h.y + 14), 160, 50 -At 697, ($h.y + 36) -WaitMs 1200
    Park
    Shot 'post-code-confirm' -Auto -AutoPad 0 -Within $script:Sheet
    Key '{ESC}' 600
    Click 'For you' -Region 241, 40, 300, 40 -At 280, 58
}

Step 'tutorial reply' {
    Reset-State
    $h = Find-Post 'Tell me, my friend' -Own
    $b = Get-CardBox $h.y
    $ic = Get-ActionIcons $b[1]
    Park
    Mark
    ClickAt $ic.reply $ic.y 1200
    Write-Text 'A reflection carries the shape of the sun, not its warmth.'
    Park
    Shot-Dialog 'composer-modal-reply'
    $f = Get-Footer
    ClickAt $f.free $f.y 800
    Focus-Editor
    Key '^{END}' 200; Key '{ENTER}' 300
    TypeText '  Understanding needs a model that can be wrong, and a mirror cannot be wrong.' 350
    Key '{ENTER}' 300; Key '+{HOME}' 150
    TypeText 'So the question is whether prediction can be wrong in the way a belief can. #Epistemology' 600
    Key '{ESC}' 600
    Key '^{ENTER}' 3000
}

Step 'checklist' {
    Reset-State
    Open-Composer
    Write-Text 'Afternoon reading plan' 300
    foreach ($line in '[ ] Reread Ethics, part II ^today !! &desk ~1h', '  [ ] Note three definitions ^today', "[ ] Reply to Spinoza's post on substance ^tomorrow", '[ ] Water the plants ^^weekly &home') {
        Key '{ENTER}' 300
        Key '+{HOME}' 150
        TypeText $line 500
        Key '{ESC}' 300
    }
    Start-Sleep -Milliseconds 1200
    Submit-Composer 2500
    $h = Find-Post 'reading plan'   # OCR tends to read "Afternoon" as "Aftemoon"
    # Boxes sit left of the text: x 315 at the top level, 337 when nested. Child first, then parent.
    $n = Find-One 'three definitions' (296, $h.y, 460, 400)
    ClickAt 337 ($n.y + 7) 900
    $r = Find-One 'Reread Ethics' (296, $h.y, 460, 400)
    ClickAt 315 ($r.y + 7) 900
    Shot-Card 'post-checklist' $h.y
}

Step 'voice' {
    # Record opens the default microphone and waits for it; on the machine these shots were made
    # on it never answered (the window hangs), so the sheet is shown idle and the note is posted
    # from an audio file instead.
    Reset-State
    Park
    Mark
    ClickAt 100 112 800
    Click 'New voice note' -WaitMs 1200
    Park
    Shot 'composer-voice-sheet' -Auto -AutoPad 0 -Within $script:Sheet
    ClickAt 590 218 300   # Choose an audio file… (Record, left of it, is never clicked)
    Send-FileDialog (MediaPath 'first-thoughts.wav')
    $c = Find-One 'What is it about' (411, 80, 618, 400)
    ClickAt ($c.x + 100) ($c.y + 6) 300
    TypeText 'First thoughts on the definitions' 600
    $x = Find-One 'Cancel' (411, 80, 618, 400)
    ClickAt ($x.x + 121) ($x.y + 6) 2500   # Post voice note
    $h = Find-Post 'First thoughts on the definitions'
    Shot-Card 'post-voice-card' $h.y
}

Step 'drawing' {
    Reset-State
    Park
    Mark
    ClickAt 100 112 800
    Click 'New drawing' -WaitMs 1200
    ClickAt 446 215 400   # the accent ink
    ClickAt 517 215 400   # width 4
    Draw-Stroke (Get-Circle 540 377 72)
    Draw-Stroke (Get-Line 620 352 800 285)
    Draw-Stroke (Get-Line 800 285 778 282)
    Draw-Stroke (Get-Line 800 285 786 302)
    Draw-Stroke (Get-Line 620 402 800 465)
    Draw-Stroke (Get-Line 800 465 786 448)
    Draw-Stroke (Get-Line 800 465 778 468)
    $c = Find-One 'What did you draw' (411, 80, 618, 600)
    ClickAt ($c.x + 100) ($c.y + 6) 300
    TypeText 'One substance, two attributes' 800
    Park
    Shot 'composer-ink-sheet' -Auto -AutoPad 0 -Within $script:Sheet
    $k = $script:LastBox
    # Cancel shares the footer row with Post drawing, which OCR misses (white on the accent).
    $pd = Find-One 'Cancel' (411, 80, 618, 700)
    Shot 'composer-ink-sheet' -Crop $k[0], $k[1], $k[2], ($pd.y + [int]($pd.h / 2) + 21 - $k[1]) -SettleMs 50
    Click 'Post drawing' -Region 411, 80, 618, 600 -At 959, 613 -WaitMs 2500
    $h = Find-Post 'One substance, two attributes'
    Shot-Card 'post-ink-card' $h.y
}

Step 'schedule' {
    Reset-State
    Show-StreamTop   # the tutorial's closing window shot shows the top of the stream
    Park
    Mark
    Open-Composer
    Write-Text 'Can a prediction be wrong in the way a belief can? [[post_1]]'
    Key '{ESC}' 600
    $f = Get-Footer
    ClickAt $f.schedule $f.y 900
    # The presets read as one OCR line: In an hour, Tomorrow at 9:00 (+118), In a week (+205).
    $pre = Find-One 'In an hour' (431, 0, 578, 900)
    ClickAt ($pre.x + 118) ($pre.y + 6) 900
    Park
    Shot-Dialog 'composer-schedule-row'
    Submit-Composer 900
    Park
    Shot 'composer-day-timeline' -SettleMs 100
    Open-Composer
    Write-Text 'Weekly check: which of my claims still lack a source?'
    $f = Get-Footer
    ClickAt $f.schedule $f.y 900
    $pre = Find-One 'In an hour' (431, 0, 578, 900)
    ClickAt ($pre.x + 205) ($pre.y + 6) 900
    Submit-Composer 2500
    Park
    Mark
    Palette 'Scheduled posts' 1500
    Park
    Shot 'composer-scheduled-sheet' -Auto -AutoPad 0 -Within $script:Sheet
    Trim-Sheet 'composer-scheduled-sheet'
    Key '{ESC}' 700
}
