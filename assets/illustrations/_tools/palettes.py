"""Palette art: one 200x240 portrait per full palette (its form body, face and accessory from `forms/**`,
recoloured with theme hues, plus a few props), one 160x120 card per step palette and the gallery's
category icons. Thinker palettes reuse the class art. Run after `forms` (it reads the form parts)."""
import json
import os
import re

from kit import *
import palette_cards

FORMS = os.path.join(ROOT, 'forms')
PALETTES = os.path.join(os.path.dirname(ROOT), 'palettes')
FACE_LAYERS = ('mouth', 'eyes', 'accessory')
FACE_CENTER = (128, 136)
# 256-unit form canvas -> 200x240 frame
FORM_SCALE, FORM_X, FORM_Y = .72, 8, 30


def inner(path):
    text = open(path, encoding='utf-8').read().strip()
    body = re.sub(r'^<svg[^>]*>', '', text)[:-len('</svg>')]
    body = re.sub(r'<defs><clipPath[^<]*(<use[^>]*/>)*</clipPath></defs>', '', body)
    return re.sub(r' id="fs\d+"', '', body)


def anchor_matrix(catalog, form):
    dx, dy, sc = catalog['forms'].get(form, {}).get('anchor', [0, 0, 1])
    cx, cy = FACE_CENTER
    return f'matrix({n(sc)} 0 0 {n(sc)} {n(cx * (1 - sc) + dx)} {n(cy * (1 - sc) + dy)})'


def form_art(spec, primary, secondary):
    kind = spec['kind']
    catalog = json.load(open(os.path.join(FORMS, kind, 'catalog.json'), encoding='utf-8'))
    parts = spec['parts']
    out = inner(os.path.join(FORMS, kind, 'body', parts['body'] + '.svg'))
    face = ''
    for layer in FACE_LAYERS:
        variant = parts.get(layer, catalog['layers'][layer]['variants'][0])
        if variant == 'none':
            continue
        folder = catalog['layers'][layer].get('dir', kind)
        face += inner(os.path.join(FORMS, folder, layer, variant + '.svg'))
    out += f'<g transform="{anchor_matrix(catalog, parts["body"])}">{face}</g>'
    out = out.replace('{{primary}}', '{{%s}}' % TOK[primary]).replace('{{secondary}}', '{{%s}}' % TOK[secondary])
    return f'<g transform="translate({FORM_X} {FORM_Y}) scale({FORM_SCALE})">{out}</g>'


def frame(hue):
    return R(8, 8, 184, 224, 28, 'e', None) + C(100, 112, 76, hue, None, o='.14')


# key -> (primary hue, secondary hue, props drawn over the frame)
def drop(x, y, s=1, f='i'):
    return T(P('M0 -9 Q7 0 7 4 A7 7 0 0 1 -7 4 Q-7 0 0 -9 Z', f, 'f', 2), x, y, s)


def dots(x, y, k=3, c='f', gap=8):
    return ''.join(C(x + i * gap, y, 2.4, c, None) for i in range(k))


def die(f='u', pips=(0, 1, 2)):
    spots = [(-7, -7), (0, 0), (7, 7), (7, -7), (-7, 7)]
    return R(-13, -13, 26, 26, 6, f, 'f', 2.5) + ''.join(C(spots[i][0], spots[i][1], 2.6, 'f', None) for i in pips)


def sheep():
    return cloud('u', 'f', 2.5) + C(-24, 2, 6, 'm', 'f', 2) + L('M-10 8 v6 M8 8 v6', 'f', 2.5)


def fish(f='i'):
    return (P('M16 0 L30 -11 L28 0 L30 11 Z', f) + E(0, 0, 20, 12, f) + C(-10, -3, 2.2, 'f', None))


def qmark_s(c='w'):
    return L('M-8 -12 Q-8 -22 0 -22 Q8 -22 8 -14 Q8 -8 0 -4 V2', c, 4) + C(0, 10, 2.8, c, None)


PROPS = {
    'melancholy_tin_can': ('i', 'b', drop(150, 70) + drop(162, 92, .7) + L('M34 60 q8 -6 14 2 q-8 6 -14 -2 Z', 'm', 2)),
    'gossiping_teacup': ('d', 'a', bubble(128, 24, 56, 30, 'u', 'l') + dots(144, 39, 3, 'f', 9) + T(heart('d'), 40, 46, .7)),
    'lamp_of_insomnia': ('w', 'b', T(moon('w'), 158, 44, 1.2) + T(star('w', 6, 2.5), 40, 40) + T(star('w', 4, 2), 58, 26) + E(150, 96, 6, 3, 'm', 'f', 2)),
    'stopped_clock': ('w', 'b', L('M162 44 l-6 10 M150 40 l2 12 M40 46 l8 8', 'm', 2.5) + T(sparkle('w', 6), 40, 72)),
    'ancient_oak': ('s', 'w', T(sun('w', 11, 8, 5), 160, 42) + T(leaf('s'), 40, 190, .8) + T(leaf('w'), 160, 196, .7, 30)),
    'restless_river': ('i', 's', T(fish('w'), 158, 200, .7) + L('M24 206 q16 -8 32 0 t32 0', 'i', 3) + L('M120 214 q14 -6 28 0', 'i', 3)),
    'moss_collective': ('s', 'i', drop(44, 44, 1, 'i') + drop(64, 30, .7, 'i') + drop(150, 40, .9, 'i') + drop(164, 66, .6, 'i')),
    'wandering_cloud': ('i', 'g', T(sun('w', 12, 8, 5), 156, 46) + drop(64, 206, .7) + drop(88, 212, .6) + drop(112, 206, .7)),
    'trickster_crow': ('g', 'w', T(R(-9, -3, 18, 6, 3, 'w', 'f', 2) + C(-11, 0, 6, None, 'w', 3), 158, 196) + C(42, 196, 8, 'w', 'f', 2) + T(sparkle('w', 6), 160, 44)),
    'octopus_polymath': ('d', 's', T(book('d'), 156, 200, .8) + T(qmark_s('s'), 44, 46)),
    'snail_stoic': ('w', 's', T(leaf('s'), 156, 58, 1.1) + L('M30 214 q40 -6 70 0', 'm', 2.5)),
    'lonely_satellite': ('i', 'w', P('M20 232 Q100 168 180 232 Z', 'i', None, o='.5') + T(star('w', 6, 2.5), 40, 40) + T(star('w', 4, 2), 160, 30) + T(star('w', 5, 2), 166, 80)),
    'pedantic_calculator': ('i', 's', L('M36 40 h16 M44 32 v16 M150 40 h16 M150 48 h16', 'i', 3.5) + L('M36 200 l12 12 M48 200 l-12 12', 's', 3)),
    'server_that_dreams': ('g', 's', C(146, 54, 4, 'u', 'f', 2) + C(156, 40, 6, 'u', 'f', 2) + T(sheep(), 158, 20, .7) + T(star('w', 5, 2), 40, 40)),
    'romantic_typewriter': ('d', 'w', T(R(-22, -26, 44, 52, 4, 'u', 'f', 2.5) + lines(-14, -14, (28, 22, 28), 8), 158, 54, .8, 8) + T(heart('d'), 40, 46, .8)),
    'library_that_remembers': ('w', 'a', T(book('a'), 44, 48, .8) + T(book('w'), 158, 200, .7) + T(star('w', 5, 2), 160, 44)),
    'ruined_cathedral': ('b', 'w', L('M60 22 L86 70 M100 16 L100 64 M140 22 L114 70', 'w', 3, '.6') + L('M30 70 q6 -6 12 0 q6 -6 12 0', 'f', 2.5)),
    'harbor_gossip': ('d', 'i', L('M138 40 q6 -6 12 0 q6 -6 12 0', 'f', 2.5) + L('M40 52 q5 -5 10 0 q5 -5 10 0', 'f', 2.5) + bubble(26, 186, 40, 24, 'u', 'r') + dots(36, 198, 3, 'f', 9)),
    'time_itself': ('w', 'g', T(clock('u', 'f', 'w', .7, 16), 160, 196) + T(star('w', 5, 2), 40, 40) + T(sparkle('w', 6), 160, 44) + T(moon('w'), 40, 196, .8)),
    'entropy': ('d', 's', R(30, 36, 10, 10, 2, 'd', 'f', 2) + T(R(-5, -5, 10, 10, 2, 's', 'f', 2), 160, 44, 1, 30) + T(R(-4, -4, 8, 8, 2, 'd', 'f', 2), 150, 200, 1, 50) + C(44, 200, 4, 's', 'f', 2)),
    'silence': ('i', 'g', T(sparkle('u', 7), 44, 44) + T(sparkle('u', 5), 160, 36) + T(sparkle('u', 6), 156, 196) + T(sparkle('u', 4), 40, 200)),
    'chance': ('d', 'w', T(die('u', (0, 1, 2)), 156, 196, 1, 14) + T(die('u', (0, 2, 3, 4)), 44, 48, .8, -12)),
}


def full_art():
    for category in sorted(os.listdir(PALETTES)):
        if category in ('steps', 'thinkers'):
            continue
        for name in sorted(os.listdir(os.path.join(PALETTES, category))):
            key = name[:-5]
            cfg = json.load(open(os.path.join(PALETTES, category, name), encoding='utf-8'))['config']
            primary, secondary, props = PROPS[key]
            body = frame(primary) + form_art(cfg['avatar'], primary, secondary) + props
            write(f'palettes/{key}.svg', 200, 240, body, None, 'wide')


def build():
    full_art()
    palette_cards.build()
