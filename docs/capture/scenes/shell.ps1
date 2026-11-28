# Shell, getting started, palette and networks (shots/shell.json). Client area 1440x900.
# Run with -SplashShot start-splash to also catch the splash window (run.ps1 takes it before the scene).
# At 1440 px the right sidebar starts closed on every place: the steps that need it open it with
# Ctrl+Alt+B (the choice is remembered per place) and close it again.

function Set-ClientSize([int]$W, [int]$H, [int]$Y = 30) {
    Set-SimulacraSize -Width ($W + 16) -Height ($H + 8) -X 40 -Y $Y
    $script:W = $W; $script:H = $H
    Start-Sleep -Milliseconds 900
}
# The right sidebar column when it is open at 1440 px.
$script:Right = 1121, 38, 319, 862

function Close-Settings {
    # Neither Escape nor Ctrl+, closes the Settings dialog: use its Close button.
    Click 'Close' -Region 1100, 770, 200, 60 -At 1215, 803 -WaitMs 700
}

Step 'welcome' -Always {
    Shot 'start-welcome'
    Skip-Welcome
}

Step 'overview' {
    Key '^1' 700
    # Open Baruch de Spinoza's thread: it goes to the Thread Inspector and to Recent.
    Click 'Precisely, and the remedy' -Region 240, 80, 520, 820
    Hover 700 880 300
    Shot 'shell-window-overview'
    Shot 'shell-title-bar' -Crop 0, 0, 1440, 38
    Shot 'shell-title-bar-actions' -Crop 1140, 0, 300, 38
    Shot 'shell-sidebar-expanded' -Crop 0, 38, 241, 862
    Shot 'shell-sidebar-places' -Crop 0, 140, 241, 320
    Shot 'shell-sidebar-footer' -Crop 0, 808, 241, 92
    Shot 'shell-user-card' -Crop 0, 846, 241, 54
    Shot 'network-switcher' -Crop 0, 44, 241, 40
}

Step 'appearance tooltip' {
    Mark
    Hover 1268 18 1500
    Shot 'shell-appearance-tooltip' -Auto -AutoPad 0
    $b = $script:LastBox
    # The Thread Inspector's View button starts at x 1381, right of the tooltip.
    $x0 = $b[0] - 8; $x1 = [Math]::Min($b[0] + $b[2] + 1, 1380)
    Shot 'shell-appearance-tooltip' -Crop $x0, 0, ($x1 - $x0), ($b[1] + $b[3] + 8) -SettleMs 50
    Hover 700 880 300
}

Step 'new menu' {
    Mark
    Click 'New' -Region 0, 90, 241, 45 -At 100, 112
    Shot 'shell-new-menu' -Auto -AutoPad 10
    Key '{ESC}'
}

Step 'right mind cadence' {
    Key '^1' 600
    Key '^%b' 900
    Hover 700 880 300
    Shot 'shell-right-mind-cadence' -Crop $script:Right
    $mg = @(Find-SimText 'MIND GRAPH' -Region 1121, 80, 319, 100)
    $tc = @(Find-SimText 'CADENCE' -Region 1121, 200, 319, 300)
    $sp = @(Find-SimText 'SPARKS' -Region 1121, 400, 319, 400)
    if ($mg -and $tc -and $sp) {
        $y0 = $mg[0].y - 14; $y1 = $tc[0].y - 18; $y2 = $sp[0].y - 16
    } else {
        $y0 = 94; $y1 = 342; $y2 = 568
    }
    Shot 'shell-mini-graph' -Crop 1121, $y0, 319, ($y1 - $y0)
    Shot 'shell-cadence-heatmap' -Crop 1121, $y1, 319, ($y2 - $y1)
    Shot 'shell-sparks' -Crop 1121, $y2, 319, (900 - $y2)
}

Step 'right vault telemetry' {
    Key '^1' 600
    if (-not @(Find-SimText 'Telemetry' -Region 1121, 40, 319, 60)) { Key '^%b' 900 }
    Click 'Telemetry' -Region 1121, 40, 319, 60 -At 1356, 68
    Hover 700 880 300
    Shot 'shell-right-vault-telemetry' -Crop $script:Right
    $k = @(Find-SimText 'GROUNDING' -Region 1121, 100, 319, 200)
    $t = @(Find-SimText 'TRENDING' -Region 1121, 200, 319, 300)
    $r = @(Find-SimText 'TELEMETRY' -Region 1121, 300, 319, 400)
    $last = @(Find-SimText 'Simone Weil' -Region 1121, 500, 319, 300)
    if ($k -and $t -and $r) {
        $y0 = $k[0].y - 14; $y1 = $t[0].y - 14; $y2 = $r[0].y - 14
    } else {
        $y0 = 146; $y1 = 282; $y2 = 490
    }
    $y3 = if ($last) { $last[-1].y + 24 } else { $y2 + 192 }
    Shot 'shell-knowledge-card' -Crop 1121, $y0, 319, ($y1 - $y0)
    Shot 'shell-trending-topics' -Crop 1121, $y1, 319, ($y2 - $y1 - 2)
    Shot 'shell-telemetry-roster' -Crop 1121, $y2, 319, ($y3 - $y2)
    Click 'Cadence' -Region 1121, 40, 319, 60 -At 1206, 68
    Key '^%b' 900
}

Step 'recent and pins' {
    Key '^1' 600
    # Seven recent items: three threads, two sources, two agents.
    Click 'This morning' -Region 240, 80, 520, 820
    Click 'Proposition of the day' -Region 240, 80, 520, 820
    Click 'Precisely, and the remedy' -Region 240, 80, 520, 820
    Click 'Tractatus' -Region 0, 460, 241, 340
    Key '^1' 700
    $e = @(Find-SimText 'Ethics' -Region 0, 460, 241, 340)
    if (-not $e) { throw 'Ethics not found under Sources' }
    ClickAt ($e[-1].x + 20) ($e[-1].y + 5) 700
    Key '^3' 900
    Click 'Sokrates' -Region 241, 80, 300, 400
    Click 'Simone Weil' -Region 241, 80, 300, 400
    Key '^1' 700
    # The sections run past the footer: scroll the sidebar to reach Members, then pin Spinoza.
    Scroll 120 600 -10
    Hover 700 880 300
    $m = @(Find-SimText 'Members' -Region 0, 140, 241, 670)
    if (-not $m) { throw 'Members header not found' }
    $sp = @(Find-SimText 'Spinoza' -Region 0, ($m[-1].y + 10), 241, 200)
    if (-not $sp) { throw 'Baruch de Spinoza not found under Members' }
    $rowY = $sp[0].y + 6
    Hover 120 $rowY 500
    ClickAt 218 $rowY 900
    # Only the sections scroll (about 350 px at 900 px high): a 1064 px tall window shows all four.
    Set-ClientSize 1440 1064 4
    Scroll 120 600 10
    Hover 700 1040 300
    $p = @(Find-SimText 'Pinned' -Region 0, 140, 241, 900)
    $sw = @(Find-SimText 'Simone Weil' -Region 0, 140, 241, 900 | Sort-Object y)
    if (-not ($p -and $sw)) { Set-ClientSize 1440 900; throw 'Pinned or the last Members row not found' }
    $top = $p[0].y - 12; $bottom = $sw[-1].y + $sw[-1].h + 12
    Shot 'shell-sidebar-sections' -Crop 0, $top, 241, ($bottom - $top)
    $recent = @(Find-SimText 'Recent' -Region 0, 140, 241, 900)
    if ($recent) {
        $rowY = $recent[0].y + 31
        Hover 120 $rowY 600
        Hover 218 $rowY 1500
        Shot 'shell-sidebar-pin-toggle' -Crop 0, ($rowY - 40), 282, 110
    }
    Hover 700 1040 300
    Set-ClientSize 1440 900
    Hover 700 880 300
}

Step 'rail' {
    Key '^b' 900
    Hover 700 880 300
    Shot 'shell-sidebar-rail' -Crop 0, 38, 65, 862
    Mark
    Hover 32 225 1600
    Shot 'shell-rail-tooltip' -Auto -AutoPad 6
    $b = $script:LastBox
    Shot 'shell-rail-tooltip' -Crop 0, 38, ($b[0] + $b[2]), 292 -SettleMs 50
    Hover 700 880 300
    Key '^b' 900
}

Step 'palette' {
    Mark
    Key '^k' 800
    # A small pad: a wider one shows bits of the posts around the card.
    Shot 'palette-blank' -Auto -AutoPad 2
    $b = $script:LastBox
    # The hint row: the last 40 px inside the card (the auto crop adds 2 px around it).
    Shot 'palette-footer' -Crop ($b[0] + 2), ($b[1] + $b[3] - 42), ($b[2] - 4), 40
    TypeText 'spin' 800
    Shot 'palette-search' -Auto -AutoPad 2
    Key '^a' 150; TypeText 'graph' 800
    Shot 'palette-mixed-groups' -Auto -AutoPad 2
    Key '^a' 150; TypeText 'zzqx' 800
    Shot 'palette-empty' -Auto -AutoPad 2
    # The shorter card leaves dimmed posts under it in the auto crop: end the crop at the card's
    # bottom border, about 15 px below the middle of its hint row.
    $e = $script:LastBox
    $f = @(Find-SimText 'commands only' -Region $b[0], $b[1], $b[2], $b[3])
    if ($f) { Shot 'palette-empty' -Crop $e[0], $e[1], $e[2], ($f[-1].y + [int]($f[-1].h / 2) + 16 - $e[1]) -SettleMs 50 }
    Key '{ESC}' 500
    Mark
    Key '^+p' 800
    Shot 'palette-commands-mode' -Auto -AutoPad 2
    Key '{ESC}' 500
    Mark
    Key '^+k' 800
    Shot 'palette-semantic-mode' -Auto -AutoPad 2
    Key '{ESC}' 500
}

Step 'shortcut sheet' {
    Mark
    Key '^/' 900
    Shot 'shell-shortcut-sheet' -Auto -AutoPad 2
    Key '{ESC}' 500
}

Step 'persona switch' {
    ClickAt 213 870 800
    Hover 700 880 400
    Shot 'shell-user-card-switched' -Crop 0, 846, 241, 54
    ClickAt 213 870 800
    Key '^4' 1000
    Click 'Senja Prakasa' -Region 241, 80, 300, 400
    Click 'Switch to Persona'
    Hover 700 880 300
    Shot 'shell-persona-lab-active' -Crop 241, 38, 1199, 862
    Click 'Arya Wiratama' -Region 241, 80, 300, 400
    Click 'Switch to Persona'
    Key '^1' 600
}

Step 'language' {
    Click 'EN' -Region 1180, 0, 80, 38 -At 1218, 18
    # The menu's border runs x 1120-1339, y 40-414, under the globe.
    Shot 'shell-language-popover' -Crop 1120, 0, 220, 418
    # Arabic is listed with its own name, which OCR cannot read: it is the last row of the list.
    $de = @(Find-SimText 'Deutsch')
    if ($de) { ClickAt ($de[0].x + 20) ($de[0].y + 3 * 33 + 6) 1500 } else { throw 'language list not found' }
    Hover 700 880 300
    Shot 'shell-window-arabic'
    ClickAt 1218 18 800
    Click 'English' -WaitMs 1200
}

Step 'toast' {
    Key '^1' 600
    Key '^+ ' 900
    TypeText 'Read Ethics, Part II' 300
    Key '{ENTER}' 250
    # The toast's border runs x 1042-1423, y 50-102 (a post header sits right under it).
    Shot 'shell-toast-confirmation' -Crop 1038, 47, 390, 56 -SettleMs 300
    Key '{ESC}' 300
}

Step 'networks' {
    Click 'Epistemology & Mind' -Region 0, 44, 241, 40 -At 96, 64
    # The menu's border runs x 12-271, y 46-295, over the switcher.
    Shot 'network-popover' -Crop 12, 43, 260, 255
    Key '{ESC}' 500
    Palette 'create network' 900
    TypeText 'Stoa' 400
    $t = @(Find-SimText 'Create network')
    if ($t) { Shot 'network-create-dialog' -Crop ($t[0].x - 22), ($t[0].y - 22), 452, 186 }
    ClickAt 896 250 250
    Shot 'network-created' -SettleMs 100
    ClickAt 96 64 700
    Click 'Epistemology' -Region 0, 70, 360, 300
    Palette 'create network' 900
    TypeText 'Epistemology & Mind' 300
    ClickAt 896 250 600
    # The refusal shows in red under the field, which makes the dialog about 22 px taller.
    $t = @(Find-SimText 'Create network')
    if ($t) { Shot 'network-name-error' -Crop ($t[0].x - 22), ($t[0].y - 22), 452, 212 } else { throw 'dialog not found' }
    Key '{ESC}' 500
}

Step 'tour' {
    Key '^2' 1000
    Hover 700 880 300
    Shot 'start-tour-vault'
    Key '^3' 1000
    Click 'Spinoza' -Region 241, 80, 400, 500
    Hover 700 880 300
    Shot 'start-tour-agent'
    Key '^1' 700
    ClickAt 520 112 400
    TypeText "Is a mirror's image a sign of the sun, or the sun itself? #Semiotics" 400
    Key '^{ENTER}' 1400
    Hover 700 880 300
    Shot 'start-tour-first-post'
}

Step 'settings welcome tour' {
    Key '^,' 1000
    $s = @(Find-SimText 'STARTUP')
    if (-not $s) { Scroll 900 600 -10; $s = @(Find-SimText 'STARTUP') }
    # The STARTUP heading and its card (Default persona, Startup screen, Welcome tour).
    if ($s) { Shot 'start-settings-welcome-tour' -Crop ($s[0].x - 8), ($s[0].y - 10), 842, 222 } else { throw 'STARTUP not found' }
    Close-Settings
}

Step 'layouts' {
    Set-ClientSize 1100 800
    Key '^1' 500
    try { Click 'Precisely' -Region 0, 80, 1100, 700 } catch { }
    # Medium: the right sidebar is a narrower column once opened.
    Key '^%b' 900
    Hover 600 780 300
    Shot 'shell-layout-medium'
    Set-ClientSize 820 700
    Key '^%b' 900
    Hover 400 680 300
    Shot 'shell-layout-compact'
    Key '^%b' 600
    Set-ClientSize 1440 900
}
