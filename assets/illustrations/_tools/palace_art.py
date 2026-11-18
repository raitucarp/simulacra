"""Memory palace room scenes (160x120, sb E2): hall, library, garden, kitchen, study and
stairwell. Each scene draws five landmarks along a walk from left to right; their positions
are the loci of `src/domain/learning/palace/rooms.rs` (keep both in step). The app paints the
numbered locus markers over the scene, so the art itself stays quiet: a wall, a floor and
the landmarks, in at most three hues plus the outline colour."""
from kit import *

G = 'palace'
FLOOR = 'M4 84 H156 V102 Q156 116 142 116 H18 Q4 116 4 102 Z'
WALL = 'M4 20 Q4 4 20 4 H140 Q156 4 156 20 V84 H4 Z'


def room(key, body, tint='a', floor='m', ground_o='.22'):
    wall = R(4, 4, 152, 112, 16, 'e', None) + P(WALL, tint, None, o='.1')
    ground = P(FLOOR, floor, None, o=ground_o) + L('M4 84 H156', 'm', 2, '.6')
    write(f'{G}/{key}.svg', 160, 120, wall + ground + body)


def legs(x0, x1, top, bottom):
    return L(f'M{x0} {top} V{bottom} M{x1} {top} V{bottom}', 'f', 2.5)


def hall():
    door = R(12, 40, 24, 44, 4, 'a') + C(31, 64, 2, 'f', None)
    mirror = E(54, 44, 9, 13, 'u', 'f', 2.5) + L('M50 38 L56 34', 'm', 2)
    chandelier = L('M80 4 V14 M70 18 Q80 26 90 18', 'f', 2) + C(70, 18, 3, 'w', None) + C(80, 22, 3, 'w', None) + C(90, 18, 3, 'w', None)
    bench = R(92, 82, 32, 6, 3, 'w') + legs(96, 120, 88, 96)
    clock = R(129, 28, 14, 56, 4, 'b') + C(136, 42, 5, 'u', 'f', 2) + L('M136 42 V39 M136 42 H138', 'f', 1.5)
    room('hall', door + mirror + chandelier + bench + clock, 'b')


def library():
    rails = L('M14 36 L20 98 M28 36 L34 98', 'f', 2.5) + L('M16 52 H30 M18 68 H32 M19 84 H33', 'f', 2)
    books = ''.join(R(41 + i * 6, 26 + (i % 2) * 2, 5, 16 - (i % 2) * 2, 1, c, None) for i, c in enumerate('aswasw'))
    shelf = R(38, 22, 34, 46, 3, 'u') + books + R(41, 48, 28, 14, 1, 'm', None, o='.5') + L('M38 44 H72', 'f', 2)
    globe = C(84, 80, 9, 's') + L('M75 80 H93 M84 71 Q90 80 84 89', 'f', 1.5) + L('M84 89 V96 M78 96 H90', 'f', 2.5)
    lamp = P('M104 22 H120 L116 34 H108 Z', 'w') + L('M112 34 V96 M106 96 H118', 'f', 2.5)
    chair = P('M128 70 Q128 64 134 64 H144 Q150 64 150 70 V90 H128 Z', 'a') + R(126, 80, 26, 10, 4, 'a') + legs(130, 148, 90, 96)
    room('library', rails + shelf + globe + lamp + chair, 'w')


def garden():
    gate = L('M10 52 V96 M30 52 V96 M10 60 H30 M10 84 H30 M16 60 V84 M24 60 V84', 'f', 2.5)
    tree = R(47, 44, 6, 40, 2, 'm', 'f', 2) + C(50, 34, 16, 's')
    fountain = E(82, 92, 16, 5, 'u') + L('M82 90 V72 M82 74 Q74 76 72 84 M82 74 Q90 76 92 84', 'i', 2.5)
    bench = R(100, 84, 28, 5, 2, 'w') + R(100, 76, 28, 4, 2, 'w', None) + legs(104, 124, 89, 97)
    birdhouse = L('M138 50 V96', 'f', 2.5) + P('M130 40 L138 32 L146 40 V50 H130 Z', 'w') + C(138, 43, 2.5, 'f', None)
    room('garden', gate + tree + fountain + bench + birdhouse, 'i', floor='s', ground_o='.3')


def kitchen():
    stove = R(12, 60, 26, 24, 3, 'm') + C(19, 64, 3, 'd', None) + C(31, 64, 3, 'd', None) + R(16, 70, 18, 10, 2, 'u', 'f', 2)
    shelf = L('M40 38 H68', 'f', 3) + R(44, 28, 7, 10, 2, 'w', 'f', 2) + R(55, 26, 9, 12, 3, 'i', 'f', 2)
    table = R(64, 82, 36, 5, 2, 'w') + legs(68, 96, 87, 98)
    sink = R(100, 70, 28, 14, 3, 'u') + L('M114 70 V62 H120', 'f', 2.5) + E(114, 74, 8, 2, 'i', None)
    window = R(124, 22, 24, 28, 3, 'i', 'f', 2.5, o='.35') + L('M136 22 V50 M124 36 H148', 'f', 2)
    room('kitchen', stove + shelf + table + sink + window, 'w')


def study():
    books = ''.join(R(13 + i * 6, 30 + j * 20, 5, 14, 1, c, None) for j in range(3) for i, c in enumerate('asw'[j:] + 'asw'[:j]))
    bookcase = R(10, 26, 26, 58, 3, 'u') + books + L('M10 46 H36 M10 66 H36', 'f', 2)
    painting = R(42, 24, 24, 20, 2, 'w') + P('M45 41 L52 32 L57 38 L60 35 L64 41 Z', 'u', None)
    desk = R(66, 78, 36, 6, 2, 'a') + legs(70, 98, 84, 98) + R(78, 70, 12, 8, 2, 'u', 'f', 2)
    plant = P('M108 90 H120 L118 98 H110 Z', 'w') + P('M114 90 Q104 80 108 72 Q114 80 114 90 Q116 78 122 74 Q122 84 114 90', 's')
    window = R(124, 26, 24, 28, 3, 'u', 'f', 2.5) + L('M136 26 V54 M124 40 H148', 'f', 2)
    room('study', bookcase + painting + desk + plant + window, 'a')


def stairwell():
    steps = P('M8 104 H32 V94 H50 V82 H68 V70 H86 V58 H104 V46 H152 V112 H8 Z', 'u')
    banister = L('M14 92 L104 32', 'w', 3) + L('M32 80 V94 M50 68 V82 M68 56 V70 M86 44 V58', 'f', 2)
    portrait = R(76, 22, 20, 18, 2, 'a') + C(86, 30, 4, 'u', None)
    landing = R(104, 40, 44, 6, 2, 'w')
    window = R(132, 10, 16, 20, 3, 'i', 'f', 2, o='.35') + L('M140 10 V30', 'f', 2)
    room('stairwell', steps + banister + portrait + landing + window, 'i')


def build():
    for make in (hall, library, garden, kitchen, study, stairwell):
        make()
