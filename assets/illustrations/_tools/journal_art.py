"""Journal page art (sb E3): the page header spot (160x120: an open notebook, a pen and a
small sun and moon for mornings and evenings) and its empty states (240x160): no entries
yet, no check-ins (the affect grid with one waiting dot), no focus sessions (an hourglass),
no capsules (an envelope), and a sealed capsule (a lock on an envelope, a calendar page)."""
from kit import *
from scenes import empty, dashed

G = 'spot'


def notebook(x, y, f='u', ink='m'):
    """An open notebook, centred on its spine."""
    pages = P(f'M{x - 44} {y - 26} Q{x - 22} {y - 32} {x} {y - 24} Q{x + 22} {y - 32} {x + 44} {y - 26} L{x + 44} {y + 26} '
              f'Q{x + 22} {y + 18} {x} {y + 26} Q{x - 22} {y + 18} {x - 44} {y + 26} Z', f, 'f', 2.5)
    spine = L(f'M{x} {y - 24} V{y + 26}', 'f', 2)
    rows = lines(x - 36, y - 12, (28, 22, 26), 9, ink, 2.5) + lines(x + 8, y - 12, (28, 18), 9, ink, 2.5)
    return pages + spine + rows


def pen(x, y, c='a'):
    return T(R(-4, -26, 8, 34, 3, c, 'f', 2.5) + P('M-4 8 L0 16 L4 8 Z', 'u', 'f', 2), x, y, 1, 35)


def journal():
    sky = T(sun('w', 7, 8, 4), 34, 30) + T(moon('i'), 128, 28)
    card(G, 'journal', notebook(80, 64) + pen(118, 74) + sky, 'a')


def journal_empty():
    slot = dashed('M150 100 h44 M150 112 h30', 'm', 2.5)
    empty('journal_empty', notebook(96, 82, 'u', 'm') + pen(140, 86, 'a') + slot + T(sparkle('w', 7), 176, 40), 'a')


def mood_empty():
    grid = R(70, 30, 100, 100, 10, 'u', 'm', 2.5) + L('M120 36 V124 M76 80 H164', 'm', 2, '.6')
    tints = C(146, 54, 14, 'w', None, o='.25') + C(146, 106, 14, 's', None, o='.25') + C(94, 106, 14, 'm', None, o='.2')
    dot = C(132, 66, 7, 'a') + C(132, 66, 13, 'a', None, o='.2')
    empty('mood_empty', grid + tints + dot + T(sparkle('w', 6), 186, 40), 'a')


def focus_empty():
    glass = P('M96 34 H144 M96 126 H144 M102 34 Q102 62 120 80 Q138 62 138 34 M102 126 Q102 98 120 80 Q138 98 138 126', None, 'f', 3)
    sand = P('M108 46 H132 Q130 62 120 72 Q110 62 108 46 Z', 'w', None) + P('M106 120 Q112 104 120 100 Q128 104 134 120 Z', 'w', None)
    empty('focus_empty', glass + sand + T(sparkle('a', 6), 166, 50) + C(76, 60, 3, 'm', None), 's')


def envelope(x, y, f='u'):
    body = R(x - 40, y - 26, 80, 52, 6, f, 'f', 2.5)
    flap = L(f'M{x - 38} {y - 22} L{x} {y + 4} L{x + 38} {y - 22}', 'f', 2.5)
    return body + flap


def capsules_empty():
    stamp = R(150, 52, 14, 16, 2, 'a', 'f', 2)
    trail = dashed('M60 120 q24 -16 50 -6', 'm', 2.5)
    empty('capsules_empty', envelope(120, 80) + stamp + trail + T(sparkle('w', 7), 66, 46), 'a')


def capsule_sealed():
    lock = R(108, 78, 24, 20, 4, 'w', 'f', 2.5) + L('M113 78 V71 Q113 64 120 64 Q127 64 127 71 V78', 'f', 2.5) + C(120, 88, 2.5, 'f', None)
    page = R(160, 34, 34, 36, 5, 'u', 'f', 2.5) + R(160, 34, 34, 10, 5, 'a', None) + lines(168, 54, (18, 12), 7, 'm', 2.5)
    empty('capsule_sealed', envelope(110, 88) + lock + page + T(sparkle('w', 6), 62, 44), 'w')


def build():
    for make in (journal, journal_empty, mood_empty, focus_empty, capsules_empty, capsule_sealed):
        make()
