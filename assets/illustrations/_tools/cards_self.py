"""Enneagram, instinctual variant and gender cards."""
from kit import *


def masquerade(f):
    return (P('M-30 -6 Q-30 -16 -16 -14 Q-6 -12 0 -6 Q6 -12 16 -14 Q30 -16 30 -6 Q30 10 16 10 Q6 10 0 2 Q-6 10 -16 10 Q-30 10 -30 -6 Z', f) +
            E(-14, -3, 6, 4, 'k', 'f', 2) + E(14, -3, 6, 4, 'k', 'f', 2))


def crown(f='w'):
    return P('M-16 8 L-18 -10 L-8 -2 L0 -14 L8 -2 L18 -10 L16 8 Z', f) + C(0, -16, 2.5, f, 'f', 2)


def rose(f='d'):
    return C(0, 0, 9, f) + L('M-3 -1 Q0 -5 3 -1 Q3 4 -2 3', 'f', 2) + T(leaf('s'), -10, 12, .5, 30)


def enneagram():
    g = 'enneagram'
    lvl = R(28, 56, 104, 22, 8, 'i') + R(66, 60, 28, 14, 7, 'u', 'f', 2) + C(80, 67, 4.5, 's', None) + L('M72 60 v14 M88 60 v14', 'f', 2)
    card(g, 'type1w9', lvl + T(leaf('s'), 116, 36, 1, -25) + L('M40 92 h80', 'm', 2), 'i')

    mg = P('M40 52 L84 34 L84 92 L40 74 Z', 'a') + R(28, 52, 14, 22, 4, 'u') + R(46, 74, 10, 18, 3, 'm', 'f', 2)
    card(g, 'type1w2', mg + T(heart('d'), 112, 50, .8) + L('M98 34 l6 -6 M100 80 l6 6', 'd', 2.5), 'a')

    hands = P('M40 70 Q42 98 80 98 Q118 98 120 70 Q102 80 80 80 Q58 80 40 70 Z', 'u') + L('M80 82 v15', 'f', 2)
    hands += T(E(0, 0, 5, 10, 'u', 'f', 2.5), 42, 64, 1, -30) + T(E(0, 0, 5, 10, 'u', 'f', 2.5), 118, 64, 1, 30)
    card(g, 'type2w1', C(80, 50, 24, 'd', None, o='.2') + T(heart('d'), 80, 52, 1.3) + hands, 'd')

    pot = P('M40 52 Q38 86 62 86 Q86 86 84 52 Z', 'd') + R(46, 46, 32, 8, 4, 'd') + C(62, 40, 4, 'f', None)
    pot += L('M84 58 Q96 50 100 40', 'f', 3) + L('M40 58 q-12 4 -4 18', 'f', 3) + L('M100 44 Q104 56 110 66', 'i', 2.5, '.8')
    cup = P('M100 70 h24 q0 18 -12 18 q-12 0 -12 -18 Z', 'u') + L('M124 74 q6 0 6 5 q0 5 -6 5', 'f', 2.5)
    card(g, 'type2w3', pot + cup + T(heart('w', 'f', 2), 112, 28, .5), 'd')

    wand = T(R(-3, 0, 6, 54, 3, 'f', None), 60, 96, 1, -140) + T(star('w', 14, 6, 'f', 2.5), 96, 44)
    card(g, 'type3w2', wand + T(sparkle('w', 7), 122, 28) + T(sparkle('a', 6), 70, 28) + T(heart('d'), 124, 72, .5), 'w')

    beam = P('M66 14 L94 14 L122 100 L38 100 Z', 'w', None, o='.18')
    tro = P('M60 40 h40 v8 q0 22 -20 22 q-20 0 -20 -22 Z', 'w') + L('M60 46 q-12 0 -10 10 q2 8 12 6 M100 46 q12 0 10 10 q-2 8 -12 6', 'f', 3)
    tro += R(76, 70, 8, 12, 0, 'w', 'f', 2) + R(64, 82, 32, 10, 3, 'm') + T(star('u', 7, 3), 80, 54)
    card(g, 'type3w4', beam + tro, 'w')

    card(g, 'type4w3', T(masquerade('a'), 80, 70, 1.4) + T(crown('w'), 80, 34, 1.1), 'a')
    card(g, 'type4w5', T(masquerade('a'), 76, 60, 1.4) + T(rose('d'), 116, 74) + T(sparkle('a', 6), 40, 30), 'a')

    col_ = R(56, 40, 34, 56, 2, 'u') + R(50, 92, 46, 8, 2, 'm') + L('M64 46 v44 M73 46 v44 M82 46 v44', 'm', 2)
    col_ += P('M50 40 h20 l6 -8 l8 6 l12 -4 v6 Z', 'm') + L('M76 44 l-6 12 l8 8 l-6 14', 'd', 2.5)
    ham = T(R(-3, 0, 6, 30, 3, 'm', 'f', 2) + R(-12, -10, 24, 12, 3, 'b'), 118, 40, 1, -40)
    card(g, 'type5w4', col_ + ham + L('M100 30 l-4 -6 M104 50 l6 2', 'w', 2.5), 'b')

    pa = P('M30 38 h32 v12 a7 7 0 1 1 0 14 v12 h-32 Z', 'b') + P('M70 38 h32 v38 h-32 v-12 a7 7 0 1 0 0 -14 Z', 'w')
    card(g, 'type5w6', T(pa, 6, 2) + T(magnifier('u'), 112, 76, .8), 'b')

    sh = T(shield('i'), 80, 58, 2.4) + C(80, 52, 6, 'k', None) + P('M77 54 L74 70 L86 70 L83 54 Z', 'k', None)
    card(g, 'type6w5', sh, 'i')

    lh = P('M68 96 L72 40 L88 40 L92 96 Z', 'u') + L('M70 60 h20 M69 78 h22', 'd', 4) + R(66, 26, 28, 14, 3, 'w') + P('M64 26 L80 14 L96 26 Z', 'd')
    lh += P('M94 30 L136 18 L136 46 Z M66 30 L24 18 L24 46 Z', 'w', None, o='.3') + L('M30 100 q10 -6 20 0 t20 0 M90 100 q10 -6 20 0 t20 0', 'i', 2.5)
    card(g, 'type6w7', lh, 'i')

    hat = P('M44 80 Q40 50 22 40 Q50 36 70 64 Q78 30 80 20 Q86 36 92 64 Q112 36 138 40 Q120 50 116 80 Z', 'd')
    hat += P('M80 22 L92 64 L70 64 Z', 'w') + R(42, 78, 76, 12, 6, 'a') + C(22, 40, 6, 'w') + C(80, 18, 6, 'a') + C(138, 40, 6, 'w')
    card(g, 'type7w6', T(hat, 12, 12, .85), 'w')

    rk = T(P('M0 -40 Q16 -24 14 8 L-14 8 Q-16 -24 0 -40 Z', 'u') + C(0, -14, 6, 'b', 'f', 2.5) +
           P('M-14 -4 L-24 14 L-14 10 Z M14 -4 L24 14 L14 10 Z', 'd') + T(flame('w', 'd'), 0, 20, .6, 180), 80, 64, 1, 30)
    card(g, 'type7w8', rk + T(star('w', 4, 2), 36, 30) + T(star('w', 4, 2), 128, 90) + C(40, 86, 3, 'm', None), 'd')

    mt = P('M18 98 L64 32 L84 60 L96 46 L142 98 Z', 'm') + P('M64 32 L52 50 L60 48 L66 54 L74 46 Z', 'u', 'f', 2)
    mt += L('M64 32 L64 12', 'f', 3) + P('M64 13 L82 18 L64 24 Z', 'd', 'f', 2)
    card(g, 'type8w7', mt, 'd')

    bear = C(56, 38, 10, 'w') + C(104, 38, 10, 'w') + C(80, 62, 32, 'w') + E(80, 72, 14, 11, 'u', 'f', 2.5)
    bear += E(80, 67, 5, 3.5, 'f', None) + L('M80 71 v4 M74 77 q6 3 12 0', 'f', 2) + C(68, 54, 3, 'f', None) + C(92, 54, 3, 'f', None)
    card(g, 'type8w9', bear + C(56, 38, 4, 'd', None, o='.5') + C(104, 38, 4, 'd', None, o='.5'), 'w')

    wh = P('M40 52 h50 q18 0 18 18 q0 18 -18 18 q-16 0 -18 -14 L40 66 Z', 'i') + C(90, 70, 7, 'u', 'f', 2) + R(34, 50, 10, 18, 3, 'm')
    wh += L('M116 38 l8 -8 M120 52 h12 M110 28 l2 -10', 'w', 3) + L('M38 50 q-10 -16 6 -24', 'f', 2)
    card(g, 'type9w8', wh, 'i')

    cl = T(cloud('u'), 72, 72, 1.6) + T(moon('w'), 104, 34, 1.2) + T(star('w', 5, 2.2), 54, 30) + T(star('w', 4, 1.8), 130, 64)
    cl += L('M56 70 q4 4 8 0 M76 70 q4 4 8 0', 'f', 2)
    card(g, 'type9w1', cl, 'a')


def house(f='w'):
    return P('M-18 16 L-18 -4 L0 -20 L18 -4 L18 16 Z', f) + R(-5, 4, 10, 12, 2, 'u', 'f', 2) + R(8, -18, 6, 10, 1, f, 'f', 2)


def crowd(f='i'):
    out = ''
    for x, y, s in ((-14, 4, .8), (14, 4, .8), (0, 8, 1)):
        out += T(P('M-10 14 Q-10 0 0 0 Q10 0 10 14 Z', f) + C(0, -8, 7, 'u'), x, y, s)
    return out


def hearts(f='d'):
    return T(heart(f), -6, 2, .9) + T(heart('w'), 8, -4, .6)


ICON = {'sp': house, 'so': crowd, 'sx': hearts}
HUE = {'sp': 'w', 'so': 'i', 'sx': 'd'}


def instinctual():
    g = 'instinctual_variant'
    for a, b in (('sp', 'so'), ('sp', 'sx'), ('so', 'sp'), ('so', 'sx'), ('sx', 'sp'), ('sx', 'so')):
        main = T(ICON[a](HUE[a]), 70, 62, 1.9)
        badge = C(120, 34, 18, 'u') + T(ICON[b](HUE[b]), 120, 34, .7)
        card(g, f'{a}_{b}', main + L('M96 50 L106 44', 'm', 2) + badge, HUE[a])


def gender():
    g = 'gender'
    arrow = T(R(-3.5, 0, 7, 26, 2, 'i', 'f', 2) + P('M-11 2 L0 -12 L11 2 Z', 'i', 'f', 2), 88, 48, 1, 45)
    card(g, 'male', C(70, 66, 30, 'i', None, o='.3') + arrow + C(70, 66, 22, 'u'), 'i')
    cross = R(76, 70, 8, 30, 2, 'a', 'f', 2) + R(66, 82, 28, 8, 2, 'a', 'f', 2)
    card(g, 'female', C(80, 48, 30, 'a', None, o='.3') + cross + C(80, 48, 22, 'u'), 'a')
    ast = R(76, 18, 8, 30, 2, 'w', 'f', 2) + T(R(-14, -4, 28, 8, 2, 'w', 'f', 2), 80, 28, 1, 30) + T(R(-14, -4, 28, 8, 2, 'w', 'f', 2), 80, 28, 1, -30)
    card(g, 'non_binary', C(80, 70, 30, 'w', None, o='.3') + ast + C(80, 70, 22, 'u'), 'w')
    pen = T(R(-5, -26, 10, 32, 2, 'w') + P('M-5 6 L0 16 L5 6 Z', 'u', 'f', 2), 104, 56, 1, 40)
    plus = R(60, 59, 20, 6, 2, 's', None) + R(67, 52, 6, 20, 2, 's', None)
    cu = C(70, 62, 22, 'u') + plus + pen + T(sparkle('s', 8), 118, 26) + T(sparkle('w', 5), 40, 32)
    card(g, 'custom', C(70, 62, 30, 's', None, o='.3') + cu, 's')


def build():
    enneagram()
    instinctual()
    gender()
