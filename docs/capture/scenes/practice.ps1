# Review, Journal, Tasks and Arena (shots/practice.json). Client area 1440x900, demo profile
# (Epistemology & Mind active, AI on the stub). demo/seed.py (seed_practice) adds what a run
# cannot make on the day or the stub cannot produce: two weeks of journal history (check-ins,
# entries, focus sessions, capsules), two tutor dialogues, a judged debate with its claim map and
# a thought experiment answered by two agents.
#
# Order: Review empty state; four items learnt through the UI (Spinoza's post, Sokrates' reply,
# two passages of the Ethics); Today, a session to its summary and the caught-up state; Items;
# a connection map and its drill; the quizzes; Statistics; Learn (tutor, goals, gaps, palace).
# Then the Journal (today, entries, mood, focus, capsules), Tasks (a checklist post typed in the
# composer), and the Arena (a slow local relay keeps a debate "speaking" long enough to capture;
# the no-key notice comes last, after the key is cleared in this throw-away profile).

# --- helpers ----------------------------------------------------------------------------------

$script:Dbg = Join-Path ([System.IO.Path]::GetTempPath()) 'simulacra-docs-practice-debug'
$script:PrGeom = Join-Path ([System.IO.Path]::GetTempPath()) 'simulacra-docs-practice-geom.py'
# The image attached to a new capsule: the one demo/seed.py draws for the seeded capsule.
$script:CapsuleImage = [System.IO.Path]::GetFullPath((Join-Path $PSScriptRoot '..\demo\media\attachments\demo-capsule-sky.png'))
# A slow relay in front of the stub (written here, started for the running debate only).
$script:SlowProxy = Join-Path ([System.IO.Path]::GetTempPath()) 'simulacra-docs-practice-relay.mjs'
# Review: the top tabs (y 139) and the Learn sub-tabs (y 189), page at its top.
$script:RTab = @{ Today = 495; Learn = 725; Statistics = 955; Items = 1185 }
$script:RSub = @{ Tutor = 495; Goals = 725; Gaps = 955; Palace = 1185 }
# Journal tabs (y 174).
$script:JTab = @{ Today = 458; Journal = 650; Mood = 840; Focus = 1032; Capsules = 1224 }

Set-Content -Path $script:PrGeom -Encoding utf8 -Value @'
"""box <png> <px> <py>: the card around a point in its empty padding (x y w h).
bottom <png> <x0> <y0> <x1> <y1>: the row under the last ink in a region."""
import sys
import numpy as np
from PIL import Image

a = np.asarray(Image.open(sys.argv[2]).convert("RGB"), dtype=np.int16)
H, W = a.shape[:2]

def differs(p, q, tol=5):
    return np.abs(p - q).max() > tol

def box(px, py):
    bg = a[py, px]
    top = py
    while top > 0 and not differs(a[top - 1, px], bg):
        top -= 1
    bottom = py
    while bottom < H - 1 and not differs(a[bottom + 1, px], bg):
        bottom += 1
    left, right = px, px
    for row in {min(top + 6, bottom), max(bottom - 6, top)}:
        x = px
        while x > 0 and not differs(a[row, x - 1], bg):
            x -= 1
        left = min(left, x)
        x = px
        while x < W - 1 and not differs(a[row, x + 1], bg):
            x += 1
        right = max(right, x)
    print(left - 1, top - 1, right - left + 3, bottom - top + 3)

def bottom(x0, y0, x1, y1):
    r = a[y0:y1, x0:x1]
    bg = np.median(r.reshape(-1, 3), axis=0)
    rows = np.where((np.abs(r - bg).max(axis=2) > 24).sum(axis=1) >= 2)[0]
    print(y0 + int(rows[-1]) + 1 if rows.size else y0)

n = [int(v) for v in sys.argv[3:]]
{"box": box, "bottom": bottom}[sys.argv[1]](*n)
'@

Set-Content -Path $script:SlowProxy -Encoding utf8 -Value @'
// Capture helper: forwards http://127.0.0.1:8766 to the stub engine on 8765 after a pause, so a
// debate's "is speaking" state stays on screen long enough to be captured.
import { createServer, request } from 'node:http';
const DELAY_MS = 4000;
createServer((req, res) => {
  const chunks = [];
  req.on('data', (c) => chunks.push(c));
  req.on('end', () => {
    setTimeout(() => {
      const up = request({ host: '127.0.0.1', port: 8765, path: req.url, method: req.method, headers: { ...req.headers, host: '127.0.0.1:8765' } },
        (upRes) => { res.writeHead(upRes.statusCode ?? 502, upRes.headers); upRes.pipe(res); });
      up.on('error', () => { res.writeHead(502); res.end(); });
      up.end(Buffer.concat(chunks));
    }, DELAY_MS);
  });
}).listen(8766, '127.0.0.1');
'@

function Park { Hover 900 20 300 }

function Save-Temp {
    $p = Join-Path ([System.IO.Path]::GetTempPath()) "sim-practice-$PID.png"
    Save-SimShot -Path $p | Out-Null
    $p
}

function Dbg([string]$Name) {
    <# A whole-window capture outside raw/, with its OCR lines, kept when a step goes wrong. #>
    New-Item -ItemType Directory -Force -Path $script:Dbg | Out-Null
    Save-SimShot -Path (Join-Path $script:Dbg "$Name.png") | Out-Null
    Get-SimText | ForEach-Object { "{0,5} {1,5} {2,4} {3,3}  {4}" -f $_.x, $_.y, $_.w, $_.h, $_.text } | Set-Content -Encoding utf8 (Join-Path $script:Dbg "$Name.txt")
}

function Get-Box([int]$X, [int]$Y) { [int[]]((python $script:PrGeom box (Save-Temp) $X $Y) -split ' ') }
function Get-Bottom([int]$X0, [int]$Y0, [int]$X1, [int]$Y1) { [int](python $script:PrGeom bottom (Save-Temp) $X0 $Y0 $X1 $Y1) }

function Find-One([string]$Text, [int[]]$Region) {
    $hit = @(Find-SimText $Text -Region $Region)
    if (-not $hit) { Dbg ("missing-" + ($Text -replace '[^\w]', '_')); throw "'$Text' not found" }
    $hit[0]
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

function ClickWord([string]$Word, [int[]]$Region, [int]$Nth = 0, [int]$WaitMs = 700, [switch]$Case) {
    <# Clicks $Word inside the OCR line that holds it (OCR merges a row of chips or buttons). #>
    $hits = @(Find-SimText $Word -Region $Region | Where-Object { -not $Case -or $_.text -clike "*$Word*" })
    if ($hits.Count -le $Nth) { Dbg ("word-" + ($Word -replace '[^\w]', '_')); throw "Word '$Word' not found on screen" }
    $h = $hits[$Nth]
    $i = if ($Case) { $h.text.IndexOf($Word) } else { $h.text.ToLower().IndexOf($Word.ToLower()) }
    $x = $h.x + [int]($h.w * ($i + $Word.Length / 2) / $h.text.Length)
    ClickAt $x ($h.y + [int]($h.h / 2)) $WaitMs
}

function Wait-NoToast {
    <# Toasts stack top right; shots wait until they are gone. #>
    try { Wait-Text '.' -Region 1040, 40, 400, 70 -Seconds 9 -Gone } catch { }
}

function Shot-Box([string]$Id, [int[]]$Box, [int]$Pad = 8) {
    <# The box with a margin, kept inside the 1440x900 client area. #>
    $x = [Math]::Max(0, $Box[0] - $Pad); $y = [Math]::Max(38, $Box[1] - $Pad)
    if ($Box[1] + $Box[3] -ge 899) { Write-Warning "$Id runs past the window bottom" }
    Shot $Id -Crop $x, $y, ([Math]::Min($Box[2] + 2 * $Pad, 1440 - $x)), ([Math]::Min($Box[3] + 2 * $Pad, 900 - $y))
}

function Send-AI {
    <# Answers "Send this to the AI?" with Send (OCR reads AI as "Al"; Send sits right of Cancel). #>
    for ($i = 0; $i -lt 12; $i++) {
        $d = @(Find-SimText 'ask again this session')
        if ($d) { ClickAt ($d[0].x + 363) ($d[0].y + 47) 1200; return $true }
        Start-Sleep -Milliseconds 300
    }
    $false
}

function Send-FileDialog([string]$Path) {
    <# Puts a full path into the file name box of the dialog Simulacra just showed, and opens it. #>
    Send-SimFileDialog $Path
}

function Write-Text([string]$Text, [int]$WaitMs = 600) {
    <# Types into a fresh field: the first key alone (an empty field can drop keys while it
       swaps its placeholder out), then the rest. #>
    TypeText $Text.Substring(0, 1) 350
    if ($Text.Length -gt 1) { TypeText $Text.Substring(1) $WaitMs }
}

function Reset-Practice {
    Key '{ESC}' 250; Key '{ESC}' 250
}

function Page-Top { Scroll 840 400 25 }

# --- Review -----------------------------------------------------------------------------------

function Review-Tab([string]$Tab) {
    Key '^6' 900
    Page-Top
    ClickAt $script:RTab[$Tab] 139 1000
}

function Learn-Sub([string]$Sub) {
    Review-Tab 'Learn'
    ClickAt $script:RSub[$Sub] 189 1100
    Page-Top
}

function Open-InspectorMenu([int]$Y) {
    <# The Thread Inspector card's "…" (x 1413 on its author row), then the Learn submenu. #>
    ClickAt 1413 $Y 900
    $l = Find-One 'Learn' @(1280, $Y, 160, 420)
    Hover ($l.x + 20) ($l.y + 6) 1200
    $l
}

function Sheet-Save {
    <# The Learn sheet's Save sits at the right of its footer, level with "Suggest with AI". #>
    $s = Find-One 'Suggest with' @(420, 300, 600, 500)
    ClickAt 975 ($s.y + 6) 1600
}

function Answer-Key([string]$Prompt) {
    <# The opening words of the right answer to a prompt (the four learnt items). #>
    if ($Prompt -match 'order of ideas') { return 'Substance is one' }
    if ($Prompt -match 'bronze|mirror|Tell me') { return 'A reflection' }
    if ($Prompt -match 'define') { return 'By substance' }
    if ($Prompt -match 'mode') { return 'A mode is' }
    $null
}

function Near-Answer([string]$Prompt) {
    <# The right answer with one word left out, so the check calls it a small slip. #>
    if ($Prompt -match 'order of ideas') { return 'Substance is one and infinite; whether conceived under the attribute of Extension or the attribute of Thought, the order and connection of ideas is the same as order and connection of things. #Monism' }
    if ($Prompt -match 'bronze|mirror|Tell me') { return 'A reflection repeats the form of light without the experience of it' }
    if ($Prompt -match 'define') { return 'By substance I understand what is in itself and conceived through itself.' }
    'A mode is an affection of substance: something that is in another and conceived through another.'
}

function Prompt-Text {
    ((@(Get-SimText | Where-Object { $_.x -ge 505 -and $_.x -lt 1180 -and $_.y -ge 215 -and $_.y -lt 420 }) | ForEach-Object text) -join ' ')
}

Step 'welcome' -Always {
    New-Item -ItemType Directory -Force -Path $script:Dbg | Out-Null
    if (-not (Get-NetTCPConnection -LocalPort 8765 -State Listen -ErrorAction SilentlyContinue)) {
        Start-Process node -ArgumentList (Join-Path $PSScriptRoot '..\stub-ai.mjs') -WindowStyle Hidden
        Start-Sleep -Seconds 2
    }
    Skip-Welcome
}

Step 'review empty' {
    Review-Tab 'Today'
    Park
    Shot 'review-empty' -Crop 380, 182, 920, 302
}

Step 'learn post' {
    Reset-Practice
    Key '^1' 900
    Palette 'Substance is one' 1600
    Park
    Mark
    Open-InspectorMenu 104 | Out-Null
    Shot 'review-post-learn-menu' -Auto -AutoPad 8 -Within 1000, 38, 1440, 600
    $q = Find-One 'Add question' @(1000, 200, 300, 200)
    ClickAt ($q.x + 30) ($q.y + 6) 1500
    Write-Text 'According to Spinoza, how does the order of ideas relate to the order of things?' 700
    $t = Find-One 'Comma separated' @(420, 300, 600, 300)
    ClickAt ($t.x + 100) ($t.y + 6) 400
    Write-Text 'Spinoza, substance' 700
    Park
    $s = Find-One 'Suggest with' @(420, 300, 600, 500)
    Shot 'review-learn-sheet-post' -Crop 419, 87, 602, ($s.y + 31 - 87)
    ClickAt 975 ($s.y + 6) 1600
    Wait-NoToast
    # The card in the stream (the palette scrolled it into view) now wears the learn chip.
    $c = Find-One 'Substance is one and' @(296, 80, 464, 820)
    $a = @(Find-SimText 'Baruch de Spinoza' -Region 296, ($c.y - 70), 200, 70) | Sort-Object y | Select-Object -Last 1
    if (-not $a) { Dbg 'learn-badge'; throw 'the author row of the card was not found' }
    Shot 'review-learn-badge' -Crop 292, ($a.y - 12), 468, 56
}

Step 'learn answer' {
    Reset-Practice
    Palette 'bronze mirror' 1600
    $s = Find-One 'Sokrates of Alopece' @(800, 250, 400, 250)
    Open-InspectorMenu ($s.y + 6) | Out-Null
    $a = Find-One 'Add answer' @(1000, 200, 300, 400)
    ClickAt ($a.x + 30) ($a.y + 6) 1500
    Write-Text 'A reflection repeats the form of the light without the experience of it.' 700
    Sheet-Save
}

Step 'learn passages' {
    Reset-Practice
    Key '^2' 1300
    Click 'Ethics' -Region 241, 80, 1199, 800 -WaitMs 1500
    Click 'Learn passages' -WaitMs 1500
    $s1 = Find-One 'By substance I understand' @(430, 190, 580, 200)
    ClickAt ($s1.x + 120) ($s1.y + 6) 1000
    Write-Text 'How does Spinoza define substance?' 700
    Click 'Add another passage' -WaitMs 1000
    $s3 = Find-One 'A mode is an affection' @(430, 190, 580, 200)
    ClickAt ($s3.x + 120) ($s3.y + 6) 1000
    Write-Text 'What is a mode, according to Spinoza?' 700
    # The sheet's body scrolls: bring "Passages to save" up under the question.
    Scroll 720 560 -2
    Park
    $f = Find-One 'Suggest with' @(420, 300, 600, 500)
    Shot 'review-learn-sheet-passages' -Crop 419, 87, 602, ($f.y + 31 - 87)
    ClickAt 975 ($f.y + 6) 1600
}

Step 'picker learn' {
    Reset-Practice
    Key '^6' 1000
    Page-Top
    Wait-NoToast
    ClickAt 1052 82 1300
    ClickAt 840 286 400
    TypeText 'order' 1300
    Park
    $b = Find-One 'Back to' @(380, 300, 920, 600)
    Shot 'review-picker-learn' -Crop 376, 168, 928, ($b.y + 32 - 168)
    ClickAt ($b.x + 40) ($b.y + 6) 1000
}

Step 'today' {
    Wait-NoToast
    Review-Tab 'Today'
    Park
    Shot 'review-today'
    Shot 'review-header' -Crop 370, 52, 940, 110
    $q = Find-One 'Quiz yourself' @(380, 380, 400, 300)
    Shot 'review-quiz-modes' -Crop 376, ($q.y - 28), 928, 182
}

Step 'session' {
    Review-Tab 'Today'
    ClickWord 'Start review' @(380, 330, 400, 50) -WaitMs 1500
    Park
    $p = Find-One 'Show answer' @(500, 380, 700, 200)
    Shot 'review-session-prompt' -Crop 488, 166, 704, ($p.y + 30 - 166)
    Key ' ' 1300
    Park
    $g = Find-One 'Again' @(500, 300, 200, 500)
    Shot 'review-session-revealed' -Crop 488, 166, 704, ($g.y + 47 - 166)
    Shot 'review-grade-buttons' -Crop 496, ($g.y - 25), 688, 68
    Key '3' 1300
    Key ' ' 1100; Key '1' 1300
    for ($i = 0; $i -lt 8; $i++) {
        if (@(Find-SimText 'Session complete')) { break }
        Key ' ' 1000; Key '3' 1300
    }
    Park
    # The summary's buttons sit under its tip (OCR misses the filled Back to today).
    $b = Find-One 'Short daily sessions' @(380, 300, 920, 400)
    Shot 'review-summary' -Crop 376, 168, 928, ($b.y + 66 - 168)
    ClickAt 452 ($b.y + 37) 1300
    Park
    $m = Find-One 'Multiple choice' @(600, 380, 500, 150)
    Shot 'review-caught-up' -Crop 380, 186, 920, ($m.y + 32 - 186)
}

Step 'items' {
    Review-Tab 'Items'
    $metas = @(Get-SimText | Where-Object { $_.x -ge 388 -and $_.x -le 400 -and $_.y -ge 200 -and $_.text -match '^(Question|Answer)' }) | Sort-Object y
    if (-not $metas) { Dbg 'items'; throw 'no item rows found' }
    $last = $metas[-1]
    ClickAt 1248 ($last.y - 12) 1200
    Park
    Shot 'review-items' -Crop 376, 168, 928, ($last.y + 36 - 168)
    ClickAt 1248 ($last.y - 12) 1200
}

Step 'map' {
    Reset-Practice
    Key '^6' 900
    Page-Top
    Wait-NoToast
    ClickAt 1225 82 1300
    ClickAt 840 286 400
    TypeText 'order' 1300
    $rows = @(Get-SimText | Where-Object { $_.x -ge 420 -and $_.x -le 430 -and $_.y -ge 320 -and $_.text -match '^Baruch de Spinoza' }) | Sort-Object y
    foreach ($r in $rows) { ClickAt 800 ($r.y + 22) 600 }
    Park
    $b = Find-One 'Back to' @(380, 300, 920, 600)
    Shot 'review-picker-map' -Crop 376, 168, 928, ($b.y + 32 - 168)
    ClickAt 1205 ($b.y + 6) 1600
    Scroll 840 600 -10
    Park
    $c = Find-One 'Cancel' @(1000, 700, 300, 200)
    $t = Find-One 'New connection map' @(380, 38, 400, 200)
    Shot 'review-map-editor' -Crop 376, ($t.y - 26), 928, ($c.y + 31 - $t.y + 26)
    # Every point needs a connection: join the last post with the concept, then save.
    $a = Find-One 'Then do not hurry' @(394, 250, 892, 480)
    $m = Find-One 'monism' @(394, 250, 892, 480)
    ClickAt ($a.x + 20) ($a.y + 6) 800
    ClickAt ($m.x + 20) ($m.y + 6) 800
    $c = Find-One 'Cancel' @(1000, 700, 300, 200)
    ClickAt ($c.x + 104) ($c.y + 6) 1500
}

Step 'drill' {
    Wait-NoToast
    Review-Tab 'Today'
    ClickWord 'Start review' @(380, 330, 400, 50) -WaitMs 1600
    $s = Find-One 'Substance is one' @(394, 280, 892, 440)
    $m = Find-One 'monism' @(394, 280, 892, 440)
    $o = Find-One 'The order and' @(394, 280, 892, 440)
    $h = Find-One 'Then do not' @(394, 280, 892, 440)
    ClickAt ($s.x + 20) ($s.y - 12) 700
    ClickAt ($m.x + 20) ($m.y + 6) 700
    ClickAt ($o.x + 20) ($o.y - 12) 700
    ClickAt ($h.x + 20) ($h.y - 12) 700
    Park
    $d = Find-One 'connections drawn' @(380, 600, 500, 250)
    Shot 'review-drill' -Crop 370, 166, 940, ($d.y + 34 - 166)
    Key ' ' 1500
    Scroll 840 600 -5
    Park
    $g = Find-One 'Graded' @(380, 600, 500, 300)
    $p = Find-One 'left' @(370, 38, 300, 120)
    Shot 'review-drill-result' -Crop 370, ($p.y - 10), 940, ($g.y + 34 - $p.y + 10)
    Page-Top
    $e = Find-One 'End' @(1200, 160, 120, 40)
    ClickAt ($e.x + 10) ($e.y + 6) 1200
}

function Quiz-Pick([switch]$Wrong) {
    <# Picks the right (or a wrong) option of the open multiple-choice question. #>
    $key = Answer-Key (Prompt-Text)
    $opts = @(Get-SimText | Where-Object { $_.x -ge 530 -and $_.x -le 545 -and $_.y -ge 480 })
    if (-not $opts) { Dbg 'quiz-options'; throw 'no options found' }
    $pick = if ($Wrong) { $opts | Where-Object { -not $key -or $_.text -notlike "$key*" } | Select-Object -First 1 }
            else { $opts | Where-Object { $key -and $_.text -like "$key*" } | Select-Object -First 1 }
    if (-not $pick) { $pick = $opts[0] }
    ClickAt ($pick.x + 120) ($pick.y + 6) 1200
}

function Move-Piece([string]$Start, [int]$Target) {
    <# Clicks the up arrow (x 1148) of the piece starting with $Start until it is piece $Target.
       A one-line piece's arrow sits just above its text line, a two-line piece's beside its first line. #>
    for ($i = 0; $i -lt 7; $i++) {
        $lines = @(Get-SimText | Where-Object { $_.x -ge 540 -and $_.x -le 552 -and $_.y -ge 280 -and $_.y -lt 760 }) | Sort-Object y
        $heads = @()
        for ($k = 0; $k -lt $lines.Count; $k++) {
            if ($k -gt 0 -and ($lines[$k].y - $lines[$k - 1].y) -lt 26) { continue }
            $two = ($k + 1 -lt $lines.Count) -and (($lines[$k + 1].y - $lines[$k].y) -lt 26)
            $heads += [pscustomobject]@{ text = $lines[$k].text; y = $lines[$k].y; two = $two }
        }
        $at = -1
        for ($k = 0; $k -lt $heads.Count; $k++) { if ($heads[$k].text -like "$Start*") { $at = $k; break } }
        if ($at -lt 0) { Dbg 'order-piece'; throw "piece '$Start' not found" }
        if ($at + 1 -le $Target) { return }
        $h = $heads[$at]
        ClickAt 1148 ($(if ($h.two) { $h.y + 5 } else { $h.y - 4 })) 700
    }
}

function Quiz-Sure([string]$Level) {
    $h = Find-One 'How sure' @(495, 380, 200, 300)
    $x = @{ Guessing = 639; Unsure = 709; Sure = 766 }[$Level]
    ClickAt $x ($h.y + 6) 500
}

Step 'quizzes' {
    Review-Tab 'Today'
    ClickWord 'Multiple choice' @(390, 380, 900, 400) -WaitMs 1500
    Quiz-Sure 'Unsure'
    Quiz-Pick -Wrong
    Park
    $n = Find-One 'Not quite' @(495, 500, 500, 400)
    Shot 'review-quiz-choice' -Crop 488, 166, 704, ($n.y + 34 - 166)
    foreach ($level in 'Sure', 'Guessing', 'Sure') {
        Key ' ' 1300
        Quiz-Sure $level
        Quiz-Pick
    }
    Key ' ' 1500
    Park
    $g = Find-One 'Back to today' @(380, 200, 920, 300)
    Shot 'review-quiz-done' -Crop 376, 168, 928, ($g.y + 30 - 168)
    ClickWord 'Back to today' @(380, 200, 920, 300) -WaitMs 1300
    # Free recall: a near answer to whichever item comes first.
    ClickWord 'Free recall' @(390, 380, 900, 400) -WaitMs 1500
    $answer = Near-Answer (Prompt-Text)
    ClickAt 790 461 400
    Write-Text $answer 700
    Key '{ENTER}' 1500
    Park
    $h = Find-One 'outlined grade' @(495, 400, 700, 400)
    Shot 'review-quiz-recall' -Crop 488, 166, 704, ($h.y + 98 - 166)
    ClickAt 1155 181 1200
    # Put in order: move one piece, then check.
    ClickWord 'Put in order' @(390, 380, 900, 400) -WaitMs 1500
    # Move the first three sentences of the passage into place (so the check shows both kinds).
    $target = 1
    foreach ($start in 'By substance', 'Except God', 'A mode is') {
        Move-Piece $start $target
        $target++
    }
    Click 'Check order' -WaitMs 1300
    Park
    $o = Find-One 'in place' @(500, 600, 680, 300)
    Shot 'review-quiz-order' -Crop 496, 204, 688, ($o.y + 34 - 204)
    ClickAt 1155 181 1200
}

Step 'stats' {
    Review-Tab 'Statistics'
    Start-Sleep -Milliseconds 800
    Park
    Shot 'review-stats'
}

Step 'tutor' {
    Learn-Sub 'Tutor'
    Park
    $e = Find-One 'Earlier dialogues' @(380, 300, 920, 500)
    $bottom = Get-Bottom 384 ($e.y + 10) 1296 899
    Shot 'review-learn-tutor-start' -Crop 376, 166, 928, ($bottom + 14 - 166)
    # The dialogue waiting for an answer.
    ClickWord 'Substance is one' @(380, $e.y, 920, 200) -WaitMs 1500
    Park
    $f = Find-One 'Finish and sum up' @(380, 300, 920, 600)
    Shot 'review-learn-tutor-dialogue' -Crop 376, 218, 928, ($f.y + 34 - 218)
    Click 'All dialogues' -WaitMs 1200
    $e = Find-One 'Earlier dialogues' @(380, 300, 920, 500)
    ClickWord 'The order and connection' @(380, $e.y, 920, 200) -WaitMs 1500
    Page-Top
    Park
    $s = Find-One 'Summary' @(380, 300, 300, 300)
    $b = Get-Box 386 ($s.y + 4)
    Shot-Box 'review-learn-tutor-summary' $b 6
    Page-Top
}

function Create-Goal {
    $c = Find-One 'Cancel' @(380, 400, 680, 300)
    ClickAt ($c.x - 70) ($c.y + 6) 1300
}

Step 'goals' {
    Learn-Sub 'Goals'
    ClickAt 1253 234 1200
    ClickAt 720 315 400
    Write-Text 'Understand the Ethics' 500
    ClickAt 720 392 400
    Write-Text 'To lead the reading group in spring' 700
    Park
    $c = Find-One 'Cancel' @(380, 400, 680, 300)
    Shot 'review-learn-goal-new' -Crop 376, 218, 928, ($c.y + 30 - 218)
    Create-Goal
    # The goal's detail: two tasks (one ticked), a concept, the Ethics and Spinoza's post.
    $t = Find-One 'Add a task' @(380, 600, 680, 300)
    ClickAt ($t.x + 100) ($t.y + 6) 400
    Write-Text 'Read Part I, Of God' 400; Key '{ENTER}' 900
    Write-Text 'Summarise the definitions of Part I' 400; Key '{ENTER}' 900
    $r = Find-One 'Of God' @(380, 600, 680, 300)
    ClickAt 399 ($r.y + 6) 900
    Scroll 720 600 -10
    $k = Find-One 'Add a concept' @(380, 300, 680, 600)
    ClickAt ($k.x + 100) ($k.y + 6) 400
    Write-Text 'substance' 400; Key '{ENTER}' 1000
    $v = Find-One 'VAULT DOCUMENTS' @(380, 300, 680, 600)
    ClickWord 'Ethics' @(380, ($v.y + 15), 300, 40) -WaitMs 1000
    Click 'Link a post' -WaitMs 1500
    Page-Top
    ClickAt 840 286 400
    TypeText 'Substance is one' 1300
    $l = Find-One 'Link' @(1130, 310, 60, 100)
    ClickAt ($l.x + 15) ($l.y + 6) 1500
    Page-Top
    Park
    Shot 'review-learn-goal-detail'
    # A second goal, three months out.
    Click 'All goals' -WaitMs 1200
    ClickAt 1253 234 1200
    ClickAt 720 315 400
    Write-Text 'Read the Tractatus' 500
    ClickAt 720 392 400
    Write-Text 'To see what the ladder is for' 600
    ClickAt 567 461 600
    Create-Goal
    Click 'All goals' -WaitMs 1200
    Park
    $bottom = Get-Bottom 384 270 1296 899
    Shot 'review-learn-goals' -Crop 376, 218, 928, ($bottom + 14 - 218)
    Click 'Weekly review' -WaitMs 1600
    Park
    Shot 'review-learn-weekly'
}

Step 'gaps' {
    Learn-Sub 'Gaps'
    Start-Sleep -Milliseconds 800
    Park
    Shot 'review-learn-gaps'
}

Step 'palace' {
    Learn-Sub 'Palace'
    ClickAt 660 344 400
    Write-Text 'My childhood home' 500
    # Build palace ends the name row (white on purple, which OCR can miss).
    $bp = @(Find-SimText 'Build palace' -Region 380, 300, 920, 100) | Select-Object -First 1
    if ($bp) { ClickAt ($bp.x + 30) ($bp.y + 6) 1600 } else { ClickAt 1238 344 1600 }
    ClickWord 'Library' @(380, 460, 300, 30) -WaitMs 1000
    # Place 1 (Door): the Spinoza question, with a cue.
    Scroll 1000 600 -10
    $s = Find-One 'Search your review items' @(380, 300, 920, 600)
    ClickAt ($s.x + 100) ($s.y + 6) 400
    TypeText 'order of ideas' 1100
    $r = Find-One 'According to Spinoza' @(380, 300, 920, 600)
    ClickAt ($r.x + 40) ($r.y + 6) 1300
    Scroll 1000 600 -10
    $c = Find-One 'Picture something' @(380, 300, 920, 600)
    ClickAt ($c.x + 100) ($c.y + 6) 400
    Write-Text 'a door made of two mirrors facing each other' 500; Key '{ENTER}' 1000
    # Place 2 (Mirror): Spinoza's post, with a cue.
    Page-Top
    ClickAt 926 463 1000
    Scroll 1000 600 -10
    Click 'Place a post' -WaitMs 1500
    TypeText 'Substance is one' 1300
    $p = Find-One 'Place here' @(1190, 310, 110, 100)
    ClickAt ($p.x + 30) ($p.y + 6) 1500
    Scroll 1000 600 -10
    $c = Find-One 'Picture something' @(380, 300, 920, 600)
    ClickAt ($c.x + 100) ($c.y + 6) 400
    Write-Text 'a giant mirror reflecting the whole sky' 500; Key '{ENTER}' 1000
    Scroll 1000 600 -10
    Park
    Shot 'review-learn-palace-editor'
    Click 'All palaces' -WaitMs 1300
    Page-Top
    Park
    $bottom = Get-Bottom 384 230 1296 899
    Shot 'review-learn-palace-list' -Crop 376, 218, 928, ($bottom + 14 - 218)
    Click 'Walk through' -Region 380, 300, 920, 200 -WaitMs 1500
    Key ' ' 1300
    Park
    $g = Find-One 'Again' @(880, 400, 420, 400)
    Shot 'review-learn-palace-walk' -Crop 370, 218, 940, ($g.y + 55 - 218)
    Click 'End walk' -WaitMs 1000
}

# --- Journal ----------------------------------------------------------------------------------

function Journal-Tab([string]$Tab) {
    Key '^8' 1000
    Page-Top
    ClickAt $script:JTab[$Tab] 174 1100
}

Step 'journal today' {
    Reset-Practice
    Journal-Tab 'Today'
    Park
    Shot 'journal-today'
}

Step 'journal entries' {
    Journal-Tab 'Journal'
    ClickWord 'Evening review' @(360, 200, 960, 40) -WaitMs 1500
    Park
    # The buttons sit one row under the links hint (OCR merges the button row).
    $c = Find-One 'Links:' @(360, 400, 960, 300)
    $c.y += 32
    Shot 'journal-editor' -Crop 356, 203, 968, ($c.y + 30 - 203)
    $w = Find-One 'What went well' @(370, 250, 600, 60)
    ClickAt ($w.x + 200) ($w.y + 6) 400
    Key '{END}' 200; Key '{ENTER}' 200
    Write-Text 'The group took the second definition further than I expected.' 600
    $c = Find-One 'Links:' @(360, 400, 960, 300)
    $c.y += 32
    for ($i = 0; $i -lt 3; $i++) {
        ClickAt 404 ($c.y + 6) 1200
        if (@(Find-SimText 'Saved' -Region 1180, 215, 140, 30 | Where-Object { $_.text -notmatch 'Not' })) { break }
    }
    Park
    Shot 'journal-header' -Crop 356, 52, 968, 142
    ClickWord 'Close' @(360, ($c.y - 6), 600, 30) -WaitMs 1200
    Page-Top
    Park
    Shot 'journal-entries'
}

Step 'journal mood' {
    Journal-Tab 'Mood'
    ClickAt 645 550 800
    ClickAt 966 336 600
    ClickAt 1065 384 400
    TypeText '7.5' 600
    Park
    # OCR misses the filled Save check-in: it sits one row under the support tick.
    $t = Find-One 'pointers to support' @(680, 450, 400, 100)
    Shot 'journal-checkin' -Crop 356, 203, 968, ($t.y + 68 - 203)
    ClickAt ($t.x - 12) ($t.y + 6) 900
    Park
    $m = Find-One 'medical advice' @(680, 450, 640, 250)
    Shot 'journal-support' -Crop 680, ($t.y - 12), 636, ($m.y + 32 - $t.y + 12)
    ClickAt 750 ($m.y + 46) 1300
    # Bring the patterns up: the heat strip near the top, the charts and associations below.
    Page-Top
    for ($i = 0; $i -lt 30; $i++) {
        $h = @(Find-SimText 'Mood over time' -Region 360, 38, 600, 862) | Select-Object -First 1
        if (-not $h -or $h.y -le 110) { break }
        Move-SimMouse 840 600 -WaitMs 60
        Send-SimWheel (-[Math]::Min(120, [int](($h.y - 70) / 2)))
        Start-Sleep -Milliseconds 400
    }
    Park
    Shot 'journal-mood'
}

Step 'journal focus' {
    Journal-Tab 'Focus'
    ClickAt 900 270 400
    Write-Text 'Draft the introduction' 600
    Park
    $s = Find-One 'Distractions can go' @(360, 300, 960, 200)
    Shot 'journal-focus-setup' -Crop 356, 203, 968, ($s.y + 62 - 203)
    $started = Get-Date
    ClickAt 404 ($s.y + 32) 1500
    Start-Sleep -Seconds 3
    Park
    $f = Find-One 'Focus mode' @(360, 300, 960, 200)
    Shot 'journal-focus-running' -Crop 356, 203, 968, ($f.y + 30 - 203)
    ClickWord 'Focus mode' @(360, 300, 960, 200) -WaitMs 1600
    Park
    Shot 'journal-focus-mode'
    ClickWord 'Leave focus mode' -WaitMs 1300
    $wait = 63 - ((Get-Date) - $started).TotalSeconds
    if ($wait -gt 0) { Start-Sleep -Seconds ([int]$wait) }
    ClickWord 'End now' @(360, 300, 960, 200) -WaitMs 1600
    Park
    # The outcome buttons sit one row under the reflection field (OCR misses that row).
    $d = Find-One 'what happened' @(360, 250, 600, 200)
    Shot 'journal-focus-reflection' -Crop 356, 203, 968, ($d.y + 75 - 203)
    ClickAt 840 ($d.y + 6) 400
    Write-Text 'Outline done, introduction half written' 600
    ClickAt 406 ($d.y + 45) 1300
    Park
    Scroll 840 600 -10
    Park
    $l = Find-One 'Sessions' @(360, 38, 300, 600)
    Shot-Box 'journal-focus-log' (Get-Box 366 ($l.y + 4)) 4
}

Step 'journal capsules' {
    Journal-Tab 'Capsules'
    ClickAt 455 219 1300
    ClickAt 840 340 400
    Write-Text 'Dear me in a month: did the reading group finish Part II? Be honest about the Tractatus.' 600
    ClickAt 440 437 400
    Key '^a' 200; TypeText '30' 800
    $a = Find-One 'Add images' @(360, 400, 600, 200)
    ClickAt ($a.x + 40) ($a.y + 6) 800
    Send-FileDialog $script:CapsuleImage
    Park
    $a = Find-One 'Add images' @(360, 400, 600, 200)
    Shot 'journal-capsule-form' -Crop 356, 203, 968, ($a.y + 30 - 203)
    ClickAt 403 ($a.y + 6) 1500
    Park
    $bottom = Get-Bottom 364 250 1316 899
    Shot 'journal-capsules' -Crop 356, 203, 968, ($bottom + 10 - 203)
    # The ready capsule comes first; its filled Open button sits left of its Delete.
    $o = @(Find-SimText 'Delete' -Region 1200, 240, 120, 200) | Sort-Object y | Select-Object -First 1
    if (-not $o) { Dbg 'capsule-open'; throw 'no ready capsule to open' }
    ClickAt 1190 ($o.y + 5) 1500
    Park
    $c = Find-One 'Close' @(360, 300, 960, 600)
    Shot 'journal-capsule-open' -Crop 356, 203, 968, ($c.y + 30 - 203)
}

# --- Tasks ------------------------------------------------------------------------------------

Step 'tasks' {
    Reset-Practice
    Key '^1' 1000
    ClickAt 520 112 400
    Key '^a' 150; Key '{BACKSPACE}' 300
    $lines = 'Reading plan', '[ ] Finish Ethics part I ^today !! &reading ~1h', '[ ] Notes on the Tractatus ^fri &reading',
        '  [ ] Propositions 1-3 ^fri', '[ ] Reply to Sokrates ^tomorrow !', '[ ] Buy notebook &errands', '[x] Index the vault'
    Write-Text $lines[0] 300
    foreach ($line in $lines[1..($lines.Count - 1)]) { Key '+{ENTER}' 250; TypeText $line 300 }
    Key '^{ENTER}' 2500
    Click 'Tasks' -Region 0, 140, 241, 320 -WaitMs 1600
    Park
    Shot 'tasks-page'
    $s = Find-One 'Show done' @(450, 200, 400, 80)
    Shot 'tasks-filters' -Crop 448, 98, 470, ($s.y + 26 - 98)
}

# --- Arena ------------------------------------------------------------------------------------

function Open-AISettings {
    Reset-Practice
    Key '^,' 1500
    ClickAt 252 248 1500
}

function Save-AISettings {
    Click 'Save & Apply' -WaitMs 1500
    Key '{ESC}' 800
    if (@(Find-SimText 'Search settings')) { Click 'Close' -Region 980, 780, 120, 40 -WaitMs 800 }
}

function Arena-Tab([string]$Tab) {
    Key '^9' 1300
    Page-Top
    ClickAt (@{ Debate = 549; Lab = 1131 }[$Tab]) 149 1200
}

function Stop-Relay {
    <# Stops the slow relay this scene started (only a node process running its relay script). #>
    Get-CimInstance Win32_Process -Filter "Name='node.exe'" | Where-Object { $_.CommandLine -like '*simulacra-docs-practice-relay.mjs*' } |
        ForEach-Object { Stop-Process -Id $_.ProcessId -Force -ErrorAction SilentlyContinue }
}

function Show-Saved([string]$Title) {
    <# Scrolls until the "Saved …" card shows (under the form with a record open, beside it
       under the idle note with none); returns its title line. #>
    for ($i = 0; $i -lt 6; $i++) {
        $h = @(Find-SimText $Title -Region 256, 38, 1180, 820) | Select-Object -First 1
        if ($h -and $h.y -lt 560) { return $h }
        Scroll 446 600 -3
    }
    Find-One $Title @(256, 38, 1180, 860)
}

Step 'arena verdict' {
    # A daily budget (the spend confirmation shows what is left) and, for the debate shots, a
    # slow relay in front of the stub so a running debate stays on screen.
    if (-not (Get-NetTCPConnection -LocalPort 8766 -State Listen -ErrorAction SilentlyContinue)) {
        $script:Relay = Start-Process node -ArgumentList $script:SlowProxy -WindowStyle Hidden -PassThru
        Start-Sleep -Seconds 1
    }
    Open-AISettings
    $u = Find-One 'Custom Endpoint URL' @(410, 300, 300, 150)
    ClickAt 830 ($u.y + 32) 400
    Key '^a' 150; TypeText 'http://127.0.0.1:8766/v1/chat/completions' 500
    Scroll 830 500 -10
    $d = Find-One 'Daily request budget' @(420, 300, 400, 500)
    ClickAt 1216 ($d.y + 14) 500
    ClickAt 1216 ($d.y + 14) 500
    Save-AISettings
    # The judged debate (seeded): its verdict and the claims that still stand.
    Arena-Tab 'Debate'
    $sv = Show-Saved 'Saved debates'
    $j = Find-One 'Understanding is nothing' @(($sv.x - 30), $sv.y, 780, 300)
    ClickAt ($j.x + 60) ($j.y + 6) 1600
    Page-Top
    for ($i = 0; $i -lt 8 -and -not (@(Find-SimText 'Judged by' -Region 712, 38, 728, 600)); $i++) { Scroll 1040 600 -3 }
    Park
    $v = Find-One 'Judged by' @(712, 38, 728, 862)
    Shot-Box 'arena-verdict' (Get-Box 720 ($v.y - 30)) 6
    for ($i = 0; $i -lt 8 -and -not (@(Find-SimText 'Defeated' -Region 712, 38, 728, 862)); $i++) { Scroll 1040 600 -3 }
    Park
    $w = Find-One 'claims still stand' @(712, 38, 728, 862)
    Shot-Box 'arena-claims' (Get-Box 720 ($w.y + 6)) 6
}

Step 'arena debate' {
    Arena-Tab 'Debate'
    # With a debate open, the setup form hides behind "New debate" (under "This debate is saved").
    $n = @(Find-SimText 'This debate is saved' -Region 256, 170, 440, 200)
    if ($n) { ClickAt 323 ($n[0].y + 39) 1300 }
    Page-Top
    $m = Find-One 'MOTION' @(256, 180, 440, 200)
    ClickAt 445 ($m.y + 48) 400
    Key '^a' 150
    Write-Text 'Symbol prediction is isomorphic to human cognitive modelling.' 800
    Park
    Shot 'arena-debate-setup'
    $d = Find-One 'DEBATERS' @(256, 300, 440, 400)
    $so = Find-One 'Sokrates of Alopece' @(256, $d.y, 440, 120)
    $sp = Find-One 'Baruch de Spinoza' @(256, $d.y, 440, 120)
    ClickAt 602 ($so.y + 6) 700
    ClickAt 654 ($sp.y + 6) 700
    $mo = Find-One 'MODERATOR' @(256, 380, 440, 520)
    ClickWord 'Simone Weil' @(256, ($mo.y + 15), 440, 40) -WaitMs 800
    Park
    $j = Find-One 'JUDGE' @(256, 380, 440, 520)
    Shot 'arena-debaters' -Crop 262, ($d.y - 10), 428, ($j.y + 46 - $d.y + 10)
    # Start: the spend confirmation, then the running debate (the relay answers slowly).
    Mark
    ClickAt 324 ($j.y + 65) 1500
    Shot 'arena-spend-confirm' -Auto -AutoPad 8 -Within 400, 60, 1040, 300
    Send-AI | Out-Null
    $deadline = (Get-Date).AddSeconds(30)
    while ((Get-Date) -lt $deadline) {
        $sp = @(Find-SimText 'is speaking' -Region 712, 180, 728, 720)
        $turns = @(Find-SimText 'Opening' -Region 712, 300, 728, 600)
        if ($sp -and $turns) { break }
        Start-Sleep -Milliseconds 300
    }
    Park
    $bottom = Get-Bottom 720 400 1418 899
    Save-SimShot -Path (Join-Path $script:RawDir 'arena-debate-running.png') -Crop 706, 177, 724, ([Math]::Min(899, $bottom + 22) - 177) | Out-Null
    Write-Host '  shot arena-debate-running' -ForegroundColor DarkGray
    # Stop it, then delete it (the seeded list keeps a stopped and a judged debate).
    $stop = Find-One 'Stop' @(712, 240, 400, 120)
    ClickAt ($stop.x + 15) ($stop.y + 6) 1500
    $del = Find-One 'Delete' @(712, 240, 400, 120)
    ClickAt ($del.x + 20) ($del.y + 6) 1500
    $sv = Show-Saved 'Saved debates'
    Park
    $sv = Find-One 'Saved debates' @(256, 38, 1180, 860)
    # A point in the card's left padding: the right column's card starts at 712, the left one's at 256.
    Shot-Box 'arena-saved' (Get-Box ($(if ($sv.x -gt 700) { 718 } else { 262 })) ($sv.y + 6)) 6
}

Step 'arena lab' {
    Arena-Tab 'Lab'
    # The preset chip in the form (the saved experiment beside it has the same name).
    ClickWord 'trolley' @(256, 38, 440, 860) -WaitMs 1200
    $t = Find-One 'Thought experiment' @(256, 38, 440, 400)
    # Scroll the form up until its agents and Run show.
    for ($i = 0; $i -lt 6 -and -not (@(Find-SimText 'AGENTS TO ASK' -Region 256, 38, 440, 740)); $i++) { Scroll 446 600 -2 }
    $a = Find-One 'AGENTS TO ASK' @(256, 38, 440, 862)
    ClickWord 'Sokrates' @(256, ($a.y + 14), 440, 40) -WaitMs 600
    ClickWord 'Spinoza' @(256, ($a.y + 14), 440, 40) -WaitMs 600
    # Bring the card's top near the page top so the whole form shows. Its title starts at x 285;
    # the page subtitle "...answer a thought experiment" starts at 360, so search only near 256.
    for ($i = 0; $i -lt 4; $i++) {
        $t = @(Find-SimText 'Thought experiment' -Region 256, 38, 60, 862) | Select-Object -First 1
        if ($t -and $t.y -le 130) { break }
        if (-not $t) { Scroll 446 600 1; break }
        Scroll 446 600 -1
    }
    Park
    $t = Find-One 'Thought experiment' @(256, 38, 60, 862)
    Shot-Box 'arena-lab-form' (Get-Box 262 ($t.y + 6)) 6
    # The saved experiment (seeded): answers and the comparison.
    $s = Show-Saved 'Saved experiments'
    $e = @(Find-SimText 'The trolley problem' -Region ($s.x - 30), ($s.y + 20), 780, 300) | Select-Object -Last 1
    if (-not $e) { Dbg 'lab-saved'; throw 'the saved experiment was not found' }
    ClickAt ($e.x + 40) ($e.y + 6) 1600
    Page-Top
    Park
    # The experiment's head card and its Answers card.
    $q = Find-One 'Should you divert' @(712, 38, 728, 300)
    $an = Find-One 'Answers' @(712, 150, 728, 300)
    $hb = Get-Box 720 ($q.y + 4)
    $cmp = @(Find-SimText 'Compare' -Region 712, ($an.y + 100), 300, 800) | Select-Object -First 1
    $bottom = if ($cmp) { $cmp.y - 26 } else { 892 }
    Shot 'arena-lab-results' -Crop 706, ($hb[1] - 6), 724, ($bottom - $hb[1] + 12)
    for ($i = 0; $i -lt 8 -and -not (@(Find-SimText 'What changes answers' -Region 712, 38, 728, 760)); $i++) { Scroll 1040 600 -3 }
    Scroll 1040 600 -3
    Park
    $c = Find-One 'Compare' @(712, 38, 728, 862)
    Shot-Box 'arena-lab-compare' (Get-Box 720 ($c.y + 6)) 6
}

Step 'arena no key' {
    Stop-Relay
    Open-AISettings
    $k = Find-One 'API Key' @(410, 280, 400, 80)
    ClickAt 830 ($k.y + 32) 400
    Key '^a' 150; Key '{BACKSPACE}' 400
    Save-AISettings
    Arena-Tab 'Debate'
    Park
    $n = Find-One 'key is set' @(256, 170, 1180, 80)
    Shot 'arena-no-key' -Crop 252, ($n.y - 20), 1176, 48
}
