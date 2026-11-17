"""Import sheet format cards (`import/<family>.svg`), 160x120 option cards: one small scene per
format family (book, PDF, office, web, text, data, image, audio, video, archive, unknown), plus
`import/drop.svg` for the window-wide drop hint."""
from kit import *

G = 'import'


def sheet(x, y, w=48, h=60, f='u', widths=(30, 22, 30, 18), fold=True):
    """A sheet of paper with a folded corner and text lines."""
    outline = f'M{n(x)} {n(y)} H{n(x + w - 12)} L{n(x + w)} {n(y + 12)} V{n(y + h)} H{n(x)} Z' if fold else None
    body = P(outline, f) if fold else R(x, y, w, h, 5, f)
    corner = P(f'M{n(x + w - 12)} {n(y)} V{n(y + 12)} H{n(x + w)}', 'e', 'f', 2) if fold else ''
    return body + corner + lines(x + 8, y + 20, widths, 8)


def book_card():
    ribbon = P('M86 40 V66 L92 60 L98 66 V40 Z', 'w', 'f', 2)
    card(G, 'book', T(book('b'), 80, 58, 1.6) + ribbon, 'b')


def pdf_card():
    band = R(44, 64, 40, 12, 3, 'd', 'f', 2)
    card(G, 'pdf', sheet(40, 26, 52, 66, widths=(28, 22, 0, 0, 24)) + band + T(sparkle('w', 6), 112, 36), 'd')


def office_card():
    heading = R(50, 34, 30, 6, 3, 'a', None)
    pen = T(P('M-4 -20 L4 -20 L4 10 L0 18 L-4 10 Z', 'w', 'f', 2.5), 112, 66, 1, 35)
    card(G, 'office', sheet(44, 24, 52, 68, widths=(0, 30, 34, 26, 30)) + heading + pen, 'a')


def web_card():
    frame = R(30, 28, 100, 64, 8, 'u') + L('M30 42 H130', 'f', 3)
    dots = ''.join(C(x, 35, 2.5, c, None) for x, c in ((40, 'm'), (49, 'm'), (58, 'm')))
    card(G, 'web', frame + dots + lines(42, 56, (44, 30, 38), 9) + C(110, 68, 11, 'i'), 'i')


def text_card():
    mark = L('M98 40 H114 M106 40 V82 M112 40 V82 M98 40 Q86 40 86 50 Q86 60 98 60 H106', 'a', 4)
    card(G, 'text', sheet(36, 26, 48, 64, fold=False, widths=(32, 28, 32, 20, 26)) + mark, 'a')


def data_card():
    grid = R(34, 28, 92, 62, 6, 'u')
    rules = L('M34 44 H126 M34 59 H126 M34 74 H126 M64 28 V90 M95 28 V90', 'f', 2.5)
    head = R(34, 28, 92, 16, 6, 's', 'f', 3)
    card(G, 'data', grid + head + rules + C(80, 82, 3, 's', None), 's')


def image_card():
    frame = R(34, 28, 92, 64, 8, 'u')
    hills = P('M38 88 L66 58 L86 76 L98 66 L122 88 Z', 's', 'f', 2.5)
    card(G, 'image', frame + hills + T(sun('w', 8, 8, 4), 106, 46), 's')


def audio_card():
    bars = ''.join(R(x, 60 - h / 2, 8, h, 4, c, 'f', 2) for x, h, c in
                   ((40, 18, 'b'), (54, 36, 'b'), (68, 52, 'a'), (82, 30, 'b'), (96, 44, 'a'), (110, 22, 'b')))
    card(G, 'audio', bars, 'b')


def video_card():
    film = R(32, 30, 96, 60, 8, 'u') + ''.join(R(x, 34, 6, 6, 1.5, 'f', None) + R(x, 80, 6, 6, 1.5, 'f', None) for x in (40, 58, 76, 94, 112))
    play = P('M72 48 L94 60 L72 72 Z', 'd', 'f', 3)
    card(G, 'video', film + play, 'd')


def archive_card():
    box = R(40, 44, 80, 48, 6, 'w') + R(36, 32, 88, 16, 5, 'w')
    zipper = ''.join(R(76, y, 8, 5, 1.5, 'f', None) for y in (52, 62, 72)) + R(74, 80, 12, 9, 3, 'u', 'f', 2)
    card(G, 'archive', box + zipper, 'w')


def unknown_card():
    box = R(46, 26, 68, 68, 10, 'u')
    mark = L('M70 50 Q70 40 80 40 Q92 40 92 50 Q92 58 80 62 V68', 'm', 5) + C(80, 80, 3.5, 'm', None)
    card(G, 'unknown', box + mark + T(sparkle('g', 6), 124, 32), 'g')


def drop_card():
    tray = P('M34 70 H58 L64 80 H96 L102 70 H126 V94 Q126 98 122 98 H38 Q34 98 34 94 Z', 'u')
    arrow = L('M80 22 V58', 'a', 6) + P('M68 48 L80 62 L92 48 Z', 'a', 'f', 2.5)
    card(G, 'drop', tray + arrow + T(sparkle('w', 6), 116, 34) + T(sparkle('b', 4), 46, 42), 'a')


def build():
    for make in (book_card, pdf_card, office_card, web_card, text_card, data_card, image_card, audio_card,
                 video_card, archive_card, unknown_card, drop_card):
        make()
