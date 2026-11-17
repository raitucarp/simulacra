"""Body kinds and the forms of objects, plants, elements and creatures (flat ontology)."""
from kit import *
from cards_world import owl


def tube(d, c, w=5):
    """Solid stroke shape with an fg outline (tentacle, tail, wind ribbon)."""
    return L(d, 'f', w + 4) + L(d, c, w)


def blobs(circles, f):
    """Union of circles with one shared outline (tree canopy, moss)."""
    return (''.join(C(x, y, r + 1.5, 'f', None) for x, y, r in circles) +
            ''.join(C(x, y, r - 1.5, f, None) for x, y, r in circles))


def spiral(cx, cy, r0, r1, turns, rot=0):
    k = int(turns * 16)
    pts = []
    for i in range(k + 1):
        t = i / k
        a = math.radians(rot + t * turns * 360)
        r = r0 + (r1 - r0) * t
        pts.append(f'{n(cx + r * math.cos(a))} {n(cy + r * math.sin(a))}')
    return 'M' + ' L'.join(pts)


# ---------- reusable subjects ----------
def mug(f='u'):
    return (tube('M16 -8 Q30 -8 30 4 Q30 16 16 14', f, 5) + P('M-18 -18 L18 -18 L16 20 Q16 26 10 26 L-10 26 Q-16 26 -16 20 Z', f) +
            L('M-16 -4 h32', 'a', 3))


def skeleton_key(f='w'):
    teeth = P('M24 4 h8 v10 h-8 Z M38 4 h6 v14 h-6 Z', f, 'f', 2.5)
    return (teeth + R(-6, -5, 54, 10, 4, f) + C(-18, 0, 16, f) + C(-18, 0, 7, 'e', 'f', 2.5) +
            L('M-4 -5 v10 M2 -5 v10', 'f', 2))


def drop(f='i'):
    return P('M0 -30 Q22 -2 22 12 Q22 32 0 32 Q-22 32 -22 12 Q-22 -2 0 -30 Z', f) + L('M-12 12 Q-12 22 -4 25', 'u', 3, '.7')


def chip(f='a'):
    pins = ''.join(L(f'M{p} -22 v-6 M{p} 22 v6 M-22 {p} h-6 M22 {p} h6', 'f', 2.5) for p in (-10, 0, 10))
    return pins + R(-22, -22, 44, 44, 6, 'u') + R(-12, -12, 24, 24, 4, f) + L('M-5 0 h10 M0 -5 v10', 'u', 2.5)


# ---------- body kinds ----------
def kinds():
    g = 'body_kind'
    p, _ = person(80, 102, 'a', (25, 25), sc=1.05)
    card(g, 'human', p, 'a')

    toes = ''.join(T(E(0, 0, 9, 12, 'b'), x, y, 1, r) for x, y, r in ((50, 52, -25), (68, 34, -8), (92, 34, 8), (110, 52, 25)))
    pad = P('M80 58 Q98 58 106 78 Q112 96 94 96 Q87 94 80 94 Q73 94 66 96 Q48 96 54 78 Q62 58 80 58 Z', 'b')
    card(g, 'creature', toes + pad, 'b')

    soil = P('M40 98 Q80 78 120 98 Z', 'm')
    stem = tube('M80 96 Q78 74 82 54', 's', 4)
    lv = T(leaf('s'), 98, 50, 1.6, -20) + MX(T(leaf('s'), 98, 62, 1.4, -10), 80)
    card(g, 'plant', stem + lv + soil, 's')

    card(g, 'object', T(mug('u'), 58, 66, 1.1) + T(skeleton_key('w'), 112, 62, .8, -60), 'i')
    card(g, 'element', T(drop('i'), 56, 62) + T(flame('d', 'w'), 106, 60, 2), 'd')
    card(g, 'machine', T(gear('m', 18, 8), 56, 52) + T(chip('a'), 106, 74, .85), 'a')

    town = R(30, 56, 22, 44, 2, 'm') + R(52, 30, 26, 70, 2, 'u') + P('M58 30 L65 14 L72 30 Z', 'u', 'f', 2.5)
    town += P('M82 64 Q82 44 98 44 Q114 44 114 64 Z', 'b') + R(80, 64, 36, 36, 2, 'u') + R(116, 50, 16, 50, 2, 'm')
    win = ''.join(R(x, y, 5, 6, 1, 'w', None) for x, y in ((58, 40), (68, 40), (58, 54), (68, 54), (58, 68), (68, 68),
                                                          (36, 64), (36, 78), (88, 72), (102, 72), (121, 60), (121, 74)))
    town += win + R(94, 86, 8, 14, 2, 'f', None) + L('M22 100 h116', 'f', 3)
    card(g, 'place', town, 'b')

    inf = 'M80 58 C94 38 124 38 124 58 C124 78 94 78 80 58 C66 38 36 38 36 58 C36 78 66 78 80 58 Z'
    card(g, 'abstraction', L(inf, 'f', 15) + L(inf, 'a', 9) + T(sparkle('w', 8), 122, 26) + T(sparkle('w', 5), 36, 92) +
         C(80, 92, 3, 'a', None), 'a')


# ---------- objects ----------
def objects():
    g = 'body_form'
    arc = 'A26 7 0 0 0'
    can = P(f'M50 44 L50 94 {arc} 102 94 L102 44 Z', 'u')
    can += P(f'M50 60 {arc} 102 60 L102 82 A26 7 0 0 1 50 82 Z', 'd', 'f', 2.5)
    can += ''.join(L(f'M51 {y} {arc} 101 {y}', 'f', 2, '.45') for y in (50, 54, 88))
    can += E(76, 44, 26, 7, 'm') + E(76, 44, 21, 5, 'f', None, o='.45')
    lid = E(0, 0, 25, 7, 'm') + E(0, 0, 18, 4, None, 'f', 2, '.4')
    can += T(lid, 108, 26, 1, -35) + L('M55 47 l3 2 l3 -2 l3 2 l3 -2 l3 2 l3 -2 l3 2 l3 -2', 'f', 2)
    card(g, 'tin_can', can, 'd')

    bk = T(book('b'), 80, 60, 1.8) + P('M84 47 L84 70 L89 65 L94 70 L94 45 Z', 'd', 'f', 2)
    card(g, 'book', bk, 'b')

    ch = R(50, 16, 9, 80, 4, 'w') + R(101, 16, 9, 80, 4, 'w') + R(54, 24, 52, 8, 3, 'w') + R(54, 38, 52, 7, 3, 'w')
    ch += R(40, 72, 9, 30, 4, 'w') + R(111, 72, 9, 30, 4, 'w') + P('M44 60 L116 60 L124 72 Q124 76 120 76 L40 76 Q36 76 36 72 Z', 'a')
    card(g, 'chair', ch, 'w')

    glow = P('M44 60 L116 60 L134 104 L26 104 Z', 'w', None, o='.2') + C(80, 46, 30, 'w', None, o='.15')
    lp = R(77, 60, 6, 30, 3, 'm', 'f', 2) + R(56, 88, 48, 10, 5, 'm') + P('M58 24 L102 24 L116 62 L44 62 Z', 'w')
    lp += L('M100 62 v14', 'f', 2) + C(100, 79, 3, 'f', None) + L('M60 34 h40', 'u', 2.5, '.6')
    card(g, 'lamp', glow + lp, 'w')

    bells = C(54, 30, 12, 'w') + C(106, 30, 12, 'w') + L('M58 94 l-8 8 M102 94 l8 8 M70 28 Q80 20 90 28', 'f', 4)
    card(g, 'clock', bells + C(80, 62, 36, 'i') + T(clock('u', 'f', None, 0, 29), 80, 62), 'i')

    mr = R(-6, 18, 12, 32, 6, 'w') + E(0, -12, 28, 32, 'w') + E(0, -12, 21, 25, 'i', 'f', 2)
    mr += L('M-12 -18 L2 -32 M-10 -4 L12 -26', 'u', 3, '.8')
    card(g, 'mirror', T(mr, 80, 58, 1, 22), 'i')

    card(g, 'key', T(skeleton_key('w'), 74, 58, 1.35, -20) + T(sparkle('w', 6), 124, 92), 'w')

    steam = L('M68 38 q-6 -7 0 -14 q6 -7 0 -14 M86 38 q-6 -7 0 -14 q6 -7 0 -14', 'm', 3)
    cup = E(78, 92, 46, 9, 'u') + E(78, 90, 20, 4, 'f', None, o='.1') + tube('M106 56 Q124 54 122 68 Q120 80 102 78', 'u', 5)
    cup += P('M44 50 L112 50 Q110 90 78 90 Q46 90 44 50 Z', 'u') + E(78, 50, 34, 7, 'w') + P('M48 66 L108 66 Q106 74 104 76 L52 76 Q50 74 48 66 Z', 'a', None)
    card(g, 'teacup', steam + cup, 'a')


# ---------- plants ----------
def plants():
    g = 'body_form'
    trunk = P('M70 102 Q76 88 74 68 L86 68 Q84 88 92 102 Z', 'm') + L('M76 76 l-8 -8 M84 74 l8 -8', 'f', 3)
    canopy = blobs(((56, 50, 20), (80, 34, 22), (104, 50, 20), (66, 66, 16), (94, 66, 16), (80, 54, 20)), 's')
    canopy += C(64, 44, 6, 'f', None, o='.12') + C(98, 58, 7, 'f', None, o='.12') + C(84, 30, 5, 'f', None, o='.12')
    acorn = E(0, 3, 4, 5, 'w', 'f', 2) + P('M-5 0 Q0 -6 5 0 Z', 'm', 'f', 2)
    card(g, 'oak_tree', trunk + canopy + T(acorn, 120, 92) + T(acorn, 108, 96, 1, 20), 's')

    arms = tube('M70 66 H58 Q52 66 52 58 V44', 's', 10) + tube('M90 56 H102 Q108 56 108 48 V34', 's', 10)
    col_ = R(68, 22, 24, 64, 12, 's') + L('M80 30 V80', 'f', 2, '.3')
    spines = L('M64 40 l-4 -2 M96 44 l4 -2 M64 58 l-4 2 M96 70 l4 2 M48 52 l-4 0 M112 42 l4 0', 'f', 2)
    pot = P('M58 86 L102 86 L96 104 L64 104 Z', 'd') + R(54, 80, 52, 9, 3, 'd')
    flower = ''.join(T(E(0, -5, 3, 5, 'w', None), 80, 22, 1, a) for a in range(0, 360, 72)) + C(80, 22, 2.5, 'd', None)
    card(g, 'cactus', arms + col_ + spines + pot + flower, 's')

    rock = P('M30 100 Q28 70 60 66 Q92 60 118 70 Q136 78 132 100 Z', 'm')
    moss = blobs(((40, 78, 8), (50, 70, 9), (62, 64, 8), (74, 62, 9), (88, 60, 8), (100, 63, 9), (112, 68, 8), (122, 76, 8), (34, 88, 7), (58, 74, 7), (96, 72, 7)), 's')
    moss += ''.join(C(x, y, 2, 'f', None, o='.25') for x, y in ((48, 72), (66, 66), (86, 62), (104, 66), (120, 76), (78, 70)))
    stalks = ''.join(L(f'M{x} {y} q{d} -10 0 -20', 'f', 2) + E(x, y - 22, 2.5, 4, 'w', 'f', 2) for x, y, d in ((62, 58, 3), (82, 54, -3), (100, 58, 3)))
    card(g, 'moss', stalks + rock + moss + L('M60 90 q10 -4 20 0', 'u', 2.5, '.4'), 's')

    grass = L('M30 100 l4 -10 l2 10 M126 100 l3 -8 l3 8', 's', 2.5)
    big = P('M70 60 Q68 88 62 100 L98 100 Q92 88 90 60 Z', 'u') + P('M38 62 Q40 24 80 22 Q120 24 122 62 Q80 72 38 62 Z', 'd')
    big += C(62, 42, 6, 'u', None) + C(84, 32, 5, 'u', None) + C(100, 48, 6, 'u', None) + C(76, 54, 4, 'u', None)
    small = P('M118 86 Q118 96 116 100 L128 100 Q126 96 126 86 Z', 'u') + P('M108 88 Q108 72 122 72 Q136 72 136 88 Q122 92 108 88 Z', 'd')
    card(g, 'mushroom', grass + big + small + C(118, 80, 2.5, 'u', None), 'd')

    stem = tube('M80 60 Q78 82 80 102', 's', 4) + T(leaf('s'), 94, 84, 1.3, -15) + MX(T(leaf('s'), 94, 76, 1.1, -25), 80)
    petals = ''.join(T(E(0, -24, 6, 12, 'w', 'f', 2), 80, 42, 1, a) for a in range(0, 360, 36))
    seeds = ''.join(C(80 + 6 * math.cos(math.radians(a)), 42 + 6 * math.sin(math.radians(a)), 1.6, 'f', None) for a in range(0, 360, 60))
    card(g, 'sunflower', stem + petals + C(80, 42, 14, 'm') + seeds + C(80, 42, 1.6, 'f', None), 'w')

    vine = L('M28 100 Q30 70 56 64 Q84 58 80 38 Q78 20 104 18 Q128 18 132 40', 's', 4)
    tendril = L(spiral(46, 82, 1, 7, 1.3, 200), 's', 2) + L(spiral(98, 46, 1, 6, 1.3, 20), 's', 2)
    lvs = T(leaf('s'), 40, 66, 1, -50) + T(leaf('s'), 74, 66, 1, 30) + T(leaf('s'), 88, 22, 1, -30) + T(leaf('s'), 118, 18, 1, 20)
    grapes = ''.join(C(x, y, 5.5, 'a', 'f', 2) for x, y in ((126, 50), (138, 50), (120, 60), (132, 60), (126, 70)))
    card(g, 'vine', vine + tendril + lvs + grapes, 's')

    bam = ''
    for x, top, h in ((54, 28, 74), (80, 14, 88), (106, 34, 68)):
        bam += R(x - 7, top, 14, h, 5, 's') + ''.join(L(f'M{x - 8} {y} h16', 'f', 3) for y in range(top + 18, top + h - 6, 20))
    bl = 'M0 0 Q12 -7 28 -2 Q12 3 0 0 Z'
    bam += T(P(bl, 's', 'f', 2), 87, 52, 1, -20) + T(P(bl, 's', 'f', 2), 61, 66, 1, -160) + T(P(bl, 's', 'f', 2), 113, 70, 1, -30)
    bam += T(P(bl, 's', 'f', 2), 87, 72, 1, 10) + T(P(bl, 's', 'f', 2), 47, 46, 1, -150)
    card(g, 'bamboo', bam, 's')

    ground = P('M22 84 Q80 74 138 84 L138 100 Q138 104 132 104 L28 104 Q22 104 22 100 Z', 'm', 'f', 3)
    root = L('M80 82 Q76 92 82 98 M80 86 l-8 6', 'f', 2)
    sd = T(P('M0 -22 Q18 -8 16 8 Q13 22 0 22 Q-13 22 -16 8 Q-18 -8 0 -22 Z', 'w') + L('M0 -16 Q6 0 0 16', 'f', 2, '.35'), 80, 58, 1, 15)
    sprout = L('M86 38 Q84 28 88 22', 's', 3) + T(leaf('s'), 96, 18, .6, -20) + MX(T(leaf('s'), 96, 20, .5, -10), 88)
    card(g, 'seed', ground + root + sd + sprout + C(44, 92, 2, 'f', None, o='.4') + C(116, 94, 2, 'f', None, o='.4'), 'w')


# ---------- elements ----------
def elements():
    g = 'body_form'
    rip = E(80, 98, 42, 6, None, 'i', 2.5) + E(80, 98, 24, 3.5, None, 'i', 2.5)
    card(g, 'water', rip + T(drop('i'), 80, 56, 1.25) + C(112, 34, 4, 'i') + C(50, 30, 3, 'i'), 'i')

    logs = T(R(-30, -5, 60, 10, 5, 'm'), 80, 96, 1, 12) + T(R(-30, -5, 60, 10, 5, 'm'), 80, 96, 1, -12)
    embers = C(46, 40, 2.5, 'w', None) + C(116, 34, 3, 'w', None) + C(106, 18, 2, 'd', None)
    card(g, 'fire', T(flame('d', 'w'), 80, 54, 2.4) + logs + embers, 'd')

    wnd = tube('M24 46 H84 Q98 46 98 34 Q98 24 88 24 Q80 24 80 32', 'i', 5)
    wnd += tube('M18 64 H110 Q126 64 126 50 Q126 38 114 38 Q104 38 104 48', 'i', 5)
    wnd += tube('M34 82 H94 Q108 82 108 92 Q108 102 98 102 Q90 102 90 94', 'i', 5)
    card(g, 'wind', wnd + T(leaf('s'), 130, 84, .8, -30) + T(leaf('s'), 50, 26, .6, 40), 'i')

    st = E(80, 92, 36, 11, 'm') + E(78, 72, 28, 10, 'm') + E(82, 54, 20, 8, 'm') + E(80, 39, 12, 6.5, 'm')
    st += L('M58 88 q10 -4 20 -3 M62 69 q8 -3 16 -3 M72 52 q6 -2 12 -2', 'u', 2.5, '.5') + E(126, 98, 8, 4, 'm', 'f', 2) + E(36, 99, 6, 3, 'm', 'f', 2)
    card(g, 'stone', st, 'b')

    rays = ''.join(L(f'M{n(80 + 28 * math.cos(a))} {n(56 + 28 * math.sin(a))} L{n(80 + (44 if i % 2 else 50) * math.cos(a))} {n(56 + (44 if i % 2 else 50) * math.sin(a))}', 'w', 3)
                   for i, a in enumerate(math.radians(k * 30 + 15) for k in range(12)))
    lt = C(80, 56, 30, 'w', None, o='.2') + rays + C(80, 56, 21, 'w', None, o='.45') + C(80, 56, 14, 'w') + C(76, 52, 5, 'u', None, o='.8')
    card(g, 'light', lt + T(sparkle('w', 6), 30, 26) + T(sparkle('w', 5), 132, 94), 'w')

    cube = P(poly(((50, 44), (80, 56), (80, 94), (50, 82))), 'i') + P(poly(((80, 56), (110, 44), (110, 82), (80, 94))), 'i', 'f', 3, '.6')
    cube += P(poly(((50, 44), (80, 32), (110, 44), (80, 56))), 'u') + L('M56 54 L56 74 M62 58 L62 66 M66 42 L80 37', 'u', 2.5, '.7')
    flake = ''.join(T(L('M0 0 V-12 M0 -8 l-4 -4 M0 -8 l4 -4', 'i', 2.5), 124, 32, 1, a) for a in range(0, 360, 60))
    card(g, 'ice', cube + flake + T(sparkle('i', 5), 36, 28), 'i')

    card(g, 'cloud', T(cloud('i', 'f', 3), 112, 36, .9) + T(cloud('u'), 76, 70, 2), 'i')

    sand = T(sun('d', 9, 8, 4), 116, 30) + P('M20 84 Q58 42 100 70 Q118 80 140 74 L140 96 L20 96 Z', 'w', 'f', 3, '.55')
    sand += P('M18 96 Q70 64 142 94 L142 98 Q142 104 136 104 L24 104 Q18 104 18 98 Z', 'w')
    sand += L('M44 94 q16 -10 32 -8 M84 92 q14 -4 28 2 M56 74 q12 -8 24 -4', 'f', 2, '.3')
    card(g, 'sand', sand, 'w')


# ---------- creatures ----------
def creatures():
    g = 'body_form'
    tail = tube('M100 96 Q124 98 122 78 Q120 68 112 70', 'w', 6)
    body = P('M58 100 Q50 76 62 62 L98 62 Q110 76 102 100 Z', 'w') + E(70, 100, 8, 4, 'w') + E(90, 100, 8, 4, 'w')
    head = P('M56 54 L54 22 L70 34 Q80 30 90 34 L106 22 L104 54 Q104 72 80 72 Q56 72 56 54 Z', 'w')
    head += P('M59 30 L60 38 L66 34 Z M101 30 L100 38 L94 34 Z', 'd', None, o='.5')
    head += E(70, 50, 3, 4, 'f', None) + E(90, 50, 3, 4, 'f', None) + P('M77 56 h6 l-3 3 Z', 'f', 'f', 2)
    head += L('M80 59 q-3 4 -6 2 M80 59 q3 4 6 2', 'f', 2) + L('M64 58 h-14 M64 62 l-12 4 M96 58 h14 M96 62 l12 4', 'f', 2, '.6')
    card(g, 'cat', tail + body + head, 'w')

    branch = L('M20 98 Q80 92 142 100', 'm', 6)
    # An iridescent (info-hued) crow: a fg silhouette would turn white on dark themes.
    crow = L('M76 88 l-2 8 M88 90 l2 8', 'f', 3) + P('M50 52 Q62 44 80 50 Q100 58 112 72 L140 90 Q126 96 110 88 Q96 94 80 90 Q58 86 52 68 Z', 'i')
    crow += L('M76 60 Q96 64 108 78 M74 70 Q90 76 100 84', 'f', 2, '.45') + C(56, 44, 13, 'i') + P('M47 37 Q34 41 22 50 Q36 53 48 53 Z', 'w')
    crow += C(55, 42, 3.4, 'u', None) + C(54.5, 42, 1.8, 'f', None) + L('M26 49 L44 47', 'f', 1.5, '.5')
    card(g, 'crow', branch + crow, 'a')

    oct_ = ''
    for d in ('M66 58 Q48 66 42 82 Q38 96 50 94 Q56 90 50 86', 'M72 62 Q62 80 62 94 Q62 102 70 100', 'M78 64 Q80 84 74 98'):
        oct_ += tube(d, 'a', 6) + MX(tube(d, 'a', 6), 80)
    head = P('M56 54 Q52 18 80 16 Q108 18 104 54 Q100 66 80 66 Q60 66 56 54 Z', 'a')
    head += C(70, 48, 5, 'u', 'f', 2) + C(90, 48, 5, 'u', 'f', 2) + C(70, 49, 2.2, 'f', None) + C(90, 49, 2.2, 'f', None)
    head += L('M74 58 q6 4 12 0', 'f', 2) + C(70, 30, 3, 'u', None, o='.5') + C(94, 28, 2, 'u', None, o='.5')
    card(g, 'octopus', oct_ + head + C(30, 40, 3, 'i', 'f', 2) + C(36, 28, 2, 'i', 'f', 2), 'a')

    wolf = P('M44 100 Q24 96 32 78 Q40 86 56 84 Q54 64 66 54 Q72 46 76 34 L76 18 L86 28 Q94 22 108 14 L110 20 '
             'Q100 28 96 36 Q92 46 96 56 Q102 70 100 84 L102 100 L92 100 L90 88 L84 100 Z', 'm')
    wolf += C(88, 28, 1.8, 'f', None) + L('M66 64 Q72 74 68 86 M92 84 v14', 'f', 2, '.4')
    card(g, 'wolf', T(moon('w'), 40, 36, 1.4) + wolf + T(star('w', 4, 1.8), 130, 40) + T(star('w', 3, 1.4), 60, 18), 'i')

    trail = '<path d="M20 90 Q34 70 48 82" stroke="{{fg_muted}}" stroke-width="2.5" stroke-dasharray="4 6"/>'
    wings = T(E(0, -16, 11, 18, 'u', 'f', 2.5, '.75'), 82, 50, 1, -20) + T(E(0, -14, 9, 15, 'u', 'f', 2.5, '.75'), 92, 52, 1, 25)
    bee = P('M108 68 L124 70 L108 74 Z', 'f') + E(86, 70, 24, 18, 'w')
    bee += ''.join(L(f'M{x} {n(70 - 18 * math.sqrt(1 - ((x - 86) / 24) ** 2) + 3)} V{n(70 + 18 * math.sqrt(1 - ((x - 86) / 24) ** 2) - 3)}', 'f', 6) for x in (82, 96))
    bee += L('M56 54 Q50 40 42 40 M60 54 Q60 40 54 34', 'f', 2.5) + C(60, 66, 12, 'f') + C(56, 63, 3, 'u', None)
    card(g, 'bee', trail + bee + wings, 'w')

    waves = L('M18 102 q8 -5 16 0 q8 5 16 0 q8 -5 16 0 q8 5 16 0 q8 -5 16 0 q8 5 16 0 q8 -5 16 0 q8 5 16 0', 'i', 2.5)
    wh = P('M26 66 Q26 42 62 42 Q96 42 114 62 L124 50 Q120 40 112 36 Q126 34 132 42 Q140 32 150 34 Q144 44 132 50 '
           'L124 66 Q112 88 74 88 Q26 88 26 66 Z', 'i')
    wh += P('M30 74 Q44 88 74 88 Q100 88 112 76 Q80 82 56 78 Q40 76 30 74 Z', 'u', None) + C(44, 64, 2.6, 'f', None) + L('M34 72 q6 3 12 2', 'f', 2)
    spout = L('M58 38 Q56 26 48 22 M58 38 Q60 26 68 22 M58 38 v-16', 'i', 3)
    card(g, 'whale', spout + wh + waves, 'i')

    sn = P('M22 100 Q20 86 32 84 L36 68 Q42 62 46 70 L50 86 L118 92 Q126 100 116 102 L30 102 Q22 102 22 100 Z', 'm')
    sn += L('M38 72 l-6 -16 M44 72 l2 -16', 'f', 2.5) + C(32, 54, 3, 'f', None) + C(46, 54, 3, 'f', None)
    sn += C(88, 62, 28, 'w') + L(spiral(88, 64, 3, 22, 2.25, 160), 'f', 2.5) + L('M36 76 q4 2 6 0', 'f', 2)
    card(g, 'snail', sn + L('M126 100 h10', 'm', 2.5, '.5'), 'w')

    br = L('M24 98 Q80 92 140 98', 'm', 6) + T(leaf('s'), 128, 90, .7, 20)
    card(g, 'owl', br + T(owl('b'), 30, 10, .78) + T(moon('w'), 126, 28, 1) + T(star('w', 4, 1.8), 34, 28), 'b')


def build():
    kinds()
    card('body_form', 'person', person(80, 102, 'b', (30, 150), sc=1.05)[0] + T(sparkle('w', 6), 122, 30), 'b')
    objects()
    plants()
    elements()
    creatures()
