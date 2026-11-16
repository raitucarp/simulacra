"""Step banners (320x120) and empty states (240x160)."""
from kit import *
from cards_world import owl
from cards_mind import qmark


def banner(step, body, hue='a', grad=None, sx=160, sw=90):
    frame = R(4, 4, 312, 112, 22, 'e', None) + C(160, 60, 52, hue, None, o='.14') + E(sx, 106, sw, 6, 'f', None, o='.08')
    write(f'steps/{step}.svg', 320, 120, frame + body, grad, 'wide')


def dashed(d, c='m', w=2.5):
    return f'<path d="{d}" stroke="{col(c)}" stroke-width="{w}" stroke-dasharray="5 6"/>'


def banners():
    hills = P('M4 94 Q80 70 160 88 Q240 68 316 90 L316 98 Q316 116 296 116 L24 116 Q4 116 4 98 Z', 's', None, o='.3')
    egg = P('M134 74 l6 -8 l7 8 l6 -8 l7 8 l7 -8 l6 8 l7 -8 l6 8 Q186 104 160 104 Q134 104 134 74 Z', 'u')
    sprout = L('M160 70 Q158 52 160 38', 's', 3) + T(leaf('s'), 148, 44, 1, 200) + T(leaf('s'), 172, 36, 1, -20)
    post = L('M74 104 V48', 'f', 3) + P('M74 52 h34 l8 7 l-8 7 h-34 Z', 'w') + P('M74 72 h-30 l-8 7 l8 7 h30 Z', 'w')
    banner('origin', hills + post + egg + sprout + T(sun('w', 14, 10, 5), 262, 38) + T(sparkle('w', 7), 196, 30), 's')

    mirror = L('M146 100 l-8 6 M174 100 l8 6 M160 100 v6', 'f', 3) + E(160, 56, 32, 44, 'w') + E(160, 56, 24, 36, 'u', 'f', 2)
    mirror += C(160, 46, 10, 'a', None, o='.6') + P('M142 82 Q144 62 160 62 Q176 62 178 82 Z', 'a', None, o='.6') + L('M146 32 l8 -6', 'u', 3)
    rack = L('M44 30 h64', 'f', 3) + L('M76 30 v-6 q0 -4 4 -4', 'f', 2) + P('M60 40 L76 30 L92 40 L100 54 L92 58 L90 50 L90 92 L62 92 L62 50 L60 58 L52 54 Z', 'a')
    sw = ''.join(C(x, 80, 11, c) for x, c in ((232, 'd'), (256, 'w'), (280, 'a')))
    banner('appearance', rack + mirror + sw + T(sparkle('w', 8), 214, 40) + T(sparkle('w', 5), 268, 30), 'a')

    stars = ((80, 66), (112, 40), (148, 52), (176, 30), (210, 50), (240, 36), (226, 76))
    con = L('M' + ' L'.join(f'{x} {y}' for x, y in stars[:6]) + ' M210 50 L226 76', 'm', 2)
    con += ''.join(T(star('w', 6, 2.6), x, y) for x, y in stars) + T(moon('w'), 46, 32, 1.2)
    con += P('M4 100 Q90 84 170 98 Q250 86 316 100 L316 98 Q316 116 296 116 L24 116 Q4 116 4 98 Z', 'a', None, o='.3')
    banner('birthsign', con + C(280, 28, 2, 'u', None) + C(130, 22, 2, 'u', None) + C(276, 70, 2, 'u', None), 'a')

    bars = ''
    for i, (frac, c, ic) in enumerate(((0.8, 'd', heart('d', 'f', 2)), (0.55, 'w', star('w', 7, 3)), (0.7, 'a', sparkle('a', 8)))):
        y = 36 + i * 24
        bars += T(ic, 34, y, .55 if i == 0 else 1) + R(48, y - 6, 70, 12, 6, 'u', 'f', 2) + R(50, y - 4, 66 * frac, 8, 4, c, None)
    d20 = P(poly(ring_pts(160, 60, 36, 6, -90)), 'a') + P('M160 34 L182 72 L138 72 Z', 'u', 'f', 2.5) + L('M160 24 L160 34 M191 78 L182 72 M129 78 L138 72', 'f', 2)
    radar = P(poly(ring_pts(250, 60, 32, 6, -90)), 'u', 'f', 2) + P(poly([(250, 34), (272, 48), (266, 72), (250, 76), (230, 66), (236, 46)]), 'w', 'f', 2, o='.8')
    banner('attributes', bars + d20 + radar, 'a')

    track = R(60, 92, 200, 8, 4, 'u', 'f', 2) + C(160, 96, 9, 'w')
    calm = C(84, 54, 22, 'b') + L('M74 52 q4 3 8 0 M86 52 q4 3 8 0 M78 62 q6 3 12 0', 'f', 2.5)
    joy = C(160, 50, 26, 'w') + C(151, 44, 3, 'f', None) + C(169, 44, 3, 'f', None) + P('M148 56 q12 14 24 0 Z', 'f', 'f', 2)
    fire = C(236, 54, 22, 'd') + L('M224 44 l8 4 M248 44 l-8 4 M228 64 q8 -6 16 0', 'f', 2.5) + C(230, 51, 2.5, 'f', None) + C(242, 51, 2.5, 'f', None)
    banner('disposition', track + calm + joy + fire, 'w')

    head = P('M128 112 L128 92 Q108 82 108 56 Q108 18 152 16 Q192 18 196 50 L206 68 L196 72 Q198 92 178 94 L176 112 Z', 'a')
    head += T(gear('u', 12, 8, 'a'), 146, 52) + T(gear('u', 8, 6, 'a'), 172, 40, 1, 15) + T(bulb('w'), 238, 40, .9)
    web = ''.join(L(f'M{a} {b} L{c} {d}', 'm', 2) for a, b, c, d in ((40, 40, 70, 70), (70, 70, 96, 46), (40, 40, 96, 46))) + ''.join(C(x, y, 6, 'b') for x, y in ((40, 40), (70, 70), (96, 46)))
    banner('mind', web + head + L('M226 72 l-10 6 M260 30 l8 -4 M262 52 l10 2', 'w', 2.5), 'a', sw=60)

    big = bubble(110, 18, 100, 64, 'a') + ''.join(R(122 + i * 12, 50 - h / 2, 6, h, 3, 'u', None) for i, h in enumerate((10, 22, 34, 18, 28, 12, 20)))
    quill = T(P('M0 0 Q4 -30 30 -46 Q22 -20 0 0 Z', 'u') + L('M2 -4 L22 -34', 'f', 2), 52, 98) + L('M50 100 Q34 92 28 100', 'a', 3)
    small = bubble(230, 40, 60, 38, 'w', 'r') + L('M242 60 q6 -8 12 0 t12 0 t12 0', 'f', 3)
    banner('voice', quill + big + small, 'a')

    arc = dashed('M40 100 Q160 -12 280 100') + T(sun('w', 14, 8, 5), 72, 50) + T(moon('b'), 250, 50, 1.3)
    desk = T(clock('u', 'f', 'b', .3, 28), 160, 62) + R(196, 84, 18, 18, 4, 'w') + L('M214 88 q6 0 6 5 q0 5 -6 5', 'f', 2.5) + R(104, 90, 34, 10, 3, 'b')
    banner('daily_life', arc + desk, 'b')

    ppl = ((70, 74, 'i'), (124, 50, 's'), (196, 50, 'd'), (250, 74, 'i'), (160, 88, 'w'))
    rel = ''.join(L(f'M{a[0]} {a[1] - 10} L{b[0]} {b[1] - 10}', 'm', 2.5) for a, b in ((ppl[0], ppl[1]), (ppl[1], ppl[2]), (ppl[2], ppl[3]), (ppl[1], ppl[4]), (ppl[2], ppl[4]), (ppl[0], ppl[4]), (ppl[3], ppl[4])))
    for x, y, c in ppl:
        rel += P(f'M{x - 14} {y + 14} Q{x - 14} {y - 2} {x} {y - 2} Q{x + 14} {y - 2} {x + 14} {y + 14} Z', c if c != 'w' else 'd') + C(x, y - 12, 10, 'u')
    rel += T(heart('d', 'f', 2), 160, 40, .5) + T(heart('d', 'f', 2), 97, 52, .4) + T(heart('d', 'f', 2), 223, 52, .4)
    banner('relationships', rel, 'i')

    bk = P('M96 88 L160 98 L224 88 L224 104 L160 112 L96 104 Z', 'm') + P('M100 52 L160 64 L220 52 L220 92 L160 104 L100 92 Z', 'a') + L('M160 64 v40', 'f', 2.5)
    runes = C(126, 74, 5, None, 'u', 2) + P('M184 68 l6 10 h-12 Z', None, 'u', 2) + L('M118 86 h18 M178 86 h22 M196 70 l6 6', 'u', 2)
    glow = ''.join(T(sparkle('w', r), x, y) for x, y, r in ((140, 34, 7), (166, 22, 9), (190, 40, 6), (154, 46, 5)))
    cand = ''.join(R(x - 7, 70, 14, 34, 3, 'u') + T(flame('w', 'u'), x, 60, .55) for x in (52, 268))
    banner('grimoire', cand + bk + runes + glow, 'w')

    sc = R(98, 22, 108, 80, 4, 'u') + R(90, 16, 124, 10, 5, 'm') + R(90, 98, 124, 10, 5, 'm')
    sc += ''.join(L(f'M110 {y} l4 4 l8 -8', 's', 3) + L(f'M130 {y} h50', 'm', 3) for y in (42, 60, 78)) + C(194, 88, 10, 'w') + T(star('u', 5, 2.2, None), 194, 88)
    banner('review', sc + T(owl('a', 'happy'), 222, 20, .7) + T(sparkle('a', 8), 40, 40) + T(sparkle('s', 6), 70, 80), 's', sx=170)


def empty(name, body, hue='a'):
    frame = P('M24 40 Q26 14 60 14 L184 16 Q220 16 220 46 L218 116 Q218 146 184 146 L56 144 Q20 144 22 112 Z', 'e', None)
    frame += E(120, 140, 70, 6, 'f', None, o='.08')
    write(f'empty/{name}.svg', 240, 160, frame + body, None)


def empties():
    st = ''.join(R(54 + i * 6, 34 + i * 26, 120, 34, 8, 'u', 'm', 2.5, o=None) + C(72 + i * 6, 51 + i * 26, 7, 'm', None, o='.4') +
                 L(f'M{86 + i * 6} {46 + i * 26} h50 M{86 + i * 6} {56 + i * 26} h30', 'm', 3, '.4') for i in range(3))
    quill = T(P('M0 0 Q4 -30 30 -46 Q22 -20 0 0 Z', 'a') + L('M2 -4 L22 -34', 'f', 2), 176, 128, 1)
    empty('no_posts', st + quill + C(194, 40, 3, 'm', None) + C(204, 32, 4, 'm', None), 'a')

    ped = R(84, 118, 72, 14, 4, 'm') + R(92, 106, 56, 12, 4, 'u', 'f', 2.5)
    ghost = dashed('M100 104 Q98 74 120 72 Q142 74 140 104 M107 52 a13 13 0 1 0 26 0 a13 13 0 1 0 -26 0', 'm', 3)
    plus = C(172, 46, 14, 'a') + L('M166 46 h12 M172 40 v12', 'u', 3)
    empty('no_agents', ped + ghost + plus + T(sparkle('w', 7), 64, 50), 'a')

    box = P('M64 76 L120 92 L176 76 L176 128 L120 142 L64 128 Z', 'm') + P('M64 76 L120 60 L176 76 L120 92 Z', 'k', 'f', 3)
    box += P('M64 76 L46 96 L102 110 L120 92 Z', 'u') + P('M176 76 L194 96 L138 110 L120 92 Z', 'u')
    docs = dashed('M104 70 v-40 h32 l8 8 v32', 'm', 3) + C(170, 32, 3, 'm', None) + C(182, 44, 2, 'm', None) + C(62, 40, 2.5, 'm', None)
    empty('no_documents', docs + box, 'w')

    lens = T(magnifier('u'), 176, 70, 1.4) + C(176, 70, 8, None, 'm', 2.5)
    empty('no_results', T(owl('a'), 50, 26, .9) + lens + C(204, 30, 3, 'm', None) + qmark(206, 116, .8, 'm'), 'a')

    plugA = R(40, 76, 50, 16, 6, 'm') + R(90, 70, 22, 28, 5, 'u') + L('M112 78 h10 M112 90 h10', 'f', 3) + L('M40 84 h-20', 'f', 3)
    plugB = R(128, 70, 22, 28, 5, 'u') + R(150, 76, 50, 16, 6, 'm') + L('M200 84 h20', 'f', 3) + E(139, 84, 3, 5, 'k', None)
    zap = L('M120 50 l-4 10 h8 l-4 10', 'w', 3) + L('M106 44 l-4 -6 M134 44 l4 -6', 'd', 2.5)
    empty('no_network', plugA + plugB + zap + C(64, 40, 3, 'm', None) + C(184, 120, 3, 'm', None), 'd')

    nodes = ((60, 50), (104, 108), (150, 40), (190, 96), (120, 70))
    g = ''.join(dashed(f'M{a[0]} {a[1]} L{b[0]} {b[1]}') for a, b in ((nodes[0], nodes[4]), (nodes[1], nodes[4]), (nodes[2], nodes[4]), (nodes[3], nodes[4])))
    g += ''.join(C(x, y, 9, 'u', 'm', 2.5) for x, y in nodes[:4]) + C(120, 70, 14, 'a') + T(sparkle('u', 7), 120, 70)
    empty('graph_empty', g, 'a')


def build():
    banners()
    empties()
