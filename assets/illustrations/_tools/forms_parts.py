"""Per-kind finish, markings, accessory and aura layers.

`finish` and `markings` are canvas-wide textures (the composer clips them to the body silhouette).
`accessory` sits on the default face anchors (the composer moves it with the face). `aura` paints
behind the body over the whole canvas.
"""
import math

from kit import *
from forms import ink, white
from forms_bodies_a import rot

FULL = 'M0 0 H256 V256 H0 Z'
SCATTER = ((48, 52), (206, 44), (226, 128), (30, 140), (60, 214), (200, 214), (128, 28), (232, 186), (22, 82))


def pattern(w, h, body):
    """A tiled texture filling the canvas (one `<pattern>`, id from the file key)."""
    return (f'<defs><pattern id="@G" width="{w}" height="{h}" patternUnits="userSpaceOnUse">{body}</pattern></defs>'
            '<rect width="256" height="256" fill="url(#@G)"/>')


def circle_d(x, y, r):
    return f'M{n(x - r)} {n(y)} a{n(r)} {n(r)} 0 1 0 {n(2 * r)} 0 a{n(r)} {n(r)} 0 1 0 {n(-2 * r)} 0'


def glow(c):
    return C(128, 136, 112, c, None, o='.12') + C(128, 136, 92, c, None, o='.14') + C(128, 136, 72, c, None, o='.12')


def sparkles(c, pts=SCATTER[:7]):
    return ''.join(T(sparkle(c, 7 + (i % 3) * 3), x, y) for i, (x, y) in enumerate(pts))


def dots(c, pts, r=4, o=None):
    return ''.join(C(x, y, r + (i % 3), c, None, o=o) for i, (x, y) in enumerate(pts))


def gloss():
    return white(P('M30 20 H86 L20 150 V96 Z', None, None), '.26') + white(P('M100 20 H118 L20 214 V180 Z', None, None), '.16')


def rust(cs=('w', 'd')):
    a, b = cs
    return (E(84, 210, 30, 16, a, None, o='.55') + E(176, 200, 24, 16, b, None, o='.4') + C(84, 84, 14, a, None, o='.5') +
            C(172, 74, 10, a, None, o='.45') + C(150, 46, 6, b, None, o='.45') + dots(a, ((104, 190), (152, 184), (80, 150), (178, 150)), 3, '.6'))


def glasses(x1=110, x2=146, y=124, r=17):
    return (C(x1, y, r, 'u', 'f', 3, o='.25') + C(x2, y, r, 'u', 'f', 3, o='.25') +
            L(f'M{x1 + r} {y - 2} q{(x2 - x1 - 2 * r) / 2} -4 {x2 - x1 - 2 * r} 0 M{x1 - r} {y - 4} L{x1 - r - 14} {y - 8} M{x2 + r} {y - 4} L{x2 + r + 14} {y - 8}', 'f', 3))


def bow(x, y, c, s=1):
    return T(P('M0 0 L-22 -12 V12 Z M0 0 L22 -12 V12 Z', c, 'f', 2.5) + C(0, 0, 5, c, 'f', 2), x, y, s)


def scarf(c, stripe='u'):
    return (P('M84 168 Q128 192 172 168 L176 184 Q128 210 80 184 Z', c) + P('M150 186 L162 230 L142 232 L136 192 Z', c) +
            L('M100 182 l-2 10 M128 190 v10 M156 182 l2 10', stripe, 3, '.6'))


def crown(c, y=100):
    return P(f'M100 {y} L96 {y - 26} L112 {y - 12} L128 {y - 32} L144 {y - 12} L160 {y - 26} L156 {y} Z', c) + C(128, y - 10, 4, 'd', None)


def leaf_at(x, y, a, c='s', s=1):
    return T(P('M-14 0 Q0 -10 14 0 Q0 10 -14 0 Z', c, 'f', 2) + L('M-10 0 H10', 'f', 1.5, '.4'), x, y, s, a)


def arcs(c, w, o, cx=128, cy=136, rs=(40, 62, 84, 106)):
    return ''.join(L(circle_d(cx, cy, r), c, w, o) for r in rs)


def flower(x, y, r, c, mid='w'):
    """Five petals around a centre (petals `c`, centre `mid`)."""
    petals = ''.join(C(x + r * math.cos(math.radians(a)), y + r * math.sin(math.radians(a)), r * .7, c, 'f', 1.5) for a in range(-90, 270, 72))
    return petals + C(x, y, r * .55, mid, 'f', 1.5)


def rays(c, o, k=12):
    return ''.join(P(f'M128 136 L{n(128 + 190 * math.cos(math.radians(a)))} {n(136 + 190 * math.sin(math.radians(a)))} '
                     f'L{n(128 + 190 * math.cos(math.radians(a + 360 / k / 2)))} {n(136 + 190 * math.sin(math.radians(a + 360 / k / 2)))} Z', c, None, o=o)
                   for a in range(0, 360, 360 // k))


def object_parts(W):
    k = 'object'
    W(k, 'finish', 'gloss', gloss() + white(C(182, 82, 6, None, None), '.6') + white(C(192, 98, 3, None, None), '.6'))
    W(k, 'finish', 'rust', rust())
    W(k, 'finish', 'worn', L('M60 70 l30 12 M170 60 l-24 18 M70 190 l26 -10 M180 180 l-20 -16 M150 214 l18 4 M100 40 l14 6', '#fff', 2.5, '.45') +
      E(84, 212, 22, 8, '#000', None, o='.12') + E(196, 60, 14, 8, '#000', None, o='.12'))
    W(k, 'finish', 'patina', E(88, 206, 30, 18, 's', None, o='.45') + C(176, 86, 18, 's', None, o='.4') + C(84, 78, 12, 'i', None, o='.35') +
      E(170, 198, 20, 12, 'i', None, o='.35') + dots('s', ((100, 52), (150, 222), (220, 140)), 4, '.4'))
    W(k, 'markings', 'label', R(76, 178, 104, 30, 6, 'Q', 'f', 2) + L('M92 190 h48 M92 198 h30', 'f', 2.5, '.4') + C(162, 193, 6, 'u', 'f', 2))
    W(k, 'markings', 'stripes', pattern(32, 256, R(0, 0, 12, 256, 0, 'Q', None, o='.4')))
    W(k, 'markings', 'polka', pattern(36, 36, C(9, 9, 6, 'Q', None, o='.6') + C(27, 27, 6, 'Q', None, o='.6')))
    W(k, 'markings', 'stickers', T(star('w'), 92, 88, .8) + T(heart('d', 'f', 2), 164, 194, .7) + T(sparkle('a', 9), 166, 86) + C(94, 194, 8, 'w', 'f', 2))
    W(k, 'accessory', 'monocle', C(146, 124, 17, 'u', 'w', 3.5, o='.25') + L('M160 136 Q174 168 158 198', 'w', 2))
    W(k, 'accessory', 'bow_tie', bow(128, 180, 'd'))
    W(k, 'accessory', 'glasses', glasses())
    W(k, 'accessory', 'mustache', P('M128 144 Q112 134 98 142 Q88 148 82 140 Q84 158 104 153 Q118 150 128 146 Q138 150 152 153 Q172 158 174 140 '
                                    'Q168 148 158 142 Q144 134 128 144 Z', 'f', None))
    W(k, 'aura', 'glow', glow('a'))
    W(k, 'aura', 'sparkles', sparkles('w'))
    W(k, 'aura', 'steam', L('M104 52 q-10 -10 0 -20 q10 -10 0 -22 M128 44 q-10 -10 0 -20 q10 -10 0 -20 M152 52 q-10 -10 0 -20 q10 -10 0 -22', 'm', 5, '.6'))
    W(k, 'aura', 'shine', rays('w', '.22'))


def plant_parts(W):
    k = 'plant'
    drops = ''.join(white(E(x, y, 4, 6, None, 'f', 1.5), '.6') for x, y in ((70, 70), (180, 90), (96, 190), (170, 180), (60, 140), (200, 140), (128, 60), (120, 214)))
    W(k, 'finish', 'dewy', drops + white(P('M40 60 Q60 30 100 26 Q64 46 52 76 Z', None, None), '.25'))
    W(k, 'finish', 'autumn', P(FULL, 'w', None, o='.3') + dots('d', ((70, 80), (180, 70), (60, 180), (196, 186), (128, 40)), 9, '.3'))
    W(k, 'finish', 'frost', white(P('M0 0 H256 V84 Q192 100 128 84 Q64 70 0 90 Z', None, None), '.35') + sparkles('u', ((60, 60), (196, 56), (128, 30), (40, 150), (216, 160))))
    W(k, 'finish', 'sunlit', C(64, 54, 110, 'w', None, o='.2') + white(C(56, 46, 60, None, None), '.15'))
    W(k, 'markings', 'veins', L('M128 236 V20 M128 200 L70 150 M128 200 L186 150 M128 150 L60 96 M128 150 L196 96 M128 100 L84 50 M128 100 L172 50', 'f', 2.5, '.25'))
    W(k, 'markings', 'spots', pattern(44, 44, C(10, 12, 6, 'Q', None, o='.5') + C(32, 34, 4, 'Q', None, o='.5')))
    W(k, 'markings', 'rings', arcs('f', 2.5, '.2'))
    W(k, 'markings', 'blossoms', flower(94, 86, 8, 'd') + flower(166, 92, 7, 'w', 'd') + flower(86, 188, 7, 'w', 'd') + flower(170, 184, 8, 'd') + flower(128, 210, 6, 'd'))
    W(k, 'accessory', 'ladybug', E(164, 152, 12, 10, 'd', 'f', 2) + C(164, 141, 5, 'f', None) + L('M164 143 V162', 'f', 1.5) +
      ink(C(159, 150, 2.2, None, None)) + ink(C(169, 155, 2.2, None, None)))
    W(k, 'accessory', 'flower', L('M84 104 L96 116', 's', 3) + flower(80, 98, 10, 'd'))
    wing = P('M0 0 Q-24 -24 -26 -6 Q-26 8 0 0 Z M0 0 Q-20 8 -16 22 Q-4 22 0 0 Z', 'b', 'f', 2)
    W(k, 'accessory', 'butterfly', T(wing + MX(wing, 0) + L('M0 -10 V12', 'f', 3) + L('M0 -10 l-6 -8 M0 -10 l6 -8', 'f', 1.5), 180, 92))
    W(k, 'accessory', 'ribbon', bow(164, 96, 'a', .9))
    W(k, 'aura', 'pollen', dots('w', SCATTER, 3, '.7') + dots('w', ((80, 40), (176, 30), (240, 60), (16, 200)), 2, '.6'))
    W(k, 'aura', 'fireflies', ''.join(C(x, y, 9, 'w', None, o='.25') + C(x, y, 3, 'w', None) for x, y in SCATTER[:8]))
    W(k, 'aura', 'sunbeam', P('M0 0 H60 L160 256 H90 Z', 'w', None, o='.16') + P('M84 0 H120 L220 256 H186 Z', 'w', None, o='.12'))
    W(k, 'aura', 'falling_leaves', ''.join(leaf_at(x, y, a, c, .8) for x, y, a, c in
                                          ((40, 50, 30, 'w'), (214, 40, -20, 's'), (230, 140, 60, 'd'), (24, 160, -40, 's'), (60, 226, 10, 'w'), (206, 222, 80, 'd'))))


def element_parts(W):
    k = 'element'
    W(k, 'finish', 'shimmer', L('M60 60 l20 -20 M180 70 l14 -14 M70 180 l18 -18 M190 190 l12 -12 M120 40 l10 -10', '#fff', 4, '.4') + sparkles('u', SCATTER[:4]))
    W(k, 'finish', 'radiant', white(C(128, 140, 80, None, None), '.16') + white(C(128, 140, 48, None, None), '.18'))
    W(k, 'finish', 'turbulent', L('M50 80 q20 -24 40 0 q20 24 40 0 q20 -24 40 0 q20 24 40 0 M30 190 q20 -24 40 0 q20 24 40 0 q20 -24 40 0 q20 24 40 0 q20 -24 40 0', 'f', 3, '.2'))
    facets = 'M128 20 L60 100 L128 136 L196 100 Z M60 100 L40 200 L128 136 M196 100 L216 200 L128 136 M40 200 L128 236 L216 200'
    W(k, 'finish', 'crystalline', L(facets, '#fff', 2.5, '.35') + white(P('M128 20 L60 100 L128 136 Z', None, None), '.12'))
    W(k, 'markings', 'ripples', arcs('#fff', 3, '.35', 128, 210, (24, 48, 72, 96)))
    W(k, 'markings', 'sparks', pattern(52, 52, T(sparkle('w', 7), 14, 14) + T(sparkle('w', 4), 40, 38)))
    W(k, 'markings', 'cracks', L('M70 96 l20 8 l-8 14 l18 8 M186 92 l-18 12 l10 10 l-14 10 M86 206 l12 -18 l16 4 l8 -14 M180 200 l-8 -18 l-16 2', 'f', 2.5, '.45'))
    W(k, 'markings', 'swirls', pattern(56, 56, L('M28 28 m-5 0 a5 5 0 1 1 10 0 a10 10 0 1 1 -20 0 a15 15 0 1 1 30 0', 'Q', 3, '.5')))
    W(k, 'accessory', 'crown', crown('w'))
    W(k, 'accessory', 'scarf', scarf('d'))
    lens = lambda x: ink(P(f'M{x - 17} 116 H{x + 17} Q{x + 17} 136 {x} 136 Q{x - 17} 136 {x - 17} 116 Z', None, None))
    W(k, 'accessory', 'shades', lens(110) + lens(146) + L('M93 116 H163 M93 116 L80 112 M163 116 L176 112', 'f', 3))
    W(k, 'accessory', 'leaf', leaf_at(92, 94, -30, 's', 1.2) + L('M104 100 l8 4', 's', 2))
    W(k, 'aura', 'glow', glow('i'))
    W(k, 'aura', 'embers', dots('d', SCATTER, 3, '.8') + dots('w', ((90, 40), (170, 30), (240, 80), (14, 110)), 2))
    W(k, 'aura', 'swirl', L('M128 136 m-100 0 a100 92 0 1 1 200 0 M128 136 m-84 30 a90 80 0 0 0 168 0', 'i', 5, '.35'))
    W(k, 'aura', 'rainbow', ''.join(L(f'M{128 - r} 200 a{r} {r} 0 0 1 {2 * r} 0', c, 12, '.45') for r, c in ((112, 'd'), (98, 'w'), (84, 'i'))))


def creature_parts(W):
    k = 'creature'
    tufts = ''.join(L(f'M{x} {y} q4 -8 8 0 q4 -8 8 0', 'f', 2, '.25') for x, y in ((60, 70), (180, 66), (50, 160), (190, 170), (90, 210), (150, 216), (120, 40)))
    W(k, 'finish', 'fluffy', tufts)
    W(k, 'finish', 'sleek', white(P('M56 60 Q100 30 160 40 Q110 46 74 84 Z', None, None), '.3') + white(P('M200 120 Q206 160 186 196 Q196 160 192 126 Z', None, None), '.2'))
    W(k, 'finish', 'scaly', pattern(24, 16, L('M0 16 a12 12 0 0 1 24 0 M-12 8 a12 12 0 0 1 24 0 M12 8 a12 12 0 0 1 24 0', 'f', 1.5, '.22')))
    W(k, 'finish', 'feathery', pattern(28, 22, L('M2 4 l12 12 l12 -12', '#fff', 2, '.25')))
    stripe = lambda y, d: P(f'M{0 if d > 0 else 256} {y} L{n(128 - d * 64)} {y + 8} L{0 if d > 0 else 256} {y + 18} Z', 'Q', None, o='.6')
    W(k, 'markings', 'stripes', ''.join(stripe(y, d) for y in (60, 100, 170, 206) for d in (1, -1)))
    W(k, 'markings', 'spots', dots('Q', ((70, 80), (190, 74), (52, 160), (204, 166), (96, 206), (160, 210), (100, 52), (40, 116), (220, 120)), 6, '.7'))
    W(k, 'markings', 'patches', P('M20 40 Q90 30 96 90 Q90 140 20 150 Z', 'Q', None, o='.75') + P('M236 150 Q180 160 176 210 Q184 240 236 240 Z', 'Q', None, o='.75'))
    W(k, 'markings', 'belly', white(E(128, 198, 48, 40, None, None), '.35'))
    W(k, 'accessory', 'collar', P('M86 176 Q128 194 170 176 L172 188 Q128 208 84 188 Z', 'd', 'f', 2.5) + C(128, 204, 8, 'w', 'f', 2))
    W(k, 'accessory', 'bow', bow(90, 88, 'd', .9))
    W(k, 'accessory', 'glasses', glasses(110, 146, 124, 15))
    W(k, 'accessory', 'scarf', scarf('a', 'w'))
    W(k, 'aura', 'hearts', ''.join(T(heart('d', None), x, y, .7 + (i % 2) * .3) for i, (x, y) in enumerate(SCATTER[:6])))
    note = lambda x, y: E(x, y, 7, 5, 'a', None) + L(f'M{x + 6} {y} V{y - 24} q8 4 10 12', 'a', 3)
    W(k, 'aura', 'notes', note(44, 70) + note(206, 54) + note(222, 170) + note(30, 180))
    W(k, 'aura', 'sparkles', sparkles('w'))
    W(k, 'aura', 'glow', glow('s'))


def machine_parts(W):
    k = 'machine'
    W(k, 'finish', 'chrome', gloss() + P('M0 170 H256 V196 H0 Z', '#000', None, o='.1'))
    W(k, 'finish', 'rust', rust(('w', 'm')))
    W(k, 'finish', 'matte', P(FULL, '#000', None, o='.18'))
    trace = 'M30 60 H80 V100 H110 M226 70 H180 V120 H150 M30 200 H90 V170 M226 196 H170 V160'
    W(k, 'finish', 'neon', L(trace, 'a', 8, '.2') + L(trace, 'a', 3) + dots('a', ((110, 100), (150, 120), (90, 170), (170, 160)), 3))
    W(k, 'markings', 'hazard', '<g transform="translate(0 196)">' + pattern(24, 24, P('M0 24 L12 0 H24 L12 24 Z', '#000', None, o='.55')).replace('256" height="256', '256" height="40') + '</g>'
      + P('M0 196 H256 V236 H0 Z', 'w', None, o='.45'))
    W(k, 'markings', 'panels', L('M20 96 H236 M20 180 H236 M80 20 V96 M176 180 V236', 'f', 2, '.4') + dots('f', ((30, 106), (226, 106), (30, 170), (226, 170)), 2, '.5'))
    W(k, 'markings', 'rivets', pattern(20, 20, C(10, 10, 2.5, 'f', None, o='.35')))
    bars = ''.join(R(x, 196, w, 28, 0, 'f', None, o='.6') for x, w in ((150, 3), (156, 2), (161, 5), (169, 2), (174, 3), (180, 6), (189, 2), (194, 4), (201, 2)))
    W(k, 'markings', 'barcode', R(144, 190, 64, 40, 3, 'u', None, o='.75') + bars)
    W(k, 'accessory', 'headset', L('M78 128 Q76 64 128 62 Q180 64 178 128', 'f', 6) + R(68, 112, 18, 32, 8, 'a', 'f', 2.5) + R(170, 112, 18, 32, 8, 'a', 'f', 2.5) +
      L('M78 142 Q84 166 112 158', 'f', 3) + C(114, 158, 4, 'a', 'f', 2))
    W(k, 'accessory', 'visor', R(88, 110, 80, 28, 14, 'i', 'f', 3, o='.5') + white(R(96, 116, 26, 6, 3, None, None), '.6'))
    W(k, 'accessory', 'tie', P('M120 170 H136 L132 180 L140 214 L128 226 L116 214 L124 180 Z', 'd', 'f', 2.5))
    W(k, 'accessory', 'gear', T(gear('m', 13), 178, 86))
    W(k, 'aura', 'signal', ''.join(L(f'M{128 - r} {136 - r * .6} q-{r * .5} {r * .6} 0 {r * 1.2} M{128 + r} {136 - r * .6} q{r * .5} {r * .6} 0 {r * 1.2}', 'i', 5, '.5')
                                   for r in (84, 100, 116)))
    bolt = lambda x, y: T(P('M2 -14 L-6 2 H0 L-2 14 L8 -4 H2 Z', 'w', 'f', 1.5), x, y)
    W(k, 'aura', 'sparks', bolt(40, 60) + bolt(214, 50) + bolt(226, 170) + bolt(30, 190))
    W(k, 'aura', 'data', ''.join(L(f'M{x} 20 v6 m0 10 v2 m0 10 v8 m0 10 v2 m0 14 v6', 's', 4, '.55') for x in (20, 40, 216, 236)) +
      ''.join(L(f'M{x} 180 v6 m0 10 v2 m0 10 v8', 's', 4, '.55') for x in (24, 228)))
    W(k, 'aura', 'glow', glow('a'))


def place_parts(W):
    k = 'place'
    W(k, 'finish', 'dusk', P(FULL, 'd', None, o='.16') + P('M0 150 H256 V256 H0 Z', 'w', None, o='.14'))
    W(k, 'finish', 'night', P(FULL, '#000', None, o='.3') + dots('w', ((70, 90), (100, 150), (166, 120), (190, 180), (60, 190), (140, 210)), 2, '.8'))
    W(k, 'finish', 'snow', white(P('M0 0 H256 V80 Q224 96 192 82 Q160 70 128 86 Q96 100 64 84 Q32 72 0 88 Z', None, None), '.7') +
      ''.join(white(C(x, y, 2.5, None, None), '.8') for x, y in SCATTER))
    W(k, 'finish', 'overgrown', P('M0 256 V196 Q30 170 52 196 Q70 150 100 190 Q120 176 140 196 Q170 160 196 194 Q224 170 256 190 V256 Z', 's', None, o='.5') +
      L('M30 196 Q24 150 40 120 M226 192 Q236 150 220 116', 's', 3, '.6'))
    W(k, 'markings', 'windows', pattern(28, 34, R(9, 9, 10, 14, 2, 'w', None, o='.55')))
    banner = lambda x, c: P(f'M{x} 50 h20 v50 l-10 -9 l-10 9 Z', c, 'f', 2)
    W(k, 'markings', 'banners', banner(88, 'd') + banner(148, 'a') + L('M60 50 H196', 'f', 3))
    W(k, 'markings', 'lanterns', L('M16 70 Q128 120 240 70', 'f', 1.5, '.5') + ''.join(R(x - 6, y, 12, 16, 5, 'w', 'f', 1.5) for x, y in ((48, 82), (88, 96), (128, 98), (168, 96), (208, 82))))
    W(k, 'markings', 'mosaic', pattern(24, 24, P('M12 0 L24 12 L12 24 L0 12 Z', 'a', None, o='.18') + C(12, 12, 3, 'Q', None, o='.4')))
    W(k, 'accessory', 'flag', L('M188 104 V40', 'f', 3) + P('M188 42 L224 52 L188 64 Z', 'd', 'f', 2))
    W(k, 'accessory', 'balloon', C(62, 62, 26, 'a', 'f', 2.5) + L('M62 36 V88 M44 76 L56 98 M80 76 L68 98', 'f', 1.5, '.5') + R(54, 96, 16, 12, 3, 'w', 'f', 2))
    W(k, 'accessory', 'birds', L('M50 60 q8 -8 14 0 q6 -8 14 0 M176 42 q6 -6 12 0 q6 -6 12 0 M200 80 q6 -6 10 0 q4 -6 10 0', 'f', 3))
    W(k, 'accessory', 'kite', P('M200 30 L222 56 L200 92 L178 56 Z', 'w', 'f', 2.5) + L('M200 30 V92 M178 56 H222', 'f', 1.5) +
      L('M200 92 q-10 14 0 24 q10 10 -4 22', 'f', 2) + bow(198, 104, 'd', .35))
    W(k, 'aura', 'clouds', T(cloud('u', None), 52, 52, 1.1) + T(cloud('u', None), 206, 40, .9) + T(cloud('u', None), 222, 150, .7))
    W(k, 'aura', 'stars', ''.join(T(star('w', 7, 3, None), x, y) for x, y in SCATTER[:7]))
    W(k, 'aura', 'sunrise', rays('w', '.18', 16) + C(128, 136, 104, 'w', None, o='.3'))
    W(k, 'aura', 'fog', ''.join(R(-20 + dx, y, 220, 18, 9, 'u', None, o='.45') for dx, y in ((0, 176), (60, 202), (20, 226))))


def abstraction_parts(W):
    k = 'abstraction'
    W(k, 'finish', 'ethereal', white(P(FULL, None, None), '.18') + L('M40 220 Q80 140 60 40 M200 220 Q170 150 196 40', '#fff', 6, '.25'))
    W(k, 'finish', 'marble', L('M20 60 Q80 80 100 140 Q120 190 200 210 M120 20 Q150 70 230 90 M30 170 Q60 160 80 200', 'f', 2, '.22') +
      L('M40 100 Q100 110 120 160', '#fff', 3, '.3'))
    W(k, 'finish', 'ink', P(FULL, '#000', None, o='.2') + L('M60 200 V236 M90 210 V244 M170 206 V240 M196 214 V232', 'f', 6, '.35'))
    W(k, 'finish', 'gilded', P(FULL, 'w', None, o='.3') + gloss())
    glyphs = pattern(48, 48, P(poly([(12, 6), (22, 22), (2, 22)]), None, 'f', 2) + L(circle_d(36, 34, 7) + ' M36 25 V43', 'f', 2))
    W(k, 'markings', 'runes', glyphs.replace('stroke="{{fg}}"', 'stroke="{{fg}}" stroke-opacity=".35"'))
    sky = L('M8 10 L30 22 L50 12 M30 22 L26 50', '#fff', 1.5, '.5') + white(C(8, 10, 2.5, None, None), '.9') + white(C(30, 22, 3, None, None), '.9') + white(C(50, 12, 2, None, None), '.9') + white(C(26, 50, 2.5, None, None), '.9')
    W(k, 'markings', 'constellation', pattern(60, 60, sky))
    W(k, 'markings', 'spirals', pattern(44, 44, L('M22 22 m-4 0 a4 4 0 1 1 8 0 a8 8 0 1 1 -16 0 a12 12 0 1 1 24 0', 'Q', 2.5, '.6')))
    tri = lambda x, y, s: f'M{x} {y - s} L{x + s} {y + s * .8} L{x - s} {y + s * .8} Z M{x} {y + s * .8} L{x - s / 2} {y - s * .1} L{x + s / 2} {y - s * .1} Z'
    W(k, 'markings', 'fractal', pattern(48, 44, L(tri(24, 22, 16), 'f', 1.5, '.3')))
    W(k, 'accessory', 'crown', crown('w', 98) + C(104, 74, 3, 'u', None) + C(152, 74, 3, 'u', None))
    laurel = ''.join(T(P('M-9 0 Q0 -7 9 0 Q0 7 -9 0 Z', 's', 'f', 2), 128 + 54 * math.cos(math.radians(d)), 120 + 50 * math.sin(math.radians(d)), 1, d + 90)
                     for d in range(160, 381, 22) if not 240 < d < 300)
    W(k, 'accessory', 'laurel', laurel)
    half = lambda x: P(f'M{x - 15} 126 H{x + 15} Q{x + 15} 140 {x} 140 Q{x - 15} 140 {x - 15} 126 Z', 'u', 'f', 3, o='.3')
    W(k, 'accessory', 'glasses', half(110) + half(146) + L('M125 128 q3 -3 6 0 M95 128 L80 120 M161 128 L176 120', 'f', 3))
    W(k, 'accessory', 'quill', T(P('M0 0 Q30 -40 60 -64 Q40 -24 6 6 Z', 'u', 'f', 2) + L('M0 0 L42 -42', 'f', 1.5, '.5') + L('M0 0 l-8 8', 'f', 3), 158, 110))
    W(k, 'aura', 'orbit', rot(E(128, 136, 118, 40, None, 'a', 3), -20, 128, 136) + C(236, 104, 7, 'a', 'f', 2) + C(28, 170, 5, 'b', 'f', 2))
    W(k, 'aura', 'halo', E(128, 34, 50, 12, None, 'w', 6) + E(128, 34, 50, 12, None, 'u', 2))
    W(k, 'aura', 'sparkles', sparkles('b'))
    W(k, 'aura', 'ripple', arcs('b', 4, '.25', 128, 136, (96, 112, 128)))


def build(W):
    for fn in (object_parts, plant_parts, element_parts, creature_parts, machine_parts, place_parts, abstraction_parts):
        fn(W)
