"""Knowledge Vault art: one card per document kind (160x120), the vault's two empty states
(240x160) and the subject index banner (320x120)."""
from kit import *
from scenes import empty, dashed


def page(x, y, w, h, f='u', rows=3, ink='m'):
    out = R(x, y, w, h, 4, f)
    return out + ''.join(L(f'M{x + 8} {y + 14 + i * 10} h{w - 16 - (i % 2) * 10}', ink, 2.5) for i in range(rows))


def kinds():
    g = 'doc_kind'
    book = P('M44 30 Q62 24 80 32 L80 96 Q62 88 44 94 Z', 'a') + P('M116 30 Q98 24 80 32 L80 96 Q98 88 116 94 Z', 'a')
    book += L('M52 44 q12 -4 22 2 M52 56 q12 -4 22 2 M86 46 q10 -6 22 -2 M86 58 q10 -6 22 -2', 'u', 2.5) + L('M80 32 v64', 'f', 3)
    card(g, 'book', book + R(100, 20, 8, 28, 2, 'w', None), 'a')
    art = R(40, 18, 80, 86, 6, 'u') + R(48, 26, 64, 14, 3, 'i') + ''.join(L(f'M{48 + c * 34} {50 + r * 9} h{26 - (r % 3) * 6}', 'm', 2.5) for c in (0, 1) for r in range(5))
    card(g, 'article', art, 'i')
    note = P('M42 22 h64 l14 14 v68 h-78 Z', 'w') + P('M106 22 v14 h14 Z', 'u', 'f', 2) + ''.join(L(f'M52 {48 + i * 12} h52', 'f', 2, '.5') for i in range(4))
    card(g, 'note', note + T(P('M0 0 L6 -26 L12 0 Z', 'a') + R(1, 0, 10, 10, 1, 'm', 'f', 2), 116, 98, 1, 30), 'w')
    web = R(32, 22, 96, 76, 8, 'u') + R(32, 22, 96, 16, 8, 'm', 'f', 3) + ''.join(C(42 + i * 9, 30, 2.5, 'f', None) for i in range(3))
    web += C(80, 70, 18, 'a', 'f', 2.5) + L('M62 70 h36 M80 52 q-10 18 0 36 q10 -18 0 -36', 'f', 2)
    card(g, 'web_page', web, 'a')
    bars = ''.join(R(34 + i * 9, 60 - h / 2, 5, h, 2.5, 's' if i % 3 else 'f', None) for i, h in enumerate((10, 22, 36, 18, 44, 28, 14, 32, 20, 8, 16)))
    card(g, 'transcript', bars + bubble(96, 18, 40, 24, 'u', 'r') + L('M106 30 h20', 'm', 2.5), 's')
    grid = ''.join(R(40 + c * 16, 24 + r * 16, 13, 13, 3, ('s', 'i', 'u')[(c * 3 + r) % 3]) for c in range(5) for r in range(5))
    card(g, 'dataset', grid, 'i')
    reel = C(80, 60, 34, 'd') + C(80, 60, 12, 'u', 'f', 2) + ''.join(C(80 + 22 * math.cos(math.radians(a)), 60 + 22 * math.sin(math.radians(a)), 6, 'u', None) for a in range(0, 360, 72))
    card(g, 'media', reel + P('M118 84 L132 92 L118 100 Z', 'w', 'f', 2), 'd')
    box = P('M44 50 L80 36 L116 50 L116 92 L80 106 L44 92 Z', 'm') + P('M44 50 L80 64 L116 50 L80 36 Z', 'u', 'f', 2.5) + L('M80 64 v42', 'f', 2.5)
    card(g, 'other', box + T(sparkle('w', 8), 120, 30) + T(qmark_small(), 80, 50), 'w')


def qmark_small():
    return L('M-4 -4 q0 -6 5 -6 q5 0 5 5 q0 4 -5 5', 'f', 2.5)


def shelf_row(y, colors):
    out = R(40, y + 30, 160, 5, 2, 'm')
    x = 52
    for i, c in enumerate(colors):
        h = 28 - (i % 3) * 4
        out += R(x, y + 30 - h, 13, h, 2, c, 'f', 2)
        x += 16
    return out


def empties():
    shelves = shelf_row(26, ('a', 'w')) + R(40, 104, 160, 5, 2, 'm') + dashed('M112 100 v-24 h20 v24 M150 100 v-20 h18 v20', 'm', 2.5)
    plus = C(176, 48, 14, 'a') + L('M170 48 h12 M176 42 v12', 'u', 3)
    empty('vault_empty', shelves + plus + T(sparkle('w', 7), 70, 96), 'a')
    lens = T(magnifier('u'), 150, 74, 1.5) + C(150, 74, 9, None, 'm', 2.5)
    cards = R(52, 44, 48, 64, 6, 'u', 'm', 2.5) + R(60, 54, 32, 6, 3, 'm', None) + R(60, 66, 24, 4, 2, 'm', None, o='.5')
    empty('vault_no_matches', cards + lens + C(196, 40, 3, 'm', None) + C(186, 30, 2, 'm', None), 'i')


def banner():
    frame = R(4, 4, 312, 112, 22, 'e', None) + C(160, 60, 52, 'a', None, o='.14') + E(160, 106, 110, 6, 'f', None, o='.08')
    cabinet = R(36, 26, 92, 80, 6, 'm') + ''.join(R(44, 34 + i * 24, 76, 18, 4, 'u', 'f', 2) + R(76, 41 + i * 24, 12, 4, 2, 'f', None) for i in range(3))
    tabs = ''.join(R(150 + i * 26, 30, 20, 26, 4, ('a', 'w', 's')[i % 3]) for i in range(5))
    cardsx = page(150, 46, 124, 56, 'u', 3)
    tree = L('M232 66 v24 M232 78 h14 M232 90 h14', 'f', 2.5) + C(232, 64, 4, 'a', None) + C(250, 78, 4, 'w', None) + C(250, 90, 4, 's', None)
    write('vault/subject_index.svg', 320, 120, frame + cabinet + tabs + cardsx + tree + T(sparkle('w', 7), 294, 26), None, 'wide')


def build():
    kinds()
    empties()
    banner()
