"""Cognition cards: intelligence style, epistemic stance, cognitive pace, humor style."""
from kit import *


def qmark(x, y, s=1, c='f', w=3):
    return T(L('M-6 -6 Q-6 -13 0 -13 Q7 -13 7 -6 Q7 -1 0 1 L0 5', c, w) + C(0, 11, w * .8, c, None), x, y, s)


def intelligence():
    g = 'intelligence_style'
    blocks = ''
    for row, cnt in enumerate((4, 3, 2)):
        y = 84 - row * 17
        x0 = 80 - cnt * 11
        blocks += ''.join(R(x0 + i * 22, y, 20, 16, 3, 'a', 'f', 2.5) for i in range(cnt))
    blocks += R(70, 33, 20, 16, 3, 'w', 'f', 2.5) + T(sparkle('w', 7), 104, 28) + L('M58 28 L70 38', 'm', 2)
    card(g, 'first_principles_deductive', blocks, 'a')

    sats = ring_pts(80, 58, 38, 5, -90)
    glyph = [R(-6, -5, 12, 10, 2, 'w', 'f', 2), P('M-5 6 L-2 -1 L-2 -6 L2 -6 L2 -1 L5 6 Z', 'b', 'f', 2),
             C(-2, 4, 3, 'f', None) + L('M1 4 L1 -6 L6 -4', 'f', 2), T(leaf('b'), 0, 0, .55), star('w', 6, 3)]
    hub = ''.join(L(f'M80 58 L{n(x)} {n(y)}', 'm', 2) for x, y in sats)
    hub += ''.join(C(x, y, 11, 'u') + T(gl, x, y) for (x, y), gl in zip(sats, glyph))
    card(g, 'polymath_synthesizer', hub + C(80, 58, 13, 'a') + T(sparkle('u', 7), 80, 58), 'a')

    bars = ''.join(R(x, 92 - h, 14, h, 3, 's', 'f', 2.5) for x, h in ((36, 18), (56, 30), (76, 44)))
    pts = ((40, 64), (58, 54), (76, 40), (96, 30))
    bars += L('M36 70 L104 26', 'm', 2, '.8') + ''.join(C(x, y, 3.5, 'u', 'f', 2) for x, y in pts)
    card(g, 'empirical_inductive', bars + T(magnifier('u'), 112, 56, 1.1), 's')

    arr = ''
    for d, (ex, ey) in (('M54 60 Q84 60 118 28', (118, 28)), ('M54 60 Q90 60 126 60', (126, 60)), ('M54 60 Q84 60 118 92', (118, 92))):
        arr += L(d, 'b', 3) + C(ex, ey, 6, 'w')
    card(g, 'lateral_divergent', arr + T(bulb('w'), 42, 60, 1.1), 'b')

    grid = L('M30 30 h24 v24 M30 30 v24 h24', 'm', 2) + ''.join(C(x, y, 4, 'u', 'f', 2) for x, y in ((30, 30), (54, 30), (30, 54), (54, 54)))
    card(g, 'systems_architect', grid + T(gear('a', 18, 9), 76, 66) + T(gear('b', 12, 7), 112, 44, 1, 12) + T(gear('a', 9, 6), 116, 82), 'a')

    quill = T(P('M0 0 Q4 -30 30 -46 Q22 -20 0 0 Z', 'u') + L('M2 -4 L22 -34', 'f', 2), 70, 90)
    ribbon = L('M68 92 Q50 80 40 90 Q30 100 22 84', 'a', 3)
    card(g, 'poetic_intuitive', ribbon + quill + T(moon('w'), 118, 34) + T(star('w', 5, 2.2), 96, 22) + T(star('w', 4, 1.8), 138, 60), 'a')

    tb = T(R(-3, 0, 6, 30, 3, 'm', 'f', 2) + R(-11, -8, 22, 10, 3, 'f', None), 60, 30, 1, -15)
    tb += T(R(-3.5, 0, 7, 30, 3.5, 'm', 'f', 2) + P('M-8 -2 Q-8 -14 0 -14 L0 -6 L6 -6 L6 -12 Q10 -8 8 -2 Q4 4 -4 3 Z', 'm', 'f', 2), 100, 30, 1, 195)
    tb += R(34, 54, 92, 42, 6, 'd') + P('M64 54 v-8 q0 -4 4 -4 h24 q4 0 4 4 v8', None, 'f', 3) + R(74, 66, 12, 8, 2, 'w', 'f', 2) + L('M34 70 h40 M86 70 h40', 'f', 2)
    card(g, 'pragmatic_realist', tb, 'd')


def stance():
    g = 'epistemic_stance'
    card(g, 'fallibilist_skeptic', T(magnifier('u'), 70, 52, 1.9) + qmark(70, 50, 1.2, 'a') + T(sparkle('w', 6), 120, 28), 'a')

    tri = P('M54 88 L80 42 L106 88 Z', 'u', 'f', 3) + C(80, 72, 30, None, 'b', 2) + C(80, 72, 2.5, 'f', None)
    comp = L('M80 18 L60 62 M80 18 L100 62', 'f', 3) + C(80, 18, 5, 'w') + L('M100 62 l2 4', 'b', 3)
    card(g, 'rationalist_deductive', tri + comp, 'b')

    door = R(70, 22, 46, 78, 4, 'm') + P('M74 26 h38 v74 h-38 Z', 'G', None) + P('M74 100 L54 104 L130 104 L112 100 Z', 'w', None, o='.35')
    fig_, _ = person(56, 100, 'a', (12, 12), 'none', .62)
    card(g, 'phenomenological_existential', door + fig_ + T(star('w', 5, 2.2), 136, 30), 'a', grad='w')

    def beaker(x, liq, mark):
        b = P(f'M{x - 10} 40 v20 l-12 28 q-2 6 4 6 h36 q6 0 4 -6 l-12 -28 v-20', 'u', 'f', 3)
        b += P(f'M{x - 17} 74 h34 l5 12 q1 4 -3 4 h-38 q-4 0 -3 -4 Z', liq, None) + L(f'M{x - 13} 40 h26', 'f', 3)
        return b + T(C(0, 0, 9, 'u') + mark, x + 12, 30)
    ok = L('M-4 0 l3 3 l6 -6', 's', 3)
    no = L('M-4 -4 l8 8 M4 -4 l-8 8', 'd', 3)
    card(g, 'pragmatic_empiricist', beaker(52, 'i', no) + beaker(108, 'i', ok), 'i')

    mon = C(80, 62, 32, 'G') + L('M54 60 q13 -8 26 0 t26 0 M58 74 q11 -7 22 0 t22 0', 'u', 2.5) + C(80, 62, 32, None, 'f', 3)
    drops = ''.join(T(P('M0 -8 Q6 0 0 5 Q-6 0 0 -8 Z', 'b', 'f', 2), x, y) for x, y in ((44, 22), (80, 14), (116, 22)))
    card(g, 'holistic_monist', mon + drops, 'b', grad='b')

    tri = L('M50 84 L74 46 M110 84 L86 46', 'm', 3) + P('M68 50 l6 -4 l1 7', None, 'm', 3) + P('M92 50 l-6 -4 l-1 7', None, 'm', 3)
    tri += C(46, 86, 14, 'a') + C(114, 86, 14, 'w') + C(80, 32, 16, 'G') + T(sparkle('u', 7), 80, 32)
    card(g, 'dialectical_critic', tri, 's', grad='s')


def pace():
    g = 'cognitive_pace'
    met = P('M60 96 L70 28 Q80 22 90 28 L100 96 Z', 'w') + R(54, 92, 52, 8, 3, 'm') + L('M80 82 L96 30', 'f', 3) + R(86, 50, 12, 9, 2, 'u', 'f', 2)
    met += L('M110 34 q8 6 6 16 M50 34 q-8 6 -6 16', 'm', 2) + C(80, 82, 4, 'f', None)
    card(g, 'deliberate_measured', met, 'w')

    snail = P('M30 96 Q30 86 44 86 L104 86 Q122 86 124 96 Z', 's') + C(72, 66, 22, 'w') + L('M72 66 m0 0 a5 5 0 0 1 8 2 a10 10 0 0 1 -12 10 a15 15 0 0 1 -14 -18 a19 19 0 0 1 22 -14', 'f', 2.5)
    snail += L('M110 86 L114 70 M118 86 L124 72', 'f', 2.5) + C(114, 69, 2.5, 'f', None) + C(124, 71, 2.5, 'f', None)
    card(g, 'slow_contemplative', snail + T(leaf('s'), 42, 34, 1, -20), 's')

    nodes = ((34, 80), (62, 40), (88, 76), (122, 34))
    zz = P('M34 80 L62 40 L88 76 L122 34', None, 'w', 4) + ''.join(C(x, y, 7, 'b') for x, y in nodes)
    zz += L('M24 30 h16 M18 40 h14 M22 50 h10', 'm', 2.5) + T(sparkle('w', 7), 132, 70)
    card(g, 'rapid_associative', zz, 'b')

    tgt = C(78, 60, 32, 'u') + C(78, 60, 22, 'd', 'f', 2.5) + C(78, 60, 12, 'u', 'f', 2.5) + C(78, 60, 4, 'd', None)
    tgt += L('M78 60 L130 22', 'f', 3) + P('M124 18 l10 -2 l-2 10 Z M120 22 l8 -6 M126 28 l6 -8', 'a', 'f', 2)
    tgt += L('M40 22 l6 6 M30 60 h8 M40 98 l6 -6', 'w', 3)
    card(g, 'hyperfocus_burst', tgt, 'd')


def mask(f, s='f'):
    return P('M-24 -20 Q0 -28 24 -20 Q26 6 0 24 Q-26 6 -24 -20 Z', f, s, 3)


def humor():
    g = 'humor_style'
    m = mask('u') + L('M-15 -6 Q-10 -11 -5 -6', 'f', 3) + P('M5 -6 Q10 -12 15 -6 Q10 -3 5 -6 Z', 'f', None)
    m += L('M-16 -14 Q-10 -17 -5 -14 M5 -18 Q10 -22 16 -16', 'f', 2) + L('M-8 8 Q2 12 10 4', 'f', 3)
    card(g, 'socratic_irony', T(m, 72, 58, 1.35) + T(sparkle('w', 7), 124, 30) + T(leaf('s'), 122, 84, .8, 30), 'a')

    fc = C(64, 58, 28, 'w') + L('M52 54 h-1 M76 54 h-1', 'f', 6) + L('M46 50 h12 M70 50 h12 M56 72 h16', 'f', 3)
    cup = R(100, 64, 26, 22, 6, 'u') + L('M126 70 q8 0 8 6 q0 6 -8 6', 'f', 3) + L('M106 56 q-3 -5 0 -10 M114 56 q-3 -5 0 -10', 'm', 2)
    card(g, 'dry_deadpan_wit', fc + cup + R(96, 86, 34, 4, 2, 'm', None), 'w')

    p, _ = person(72, 100, 's', (15, 160), 'smile')
    drop = P('M98 30 Q104 38 98 42 Q92 38 98 30 Z', 'i', 'f', 2)
    card(g, 'warm_self_deprecating', p + drop + T(heart('d'), 40, 40, .55) + L('M64 44 l-3 2 M80 44 l3 2', 'd', 2, '.7'), 's')

    def paddle(x, y, r, c):
        return T(R(-4, 14, 8, 22, 4, 'm', 'f', 2) + C(0, 0, 17, c), x, y, 1, r)
    pp = paddle(44, 54, -30, 'd') + paddle(116, 54, 30, 'a') + C(80, 30, 6, 'u')
    pp += L('M56 40 Q66 24 74 28 M86 28 Q96 24 104 40', 'm', 2, '.8') + T(sparkle('w', 6), 80, 86)
    card(g, 'playful_banter', pp, 'w')

    ledge = R(26, 62, 50, 38, 4, 'm')
    clk = P('M26 62 Q24 36 54 34 Q86 34 90 56 Q93 74 90 92 Q88 102 82 98 Q78 92 80 80 Q80 68 72 62 Z', 'u')
    clk += L('M56 50 L56 40 M56 50 L66 54', 'f', 3) + C(56, 50, 2.5, 'f', None) + P('M85 98 Q87 106 84 108 Q81 106 85 98 Z', 'u', 'f', 2)
    fish = T(P('M-12 0 Q0 -10 10 0 Q0 10 -12 0 Z M10 0 l8 -6 v12 Z', 'w', 'f', 2) + C(-5, -1, 1.5, 'f', None), 124, 30, 1, -10)
    card(g, 'absurdist_surreal', ledge + clk + fish + C(40, 26, 5, 'b'), 'b')

    cd = C(80, 36, 20, 'w', None, o='.25') + T(flame('w', 'u'), 80, 34, .9) + L('M80 47 v4', 'f', 2)
    cd += R(68, 50, 24, 40, 4, 'u') + T(heart('d', 'f', 2), 80, 70, .45) + E(80, 92, 26, 6, 'm')
    card(g, 'earnest_serious', cd, 'd')


def build():
    intelligence()
    stance()
    pace()
    humor()
