# Knowledge Vault, import and search (shots/vault.json). Client area 1440x900, demo profile.
# The vault shots come first, while Epistemology & Mind holds its two seed books; the import
# sheet, the search page and search by meaning follow; the Hume book is imported, catalogued and
# cited near the end, and the embedding settings are changed last (they break search by meaning).
#
# Sample files (an EPUB of Hume's Enquiry, a PNG, a 2 MB PDF and a ~39 MB PDF) are generated into
# the temp folder. Files reach the app by "paste anywhere" (a file list on the clipboard, Ctrl+V)
# instead of the native picker; the drop overlay is shown by an OLE drag started from a hidden
# helper process that shares Simulacra's input queue and is cancelled before it drops.

# --- helpers ----------------------------------------------------------------------------------

$script:ToastArea = 1040, 34, 1440, 112
$script:Files = Join-Path ([System.IO.Path]::GetTempPath()) 'simulacra-docs-vault-files'
$script:FileGen = Join-Path ([System.IO.Path]::GetTempPath()) 'simulacra-docs-vault-files.py'
$script:DragHelper = Join-Path ([System.IO.Path]::GetTempPath()) 'simulacra-docs-vault-drag.ps1'
$script:Query = 'where do our ideas come from?'

Set-Content -Path $script:FileGen -Encoding utf8 -Value @'
"""Sample files for the vault import shots: hume-enquiry.epub, photo.png, a 2 MB PDF and a ~39 MB PDF."""
import os
import sys
import zipfile

OUT = sys.argv[1]
os.makedirs(OUT, exist_ok=True)

# David Hume, An Enquiry Concerning Human Understanding (1748), public domain: short excerpts.
SECTIONS = [
    ("Section I. Of the Different Species of Philosophy", [
        "Moral philosophy, or the science of human nature, may be treated after two different manners; each of which has its peculiar merit, and may contribute to the entertainment, instruction, and reformation of mankind.",
        "The one considers man chiefly as born for action; and as influenced in his measures by taste and sentiment; pursuing one object, and avoiding another, according to the value which these objects seem to possess.",
        "The other species of philosophers consider man in the light of a reasonable rather than an active being, and endeavour to form his understanding more than cultivate his manners.",
        "Accurate and just reasoning is the only catholic remedy, fitted for all persons and all dispositions; and is alone able to subvert that abstruse philosophy and metaphysical jargon.",
    ]),
    ("Section II. Of the Origin of Ideas", [
        "Every one will readily allow, that there is a considerable difference between the perceptions of the mind, when a man feels the pain of excessive heat, or the pleasure of moderate warmth, and when he afterwards recalls to his memory this sensation, or anticipates it by his imagination.",
        "Here therefore we may divide all the perceptions of the mind into two classes or species, which are distinguished by their different degrees of force and vivacity. The less forcible and lively are commonly denominated Thoughts or Ideas.",
        "By the term impression, then, I mean all our more lively perceptions, when we hear, or see, or feel, or love, or hate, or desire, or will.",
        "Nothing, at first view, may seem more unbounded than the thought of man; but all this creative power of the mind amounts to no more than the faculty of compounding, transposing, augmenting, or diminishing the materials afforded us by the senses and experience.",
        "In short, all the materials of thinking are derived either from our outward or inward sentiment: all our ideas or more feeble perceptions are copies of our impressions or more lively ones.",
        "When we entertain any suspicion that a philosophical term is employed without any meaning or idea, we need but enquire, from what impression is that supposed idea derived?",
    ]),
    ("Section III. Of the Association of Ideas", [
        "It is evident that there is a principle of connexion between the different thoughts or ideas of the mind, and that, in their appearance to the memory or imagination, they introduce each other with a certain degree of method and regularity.",
        "To me, there appear to be only three principles of connexion among ideas, namely, Resemblance, Contiguity in time or place, and Cause or Effect.",
        "A picture naturally leads our thoughts to the original; the mention of one apartment in a building naturally introduces an enquiry concerning the others; and if we think of a wound, we can scarcely forbear reflecting on the pain which follows it.",
    ]),
    ("Section IV. Sceptical Doubts concerning the Operations of the Understanding", [
        "All the objects of human reason or enquiry may naturally be divided into two kinds, to wit, Relations of Ideas, and Matters of Fact.",
        "Propositions of the first kind are discoverable by the mere operation of thought, without dependence on what is anywhere existent in the universe.",
        "The contrary of every matter of fact is still possible. That the sun will not rise to-morrow is no less intelligible a proposition, and implies no more contradiction, than the affirmation, that it will rise.",
        "All reasonings concerning matter of fact seem to be founded on the relation of Cause and Effect, and the knowledge of this relation arises entirely from experience.",
    ]),
    ("Section V. Sceptical Solution of these Doubts", [
        "Custom, then, is the great guide of human life. It is that principle alone which renders our experience useful to us, and makes us expect, for the future, a similar train of events with those which have appeared in the past.",
        "Without the influence of custom, we should be entirely ignorant of every matter of fact beyond what is immediately present to the memory and senses.",
        "Belief is nothing but a more vivid, lively, forcible, firm, steady conception of an object, than what the imagination alone is ever able to attain.",
    ]),
    ("Section VII. Of the Idea of Necessary Connexion", [
        "The great advantage of the mathematical sciences above the moral consists in this, that the ideas of the former, being sensible, are always clear and determinate.",
        "When we look about us towards external objects, and consider the operation of causes, we are never able, in a single instance, to discover any power or necessary connexion.",
        "This connexion, therefore, which we feel in the mind, this customary transition of the imagination from one object to its usual attendant, is the sentiment or impression from which we form the idea of power.",
    ]),
    ("Section X. Of Miracles", [
        "A wise man, therefore, proportions his belief to the evidence.",
        "A miracle is a violation of the laws of nature; and as a firm and unalterable experience has established these laws, the proof against a miracle is as entire as any argument from experience can possibly be imagined.",
        "No testimony is sufficient to establish a miracle, unless the testimony be of such a kind, that its falsehood would be more miraculous, than the fact which it endeavours to establish.",
    ]),
    ("Section XII. Of the Academical or Sceptical Philosophy", [
        "It seems evident, that men are carried, by a natural instinct or prepossession, to repose faith in their senses.",
        "A Copernican or Ptolemaic, who supports each his different system of astronomy, may hope to produce a conviction, which will remain constant and durable, with his audience.",
        "If we take in our hand any volume; of divinity or school metaphysics, for instance; let us ask, Does it contain any abstract reasoning concerning quantity or number? No. Does it contain any experimental reasoning concerning matter of fact and existence? No. Commit it then to the flames: for it can contain nothing but sophistry and illusion.",
    ]),
]


def epub(path):
    with zipfile.ZipFile(path, "w") as z:
        z.writestr(zipfile.ZipInfo("mimetype"), "application/epub+zip", compress_type=zipfile.ZIP_STORED)
        z.writestr("META-INF/container.xml", """<?xml version="1.0"?>
<container version="1.0" xmlns="urn:oasis:names:tc:opendocument:xmlns:container">
  <rootfiles><rootfile full-path="OEBPS/content.opf" media-type="application/oebps-package+xml"/></rootfiles>
</container>""", compress_type=zipfile.ZIP_DEFLATED)
        manifest, spine, nav = [], [], []
        for i, (title, paras) in enumerate(SECTIONS, 1):
            body = "\n".join(f"<p>{p}</p>" for p in paras)
            z.writestr(f"OEBPS/s{i}.xhtml", f"""<?xml version="1.0" encoding="utf-8"?>
<html xmlns="http://www.w3.org/1999/xhtml" xml:lang="en"><head><title>{title}</title></head>
<body><h2>{title}</h2>
{body}
</body></html>""", compress_type=zipfile.ZIP_DEFLATED)
            manifest.append(f'<item id="s{i}" href="s{i}.xhtml" media-type="application/xhtml+xml"/>')
            spine.append(f'<itemref idref="s{i}"/>')
            nav.append(f'<li><a href="s{i}.xhtml">{title}</a></li>')
        z.writestr("OEBPS/nav.xhtml", f"""<?xml version="1.0" encoding="utf-8"?>
<html xmlns="http://www.w3.org/1999/xhtml" xmlns:epub="http://www.idpf.org/2007/ops"><head><title>Contents</title></head>
<body><nav epub:type="toc"><ol>{''.join(nav)}</ol></nav></body></html>""", compress_type=zipfile.ZIP_DEFLATED)
        z.writestr("OEBPS/content.opf", f"""<?xml version="1.0" encoding="utf-8"?>
<package xmlns="http://www.idpf.org/2007/opf" version="3.0" unique-identifier="id">
  <metadata xmlns:dc="http://purl.org/dc/elements/1.1/">
    <dc:identifier id="id">urn:simulacra-docs:hume-enquiry</dc:identifier>
    <dc:title>An Enquiry Concerning Human Understanding</dc:title>
    <dc:creator>David Hume</dc:creator>
    <dc:language>en</dc:language>
    <dc:date>2003-01-01</dc:date>
  </metadata>
  <manifest>
    <item id="nav" href="nav.xhtml" media-type="application/xhtml+xml" properties="nav"/>
    {''.join(manifest)}
  </manifest>
  <spine>{''.join(spine)}</spine>
</package>""", compress_type=zipfile.ZIP_DEFLATED)


def png(path):
    from PIL import Image, ImageDraw
    w, h = 1600, 900
    img = Image.new("RGB", (w, h))
    d = ImageDraw.Draw(img)
    for y in range(h):
        t = y / h
        d.line([(0, y), (w, y)], fill=(int(40 + 180 * t), int(70 + 90 * t), int(140 - 40 * t)))
    d.ellipse((1100, 180, 1300, 380), fill=(250, 220, 150))
    d.polygon([(0, 900), (0, 620), (380, 430), (720, 640), (1040, 470), (1600, 700), (1600, 900)], fill=(30, 45, 60))
    img.save(path)


def pdf(path, pages, target=None):
    """A plain text PDF: `pages` pages of uncompressed text, padded to `target` bytes."""
    lines = [p for _, ps in SECTIONS for p in ps]
    kids, objs, next_id = [], {}, 4
    for n in range(pages):
        text, y, k = [], 760, n
        while y > 60:
            s = lines[k % len(lines)][:90].replace("\\", "").replace("(", "").replace(")", "")
            text.append(f"BT /F1 10 Tf 56 {y} Td ({s}) Tj ET")
            y -= 13
            k += 1
        stream = "\n".join(text).encode("latin-1")
        cid, pid = next_id, next_id + 1
        next_id += 2
        objs[cid] = b"<< /Length %d >>\nstream\n" % len(stream) + stream + b"\nendstream"
        objs[pid] = f"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Contents {cid} 0 R /Resources << /Font << /F1 3 0 R >> >> >>".encode()
        kids.append(pid)
    objs[1] = b"<< /Type /Catalog /Pages 2 0 R >>"
    objs[2] = f"<< /Type /Pages /Kids [{' '.join(f'{k} 0 R' for k in kids)}] /Count {len(kids)} >>".encode()
    objs[3] = b"<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>"
    data = bytearray(b"%PDF-1.4\n")
    offsets = {}
    for i in range(1, next_id):
        offsets[i] = len(data)
        data += f"{i} 0 obj\n".encode() + objs[i] + b"\nendobj\n"
    if target:
        pad = target - len(data) - 200 - 20 * next_id
        while pad > 0:
            chunk = min(pad, 4000)
            data += b"%" + b"x" * (chunk - 2) + b"\n"
            pad -= chunk
    xref = len(data)
    data += f"xref\n0 {next_id}\n0000000000 65535 f \n".encode()
    for i in range(1, next_id):
        data += f"{offsets[i]:010d} 00000 n \n".encode()
    data += f"trailer\n<< /Size {next_id} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n".encode()
    if target and len(data) < target:
        data = data[:-6] + b" " * (target - len(data)) + data[-6:]
    with open(path, "wb") as f:
        f.write(data)


epub(os.path.join(OUT, "hume-enquiry.epub"))
png(os.path.join(OUT, "photo.png"))
pdf(os.path.join(OUT, "lecture-notes.pdf"), 40, target=2 * 1024 * 1024)
pdf(os.path.join(OUT, "collected-works.pdf"), 5800)
'@

# A drag source for the drop overlay: a hidden STA process that shares Simulacra's input queue
# (so OLE's drag loop sees the pointer), offers one file, and cancels once the flag file exists.
Set-Content -Path $script:DragHelper -Encoding utf8 -Value @'
param([string]$File, [string]$Flag, [int]$TargetPid)
Add-Type -AssemblyName System.Windows.Forms
Add-Type -Namespace SimDocs -Name DragIn -MemberDefinition (
    '[DllImport("user32.dll")] public static extern uint GetWindowThreadProcessId(IntPtr h, out uint pid);' +
    '[DllImport("user32.dll")] public static extern bool AttachThreadInput(uint a, uint b, bool attach);' +
    '[DllImport("kernel32.dll")] public static extern uint GetCurrentThreadId();')
[void][System.Windows.Forms.Application]::OleRequired()
$target = 0
$thread = [SimDocs.DragIn]::GetWindowThreadProcessId((Get-Process -Id $TargetPid).MainWindowHandle, [ref]$target)
$me = [SimDocs.DragIn]::GetCurrentThreadId()
$attached = [SimDocs.DragIn]::AttachThreadInput($me, $thread, $true)
$source = New-Object System.Windows.Forms.Control
$source.CreateControl()
$data = New-Object System.Windows.Forms.DataObject
$list = New-Object System.Collections.Specialized.StringCollection
[void]$list.Add($File)
$data.SetFileDropList($list)
$script:started = Get-Date
$source.add_QueryContinueDrag({ param($s, $e)
    if ((Test-Path $Flag) -or ((Get-Date) - $script:started).TotalSeconds -gt 20) { $e.Action = [System.Windows.Forms.DragAction]::Cancel }
    else { $e.Action = [System.Windows.Forms.DragAction]::Continue } })
[void]$source.DoDragDrop($data, [System.Windows.Forms.DragDropEffects]::Copy)
if ($attached) { [void][SimDocs.DragIn]::AttachThreadInput($me, $thread, $false) }
'@

# Trims what an auto crop caught below the palette (text behind its scrim that moved): the
# palette ends at its last full-width border row.
$script:TrimPy = Join-Path ([System.IO.Path]::GetTempPath()) 'simulacra-docs-vault-trim.py'
Set-Content -Path $script:TrimPy -Encoding utf8 -Value @'
import sys
import numpy as np
from PIL import Image
path, pad = sys.argv[1], int(sys.argv[2])
im = Image.open(path).convert("RGB")
a = np.asarray(im).astype(int)
h, w, _ = a.shape
for y in range(h - 1, h // 2, -1):
    row = a[y, pad + 16:w - pad - 16]
    if (np.abs(row - np.array([30, 30, 34])).max(axis=1) <= 8).mean() > 0.8:
        if y + 2 < h:
            im.crop((0, 0, w, y + 2)).save(path)
        break
'@

function Shot-Palette([string]$Id) {
    Shot $Id -Auto -AutoPad 3
    python $script:TrimPy (Join-Path $script:RawDir "$Id.png") 3
}

function Ensure-Files {
    $need = 'hume-enquiry.epub', 'photo.png', 'lecture-notes.pdf', 'collected-works.pdf'
    if (@($need | Where-Object { -not (Test-Path (Join-Path $script:Files $_)) }).Count) {
        python $script:FileGen $script:Files | Out-Null
    }
}

function Set-ClientSize([int]$W, [int]$H) {
    Set-SimulacraSize -Width ($W + 16) -Height ($H + 8) -X 40 -Y 30
    $script:W = $W; $script:H = $H
    Start-Sleep -Milliseconds 900
}

function Park { Hover 700 880 300 }

function Set-FileClipboard([string]$Name) {
    <# Puts one file on the clipboard as a file list (what Explorer's Copy does). #>
    Set-SimClipboardFile (Join-Path $script:Files $Name)
}

function Paste-File([string]$Name, [int]$WaitMs = 2000, [switch]$Mark) {
    <# Opens the import sheet for a sample file: paste with nothing focused. #>
    Key '{ESC}' 200
    ClickAt 1000 760 300
    Set-FileClipboard $Name
    if ($Mark) { Park; Mark }
    Key '^v' $WaitMs
    Restore-SimClipboard
}

function Close-Sheet {
    <# The import sheet's close cross: 34 px in from its right edge, 54 px below its top
       ($script:LastBox holds the sheet from an auto shot with -AutoPad 0). #>
    $b = $script:LastBox
    ClickAt ($b[0] + $b[2] - 34) ($b[1] + 54) 700
}

function ClickWord([string]$Word, [int[]]$Region, [int]$Nth = 0, [int]$WaitMs = 600) {
    <# Clicks $Word inside the OCR line that holds it (OCR merges a row of chips or buttons into
       one line): its x is estimated from its place in the line's text. #>
    $hits = @(Find-SimText $Word -Region $Region)
    if ($hits.Count -le $Nth) { throw "Word '$Word' not found on screen" }
    $h = $hits[$Nth]
    $i = $h.text.ToLower().IndexOf($Word.ToLower())
    $x = $h.x + [int]($h.w * ($i + $Word.Length / 2) / $h.text.Length)
    ClickAt $x ($h.y + [int]($h.h / 2)) $WaitMs
}

function Wait-Text([string]$Text, [int[]]$Region, [int]$Seconds = 25, [switch]$Gone) {
    $deadline = (Get-Date).AddSeconds($Seconds)
    while ((Get-Date) -lt $deadline) {
        $hit = @(Find-SimText $Text -Region $Region)
        if ($Gone -and -not $hit) { return }
        if (-not $Gone -and $hit) { return $hit[0] }
        Start-Sleep -Milliseconds 700
    }
    if ($Gone) { throw "'$Text' still showing after $Seconds s" } else { throw "'$Text' did not appear within $Seconds s" }
}

function Go-VaultHome {
    <# The vault's browse page: Ctrl+2 reopens the vault where it was left, so step Back out of an
       entry, the reader, the editor or the subject index. #>
    Key '{ESC}' 200
    Key '^2' 900
    for ($i = 0; $i -lt 4; $i++) {
        $b = @(Find-SimText 'Back' -Region 241, 80, 500, 60)
        if (-not $b) { return }
        ClickAt ($b[0].x + 12) ($b[0].y + 6) 900
    }
}

function Switch-Network([string]$Name) {
    ClickAt 96 64 700
    Click $Name -Region 0, 70, 360, 300
    Start-Sleep -Milliseconds 600
}

function Open-Settings([string]$Pane, [int]$Y) {
    Key '{ESC}' 200
    Key '^,' 1000
    Click $Pane -Region 180, ($Y - 25), 220, 50 -At 250, $Y
}

function Shot-StreamCard([string]$Id, [string]$Handle, [string]$Marker) {
    <# The first stream card by $Handle that holds $Marker: from its author row to the next card. #>
    $lines = @(Get-SimText | Where-Object { $_.x -ge 241 -and $_.x -lt 760 })
    $m = @($lines | Where-Object { $_.text -like "*$Marker*" -and $_.y -gt 80 })[0]
    if (-not $m) { throw "'$Marker' not found in the stream" }
    $authors = @($lines | Where-Object { $_.text -like '*@*' -and $_.x -ge 300 -and $_.x -lt 560 })
    $top = @($authors | Where-Object { $_.y -lt $m.y -and $_.text -like "*$Handle*" } | Sort-Object y)[-1]
    $next = @($authors | Where-Object { $_.y -gt $m.y } | Sort-Object y)[0]
    $y0 = if ($top) { $top.y - 16 } else { $m.y - 120 }
    $y1 = if ($next) { $next.y - 16 } else { [Math]::Min(900, $m.y + 80) }
    Shot $Id -Crop 241, $y0, 519, ($y1 - $y0)
}

function Open-AgentWrite {
    <# Personas & Agents › Baruch de Spinoza › Write a post… (the dialog opens on New post). #>
    Key '{ESC}' 200
    Key '^3' 1100
    Click 'Baruch de Spinoza' -Region 241, 80, 400, 500
    Click 'Write a post' -At 560, 300 -WaitMs 900
}

function Write-AndOpen([string]$Id, [string]$Marker) {
    <# Writes with the open dialog, waits for "Posted" and opens the post in the timeline. #>
    $w = @(Find-SimText 'Write' -Region 700, 300, 400, 80 | Where-Object { $_.text -notlike '*post*' })
    if ($w) { ClickAt ($w[0].x + 16) ($w[0].y + 6) 900 } else { throw 'the Write button was not found' }
    # The run status appears under the profile card's buttons, wherever the card ends.
    $done = Wait-Text 'Posted' -Region 440, 300, 900, 400 -Seconds 40
    ClickWord 'Open the post' -Region 900, ($done.y - 12), 500, 40 -WaitMs 1500
    Park
    Shot-StreamCard $Id 'baruch_spinoza' $Marker
}

# --- the vault -------------------------------------------------------------------------------

Step 'welcome' -Always {
    Ensure-Files
    Skip-Welcome
}

Step 'vault browse' {
    Go-VaultHome
    Park
    Shot 'vault-overview'
    Shot 'vault-toolbar' -Crop 478, 156, 956, 48
    Shot 'vault-facet-rail' -Crop 241, 84, 231, 676
    Shot 'vault-shelf-card' -Crop 622, 203, 148, 322
    $s = @(Find-SimText 'Sources' -Region 0, 440, 241, 200)
    if (-not $s) { throw 'the Sources section was not found' }
    Shot 'vault-sources-sidebar' -Crop 0, ($s[0].y - 12), 241, 86
}

Step 'sort menu' {
    Go-VaultHome
    Park; Mark
    Click 'Sort' -Region 1100, 160, 220, 40 -At 1236, 180
    Hover 1380 600 300
    Shot 'vault-sort-menu' -Auto -AutoPad 10
    Key '{ESC}' 400
}

Step 'subject tree' {
    Go-VaultHome
    ClickAt 254 162 600
    Park
    Shot 'vault-subject-tree' -Crop 241, 126, 231, 126
    ClickAt 254 162 600
}

Step 'active filters' {
    Go-VaultHome
    Click 'Treatise' -Region 241, 440, 230, 100
    Park
    Shot 'vault-active-filters' -Crop 476, 204, 236, 34
    Click 'Clear filters' -Region 476, 200, 400, 40 -At 627, 225
}

Step 'table' {
    Go-VaultHome
    Click 'Table' -Region 1310, 160, 130, 40 -At 1396, 180
    Park
    Shot 'vault-table' -Crop 478, 205, 956, 172
    Click 'Shelf' -Region 1310, 160, 130, 40 -At 1342, 180
}

Step 'no matches' {
    Go-VaultHome
    ClickAt 660 180 400
    TypeText 'zzzz' 900
    Park
    Shot 'vault-no-matches' -Crop 478, 156, 956, 380
    Key '^a' 150; Key '{BACKSPACE}' 600
    Key '{ESC}' 300
}

Step 'ethics entry' {
    Go-VaultHome
    ClickAt 551 300 1200
    Hover 1180 890 300
    Shot 'vault-entry'
    Shot 'vault-entry-record' -Crop 1114, 127, 320, 741
    # The entry scrolls a little: at its end Filed under, Contents, Cite this and Related entries
    # all show.
    Scroll 1180 880 -10
    Hover 1180 890 300
    Shot 'vault-entry-filing' -Crop 248, 268, 864, 250
    Shot 'vault-entry-related' -Crop 248, 732, 864, 160
    $copy = @(Find-SimText 'Copy' -Region 1040, 540, 80, 60)
    if ($copy) { ClickAt ($copy[0].x + 6) ($copy[0].y + 6) 250 } else { ClickAt 1070 580 250 }
    Hover 1180 890 100
    Shot 'vault-entry-citations' -Crop 248, 520, 864, 202 -SettleMs 100
    Scroll 1180 880 10
}

Step 'delete dialog' {
    $d = @(Find-SimText 'Delete' -Region 241, 300, 300, 80)
    if (-not $d) { Go-VaultHome; ClickAt 551 300 1200 }
    Park; Mark
    ClickWord 'Delete' -Region 241, 300, 900, 80 -WaitMs 800
    Hover 1300 880 300
    Shot 'vault-delete-dialog' -Auto -AutoPad 12
    Click 'Cancel' -Region 400, 60, 700, 300
}

Step 'tractatus entry' {
    Go-VaultHome
    ClickAt 695 300 1200
    Hover 1180 890 300
    Shot 'vault-entry-title' -Crop 246, 128, 866, 240
    # "Continue reading (35%)" (white on violet) escapes OCR: it is the first button of the row.
    $lp = @(Find-SimText 'Learn passages' -Region 241, 260, 900, 120)
    $y = if ($lp) { $lp[0].y + 7 } else { 311 }
    ClickAt 340 $y 1200
    Hover 1180 890 300
    Shot 'vault-reader'
    Go-VaultHome
}

Step 'subject index' {
    Go-VaultHome
    Click 'Subject index' -Region 1060, 100, 130, 40 -At 1124, 122 -WaitMs 900
    Park
    Shot 'vault-subject-index'
    Go-VaultHome
}

Step 'editor' {
    Go-VaultHome
    Click 'Add document' -Region 1290, 100, 150, 40 -At 1360, 122 -WaitMs 900
    Hover 1330 880 300
    Shot 'vault-editor'
    # A date that is not a date: the problem shows on Save.
    ClickAt 840 226 300
    TypeText 'Test' 200
    ClickAt 1025 390 300
    TypeText 'abc' 300
    Scroll 1330 600 -30
    Click 'Save' -Region 1100, 600, 200, 300
    Scroll 1330 400 30
    Hover 1330 880 300
    Shot 'vault-editor-problems' -Crop 460, 345, 760, 112
    # Both folded sections open, ISBN chosen; scrolled so the ISBN chips sit near the top.
    Scroll 1330 600 -5
    Click 'Publication details' -Region 460, 38, 760, 862
    Scroll 1330 600 -5
    ClickWord 'ISBN' -Region 460, 38, 760, 862
    Click 'Classification' -Region 460, 38, 760, 862
    # The two cards are taller than half the window: from the end of the form, scroll up in
    # small wheel steps until the ISBN chips are just below the top.
    Scroll 1330 600 -30
    $isbn = $null
    for ($i = 0; $i -lt 25; $i++) {
        $isbn = @(Find-SimText 'ISBN' -Region 460, 38, 760, 862)
        if ($isbn -and $isbn[0].y -ge 56) { break }
        Move-SimMouse 1330 600 -WaitMs 60
        Send-SimWheel (24)
        Start-Sleep -Milliseconds 400
    }
    if (-not $isbn) { throw 'the ISBN chips were not found' }
    $lines = @(Get-SimText | Where-Object { $_.x -ge 460 -and $_.x -lt 1220 })
    $a = @($lines | Where-Object { $_.text -match '^Abstract$' })
    $per = @($lines | Where-Object { $_.text -match '^Period$' })
    $bottom = if ($a) { $a[0].y - 18 } elseif ($per) { [Math]::Min(898, $per[0].y + 100) } else { 898 }
    $top = [Math]::Max(40, $isbn[0].y - 16)
    Hover 1330 880 300
    Shot 'vault-editor-sections' -Crop 460, $top, 760, ($bottom - $top)
    Scroll 1330 600 -30
    Click 'Cancel' -Region 1000, 700, 220, 200 -At 1120, 872
    Go-VaultHome
}

Step 'empty vault' {
    Switch-Network 'Sci-Fi'
    Key '^2' 1000
    Park
    Shot 'vault-empty' -Crop 241, 80, 1199, 384
    Switch-Network 'Epistemology'
}

Step 'compact drawer' {
    Go-VaultHome
    Set-ClientSize 720 900
    Click 'Filters' -Region 400, 160, 200, 40 -At 483, 180 -WaitMs 900
    Hover 600 880 300
    Shot 'vault-compact-drawer'
    ClickAt 600 600 600
    Set-ClientSize 1440 900
}

# --- import ------------------------------------------------------------------------------------

Step 'import sheet' {
    Go-VaultHome
    Paste-File 'hume-enquiry.epub' -Mark
    Hover 1300 880 300
    Shot 'import-tutorial-sheet' -Auto -AutoPad 0
    $b = $script:LastBox
    Shot 'import-sheet'
    Shot 'import-sheet-header' -Crop $b[0], $b[1], $b[2], 110
    $split = @(Find-SimText 'SPLIT INTO' -Region $b[0], $b[1], $b[2], $b[3])
    $sy = if ($split) { $split[0].y - 12 } else { 485 }
    Shot 'import-sheet-units' -Crop $b[0], $sy, $b[2], ($b[1] + $b[3] - 50 - $sy)
    Shot 'import-sheet-footer' -Crop $b[0], ($b[1] + $b[3] - 50), $b[2], 50
    Click 'Pseudonym' -Region 840, 400, 260, 50 -At 960, 426
    TypeText 'Philo' 600
    Hover 1300 880 300
    $p = @(Find-SimText 'POST AS' -Region $b[0], $b[1], $b[2], $b[3])
    $py = if ($p) { $p[0].y - 12 } else { 378 }
    Shot 'import-sheet-post-as' -Crop $b[0], $py, $b[2], 146
    $script:LastBox = $b
    Close-Sheet
}

Step 'drop overlay' {
    # A drag from Explorer needs the real pointer: OLE's drag loop follows it.
    if (Test-SimBackground) { throw 'needs the desktop: run with -Foreground' }
    Go-VaultHome
    $flag = Join-Path ([System.IO.Path]::GetTempPath()) 'simulacra-docs-vault-drag.flag'
    Remove-Item $flag -ErrorAction SilentlyContinue
    Hover 1000 700 200
    $sim = (Get-Process simulacra | Select-Object -First 1).Id
    $helper = Start-Process powershell.exe -PassThru -WindowStyle Hidden -ArgumentList '-NoProfile', '-STA', '-WindowStyle', 'Hidden',
        '-File', $script:DragHelper, '-File', (Join-Path $script:Files 'hume-enquiry.epub'), '-Flag', $flag, '-TargetPid', $sim
    Start-Sleep -Milliseconds 2500
    Focus-Simulacra
    for ($i = 1; $i -le 10; $i++) {
        $pt = ConvertTo-ScreenPoint (1000 - $i * 10) (700 - $i * 20)
        [SimWin]::SetCursorPos($pt.X, $pt.Y) | Out-Null
        Start-Sleep -Milliseconds 60
    }
    Start-Sleep -Milliseconds 800
    try {
        if (-not @(Find-SimText 'Drop to import')) { throw 'the drop overlay did not show' }
        Shot 'import-drop-overlay' -SettleMs 50
    } finally {
        New-Item -ItemType File -Path $flag -Force | Out-Null
        foreach ($p in @(@(880, 480), @(890, 490), @(900, 500))) {
            $pt = ConvertTo-ScreenPoint $p[0] $p[1]; [SimWin]::SetCursorPos($pt.X, $pt.Y) | Out-Null; Start-Sleep -Milliseconds 300
        }
        if (-not $helper.WaitForExit(5000)) { Stop-Process -Id $helper.Id -Force -ErrorAction SilentlyContinue }
        Remove-Item $flag -ErrorAction SilentlyContinue
    }
    Park
}

Step 'import reading' {
    Go-VaultHome
    Paste-File 'collected-works.pdf' -WaitMs 250 -Mark
    Shot 'import-sheet-reading' -Auto -AutoPad 0 -SettleMs 50
    try { Click 'Cancel' -Region 340, 250, 760, 400 -WaitMs 800 } catch { }
    if (@(Find-SimText 'collected-works')) { Close-Sheet }
    if (@(Find-SimText 'collected-works')) { Key '{ESC}' 300; ClickAt 1066 145 700 }
}

Step 'import image' {
    Go-VaultHome
    Paste-File 'photo.png' -WaitMs 1800 -Mark
    Hover 1300 880 300
    Shot 'import-sheet-image' -Auto -AutoPad 0
    Close-Sheet
}

Step 'import failed' {
    # Largest file down to 1 MB (− steps 50 → 45 … 5 → 1), then a 2 MB file; the cap is reset after.
    Open-Settings 'Import' 429
    for ($i = 0; $i -lt 11; $i++) { ClickAt 1109 396 120 }
    Key '{ESC}' 600
    Go-VaultHome
    Paste-File 'lecture-notes.pdf' -WaitMs 1500 -Mark
    Hover 1300 880 300
    Shot 'import-sheet-failed' -Auto -AutoPad 0
    Close-Sheet
    Open-Settings 'Import' 429
    ClickAt 1081 396 400
    Key '{ESC}' 600
}

# --- search ------------------------------------------------------------------------------------

Step 'search page' {
    Key '{ESC}' 200
    Click 'Search' -Region 0, 430, 241, 30 -At 58, 445 -WaitMs 900
    Park
    Shot 'search-page'
    ClickAt 840 128 300
    TypeText 'from:sokrates' 900
    Click 'Save search' -Region 440, 260, 400, 40 -At 540, 285
    ClickAt 840 128 300
    Key '^a' 150; Key '{BACKSPACE}' 400
    TypeText 'the -from:architect' 1200
    Park
    Shot 'search-results'
    $f = @(Find-SimText 'Filters' -Region 441, 140, 200, 60)
    $fy = if ($f) { $f[0].y - 12 } else { 156 }
    Shot 'search-chips' -Crop 450, $fy, 790, 146
}

Step 'meaning search' {
    Key '{ESC}' 300
    Key '^1' 900
    Park; Mark
    Key '^+k' 900
    Shot-Palette 'search-meaning-prompt'
    TypeText $script:Query 600
    Wait-Text 'By meaning' -Seconds 20 | Out-Null
    Start-Sleep -Milliseconds 800
    Shot-Palette 'search-meaning-results'
    Key '{ESC}' 500
}

Step 'timeline' {
    Key '{ESC}' 300
    Key '^1' 900
    Hover 636 59 1600
    Shot 'search-feed-order' -Crop 241, 38, 690, 92
    Park
    # Baruch de Spinoza's reply in the stream: select it, then its "Find similar".
    $a = @(Find-SimText 'Baruch de Spinoza' -Region 241, 150, 520, 750)
    if (-not $a) { throw "Spinoza's post was not found in the stream" }
    $ay = $a[0].y
    ClickAt 500 ($ay + 60) 900
    Park
    $r = @(Find-SimText 'Related thoughts' -Region 760, 80, 680, 800)
    if (-not $r) { throw 'Related thoughts were not found in the inspector' }
    # End above the first row that the reply bar would cut (a row is about 74 px tall).
    $rows = @(Find-SimText '@' -Region 1100, ($r[0].y + 30), 340, 840 | Sort-Object y)
    $bottom = 848
    $cut = @($rows | Where-Object { $_.y + 66 -gt 848 })
    if ($cut) { $bottom = $cut[0].y - 8 }
    Shot 'search-related-thoughts' -Crop 762, ($r[0].y - 12), 676, ($bottom - $r[0].y + 12)
    Park; Mark
    ClickAt 733 ($ay + 10) 900
    Hover 290 ($ay + 10) 300
    Click 'Find similar' -Region 500, 0, 260, 900 -WaitMs 2500
    Hover 1300 880 300
    Shot 'search-similar-sheet' -Auto -AutoPad 0 -Within 380, 40, 1060, 900
    Key '{ESC}' 500
}

# --- grounding ---------------------------------------------------------------------------------

Step 'agent grounding' {
    Key '{ESC}' 300
    Key '^3' 1100
    Click 'Baruch de Spinoza' -Region 241, 80, 400, 500
    Hover 1300 880 300
    Mark
    Click 'Write a post' -At 560, 300 -WaitMs 900
    ClickWord 'Ethics' -Region 495, 220, 460, 60
    Hover 1300 880 300
    Shot 'vault-trigger-document' -Auto -AutoPad 0
    Write-AndOpen 'vault-post-sources' 'Sources'
}

# --- the tutorial: Hume's Enquiry --------------------------------------------------------------

Step 'hume catalogue' {
    Go-VaultHome
    Paste-File 'hume-enquiry.epub' -WaitMs 2000
    ClickWord 'Just add to vault' -Region 700, 740, 400, 60 -WaitMs 1500
    Go-VaultHome
    ClickAt 551 300 1200
    ClickWord 'Edit' -Region 241, 280, 900, 120 -WaitMs 900
    ClickAt 1025 390 300
    Key '^a' 150
    TypeText '1748' 300
    Scroll 1330 600 -5
    Click 'Classification' -Region 460, 38, 760, 862
    Scroll 1330 600 -5
    $s = @(Get-SimText | Where-Object { $_.text -match '^\W?\s?Subjects$' -and $_.x -lt 520 })[0]
    if (-not $s) { throw 'the Subjects field was not found' }
    ClickAt 840 ($s.y + 38) 300
    TypeText 'Epistemology' 500
    ClickWord 'Philosophy' -Region 470, ($s.y + 70), 750, 60
    $c = @(Find-SimText 'Foundations' -Region 470, ($s.y + 100), 300, 250)
    if (-not $c) { throw 'the Foundations chip was not found' }
    ClickAt ($c[-1].x + 30) ($c[-1].y + 6) 400
    ClickWord 'Philosophy & psychology' -Region 470, ($s.y + 200), 750, 120
    ClickWord 'Essay' -Region 470, ($s.y + 300), 750, 160
    ClickWord 'Early modern' -Region 470, ($s.y + 380), 750, 160
    Hover 1330 880 300
    Click 'Save' -Region 1100, 780, 200, 120 -WaitMs 1500
    Go-VaultHome
    ClickAt 551 300 1200
    Hover 1180 890 300
    Shot 'vault-tutorial-entry'
}

Step 'hume meaning' {
    Key '{ESC}' 300
    Key '^1' 900
    Park
    Wait-Text 'Indexing' -Region 1000, 700, 440, 200 -Seconds 90 -Gone
    Mark
    Key '^+k' 900
    TypeText $script:Query 600
    Wait-Text 'By meaning' -Seconds 20 | Out-Null
    Start-Sleep -Milliseconds 800
    Shot-Palette 'search-tutorial-meaning'
    Key '{ESC}' 500
}

Step 'hume citation' {
    Open-AgentWrite
    $t = @(Find-SimText 'TOPIC' -Region 495, 120, 460, 120)
    $ty = if ($t) { $t[0].y + 30 } else { 198 }
    ClickAt 700 $ty 300
    TypeText 'impressions and ideas' 400
    ClickWord 'Enquiry' -Region 495, 200, 460, 90
    Write-AndOpen 'vault-tutorial-citation' 'Enquiry'
}

# --- embedding settings (last: they stop search by meaning) ------------------------------------

Step 'meaning fallback' {
    Open-Settings 'Embeddings' 285
    Scroll 830 600 -5
    $s = @(Get-SimText | Where-Object { $_.text -match '^Semantic search$' -and $_.x -lt 700 })
    if (-not $s) { throw 'the Semantic search row was not found' }
    ClickAt 1213 ($s[0].y + 8) 600
    Key '{ESC}' 600
    Key '^1' 900
    Park; Mark
    Key '^+k' 900
    TypeText 'substance' 600
    Wait-Text 'Matching words' -Seconds 20 | Out-Null
    Start-Sleep -Milliseconds 600
    Shot-Palette 'search-meaning-fallback'
    Key '{ESC}' 500
    Open-Settings 'Embeddings' 285
    Scroll 830 600 -5
    $s = @(Get-SimText | Where-Object { $_.text -match '^Semantic search$' -and $_.x -lt 700 })
    if ($s) { ClickAt 1213 ($s[0].y + 8) 600 }
    Key '{ESC}' 600
}

Step 'indexing paused' {
    # An address where nothing listens, then Rebuild: the retries fail and indexing pauses.
    Open-Settings 'Embeddings' 285
    $a = @(Find-SimText 'Local server address' -Region 400, 80, 860, 760)
    if (-not $a) { throw 'the Local server address row was not found' }
    ClickAt 1108 ($a[0].y + 18) 300
    Key '^a' 150
    TypeText 'http://127.0.0.1:9' 600
    Scroll 830 600 -5
    ClickWord 'Rebuild' -Region 1100, 300, 160, 500 -WaitMs 800
    Key '{ESC}' 600
    Key '^1' 900
    Park
    $p = Wait-Text 'Indexing paused' -Region 900, 600, 540, 300 -Seconds 90
    Start-Sleep -Milliseconds 500
    # The whole toast, from its title to its bottom edge (it sits 16 px above the window's
    # bottom), however many lines its message wraps to.
    Shot 'search-indexing-paused' -Crop ($p.x - 32), ($p.y - 14), 264, (886 - ($p.y - 14))
}
