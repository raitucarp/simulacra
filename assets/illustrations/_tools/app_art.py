"""Empty states across the app (240x160, sb D3): the idle inspector, the actions sheet before
an action is picked, the image ideas helper and its empty result grid, an import with nothing
readable, no stored keys and a vault without subjects."""
from kit import *
from scenes import empty, dashed


def inspector_idle():
    thread = L('M74 44 V116', 'm', 3, '.5')
    posts = ''.join(R(60, y, 92, 24, 7, 'u', 'm', 2.5) + C(74, y + 12, 6, c, None) + L(f'M86 {y + 9} h44 M86 {y + 16} h28', 'm', 2.5, '.5')
                    for y, c in ((32, 'a'), (68, 'm'), (104, 'm')))
    lens = T(magnifier('u'), 172, 74, 1.4) + C(172, 74, 9, None, 'a', 2.5)
    empty('inspector_idle', thread + posts + lens + T(sparkle('w', 7), 196, 34), 'a')


def actions_idle():
    cloud_ = T(cloud('u', 'f', 3), 92, 70, 2)
    wand = T(R(-4, 0, 8, 58, 4, 'f', None) + R(-4, 0, 8, 12, 4, 'w', None), 170, 64, 1, 40)
    sparks = ''.join(T(sparkle(c, r), x, y) for x, y, r, c in ((178, 40, 9, 'w'), (196, 62, 6, 'a'), (150, 30, 5, 'a'), (204, 32, 4, 'w')))
    empty('actions_idle', cloud_ + lines(68, 66, (40, 26), 9, 'm', 3) + wand + sparks, 'a')


def image_ideas():
    legs = L('M96 132 L112 46 M152 132 L136 46 M124 132 V100', 'f', 3)
    canvas = R(86, 30, 76, 66, 5, 'u') + P('M92 90 L114 64 L128 78 L138 70 L156 90 Z', 's', None, o='.7') + C(144, 48, 7, 'w', None)
    board = P('M36 112 Q30 88 54 84 Q78 80 78 100 Q78 108 70 108 Q64 108 64 116 Q64 124 52 124 Q40 124 36 112 Z', 'e', 'f', 3)
    paints = C(48, 98, 5, 'd', None) + C(60, 92, 5, 'w', None) + C(68, 100, 4, 's', None)
    empty('image_ideas', legs + canvas + board + paints + T(sparkle('w', 8), 186, 40) + T(sparkle('w', 5), 198, 70), 's')


def image_results():
    frames = ''.join(dashed(f'M{x} {y} h46 v40 h-46 Z', 'm', 2.5) for x, y in ((60, 30), (126, 30), (60, 84), (126, 84)))
    hill = P('M64 120 L80 102 L92 112 L102 104 L110 120 Z', 'a', None, o='.6')
    empty('image_results', frames + hill + T(sparkle('w', 8), 194, 44) + T(sparkle('a', 6), 44, 60), 'a')


def import_nothing():
    box = P('M64 80 L120 96 L176 80 L176 128 L120 142 L64 128 Z', 'm') + P('M64 80 L46 62 L102 46 L120 64 Z', 'u') + P('M176 80 L194 62 L138 46 L120 64 Z', 'u')
    page = dashed('M104 76 v-40 h32 l8 8 v32', 'm', 3)
    lens = T(magnifier('u'), 182, 108, 1) + C(182, 108, 6, None, 'm', 2)
    empty('import_nothing', page + box + lens + C(60, 40, 3, 'm', None), 'w')


def credentials_none():
    ring = C(78, 64, 20, None, 'm', 6)
    key_ = C(118, 92, 18, 'w') + C(118, 92, 7, 'e', None) + R(132, 86, 56, 12, 4, 'w', None) + R(166, 98, 8, 14, 2, 'w', None) + R(180, 98, 8, 10, 2, 'w', None)
    link = L('M92 78 L106 84', 'm', 5)
    tag = dashed('M44 96 h36 v26 h-36 Z', 'm', 3) + C(52, 109, 3, 'm', None)
    empty('credentials_none', ring + link + key_ + tag + T(sparkle('a', 8), 192, 48) + T(sparkle('a', 5), 172, 34), 'w')


def subjects_none():
    tabs = ''.join(dashed(f'M{x} 58 v-14 q0 -4 4 -4 h16 q4 0 4 4 v14', 'm', 2.5) for x in (70, 98, 126, 154))
    drawer = R(60, 58, 124, 70, 8, 'u') + R(108, 84, 28, 8, 4, 'm', None)
    empty('subjects_none', tabs + drawer + T(sparkle('w', 7), 196, 46) + C(48, 46, 3, 'm', None), 'a')


def build():
    for make in (inspector_idle, actions_idle, image_ideas, image_results, import_nothing, credentials_none, subjects_none):
        make()
