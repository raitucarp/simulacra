"""Reusable spot illustrations (160x120, sb D3): small scenes for ideas, connections, memory,
reading, writing, debate and calm, used wherever a screen needs a light touch of art
(onboarding cards, quiet notes, side panels)."""
from kit import *

G = 'spot'


def ideas():
    rays = L('M80 18 v-6 M104 28 l5 -5 M56 28 l-5 -5 M112 50 h7 M48 50 h-7', 'w', 3)
    glow = C(80, 50, 26, 'w', None, o='.22')
    card(G, 'ideas', glow + T(bulb('w'), 80, 54, 1.5) + rays + T(sparkle('a', 7), 120, 82) + T(sparkle('a', 5), 40, 80), 'w')


def connections():
    nodes = ((46, 40, 'a'), (114, 36, 'b'), (80, 84, 's'), (122, 82, 'a'))
    links = L('M46 40 L80 84 L114 36 Z M80 84 L122 82 M114 36 L122 82', 'm', 2.5)
    dots = ''.join(C(x, y, 11, c) + C(x, y, 4, 'u', None) for x, y, c in nodes)
    card(G, 'connections', links + dots + T(sparkle('a', 6), 30, 76), 'b')


def memory():
    cards = ''.join(T(R(-28, -18, 56, 36, 6, c, 'f', 2.5), 74 + i * 6, 58 + i * 8, 1, r) for i, (c, r) in enumerate((('m', -10), ('u', -3), ('u', 6))))
    lines_ = T(lines(-16, -4, (30, 20), 8, 'm', 3), 86, 74, 1, 6)
    knot = L('M118 26 q8 10 0 18 q-8 8 0 18', 'd', 3) + C(118, 26, 4, 'd', None)
    card(G, 'memory', cards + lines_ + knot + T(sparkle('w', 6), 40, 34), 'a')


def reading():
    lamp = L('M118 98 V40 Q118 30 108 30', 'f', 3) + P('M96 24 L120 24 L112 38 L104 38 Z', 'w') + C(108, 44, 9, 'w', None, o='.3')
    card(G, 'reading', T(book('a'), 70, 66, 1.4) + lamp + T(sparkle('w', 5), 40, 34), 'a')


def writing():
    page = R(42, 22, 58, 76, 6, 'u') + lines(52, 38, (38, 30, 38, 22), 9, 'm', 3)
    quill = T(P('M0 0 Q4 -30 30 -46 Q22 -20 0 0 Z', 'b') + L('M2 -4 L22 -34', 'f', 2), 100, 92, .9)
    card(G, 'writing', page + quill + L('M94 96 q-8 -4 -14 0', 'f', 2.5) + T(sparkle('w', 6), 124, 30), 'b')


def debate():
    left = bubble(26, 24, 58, 38, 'a') + L('M38 44 h32', 'u', 3)
    right = bubble(78, 52, 58, 38, 'w', 'r') + L('M92 72 h30', 'f', 3)
    card(G, 'debate', left + right + T(sparkle('b', 6), 126, 30), 'w')


def calm():
    cup = P('M54 62 H100 V80 Q100 96 84 96 H70 Q54 96 54 80 Z', 's') + L('M100 68 q12 0 12 9 q0 9 -12 9', 'f', 3)
    steam = L('M68 52 q-5 -8 0 -16 q5 -8 0 -14 M84 52 q-5 -8 0 -16 q5 -8 0 -14', 'm', 2.5)
    leaf_ = T(leaf('s'), 124, 40, 1, -20)
    card(G, 'calm', cup + steam + leaf_ + R(46, 96, 64, 6, 3, 'm'), 's')


def build():
    for make in (ideas, connections, memory, reading, writing, debate, calm):
        make()
