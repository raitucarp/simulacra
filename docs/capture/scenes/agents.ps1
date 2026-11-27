# Personas & Agents, the Agent Forge, engines and the autonomous loop (shots/agents.json).
# Client area 1440x900, demo profile (Epistemology & Mind active, AI and embeddings on the stub).
#
# Order: the roster and an agent's tabs; Spinoza writes a post by hand; the scheduler switch; a
# test credential is stored for the key picker. Then the Forge: the start screen, the resume
# prompt, and one Socratic Gadfly draft walked step by step (Embodiment last, as it turns the
# draft into a tin can); then Hypatia is forged from the Empirical Scientist palette, brought to
# life and asked for a post. A failing endpoint shows the failure notice (the stub is put back),
# an empty network ("Empty Lab", left in place) shows the empty roster, and Hypatia and the test
# credential are deleted again so later scenes see the seed roster and keys.

# --- helpers ----------------------------------------------------------------------------------

$script:Opt = 1021, 98, 419, 754          # the Forge options panel (x, y, w, h)
$script:OptBottom = 850                   # the save bar starts below this row
$script:ToastArea = 900, 34, 1440, 320
$script:Topic = 'What would count as evidence against free will?'
$script:AgGeom = Join-Path ([System.IO.Path]::GetTempPath()) 'simulacra-docs-agents-geom.py'
$script:Dbg = Join-Path ([System.IO.Path]::GetTempPath()) 'simulacra-docs-agents-debug'

Set-Content -Path $script:AgGeom -Encoding utf8 -Value @'
"""Geometry read from a capture.
box <png> <px> <py> [tol]: the card around (px, py), a point in the card's empty padding: walks up
  and down that column to the borders, then left and right along a row just under the top border.
bbox <png> <x0> <y0> <x1> <y1> [tol]: the bounding box of everything that differs from the
  region's background (a lone card on an empty page).
Prints "x y w h".
"""
import sys
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
    # Content can stop a walk early: walk several padding rows and keep the farthest reach.
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
    # the borders themselves are one pixel outside
    return left - 1, top - 1, right - left + 3, bottom - top + 3

def bbox(x0, y0, x1, y1, tol=10):
    r = a[y0:y1, x0:x1]
    edge = np.concatenate([r[0], r[-1], r[:, 0], r[:, -1]])
    bg = np.median(edge, axis=0)
    mask = np.abs(r - bg).max(axis=2) > tol
    ys, xs = np.nonzero(mask)
    if not len(xs):
        return x0, y0, x1 - x0, y1 - y0
    return x0 + xs.min(), y0 + ys.min(), xs.max() - xs.min() + 1, ys.max() - ys.min() + 1

mode = sys.argv[1]
n = [int(v) for v in sys.argv[3:]]
print(*{"box": box, "bbox": bbox}[mode](*n))
'@

function Park { Hover 900 20 300 }

function Save-Temp {
    $p = Join-Path ([System.IO.Path]::GetTempPath()) "sim-agents-$PID.png"
    Save-SimShot -Path $p | Out-Null
    $p
}

function Dbg([string]$Name) {
    <# A whole-window capture outside raw/, kept when a step cannot find what it needs. #>
    Save-SimShot -Path (Join-Path $script:Dbg "$Name.png") | Out-Null
}

function Dump([string]$Name) {
    <# Dbg plus the OCR lines (x y w h text) beside it. #>
    Dbg $Name
    Get-SimText | ForEach-Object { "{0,5} {1,5} {2,4} {3,3}  {4}" -f $_.x, $_.y, $_.w, $_.h, $_.text } | Set-Content -Encoding utf8 (Join-Path $script:Dbg "$Name.txt")
}

function ClickWord([string]$Word, [int[]]$Region, [int]$Nth = 0, [int]$WaitMs = 700) {
    <# Clicks $Word inside the OCR line that holds it (OCR merges a row of chips, tabs or buttons
       into one line): its x is estimated from its place in the line's text. #>
    $hits = @(Find-SimText $Word -Region $Region)
    if ($hits.Count -le $Nth) { throw "Word '$Word' not found on screen" }
    $h = $hits[$Nth]
    $i = $h.text.ToLower().IndexOf($Word.ToLower())
    $x = $h.x + [int]($h.w * ($i + $Word.Length / 2) / $h.text.Length)
    ClickAt $x ($h.y + [int]($h.h / 2)) $WaitMs
}

function Find-One([string]$Text, [int[]]$Region) {
    $hit = @(Find-SimText $Text -Region $Region)
    if (-not $hit) { throw "'$Text' not found" }
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

function Get-Box([int]$X, [int]$Y) {
    <# The card around a point in its empty padding, as x,y,w,h. #>
    [int[]]((python $script:AgGeom box (Save-Temp) $X $Y) -split ' ')
}

function Get-BBox([int[]]$Area) {
    [int[]]((python $script:AgGeom bbox (Save-Temp) $Area[0] $Area[1] $Area[2] $Area[3]) -split ' ')
}

function Shot-Box([string]$Id, [int[]]$Box, [int]$Pad = 10) {
    Shot $Id -Crop ([Math]::Max(0, $Box[0] - $Pad)), ([Math]::Max(0, $Box[1] - $Pad)), ($Box[2] + 2 * $Pad), ($Box[3] + 2 * $Pad)
}

function Shot-Card([string]$Id, [string]$Anchor, [int]$Pad = 10) {
    <# The options-panel card whose text holds $Anchor (cards start at x 1037; x 1043 is padding). #>
    Park
    $a = Find-One $Anchor $script:Opt
    Shot-Box $Id (Get-Box 1043 ($a.y + [int]($a.h / 2))) $Pad
}

function Find-Case([string]$Text, [int[]]$Region) {
    <# Case-sensitive Find-SimText: a heading, not Mnemo's sentence that mentions it. #>
    @(Find-SimText $Text -Region $Region | Where-Object { $_.text -clike "*$Text*" })
}

function Wheel([int]$Px) {
    <# Scrolls the options panel by about $Px pixels (positive = down). A wheel delta of 120 moves
       it 260 px; the pointer stays at x 1029, off the sideways-scrolling palette strips. #>
    Move-SimMouse 1029 500 -WaitMs 60
    Send-SimWheel (- [int]($Px / 2.17))
    Start-Sleep -Milliseconds 450
}

function Show-Section([string]$Heading, [int]$Top = 128) {
    <# Scrolls the options panel from its top until $Heading sits at about $Top (or the panel can
       go no further); returns the heading's OCR line. #>
    Scroll 1029 500 30
    $prev = $null; $stuck = 0
    for ($i = 0; $i -lt 16; $i++) {
        $h = @(Find-Case $Heading $script:Opt)
        if ($h) {
            $y = $h[0].y
            Write-Host "    section '$Heading' at y $y" -ForegroundColor DarkGray
            if ($y -ge $Top - 12 -and $y -le $Top + 50) { return $h[0] }
            # Twice unmoved: the end of the panel (once can be a panel still growing).
            if ($null -ne $prev -and [Math]::Abs($prev - $y) -lt 4) { if (++$stuck -ge 2) { return $h[0] } } else { $stuck = 0 }
            $prev = $y
            Wheel ([Math]::Max(-700, [Math]::Min(700, $y - $Top)))
        } elseif ($null -ne $prev) {
            Wheel -120   # it went past the top: come back a little
            $prev = $null
        } else {
            Wheel 500
        }
    }
    throw "section '$Heading' not found"
}

function Shot-Section([string]$Id, [string]$Heading, [int]$Above = 26, [int]$Bottom = $script:OptBottom) {
    <# The options panel from the card that holds $Heading ($Above px over the heading's text)
       down to the save bar. #>
    $h = Show-Section $Heading ([Math]::Max(128, 102 + $Above))
    Park
    $y0 = [Math]::Max(99, $h.y - $Above)
    Shot $Id -Crop 1021, $y0, 419, ($Bottom - $y0)
}

function Go-Step([int]$N) {
    <# A step of the Forge rail (14 rows, 51 px apart). #>
    ClickAt 360 ([int](141 + ($N - 1) * 50.9)) 1300
}

function Test-ForgeSteps { [bool](@(Find-SimText 'OF 14' -Region 340, 50, 200, 30)) }

function Open-NewAgent {
    <# Personas & Agents › New agent with the pointer away from the gallery (a hovered card
       becomes the preview); answers a resume prompt with Start fresh. #>
    Key '{ESC}' 200
    $lines = @(Get-SimText)
    if ($lines | Where-Object { $_.text -like '*Search settings*' }) { ClickAt 1226 112 800 }
    if ($lines | Where-Object { $_.text -like '*OF 14*' -and $_.x -ge 340 -and $_.x -le 540 -and $_.y -le 80 }) { ClickAt 271 67 1200 }
    elseif ($lines | Where-Object { $_.text -like '*Choose a palette*' }) { ClickAt 1408 82 1000 }
    Key '^3' 1000
    Park
    Palette 'New agent' 1600
    Park
    if (@(Find-SimText 'Resume your draft')) {
        Click 'Start fresh' -WaitMs 1500
        Park
    }
    if (-not @(Find-SimText 'Choose a palette')) { throw 'the start screen did not open' }
}

function New-Draft([string]$Palette) {
    <# A fresh draft from a palette of the All shelf, open on its Origin step. #>
    Open-NewAgent
    ClickAt 305 162 800
    Click $Palette -Region 241, 230, 880, 670 -WaitMs 900
    Click "Begin as $Palette" -Region 1121, 128, 319, 772 -WaitMs 1800
    Park
    if (-not (Test-ForgeSteps)) { throw "the $Palette draft did not open" }
}

function Ensure-Draft([string]$Palette = 'Socratic Gadfly') {
    if (-not (Test-ForgeSteps)) { New-Draft $Palette }
}

$script:Card = 622, 100, 680, 500        # the Profile tab card, centred in the detail pane (x, y, w, h)

function Click-NameField {
    <# The Name field of the Origin step: just under its label in the options panel. #>
    $n = @(Find-SimText 'Name' -Region $script:Opt | Where-Object { $_.text -cmatch '^Name' -and $_.x -le 1070 })
    if (-not $n) { Dbg 'name-field'; throw 'the Name label was not found' }
    ClickAt 1230 ($n[0].y + 40) 400
}

function Get-CardBox {
    <# The Profile tab card, from a point in its left padding beside the biography. #>
    $b = Find-One 'Edit in Forge' $script:Card
    Get-Box ($script:Card[0] + 8) ($b.y - 30)
}

function Profile-Tab([string]$Name, [int]$WaitMs = 1000) {
    <# Clicks a profile tab (Profile, Posts, Beliefs, Recommended): at its OCR'd label when OCR
       reads it (it often misses the selected one), else at its place in the 1440 px layout. #>
    $at = @{ Profile = 755; Posts = 893; Beliefs = 1033; Recommended = 1170 }
    $hit = @(Find-SimText $Name -Region 600, 50, 760, 40) | Select-Object -First 1
    $x = if ($hit) { [int]($hit.x + $hit.w / 2) } else { $at[$Name] }
    ClickAt $x 71 $WaitMs
}

function Open-Agent([string]$Name) {
    Key '{ESC}' 200
    if (Test-ForgeSteps) { ClickAt 271 67 1200 }
    Key '^3' 1100
    Click $Name -Region 241, 110, 243, 400 -WaitMs 900
    Profile-Tab 'Profile' 800
}

function Click-Write {
    <# The Write button of the write dialog (Cancel and Write may OCR as one line). #>
    $c = @(Find-SimText 'Cancel' -Region 480, 250, 500, 250)
    if (-not $c) { throw 'the write dialog buttons were not found' }
    if ($c[0].text -match 'Write') { ClickWord 'Write' -Region 480, ($c[0].y - 4), 500, 24 -WaitMs 900 }
    else { ClickAt ($c[0].x + $c[0].w + 50) ($c[0].y + 6) 900 }
}

function Confirm-Spend {
    <# Answers "Send this to the AI?" with Send (Cancel and Send may OCR as one line). #>
    $d = @(Find-SimText 'Send this to the AI')
    if (-not $d) { return }
    $c = Find-One 'Cancel' @($d[0].x, ($d[0].y + 30), 460, 110)
    if ($c.text -match 'Send') { ClickWord 'Send' -Region $c.x, ($c.y - 2), ($c.w + 4), 20 -WaitMs 1200 }
    else { ClickAt ($c.x + $c.w + 42) ($c.y + 6) 1200 }
}

function Open-Settings([string]$Pane) {
    Key '{ESC}' 200
    Key '^,' 1200
    Click $Pane -Region 100, 80, 250, 700 -WaitMs 1000
}

function Close-Settings {
    <# Esc can stop at a focused field: the dialog's X closes it for sure. #>
    Key '{ESC}' 500
    if (@(Find-SimText 'Search settings')) { ClickAt 1226 112 800 }
    if (@(Find-SimText 'Search settings')) { throw 'Settings would not close' }
}

function Set-Endpoint([string]$Url) {
    <# Settings › AI & Models: the Custom Endpoint URL, then Save & Apply Settings. #>
    Open-Settings 'Models'
    $f = @(Find-SimText 'chat/completions' -Region 400, 380, 860, 80)
    if (-not $f) { throw 'the endpoint URL field was not found' }
    ClickAt ($f[0].x + $f[0].w + 30) ($f[0].y + 6) 300
    Key '^a' 150
    TypeText $Url 400
    Click 'Save & Apply' -Region 900, 760, 400, 80 -At 1153, 803 -WaitMs 1200
    Close-Settings
}

# --- the roster -------------------------------------------------------------------------------

Step 'welcome' -Always {
    New-Item -ItemType Directory -Force -Path $script:Dbg | Out-Null
    Skip-Welcome
}

Step 'lab' {
    Key '{ESC}' 200
    Key '^3' 1200
    Park
    Shot 'lab-mode-switch' -Crop 241, 42, 243, 66
    $row = Find-One 'Baruch de Spinoza' @(241, 110, 243, 400)
    Shot 'lab-contact-row' -Crop 244, ($row.y - 14), 236, 58
    ClickAt 360 ($row.y + 10) 900
    Profile-Tab 'Profile' 800
    Park
    Shot 'lab-overview'
    Shot-Box 'lab-profile-card' (Get-CardBox) 10
    Profile-Tab 'Posts' 1200
    Park
    Shot 'lab-posts-tab' -Crop 590, 44, 740, 856
    Profile-Tab 'Beliefs' 1200
    Park
    $f = Find-One 'Beliefs form' @(484, 100, 956, 600)
    Shot 'lab-beliefs-tab' -Crop 606, 44, 718, ($f.y + 40 - 44)
    Profile-Tab 'Recommended' 1500
    Wait-Text '%' -Region 1200, 140, 240, 200 -Seconds 30 | Out-Null
    Park
    Shot 'lab-recommended-tab' -Crop 600, 44, 720, 856
    Profile-Tab 'Profile' 800
}

Step 'persona card' {
    Key '{ESC}' 200
    Key '^4' 1200
    Click 'Arya Wiratama' -Region 241, 110, 243, 200 -WaitMs 900
    Profile-Tab 'Profile' 800
    Park
    Find-One 'Switch to Persona' $script:Card | Out-Null
    Shot-Box 'lab-persona-card' (Get-CardBox) 10
    Key '^3' 900
}

Step 'write' {
    Open-Agent 'Baruch de Spinoza'
    Park
    Mark
    ClickWord 'Write a post' -Region $script:Card -WaitMs 1200
    $t = Find-One 'What should it be' @(480, 150, 500, 80)
    ClickAt ($t.x + 60) ($t.y + 6) 300
    TypeText $script:Topic 400
    ClickWord 'Ethics' -Region 480, 220, 500, 50 -WaitMs 600
    Park
    Shot 'agent-trigger-dialog' -Auto -AutoPad 1
    Click-Write
    Wait-Text 'Posted' -Region $script:Card -Seconds 40 | Out-Null
    Park
    $p = Find-One 'Posted' $script:Card
    $b = Get-CardBox
    Shot 'lab-run-status' -Crop ($b[0] + 7), ($p.y - 64), ($b[2] - 14), 94
}

Step 'scheduler' {
    Open-Settings 'Agents & Simulation'
    # The pane's subtitle also starts with "Autonomous posting": take the row below AUTONOMY.
    $a = Find-One 'Autonomous posting' @(420, 195, 400, 50)
    ClickAt 1211 ($a.y + 8) 1500
    Park
    $g = Find-One 'AUTONOMY' @(400, 120, 300, 100)
    $c = Find-One 'Concurrent gen' @(400, 200, 400, 400)
    Shot 'agent-scheduler-status' -Crop ($g.x - 8), ($g.y - 12), 840, ($c.y + 62 - $g.y)
    ClickAt 1211 ($a.y + 8) 900
    if (-not @(Find-SimText 'Scheduler Off' -Region 420, 250, 300, 60)) { throw 'autonomous posting did not switch off again' }
    Close-Settings
}

Step 'credential' {
    # A stored key for the Forge key picker (a test value, not a real key).
    Open-Settings 'Models'
    Scroll 760 500 -10
    Click 'Add credential' -WaitMs 1000
    ClickWord 'OpenA' -Region 420, 380, 820, 60 -WaitMs 600
    $n = Find-One 'Personal Open' @(420, 420, 820, 100)
    ClickAt ($n.x + 60) ($n.y + 6) 300
    TypeText 'Personal OpenAI key' 300
    $k = Find-One 'API key' @(420, 480, 300, 60)
    ClickAt ($k.x + 200) ($k.y + 38) 300
    TypeText 'test-key-0000' 300
    Click 'credential' -Region 1080, 620, 180, 50 -At 1175, 650 -WaitMs 1000
    Click 'Save & Apply' -Region 900, 760, 400, 80 -At 1153, 803 -WaitMs 1200
    Close-Settings
}

# --- the Forge: start screen ------------------------------------------------------------------

Step 'forge start' {
    Open-NewAgent
    Shot 'forge-start'
    ClickAt 410 162 900   # Thinkers
    Click 'Empirical Scientist' -Region 241, 230, 880, 670 -WaitMs 900
    Park
    Shot 'forge-start-preview' -Crop 1121, 128, 319, 772
    ClickAt 305 162 900   # All
    Scroll 700 600 -20
    Park
    $o = Find-One 'Other ways to begin' @(241, 300, 880, 600)
    Shot 'forge-start-tiles' -Crop 244, ($o.y - 16), 872, (900 - $o.y + 16 - 4)
    Click 'Roll a random thinker' -WaitMs 1500
    Park
    $k = Find-One 'Keep this one' @(1121, 128, 319, 772)
    Shot 'forge-roll' -Crop 1121, 128, 319, ($k.y + 40 - 128)
    Click 'Describe them in words' -Region 241, 600, 880, 300 -WaitMs 1200
    Park
    $f = Find-One 'Fill in the sheet' @(1121, 128, 319, 772)
    Shot 'forge-describe' -Crop 1121, 128, 319, ($f.y + 34 - 128)
    ClickAt 1408 82 1200
}

Step 'forge resume' {
    Open-NewAgent
    Scroll 700 600 -20
    Click 'Start from scratch' -WaitMs 1800
    Park
    Click-NameField
    TypeText 'Hypatia' 600
    ClickAt 271 67 1200
    Key '^3' 800
    Park
    Palette 'New agent' 1600
    Park
    Wait-Text 'Resume your draft' -Seconds 5 | Out-Null
    Shot-Box 'forge-resume' (Get-BBox @(250, 45, 1430, 895)) 14
    Click 'Start fresh' -WaitMs 1200
}

# --- the Forge: one draft, step by step -------------------------------------------------------

Step 'forge origin' {
    New-Draft 'Socratic Gadfly'
    Shot 'forge-steps-overview'
    Shot 'forge-topbar' -Crop 241, 38, 1199, 60
    Shot 'forge-save-bar' -Crop 241, 851, 1199, 49
    Shot-Card 'forge-suggest-bar' 'Not sure what to pick'
    # Hover cards: cut to the card itself.
    Hover 1257 67 1500
    $k = Find-One 'Keyboard' @(1060, 70, 380, 60)
    # The card's top border runs over the Next button: start one row under it.
    $b = Get-Box ($k.x - 10) ($k.y + 6)
    Shot 'forge-keyboard-hint' -Crop $b[0], ($b[1] + 1), $b[2], ($b[3] - 1)
    Park
    # The explainer card: from a point in its padding left of WHAT IT IS (its background is too
    # close to the stage under it for a Mark/-Auto diff to find its edges).
    $h = Find-One 'Handle' $script:Opt
    Hover ($h.x + $h.w - 6) ($h.y + 6) 1500
    $w = @(Find-SimText 'WHAT IT IS' -Region 600, 100, 840, 790)
    if (-not $w) { Hover ($h.x + $h.w - 5) ($h.y + 7) 2000; $w = @(Find-SimText 'WHAT IT IS' -Region 600, 100, 840, 790) }
    if (-not $w) { Dbg 'explainer'; throw 'the explainer card did not open' }
    # The box reaches a row or two past the bottom border (the text it covers): trim those.
    $b = Get-Box ($w[0].x - 8) ($w[0].y + 6)
    Shot 'forge-explainer' -Crop $b[0], $b[1], $b[2], ($b[3] - 2)
    Park
    Mark
    Click 'Suggest with' -Region $script:Opt -WaitMs 1200
    Park
    Shot 'agent-spend-confirm' -Auto -AutoPad 3 -Within 470, 60, 990, 420
    Key '{ESC}' 800
    ClickAt 1388 67 900
    ClickAt 1388 67 900
    ClickAt 1388 67 1200
    Park
    Shot 'forge-rail' -Crop 241, 98, 212, 740
    Shot-Section 'forge-birthsign' 'Personality type'
}

Step 'forge appearance' {
    Ensure-Draft
    Go-Step 3
    ClickWord 'Portrait' -Region $script:Opt -WaitMs 900
    ClickWord 'Hairstyle' -Region $script:Opt -WaitMs 900
    Shot-Section 'forge-appearance' 'Avatar style'
    ClickWord 'Emblem' -Region $script:Opt -WaitMs 1000
    Shot-Card 'forge-appearance-emblem' 'The emblem is drawn'
    ClickWord 'Portrait' -Region $script:Opt -WaitMs 900
    Click 'Image ideas' -Region $script:Opt -WaitMs 1500
    Click 'Surprise me' -WaitMs 1500
    Park
    Shot 'forge-image-ideas'
    Key '{ESC}' 900
}

Step 'forge traits' {
    Ensure-Draft
    Go-Step 5
    Shot-Section 'forge-attributes' 'Share a fixed pool' -Above 62
    Go-Step 6
    Shot-Section 'forge-disposition' 'How they bond and act'
    Go-Step 7
    Shot-Section 'forge-mind' 'Intelligences'
}

Step 'forge voice' {
    Ensure-Draft
    Go-Step 8
    # The strip is folded to its title line: unfold it (it stays open for the rest of the session).
    $s = Show-Section 'Palettes for this step'
    if (-not @(Find-SimText 'Plain Speaker' -Region $script:Opt)) {
        ClickAt ($s.x + 60) ($s.y + 6) 1200
        $s = Show-Section 'Palettes for this step'
    }
    $p = Find-One 'Plain Speaker' $script:Opt
    # Hovering the card lists the changes it would make under the strip (Word choice, ...).
    foreach ($dy in -40, -30, -50) {
        Park
        Hover ($p.x + 20) ($p.y + $dy) 1400
        if (@(Find-SimText 'Word choice' -Region $script:Opt)) { break }
    }
    if (-not @(Find-SimText 'Word choice' -Region $script:Opt)) { Dbg 'palette-hover'; throw 'the Plain Speaker preview did not show' }
    $b = Get-Box 1043 ($s.y + 6)
    Shot-Box 'forge-palette-strip' $b 10
    Park
    Shot-Section 'forge-voice' 'How they sound'
    Go-Step 9
    Shot-Section 'forge-rhetoric' 'How they persuade'
    $f = Show-Section 'Post format' 300
    Start-Sleep -Milliseconds 500
    Hover 1368 ($f.y + 6) 1800
    $w = @(Find-SimText 'starting picks' -Region 1080, 100, 360, 790)
    if (-not $w) { Park; Hover 1370 ($f.y + 7) 2000; $w = @(Find-SimText 'starting picks' -Region 1080, 100, 360, 790) }
    if (-not $w) { Dbg 'why-card'; throw 'the Why? card did not open' }
    $w = $w[0]
    # The box's left column is one pixel outside the card's edge: skip it.
    $b = Get-Box ($w.x - 10) ($w.y + 6)
    Shot 'forge-why-card' -Crop ($b[0] + 1), $b[1], ($b[2] - 1), $b[3]
    Park
}

Step 'forge life' {
    Ensure-Draft
    Go-Step 10
    Shot-Section 'forge-daily-life' 'Body clock'
    Show-Section 'Their week' | Out-Null
    Shot-Card 'forge-week-strip' 'Their week'
    Go-Step 11
    Shot-Section 'forge-relationships' 'Rapport'
    Go-Step 12
    $o = @(Find-Case 'Open the grimoire' $script:Opt)
    if (-not $o) { throw 'Open the grimoire was not found' }
    ClickAt ($o[0].x + 40) ($o[0].y + 6) 1500
    Park
    Shot-Section 'forge-grimoire' 'Close the grimoire' -Above 104
}

function Get-TabRow {
    <# The y of the Engine step's tab row: three or more of its labels on one line (OCR often
       misses the selected tab's label, and "Request" is also a section heading). #>
    $labels = @(Get-SimText | Where-Object { $_.x -ge 1021 -and $_.y -ge 100 -and $_.y -le 840 -and $_.text -cmatch '^(Model|Request|Answer|Image|Tools)$' })
    foreach ($l in $labels) {
        if (@($labels | Where-Object { [Math]::Abs($_.y - $l.y) -le 5 }).Count -ge 3) { return $l.y }
    }
    $null
}

function Engine-Tab([string]$Name) {
    <# One of the Engine step's five section tabs (equal widths across the options panel). #>
    $x = @{ Model = 1074; Request = 1152; Answer = 1230; Image = 1308; Tools = 1386 }[$Name]
    Scroll 1029 500 30
    $y = Get-TabRow
    if ($null -eq $y) { Wheel 300; $y = Get-TabRow }
    if ($null -eq $y) { Dbg 'engine-tabs'; throw 'the Engine tab row was not found' }
    ClickAt $x ($y + 4) 1200
    $y
}

function Click-Option([string]$Label, [int]$WaitMs = 1100) {
    <# Scrolls the options panel to a card or chip label (case-sensitive) and clicks it. #>
    $h = Show-Section $Label 480
    $i = $h.text.IndexOf($Label)
    if ($i -lt 0) { $i = 0 }
    $x = $h.x + [int]($h.w * ($i + $Label.Length / 2) / [Math]::Max(1, $h.text.Length))
    ClickAt $x ($h.y + [int]($h.h / 2)) $WaitMs
}

Step 'engine' {
    Ensure-Draft
    Go-Step 13
    Scroll 1029 500 30
    Park
    $y = Get-TabRow
    if ($null -eq $y) { throw 'the Engine tab row was not found' }
    Shot 'engine-tabs' -Crop 1029, ($y - 36), 403, 62
    Engine-Tab 'Request' | Out-Null
    Show-Section 'Test call' | Out-Null
    Click 'Send a test call' -Region $script:Opt -WaitMs 1500
    Confirm-Spend
    Start-Sleep -Seconds 3
    Show-Section 'Test call' | Out-Null
    Wait-Text 'model answered' -Region $script:Opt -Seconds 30 | Out-Null
    Shot-Card 'engine-test-call' 'Test call'
    Show-Section 'Request mode' | Out-Null
    Click 'Custom' -Region $script:Opt -WaitMs 1200
    Shot-Section 'engine-request-custom' 'Request builder'
}

Step 'engine options' {
    Ensure-Draft
    Go-Step 13
    # With a custom request the Model tab has no Mind source cards: back to the native API first.
    Engine-Tab 'Request' | Out-Null
    Click-Option 'Native'
    Engine-Tab 'Model' | Out-Null
    Click-Option 'Own model'
    Click-Option 'OpenA'
    Shot-Section 'engine-model-own' 'Mind source'
    # The Key field sits at the end of the Own model card, at the end of the panel.
    Show-Section 'Manage keys' 600 | Out-Null
    Park
    $k = @(Find-Case 'Key' @(1040, 400, 90, 440))
    if (-not $k) { Dbg 'engine-key'; throw 'the Key field was not found' }
    $b = Get-Box 1043 ($k[0].y + 6)
    $y0 = $k[0].y - 14
    Shot 'engine-credential-picker' -Crop 1038, $y0, 386, ([Math]::Min($b[1] + $b[3] + 6, 851) - $y0)
    Engine-Tab 'Answer' | Out-Null
    Click-Option 'Structured'
    Click-Option 'Illustrated'
    Shot-Section 'engine-output-json' 'Schema preset'
    Engine-Tab 'Image' | Out-Null
    Click-Option 'Local Stable'
    Click-Option 'Comfy'
    Shot-Section 'engine-image' 'Local Stable' -Above 120
    Engine-Tab 'Tools' | Out-Null
    Click-Option 'Vault search'
    Click-Option 'Web search'
    Shot-Section 'engine-tools' 'What the agent may look up' -Above 56
}

Step 'embodiment' {
    Ensure-Draft
    Go-Step 2
    Show-Section 'What are they' | Out-Null
    $o = @(Find-Case 'Object' $script:Opt)
    if (-not $o) { throw 'the Object card was not found' }
    ClickAt ($o[0].x + 10) ($o[0].y + 6) 1200
    Show-Section 'Which form' | Out-Null
    Click 'Tin can' -Region $script:Opt -WaitMs 1200
    Shot-Section 'forge-embodiment' 'Which form'
}

# --- Hypatia: forged, brought to life and asked for a post -------------------------------------

Step 'hypatia' {
    New-Draft 'Empirical Scientist'
    Click-NameField
    TypeText 'Hypatia' 800
    $s = @(Find-SimText 'Suggest' -Region 1300, 500, 140, 250 | Where-Object { $_.text -notlike '*with*' })
    if (-not $s) { throw 'the Suggest button beside Handle was not found' }
    ClickAt ($s[0].x + 20) ($s[0].y + 6) 1200
    Shot-Section 'forge-origin' 'Who they are'
    Go-Step 14
    # The panel scrolled to its end: the lower trading card, Things to check and Bring to life.
    Show-Section 'Bring to life' 600 | Out-Null
    Park
    # From just above the palette badge, so the handle cut by the panel's top edge stays out.
    $y0 = 99
    $p = @(Find-SimText 'Empirical Scientist' -Region 1021, 99, 419, 200)
    if ($p) { $y0 = [Math]::Max(99, $p[0].y - 16) }
    Shot 'forge-review' -Crop 1021, $y0, 419, ($script:OptBottom - $y0)
    Click 'Bring to life' -Region $script:Opt -WaitMs 1500
    Confirm-Spend
    Wait-Text 'has joined' -Seconds 30 | Out-Null
    Park
    Shot-Box 'forge-celebrate' (Get-BBox @(250, 45, 1430, 895)) 14
    Click 'Back to the roster' -WaitMs 1500
}

Step 'first reply' {
    Open-Agent 'Hypatia'
    ClickWord 'Write a post' -Region $script:Card -WaitMs 1200
    $t = Find-One 'What should it be' @(480, 150, 500, 80)
    ClickAt ($t.x + 60) ($t.y + 6) 300
    TypeText $script:Topic 400
    Click-Write
    Wait-Text 'Posted' -Region $script:Card -Seconds 40 | Out-Null
    ClickWord 'Open the post' -Region $script:Card -WaitMs 1800
    Park
    Shot 'agent-first-reply' -Crop 241, 38, 519, 300
}

# --- the failure notice, the empty roster, and putting things back ---------------------------

Step 'failure' {
    Set-Endpoint 'http://127.0.0.1:9/v1/chat/completions'
    try {
        Open-Agent 'Baruch de Spinoza'
        ClickWord 'Write a post' -Region $script:Card -WaitMs 1200
        Click-Write
        Wait-Text 'answer' -Region $script:ToastArea -Seconds 40 | Out-Null
        Park
        $with = Join-Path ([System.IO.Path]::GetTempPath()) "sim-agents-with-$PID.png"
        Save-SimShot -Path $with | Out-Null
        $gone = $true
        try { Wait-Text 'answer' -Region $script:ToastArea -Seconds 25 -Gone } catch { $gone = $false }
        if ($gone) {
            $without = Save-Temp
            $out = Join-Path $script:RawDir 'agent-failure-notice.png'
            $box = python $script:AutoCrop $without $with $out 3 $script:ToastArea[0] $script:ToastArea[1] $script:ToastArea[2] $script:ToastArea[3]
            Write-Host "  shot agent-failure-notice (toast $box)" -ForegroundColor DarkGray
        } else {
            $l = @(Get-SimText | Where-Object { $_.x -ge 900 -and $_.y -lt 320 -and $_.y -gt 34 })
            $x0 = ($l | Measure-Object x -Minimum).Minimum; $y0 = ($l | Measure-Object y -Minimum).Minimum
            $x1 = ($l | ForEach-Object { $_.x + $_.w } | Measure-Object -Maximum).Maximum
            $y1 = ($l | ForEach-Object { $_.y + $_.h } | Measure-Object -Maximum).Maximum
            Shot 'agent-failure-notice' -Crop ($x0 - 20), ($y0 - 16), ($x1 - $x0 + 40), ($y1 - $y0 + 32)
        }
    } finally {
        Set-Endpoint 'http://127.0.0.1:8765/v1/chat/completions'
    }
}

Step 'empty roster' {
    Key '{ESC}' 200
    Palette 'create network' 900
    TypeText 'Empty Lab' 400
    $c = Find-One 'Cancel' @(300, 80, 900, 400)
    if ($c.text -match 'Create') { ClickWord 'Create' -Region $c.x, ($c.y - 2), ($c.w + 4), 20 -WaitMs 1200 }
    else { ClickAt ($c.x + $c.w + 40) ($c.y + 6) 1200 }
    Key '^3' 1200
    if (-not @(Find-SimText 'EMPTY LAB' -Region 241, 80, 243, 30)) { throw 'the Empty Lab network was not created' }
    Park
    Wait-Text 'created' -Region 900, 34, 540, 120 -Seconds 15 -Gone
    $b = Get-BBox @(490, 120, 1430, 890)
    Shot-Box 'lab-empty-roster' $b 48
    ClickAt 96 64 700
    Click 'Epistemology' -Region 0, 70, 360, 300 -WaitMs 900
}

Step 'cleanup' {
    # Hypatia leaves again (the trash icon right of Write a post, then Remove in its confirm), so
    # later scenes find the seed roster.
    Open-Agent 'Hypatia'
    Park
    $w = Find-One 'Write a post' $script:Card
    ClickAt ($w.x + $w.w + 35) ($w.y + 6) 1000
    $d = Wait-Text 'Remove Hypatia' -Seconds 5
    $r = @(Find-SimText 'Remove' -Region ($d.x + 200), ($d.y + 60), 400, 60)
    if (-not $r) { Dbg 'remove-confirm'; throw 'the Remove button was not found' }
    $r = $r[-1]
    $i = $r.text.LastIndexOf('Remove')
    ClickAt ($r.x + [int]($r.w * ($i + 3) / $r.text.Length)) ($r.y + 6) 1200
    if (@(Find-SimText 'Hypatia' -Region 241, 110, 243, 400)) { throw 'Hypatia is still in the roster' }
    Key '^1' 600
}

Step 'cleanup key' {
    # The test credential goes too (its row's trash icon), so Settings shows the seed keys again.
    Open-Settings 'Models'
    Scroll 760 500 -10
    $c = @(Find-SimText 'Personal Open' -Region 420, 200, 820, 560)
    if (-not $c) {
        Dump 'credential-list'
        throw 'the test credential was not found'
    }
    ClickAt 1219 ($c[0].y + 10) 1000
    if (@(Find-SimText 'Personal Open' -Region 420, 200, 820, 560)) { Dbg 'credential-remove'; throw 'the test credential was not removed' }
    Click 'Save & Apply' -Region 900, 760, 400, 80 -At 1153, 803 -WaitMs 1200
    Close-Settings
    Key '^1' 600
}
