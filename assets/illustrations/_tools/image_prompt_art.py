"""Image ideas option cards (160x120, sb D3): one small preview per style, mood, palette,
framing and thing to avoid, shown on the chips of the image prompt builder."""
from kit import *
from scenes import dashed


def scene_hill(f='s', x=40, y=88, w=80):
    return P(f'M{x} {y} L{x + w * .35} {y - 34} L{x + w * .55} {y - 18} L{x + w * .7} {y - 28} L{x + w} {y} Z', f, 'f', 2.5)


def styles():
    g = 'image_style'
    card(g, 'illustration', R(36, 26, 88, 64, 8, 'u') + scene_hill('s', 40, 88, 80) + C(104, 44, 8, 'w'), 's')
    blots = C(64, 54, 22, 'i', None, o='.45') + C(92, 50, 20, 'a', None, o='.45') + C(80, 74, 20, 'b', None, o='.45')
    brush = T(R(-3, 0, 6, 30, 3, 'f', None) + P('M-5 0 Q0 -14 5 0 Z', 'a', 'f', 2), 120, 76, 1, 30)
    card(g, 'watercolour', blots + brush, 'i')
    strokes = L('M40 84 Q60 40 80 70 T120 50', 'f', 3) + L('M44 92 h66', 'f', 2, '.6') + L('M50 60 q6 -8 12 0', 'f', 2)
    nib = T(P('M0 -22 L9 0 L0 10 L-9 0 Z', 'f', 'f', 2) + C(0, -2, 2, 'u', None), 118, 40, 1, 35)
    card(g, 'ink', strokes + nib, 'm')
    cube = P('M80 24 L114 42 L80 60 L46 42 Z', 'a') + P('M46 42 L80 60 V98 L46 80 Z', 'u') + P('M114 42 L80 60 V98 L114 80 Z', 'm')
    card(g, 'isometric', cube, 'a')
    cam = R(38, 40, 84, 52, 8, 'u') + R(58, 30, 24, 12, 4, 'm') + C(80, 66, 18, 'i') + C(80, 66, 8, 'u', 'f', 2) + C(110, 50, 3, 'w', None)
    card(g, 'photographic', cam, 'i')
    px_cells = ((1, 0), (2, 0), (4, 0), (5, 0), (0, 1), (1, 1), (2, 1), (3, 1), (4, 1), (5, 1), (6, 1), (1, 2), (2, 2), (3, 2), (4, 2), (5, 2), (2, 3), (3, 3), (4, 3), (3, 4))
    sprite = ''.join(R(52 + x * 8, 32 + y * 8, 8, 8, 0, 'd', None) for x, y in px_cells)
    card(g, 'pixel', sprite + R(52, 80, 56, 8, 0, 'm', None, o='.5'), 'd')
    paper = T(R(-26, -18, 52, 36, 2, 'w'), 66, 52, 1, -8) + T(R(-24, -18, 48, 36, 2, 'a'), 96, 66, 1, 7) + T(C(0, 0, 14, 'u'), 72, 80)
    tape = T(R(-10, -4, 20, 8, 1, 'm', None, o='.6'), 66, 34, 1, -8)
    card(g, 'collage', paper + tape, 'w')
    clay = P('M52 92 Q48 52 80 48 Q112 52 108 92 Z', 'b') + C(80, 40, 16, 'b') + E(74, 34, 5, 3, 'u', None, o='.6') + C(74, 40, 2.5, 'f', None) + C(86, 40, 2.5, 'f', None)
    card(g, 'clay', clay, 'b')


def moods():
    g = 'image_mood'
    water = L('M40 88 h80 M50 96 h60', 'i', 3, '.7')
    card(g, 'calm', T(moon('w'), 80, 48, 1.3) + water + C(46, 36, 2, 'm', None) + C(116, 44, 2, 'm', None), 'i')
    card(g, 'hopeful', T(sun('w', 14, 10, 6), 80, 80) + P('M30 82 H130 V96 H30 Z', 's', None, o='.5') + T(sparkle('w', 6), 118, 34), 'w')
    balls = C(56, 70, 12, 'd') + C(86, 50, 10, 'w') + C(108, 76, 13, 'a') + L('M56 50 q4 -6 8 0 M120 50 q4 -6 8 0', 'f', 2.5)
    card(g, 'playful', balls + T(sparkle('w', 5), 40, 40), 'w')
    eye = P('M40 60 Q80 24 120 60 Q80 96 40 60 Z', 'u') + C(80, 60, 14, 'b') + C(80, 60, 6, 'f', None)
    card(g, 'mysterious', eye + T(sparkle('w', 5), 44, 32) + T(sparkle('w', 4), 120, 92), 'b')
    bolt = P('M86 52 L70 82 H82 L74 104 L98 70 H86 L94 52 Z', 'w', 'f', 2.5)
    card(g, 'dramatic', T(cloud('m', 'f', 3), 80, 44, 1.5) + bolt, 'd')
    rain = ''.join(L(f'M{x} {y} l-3 9', 'i', 3) for x, y in ((62, 72), (78, 80), (94, 72), (110, 82), (70, 92)))
    card(g, 'melancholic', T(cloud('u', 'f', 3), 84, 50, 1.4) + rain, 'i')


def palettes():
    g = 'image_palette'
    sets = {'match_theme': ('a', 'b', 'g'), 'warm': ('d', 'w', 'a'), 'cool': ('i', 'b', 's'), 'monochrome': ('f', 'm', 'u'),
            'pastel': ('a', 'i', 'w'), 'vivid': ('d', 'a', 'w'), 'earthy': ('w', 's', 'm')}
    for key, hues in sets.items():
        o = '.45' if key in ('pastel', 'earthy') else None
        blobs = ''.join(C(56 + i * 24, 60 + (i % 2) * 8, 18, c, 'f', 2.5, o=o) for i, c in enumerate(hues))
        extra = T(sparkle('w', 7), 122, 34) if key == 'vivid' else ''
        if key == 'match_theme':
            extra = T(R(-3, 0, 6, 26, 3, 'f', None) + P('M-5 0 Q0 -12 5 0 Z', 'a', 'f', 2), 120, 36, 1, 35)
        card(g, key, blobs + extra, hues[0] if hues[0] != 'f' else 'a')


def framings():
    g = 'image_framing'
    frame = R(44, 22, 72, 76, 8, 'u')
    bust = P('M56 98 Q56 72 80 72 Q104 72 104 98 Z', 'a') + C(80, 54, 14, 'e', 'f', 3)
    card(g, 'head_shoulders', frame + bust, 'a')
    card(g, 'emblem', T(shield('w'), 80, 60, 2.2) + T(star('u', 9, 4), 80, 56), 'w')
    body, _ = person(80, 100, 'b', (30, 30), sc=.95)
    card(g, 'full_figure', R(50, 14, 60, 92, 8, 'u') + body, 'b')
    wide = R(26, 34, 108, 56, 6, 'u') + scene_hill('s', 30, 88, 100) + T(sun('w', 7, 8, 4), 112, 50)
    card(g, 'hero', wide, 's')
    nodes = L('M50 44 L80 76 L112 46 M80 76 V96', 'm', 2.5) + ''.join(R(x - 10, y - 8, 20, 16, 4, c) for x, y, c in ((50, 44, 'i'), (112, 46, 'i'), (80, 76, 'a')))
    card(g, 'diagram', nodes, 'i')
    shapes = C(64, 52, 20, 'd', None, o='.7') + R(76, 44, 40, 40, 4, 'w', None, o='.7') + P('M50 96 L80 64 L104 96 Z', 'a', None, o='.7')
    card(g, 'abstract', shapes, 'd')


def no_sign(x=112, y=36):
    return C(x, y, 13, 'u', 'd', 3) + L(f'M{x - 9} {y - 9} L{x + 9} {y + 9}', 'd', 3)


def avoids():
    g = 'image_avoid'
    card(g, 'text', R(40, 34, 64, 60, 6, 'u') + lines(50, 50, (44, 34, 44, 24), 9, 'm', 3) + no_sign(), 'd')
    stamp = R(40, 34, 64, 60, 6, 'u') + T(R(-34, -8, 68, 16, 3, 'm', None, o='.45'), 72, 64, 1, -30)
    card(g, 'watermark', stamp + no_sign(), 'd')
    warp = ''.join(L(f'M40 {y} q16 {d} 32 0 t32 0', 'm', 2.5) for y, d in ((46, -8), (62, 8), (78, -8)))
    card(g, 'distortion', warp + no_sign(), 'd')
    bits = ''.join(R(x, y, 10, 10, 2, c, None) for x, y, c in ((40, 48, 'm'), (56, 72, 'w'), (74, 52, 'm'), (60, 90, 'a'), (90, 80, 'm'), (44, 82, 'w'), (84, 64, 'a')))
    card(g, 'clutter', bits + no_sign(), 'd')
    blur = C(70, 66, 24, 'i', None, o='.2') + C(70, 66, 17, 'i', None, o='.3') + C(70, 66, 10, 'i', None, o='.5')
    card(g, 'blur', blur + no_sign(), 'd')


def build():
    styles()
    moods()
    palettes()
    framings()
    avoids()
