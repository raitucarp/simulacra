"""Machine, place and abstraction forms, plus the body parameters (material, scale, time, senses, motion, stance)."""
from kit import *


def dash(d, c='f', w=2.5, arr='4 5', o=None):
    op = f' stroke-opacity="{o}"' if o is not None else ''
    return f'<path d="{d}" stroke="{col(c)}" stroke-width="{n(w)}" stroke-dasharray="{arr}"{op}/>'


def band(d, c, w=5):
    """Solid stroke shape with an fg outline (river, handle, fork)."""
    return L(d, 'f', w + 4) + L(d, c, w)


def mini(x, y, sc=.4, f='a', arms=False):
    """Tiny reference person, feet at (x, y); strokes stay ~2px after scaling."""
    w = 2 / sc
    out = (L('M-14 -34 L-28 -60 M14 -34 L28 -60', 'f', w * 2.4) + L('M-14 -34 L-28 -60 M14 -34 L28 -60', f, w * .9)) if arms else ''
    out += P('M-16 0 Q-19 -40 0 -44 Q19 -40 16 0 Z', f, 'f', w) + C(0, -58, 13, 'u', 'f', w)
    return T(out, x, y, sc)


def being(x, y, sc=1, f='s', eyes='open', sprout=True):
    """A non-human companion figure (bottom centre at x, y, ~44 tall)."""
    w = 2.5 / sc
    out = P('M-20 0 Q-26 -26 -12 -36 Q0 -44 12 -36 Q26 -26 20 0 Z', f, 'f', w)
    if sprout:
        out += L('M0 -42 Q-1 -48 3 -52', 'f', w) + P('M3 -52 Q12 -58 14 -48 Q6 -45 3 -52 Z', f, 'f', w * .8)
    if eyes == 'big':
        out += E(0, -22, 10, 8, 'u', 'f', w) + C(-2, -22, 4.5, 'f', None)
    elif eyes == 'closed':
        out += L('M-11 -21 q3.5 3 7 0 M4 -21 q3.5 3 7 0 M-4 -10 h8', 'f', w)
    else:
        out += C(-7, -21, 3, 'f', None) + C(7, -21, 3, 'f', None)
        out += L('M-12 -30 l8 4 M12 -30 l-8 4', 'f', w) if eyes == 'angry' else L('M-4 -12 q4 3 8 0', 'f', w * .8)
    return T(out, x, y, sc)


def arc_arrow(cx, cy, r, a0, a1, c='m', w=3):
    def p(a, rr=r):
        t = math.radians(a)
        return cx + rr * math.cos(t), cy + rr * math.sin(t)
    (x0, y0), (x1, y1) = p(a0), p(a1)
    d = f'M{n(x0)} {n(y0)} A{r} {r} 0 {1 if a1 - a0 > 180 else 0} 1 {n(x1)} {n(y1)}'
    t = math.radians(a1)
    tx, ty = -math.sin(t), math.cos(t)
    head = poly(((x1 + tx * 7, y1 + ty * 7), p(a1, r + 5), p(a1, r - 5)))
    return L(d, c, w) + P(head, c, None)


def drop(x, y, s=1, f='i'):
    return T(P('M0 -9 Q7 0 7 4 Q7 9 0 9 Q-7 9 -7 4 Q-7 0 0 -9 Z', f, 'f', 2), x, y, s)


def orb(x, y, r, f='a'):
    return C(x, y, r, f) + L(f'M{n(x - r * .6)} {n(y - r * .2)} Q{n(x - r * .5)} {n(y - r * .6)} {n(x - r * .1)} {n(y - r * .65)}', 'u', 3, '.6')


# ---------- machines ----------
def machines():
    g = 'body_form'
    arm_ = lambda x, r: T(R(-5, 0, 10, 26, 5, 'm', 'f', 2), x, 62, 1, r)
    rb = L('M80 24 v-8', 'f', 3) + C(80, 14, 4, 'd') + arm_(60, 25) + arm_(100, -25)
    rb += R(66, 86, 10, 14, 3, 'm', 'f', 2) + R(84, 86, 10, 14, 3, 'm', 'f', 2)
    rb += R(58, 58, 44, 32, 7, 'i') + R(70, 64, 20, 12, 3, 'u', 'f', 2) + C(76, 70, 2.5, 'd', None) + C(84, 70, 2.5, 's', None)
    rb += R(52, 33, 8, 12, 3, 'm', 'f', 2) + R(100, 33, 8, 12, 3, 'm', 'f', 2) + R(58, 24, 44, 30, 9, 'i')
    rb += C(70, 38, 6, 'u', 'f', 2) + C(90, 38, 6, 'u', 'f', 2) + C(70, 38, 2.5, 'f', None) + C(90, 38, 2.5, 'f', None) + L('M74 48 h12', 'f', 2)
    card(g, 'robot', rb, 'i')

    pn = lambda x: R(x, -9, 34, 18, 2, 'b') + L(f'M{x + 11} -9 v18 M{x + 23} -9 v18 M{x} 0 h34', 'u', 1.5)
    st = pn(-54) + pn(20) + L('M-20 0 h40', 'f', 3) + R(-12, -13, 24, 26, 5, 'm') + L('M0 -13 v-6', 'f', 2.5)
    st += P('M-12 -28 Q0 -12 12 -28 Z', 'u', 'f', 2) + L('M0 -22 v-10', 'f', 2) + C(0, -33, 2.5, 'f', None)
    sat = dash('M18 92 Q80 56 142 78', 'm', 2.5, '4 6') + T(st, 80, 56, 1, -20)
    card(g, 'satellite', sat + T(sparkle('w', 6), 34, 30) + T(sparkle('w', 4), 128, 98), 'b')

    sv = R(50, 14, 60, 86, 6, 'm')
    for y in (20, 40, 60, 80):
        sv += R(56, y, 48, 14, 3, 'u', 'f', 2) + C(63, y + 7, 2.5, 's', None) + C(71, y + 7, 2.5, 'w', None) + L(f'M82 {y + 7} h16', 'm', 2)
    sv += L('M120 44 q6 7 0 14 M127 38 q12 13 0 26 M40 44 q-6 7 0 14 M33 38 q-12 13 0 26', 's', 2.5)
    card(g, 'server', sv, 's')

    tw = R(54, 18, 52, 46, 2, 'u') + lines(62, 28, (34, 26, 30), 8, 'm', 2.5)
    tw += P('M34 98 L44 66 L116 66 L126 98 Z', 'a') + R(42, 58, 76, 10, 5, 'm', 'f', 2.5) + C(38, 63, 5, 'm', 'f', 2) + C(122, 63, 5, 'm', 'f', 2)
    for i, y in enumerate((76, 84)):
        tw += ''.join(C(x, y, 3, 'u', 'f', 1.5) for x in range(52 + i * 5, 110, 10))
    card(g, 'typewriter', tw + R(64, 90, 32, 4, 2, 'u', None), 'a')

    dr = P('M70 70 L52 102 L108 102 L90 70 Z', 'i', None, o='.18') + L('M70 50 L44 40 V34 M90 50 L116 40 V34', 'f', 3)
    dr += E(44, 32, 20, 4, 'i', 'f', 2, '.6') + E(116, 32, 20, 4, 'i', 'f', 2, '.6') + C(44, 32, 2.5, 'f', None) + C(116, 32, 2.5, 'f', None)
    dr += L('M70 60 l-6 10 M90 60 l6 10', 'f', 2.5) + R(64, 44, 32, 18, 9, 'm') + C(80, 53, 2.5, 'd', None)
    card(g, 'drone', dr + C(80, 66, 6, 'f', None) + C(80, 66, 2.5, 'i', None), 'i')

    rd = L('M108 44 L122 22', 'f', 3) + C(122, 21, 3.5, 'f', None) + L('M128 16 q5 5 0 10 M134 11 q10 10 0 20', 'w', 2.5)
    rd += L('M52 44 Q52 30 74 30 Q96 30 96 44', 'f', 3) + R(34, 44, 92, 52, 10, 'w')
    rd += C(60, 70, 16, 'u', 'f', 2.5) + C(60, 70, 9, None, 'f', 2) + C(60, 70, 3, 'f', None)
    rd += R(86, 54, 30, 14, 3, 'u', 'f', 2) + L('M92 61 h18', 'm', 2) + L('M102 56 v10', 'd', 2.5)
    rd += C(93, 84, 5, 'm', 'f', 2) + C(109, 84, 5, 'm', 'f', 2) + R(42, 96, 10, 5, 2, 'f', None) + R(108, 96, 10, 5, 2, 'f', None)
    card(g, 'radio', rd, 'w')

    ca = R(52, 12, 56, 88, 9, 'm') + R(59, 19, 42, 18, 4, 's') + R(84, 23, 5, 10, 1.5, 'k', None) + R(92, 23, 5, 10, 1.5, 'k', None)
    for r in range(4):
        for c in range(3):
            fill = 'd' if (r, c) == (3, 2) else 'w' if c == 2 else 'u'
            ca += R(59 + c * 15, 44 + r * 13, 12, 10, 3, fill, 'f', 2)
    card(g, 'calculator', ca, 's')

    tr = L('M18 99 h124', 'm', 3) + C(60, 28, 5, 'u', 'f', 2) + C(48, 22, 7, 'u', 'f', 2) + C(34, 24, 5, 'u', 'f', 2)
    tr += R(52, 34, 12, 20, 2, 'm', 'f', 2.5) + R(36, 52, 62, 32, 8, 'd') + R(92, 34, 34, 50, 4, 'd') + R(88, 28, 42, 8, 3, 'm', 'f', 2.5)
    tr += R(100, 42, 18, 16, 3, 'u', 'f', 2) + C(36, 66, 5, 'w', 'f', 2) + P('M38 82 L26 94 L40 94 Z', 'm', 'f', 2)
    tr += ''.join(C(x, 88, 9, 'm', 'f', 3) + C(x, 88, 2.5, 'f', None) for x in (54, 78, 108))
    card(g, 'train', tr + L('M54 88 H108', 'f', 2.5) + L('M46 62 h40', 'u', 2.5, '.5'), 'd')


# ---------- places ----------
def pine(x, b, h, f):
    w = h * .4
    return (R(x - 3, b - h * .22, 6, h * .22, 2, 'm', 'f', 2) +
            P(poly(((x, b - h * .7), (x + w, b - h * .2), (x - w, b - h * .2))), f, 'f', 2.5) +
            P(poly(((x, b - h), (x + w * .75, b - h * .48), (x - w * .75, b - h * .48))), f, 'f', 2.5))


def places():
    g = 'body_form'
    ct = T(moon('w'), 34, 30, .8)
    for x, y, w, f in ((28, 56, 20, 'm'), (48, 34, 22, 'i'), (70, 48, 18, 'm'), (88, 24, 24, 'b'), (112, 52, 20, 'm')):
        ct += R(x, y, w, 100 - y, 2, f) + dash(f'M{x + 6} {y + 8} V94 M{x + w - 6} {y + 8} V94', 'u', 2.5, '3 5', '.7')
    card(g, 'city', ct + L('M100 24 v-12', 'f', 2.5) + L('M20 100 h120', 'f', 3), 'i')

    isl = E(80, 92, 56, 11, 'i') + P('M38 92 Q80 62 122 92 Z', 'w') + band('M84 84 Q80 60 92 40', 'm', 4)
    isl += P('M92 40 Q74 28 60 40 Q76 34 92 40 Z M92 40 Q86 22 70 18 Q90 18 92 40 Z M92 40 Q104 22 122 26 Q104 30 92 40 Z '
             'M92 40 Q114 40 118 58 Q104 46 92 40 Z', 's')
    isl += C(88, 44, 3.5, 'w', 'f', 2) + C(95, 45, 3.5, 'w', 'f', 2) + L('M28 98 q8 -5 16 0 M116 98 q8 -5 16 0', 'u', 2.5)
    card(g, 'island', isl + T(sun('w', 6, 8, 3), 36, 30), 'i')

    lb = R(28, 16, 104, 86, 5, 'u') + L('M28 44 h104 M28 72 h104', 'f', 3)
    rows = ((44, ((10, 22, 'a'), (8, 18, 'b'), (12, 24, 'w'), (8, 20, 'm'), (10, 22, 'b'), (11, 17, 'a'))),
            (72, ((9, 20, 'w'), (11, 24, 'a'), (8, 18, 'm'), (10, 22, 'b'), (12, 20, 'w'), (9, 24, 'a'))),
            (100, ((12, 22, 'b'), (9, 18, 'a'), (10, 24, 'w'), (8, 20, 'b'), (11, 22, 'm'), (9, 18, 'w'))))
    for b, specs in rows:
        x = 34
        for w, h, c in specs:
            lb += R(x, b - h, w, h, 1.5, c, 'f', 2)
            x += w + 2
        lb += T(R(0, -21, 9, 21, 1.5, 'w' if b == 72 else 'a', 'f', 2), x + 1, b - 1, 1, 22)
    card(g, 'library', lb, 'a')

    cd = ''
    for x in (32, 106):
        cd += R(x, 42, 22, 58, 2, 'm') + P(f'M{x - 2} 44 L{x + 11} 18 L{x + 24} 44 Z', 'b') + L(f'M{x + 11} 18 v-7 M{x + 8} 14 h6', 'f', 2)
        cd += P(f'M{x + 7} 72 v-10 q4 -7 8 0 v10 Z', 'k', 'f', 2)
    cd += P('M54 100 V54 L80 30 L106 54 V100 Z', 'u') + L('M80 30 v-8 M77 25 h6', 'f', 2)
    cd += C(80, 56, 10, 'w', 'f', 2.5) + L('M80 47 v18 M71 56 h18 M74 50 l12 12 M86 50 l-12 12', 'f', 1.5)
    cd += P('M70 100 V84 Q70 74 80 74 Q90 74 90 84 V100 Z', 'k', 'f', 2.5)
    card(g, 'cathedral', cd, 'b')

    cac = P('M55 96 V66 Q55 60 59 60 Q63 60 63 66 V96 Z M55 82 H50 Q46 82 46 78 V70 Q46 67 48.5 67 Q51 67 51 70 V77 H55 Z '
            'M63 76 H68 Q72 76 72 72 V66 Q72 63 69.5 63 Q67 63 67 66 V71 H63 Z', 's', 'f', 2.5)
    ds = T(sun('w', 11, 8, 5), 116, 34) + P('M22 96 Q56 60 92 84 Q116 66 140 96 Z', 'w', 'f', 2.5, '.5') + cac
    ds += P('M26 100 Q76 72 136 100 Z', 'w') + L('M30 44 q6 -4 12 0 t12 0 M40 56 q5 -3 10 0 t10 0', 'm', 2, '.6')
    card(g, 'desert', ds, 'w')

    fo = L('M22 98 h116', 'f', 3) + dash('M54 101 Q67 107 80 101 Q93 107 104 101', 'w', 2, '2 4')
    fo += pine(32, 98, 40, 'm') + pine(128, 98, 42, 'm') + pine(54, 98, 62, 's') + pine(106, 98, 58, 's') + pine(80, 98, 80, 's')
    card(g, 'forest', fo, 's')

    hb = R(18, 84, 124, 20, 9, 'i', None, o='.35') + L('M84 98 q6 -4 12 0 M114 92 q6 -4 12 0 M30 98 q6 -4 12 0', 'i', 2.5)
    hb += R(18, 74, 54, 8, 2, 'm') + L('M26 82 v14 M46 82 v14 M64 82 v14', 'f', 3)
    hb += P('M48 32 L76 22 L76 42 Z', 'w', None, o='.35') + P('M30 74 L34 36 L46 36 L50 74 Z', 'u') + L('M34 62 h12 M35 48 h10', 'd', 4)
    hb += R(32, 28, 16, 9, 2, 'w', 'f', 2) + P('M30 28 L40 18 L50 28 Z', 'd')
    hb += P('M106 36 L88 74 L106 74 Z M110 32 L132 76 L110 76 Z', 'u') + L('M108 80 V28', 'f', 3) + P('M82 80 H136 L126 94 H92 Z', 'w')
    card(g, 'harbor', hb, 'i')

    fl = lambda x, t, h: L(f'M{x + 5} {t + 5} V{t + h - 3} M{x + 11} {t + 5} V{t + h - 3}', 'm', 2)
    rn = R(34, 40, 16, 60, 2, 'u') + fl(34, 40, 60) + R(30, 34, 24, 7, 2, 'm')
    rn += P('M66 100 V66 L70 60 L74 66 L78 58 L82 64 V100 Z', 'u') + fl(66, 64, 36)
    rn += R(100, 30, 16, 70, 2, 'u') + fl(100, 30, 70) + R(96, 24, 24, 7, 2, 'm') + P('M88 24 L120 24 L120 16 L96 16 L92 20 Z', 'm')
    rn += T(R(-12, -7, 24, 14, 3, 'm'), 126, 94, 1, -10) + L('M20 100 h120', 'f', 3)
    rn += L('M40 100 Q32 82 44 68 Q52 56 42 44', 's', 2.5) + T(leaf('s'), 36, 82, .6, -40) + T(leaf('s'), 50, 60, .6, 30)
    card(g, 'ruin', rn, 's')


# ---------- abstractions ----------
def hourglass(sand='w'):
    hg = R(-22, -36, 44, 7, 3, 'b') + R(-22, 29, 44, 7, 3, 'b') + P('M-17 -29 h34 Q17 -10 2 0 Q17 10 17 29 h-34 Q-17 10 -2 0 Q-17 -10 -17 -29 Z', 'u')
    return hg + P('M-10 -18 h20 Q7 -8 0 -3 Q-7 -8 -10 -18 Z M-12 28 Q-8 14 0 12 Q8 14 12 28 Z', sand, None)


def abstractions():
    g = 'body_form'
    tm = arc_arrow(80, 60, 44, 150, 290, 'm') + arc_arrow(80, 60, 44, 330, 470, 'm')
    card(g, 'time', tm + T(hourglass('w') + L('M0 0 v12', 'w', 2), 80, 60), 'b')

    js = L('M80 28 V90', 'f', 3) + R(62, 88, 36, 8, 3, 'm') + L('M42 44 L118 32', 'f', 3) + C(80, 37, 4.5, 'w')
    js += L('M42 44 L28 74 M42 44 L56 74 M118 32 L106 60 M118 32 L130 60', 'f', 2)
    js += P('M24 74 h36 q-4 12 -18 12 q-14 0 -18 -12 Z', 'w') + P('M100 60 h36 q-4 12 -18 12 q-14 0 -18 -12 Z', 'w')
    js += R(34, 62, 16, 12, 2, 'm', 'f', 2) + T(leaf('u'), 118, 52, .7, -15)
    card(g, 'justice', js, 'w')

    ab = R(28, 18, 104, 76, 6, 'm') + R(36, 26, 88, 60, 3, 'u', 'f', 2) + L('M38 94 l-4 8 M122 94 l4 8', 'f', 3)
    for y, c, left, right in ((38, 'a', 3, 2), (56, 'w', 2, 3), (74, 'i', 1, 4)):
        ab += L(f'M36 {y} h88', 'f', 2)
        ab += ''.join(E(44 + i * 10, y, 5, 7, c, 'f', 2) for i in range(left)) + ''.join(E(116 - i * 10, y, 5, 7, c, 'f', 2) for i in range(right))
    card(g, 'number', ab, 'a')

    sl = C(70, 60, 30, 'u') + L('M54 52 q5 5 10 0 M76 52 q5 5 10 0', 'f', 2.5) + L('M54 72 H84', 'f', 3)
    sl += L(''.join(f'M{x} 68 v8 ' for x in range(57, 84, 5)), 'f', 1.5) + R(82, 70, 7, 12, 2.5, 'w', 'f', 2) + C(85.5, 78, 1.5, 'f', None)
    sl += P('M108 32 h7 l9 -8 v28 l-9 -8 h-7 Z', 'm', 'f', 2) + L('M130 33 l8 8 M138 33 l-8 8', 'd', 2.5)
    card(g, 'silence', sl, 'i')

    pol = lambda img: R(-18, -21, 36, 42, 3, 'u') + R(-13, -16, 26, 25, 1.5, 'b', None) + img
    me = T(pol(C(-5, -9, 3.5, 'w', None) + P('M-13 9 L-4 -2 L4 5 L8 1 L13 9 Z', 'm', None)), 52, 54, 1, -10)
    me += T(pol(T(heart('d', 'f', 2), 0, -3, .55)), 108, 66, 1, 9) + dash('M118 18 h18 v22 h-18 Z', 'm', 2, '3 4')
    me += L('M48 34 Q80 52 111 46 Q118 30 126 22', 'd', 2) + C(48, 34, 4, 'd', 'f', 2) + C(111, 46, 4, 'd', 'f', 2) + C(126, 22, 3.5, 'd', 'f', 2)
    card(g, 'memory', me, 'b')

    pip = lambda pts, c: ''.join(C(x, y, 3.2, c, None) for x, y in pts)
    d1 = R(-19, -19, 38, 38, 8, 'u') + pip(((-10, -10), (10, -10), (0, 0), (-10, 10), (10, 10)), 'f')
    d2 = R(-17, -17, 34, 34, 8, 'd') + pip(((-8, -8), (0, 0), (8, 8)), 'u')
    ch = T(d1, 60, 72, 1, -14) + T(d2, 108, 48, 1, 20) + L('M28 40 q6 -8 14 -8 M122 86 q8 0 12 -6', 'm', 2.5)
    card(g, 'chance', ch + T(sparkle('w', 6), 124, 18) + T(sparkle('w', 5), 30, 92), 'd')

    lv = L('M24 88 L130 36', 'f', 3) + L('M24 88 l-1 -9 M24 88 l-9 1 M30 85 l-1 -9 M30 85 l-9 1', 'f', 2.5)
    lv += T(heart('d'), 66, 58, 1.7) + T(heart('a'), 98, 64, 1.45) + P(poly(((138, 32), (128, 31.5), (132.5, 40.5))), 'f', None)
    card(g, 'love', lv + T(sparkle('w', 5), 120, 92), 'd')

    en = ''.join(R(26 + c * 12, 30 + r * 12, 9, 9, 2, 'a', 'f', 1.5) for r in range(4) for c in range(3))
    for x, y, rot, s, o in ((68, 34, 20, 8, 1), (74, 58, 35, 8, .9), (66, 80, 15, 8, 1), (88, 44, 50, 7, .8), (92, 70, 60, 6, .7), (84, 92, 30, 6, .7)):
        en += T(R(-s / 2, -s / 2, s, s, 1.5, 'a', 'f', 1.5, None if o == 1 else o), x, y, 1, rot)
    for x, y, r, o in ((106, 30, 3, .7), (110, 56, 2.5, .6), (104, 82, 3, .6), (122, 40, 2, .5), (120, 70, 2.5, .45),
                       (128, 92, 2, .4)):
        en += C(x, y, r, 'a', None, o=str(o))
    card(g, 'entropy', en, 'a')


# ---------- material ----------
def material():
    g = 'material'
    fl = T(heart('d'), 76, 58, 2.3) + L('M20 60 H46 L52 48 L60 76 L68 52 L73 60 H136', 'f', 3)
    card(g, 'flesh', fl + P('M124 76 Q132 88 132 92 Q132 100 124 100 Q116 100 116 92 Q116 88 124 76 Z', 'd'), 'd')

    fb = P('M50 56 H116 Q126 56 126 76 Q126 96 116 96 H50 Z', 'w') + L('M70 62 h28 M80 70 h34 M66 84 h30 M84 90 h24', 'f', 2, '.45')
    fb += E(50, 76, 14, 20, 'u') + E(50, 76, 9, 13, None, 'w', 2) + E(50, 76, 4, 6, None, 'w', 2) + L('M50 62 l2 8', 'f', 2)
    fb += L('M100 56 Q102 44 112 38', 'f', 2.5) + T(leaf('s'), 118, 32, .8, -30) + T(leaf('s'), 98, 44, .6, 200)
    card(g, 'fiber', fb, 'w')

    ing = lambda x, y: P(f'M{x} {y} h48 l-7 -16 h-34 Z', 'm') + L(f'M{x + 12} {y - 11} h18', 'u', 2.5, '.7')
    mt = ing(24, 98) + ing(76, 98) + ing(50, 82)
    mt += T(P(poly(ring_pts(0, 0, 14, 6, 30)), 'i') + C(0, 0, 6, 'u', 'f', 2.5), 112, 44) + T(sparkle('u', 6), 66, 70)
    card(g, 'metal', mt + T(sparkle('i', 5), 40, 52), 'i')

    st = P(poly(((28, 98), (34, 70), (52, 52), (78, 48), (96, 62), (102, 98))), 'm')
    st += L('M52 52 L60 72 L34 70 M60 72 L78 48 M60 72 L70 98 M96 62 L70 98', 'f', 2, '.5')
    st += P('M112 98 Q100 92 102 80 Q104 70 112 66 V58 H128 V66 Q136 70 138 80 Q140 92 128 98 Z', 'w') + R(108, 52, 24, 7, 3, 'w')
    card(g, 'stone', st + L('M104 82 h32', 'f', 2, '.5') + E(24, 100, 5, 3, 'm', 'f', 2), 'w')

    def cr(w, h):
        return (P(f'M{-w} 0 V{-h + w} L0 {-h} L{w} {-h + w} V0 Z', 'a', 'f', 2.5, '.55') +
                P(f'M{-w} {-h + w} L0 {-h} V0 H{-w} Z', 'u', None, '.35') + L(f'M0 {-h} V0', 'f', 2))
    gl = T(cr(8, 40), 60, 98, 1, -22) + T(cr(8, 36), 100, 98, 1, 24) + T(cr(11, 60), 80, 98) + E(80, 98, 36, 6, 'm')
    card(g, 'glass', gl + T(sparkle('b', 7), 112, 34) + T(sparkle('b', 5), 46, 40), 'a')

    pp = T(R(-26, -34, 52, 68, 3, 'u'), 76, 62, 1, -8) + P('M48 26 H94 L106 38 V96 H48 Z', 'u') + P('M94 26 V38 H106 Z', 'm', 'f', 2)
    pp += lines(56, 46, (40, 32, 40, 26, 36), 9, 'm', 2.5)
    qu = P('M0 0 Q-8 -30 6 -54 Q16 -28 0 0 Z', 'b') + L('M0 0 L5 -40', 'f', 2) + L('M0 0 v8', 'f', 3)
    card(g, 'paper', pp + T(qu, 112, 84, 1, 22) + C(110, 98, 3, 'f', None), 'b')

    cup = P('M-13 -18 H13 L9 18 H-9 Z', 'u') + P('M-11 -2 H11 L9 18 H-9 Z', 'i', None)
    lq = P('M64 30 Q84 34 86 90 L78 90 Q78 44 60 38 Z', 'i') + T(cup, 50, 40, 1, 62)
    lq += E(84, 94, 36, 8, 'i') + E(84, 94, 20, 4, None, 'u', 2) + drop(58, 80, .7) + drop(112, 78, .8) + drop(104, 62, .5)
    card(g, 'liquid', lq, 'i')

    ar = L('M24 46 H86 Q100 46 100 34 Q100 24 90 24 Q82 24 82 32', 'g', 3) + L('M30 64 H110 Q126 64 126 50 Q126 38 114 38 Q104 38 104 48', 'g', 3)
    ar += L('M38 82 H90 Q104 82 104 92 Q104 102 94 102 Q86 102 86 94', 'g', 3)
    ar += C(118, 24, 4, None, 'g', 2) + C(130, 30, 2.5, None, 'g', 2) + C(32, 98, 3, None, 'g', 2) + T(cloud('u', 'f', 2), 124, 94, .5)
    card(g, 'air', ar, 'g')

    en = C(80, 58, 32, 'w', None, o='.2') + P('M90 16 L58 64 H78 L68 102 L104 50 H84 Z', 'w')
    en += L('M42 40 l-8 -6 M38 62 h-10 M44 84 l-8 6 M118 40 l8 -6 M122 62 h10 M116 84 l8 6', 'w', 3)
    card(g, 'energy', en + T(sparkle('a', 6), 120, 22) + T(sparkle('a', 5), 40, 100), 'w')

    cube = 'M54 52 h40 v40 h-40 Z M54 52 l18 -16 h40 l-18 16 M112 36 v40 l-18 16'
    im = C(82, 62, 26, 'a', None, o='.18') + dash(cube, 'a', 3, '5 6') + T(sparkle('w', 9), 76, 70)
    card(g, 'immaterial', im + T(sparkle('w', 5), 100, 50) + C(124, 96, 2.5, 'a', None) + C(36, 30, 2, 'a', None), 'a')


# ---------- scale ----------
def scale():
    g = 'scale'
    mi = T(magnifier('u'), 68, 50, 2.2) + E(60, 42, 9, 6, 's', 'f', 2) + L('M51 42 q-4 4 -8 1', 'f', 2)
    mi += C(80, 60, 9, 's', 'f', 2) + C(82, 58, 3.5, 'f', None) + C(58, 62, 3, 's', None) + C(72, 32, 2.5, 's', None)
    card(g, 'microscopic', mi + C(28, 100, 1.6, 'f', None) + C(34, 96, 1.2, 'f', None), 's')

    pk = P(poly(((80, 20), (92, 34), (80, 50), (68, 34))), 'w') + L('M68 34 H92 M80 20 V50', 'f', 1.5, '.5')
    pk += P('M42 40 H118 V70 Q118 98 80 100 Q42 98 42 70 Z', 'i') + dash('M50 48 H110 V70 Q110 90 80 92 Q50 90 50 70 Z', 'u', 2, '3 4')
    card(g, 'pocket', pk + C(47, 45, 2.5, 'w', None) + C(113, 45, 2.5, 'w', None) + T(sparkle('w', 5), 106, 24), 'a')

    p, _ = person(52, 100, 'a', (20, 20))
    hs = dash('M28 32 H132', 'm', 2, '4 5') + p + R(92, 32, 30, 68, 15, 'b') + C(101, 50, 3, 'f', None) + C(113, 50, 3, 'f', None)
    hs += L('M138 32 V100', 'f', 2.5) + L('M134 32 h8 M136 49 h4 M134 66 h8 M136 83 h4 M134 100 h8', 'f', 2)
    card(g, 'human_sized', hs, 'b')

    bd = R(70, 14, 44, 86, 3, 'm') + dash('M80 22 V82 M92 22 V82 M104 22 V82', 'w', 3, '4 5') + R(86, 86, 12, 14, 2, 'k', 'f', 2)
    bd += L('M128 16 V98', 'f', 2) + P('M128 12 l-4 7 h8 Z M128 102 l-4 -7 h8 Z', 'f', None) + L('M24 100 h96', 'f', 3)
    card(g, 'building', bd + mini(48, 100, .32, 'a'), 'w')

    ls = P('M20 82 L44 50 L58 64 L78 34 L100 62 L116 46 L140 82 Z', 'm', 'f', 2.5) + P('M78 34 L86 45 L80 43 L74 47 L70 44 Z', 'u', None)
    ls += P('M18 98 Q40 72 76 82 Q108 92 142 72 V98 Z', 's') + P('M66 98 Q70 88 86 86 Q76 92 80 98 Z', 'i', None)
    card(g, 'landscape', ls + T(sun('w', 7, 8, 3), 128, 26) + mini(44, 82, .22, 'w'), 's')

    pl = C(80, 64, 34, 'i') + P('M58 46 Q66 40 74 46 Q78 54 70 58 Q66 66 58 62 Q50 56 58 46 Z M88 70 Q98 64 106 72 Q110 84 98 88 Q90 90 88 82 Z '
                                'M92 40 Q100 37 104 44 Q98 48 92 40 Z', 's', None) + C(80, 64, 34, None, 'f', 3)
    card(g, 'planetary', pl + mini(80, 30, .24, 'w') + T(star('w', 4, 1.8), 34, 30) + T(star('w', 4, 1.8), 130, 36) + C(124, 96, 2, 'f', None), 'i')

    gx = L('M80 58 Q80 34 104 38 Q124 44 122 66', 'a', 5) + L('M80 58 Q80 82 56 78 Q36 72 38 50', 'a', 5)
    gx += L('M104 38 Q130 34 136 56 M56 78 Q30 82 24 60', 'a', 3, '.5') + C(80, 58, 14, 'w', None, o='.3') + C(80, 58, 8, 'w')
    gx += C(124, 94, 6, 'b') + E(124, 94, 11, 3, None, 'f', 2) + T(sparkle('w', 5), 36, 28) + T(sparkle('w', 4), 128, 22)
    card(g, 'cosmic', gx + C(50, 100, 1.8, 'u', None) + C(104, 18, 1.8, 'u', None) + C(140, 74, 1.5, 'u', None), 'a')


# ---------- temporality ----------
def temporality():
    g = 'temporality'
    sw = R(74, 18, 12, 8, 3, 'm', 'f', 2.5) + L('M80 26 v6', 'f', 3) + T(R(-4, -4, 8, 8, 2, 'm', 'f', 2), 104, 38, 1, 45)
    sw += T(clock('u', 'f', 'd', .12, 30), 80, 64) + L('M24 52 h16 M20 64 h18 M26 76 h14', 'm', 3)
    card(g, 'moments', sw + T(sparkle('w', 7), 122, 96), 'd')

    cal = R(36, 24, 88, 74, 8, 'u') + P('M36 42 V32 Q36 24 44 24 H116 Q124 24 124 32 V42 Z', 'w') + L('M58 18 v12 M102 18 v12', 'f', 4)
    cal += C(48, 33, 4, 'u', 'f', 2) + T(moon('u', 'f'), 113, 33, .4)
    cal += ''.join(R(44 + c * 11, 50 + r * 14, 8, 9, 2, 'i' if c > 4 else 'm', None) for r in range(3) for c in range(7))
    card(g, 'days', cal + R(64, 62, 12, 13, 3, None, 'w', 2.5), 'w')

    se = P('M80 58 V22 A36 36 0 0 0 44 58 Z', 's', None) + P('M80 58 H116 A36 36 0 0 0 80 22 Z', 'w', None)
    se += P('M80 58 V94 A36 36 0 0 0 116 58 Z', 'd', None) + P('M80 58 H44 A36 36 0 0 0 80 94 Z', 'u', None)
    se += C(80, 58, 36, None, 'f', 3) + L('M80 22 V94 M44 58 H116', 'f', 2.5)
    se += L('M62 50 V40', 'f', 2) + P('M62 42 Q54 42 54 34 Q62 34 62 42 Z M62 40 Q70 40 70 32 Q62 32 62 40 Z', 'u', 'f', 2)
    se += T(sun('u', 5, 8, 3), 98, 40) + T(leaf('u'), 98, 76, .8, -30) + L('M62 68 v16 M55 72 l14 8 M55 80 l14 -8', 'f', 2.5)
    card(g, 'seasons', se, 's')

    el, eh = person(40, 100, 'm', (None, 12), 'flat', .7)
    ad, _ = person(80, 100, 'a', (15, 15), sc=.9)
    ch, _ = person(118, 100, 'b', (None, None), sc=.5)
    hx, hy = eh[1]
    card(g, 'lifetimes', el + L(f'M{n(hx)} {n(hy)} V100 M{n(hx)} {n(hy)} q0 -6 -5 -6', 'f', 3) + ad + ch, 'i')

    ce = L('M22 96 H132', 'f', 3) + P('M138 96 l-8 -5 v10 Z', 'f', None) + L('M42 92 v8 M80 92 v8 M118 92 v8', 'f', 2.5)
    ce += P('M24 88 L42 58 L60 88 Z', 'w') + L('M42 58 L48 88', 'f', 2)
    ce += P('M62 54 L80 42 L98 54 Z', 'u') + ''.join(R(x, 56, 5, 26, 1, 'u', 'f', 2) for x in (66, 74, 82, 90)) + R(62, 82, 36, 6, 1.5, 'u', 'f', 2)
    ce += R(108, 30, 20, 58, 2, 'b') + dash('M114 36 V82 M122 36 V82', 'u', 2.5, '3 4', '.7') + L('M118 30 v-10', 'f', 2)
    card(g, 'centuries', ce + dash('M30 48 Q80 0 130 18', 'm', 2, '3 6'), 'b')

    def top(yb):
        return f'M26 42 Q26 34 34 34 H126 Q134 34 134 42 V{yb} Q107 {yb - 7} 80 {yb} Q53 {yb + 7} 26 {yb} Z'
    gl = R(26, 34, 108, 66, 8, 'm') + P(top(86), 'd', None, '.7') + P(top(72), 'w', None, '.55') + P(top(58), 'w', None) + P(top(44), 's', None)
    gl += L('M104 80 a2 2 0 1 1 2 -2 a4 4 0 1 1 -4 -4 a6 6 0 1 1 6 6', 'f', 2) + C(48, 92, 2.5, 'u', None, o='.5') + C(110, 94, 3, 'u', None, o='.5')
    gl += R(26, 34, 108, 66, 8, None, 'f', 3) + L('M44 34 l-3 -8 M48 34 l0 -10 M52 34 l3 -8 M110 34 l-2 -7 M114 34 l2 -7', 's', 2.5)
    card(g, 'geologic', gl, 'w')

    hg = T(hourglass('a'), 80, 60, 1, 90)
    card(g, 'timeless', hg + T(star('w', 7, 3), 44, 30) + T(sparkle('w', 7), 120, 26) + T(star('w', 5, 2.2), 122, 92) + T(sparkle('w', 5), 38, 90), 'a')


# ---------- perception ----------
def perception():
    g = 'perception'
    hs = C(54, 60, 6, 'u', 'f', 2.5) + C(106, 60, 6, 'u', 'f', 2.5) + C(80, 58, 26, 'u')
    hs += E(70, 52, 5, 6, 'u', 'f', 2) + E(90, 52, 5, 6, 'u', 'f', 2) + C(70, 53, 2.5, 'f', None) + C(90, 53, 2.5, 'f', None)
    hs += L('M80 56 l-3 8 h5 M72 72 q8 6 16 0', 'f', 2.5) + L('M116 52 q5 8 0 16 M123 46 q9 14 0 28', 'a', 2.5)
    hs += L('M34 30 l10 8 M30 44 l11 2 M44 22 l6 10', 'w', 2.5) + L('M44 98 q-5 -6 0 -10 q5 -4 0 -10 M34 100 q-4 -5 0 -9', 's', 2.5)
    card(g, 'human_senses', hs, 'a')

    lt = L('M92 52 L142 32', 'd', 4) + L('M94 58 L142 52', 'w', 4) + L('M96 64 L142 72', 'i', 4)
    lt += P('M78 24 L108 82 L48 82 Z', 'u') + L('M18 70 L62 58', 'f', 3) + L('M70 40 L60 62', 'u', 3, '.5')
    card(g, 'light', lt, 'w')

    fork = 'M68 18 V50 Q68 64 80 64 Q92 64 92 50 V18'
    vb = band(fork, 'm', 4) + band('M80 64 V98', 'm', 4)
    vb += L('M56 24 q-8 12 0 24 M46 18 q-14 18 0 36 M104 24 q8 12 0 24 M114 18 q14 18 0 36', 'b', 3)
    card(g, 'vibration', vb + L('M26 84 q6 -8 12 0 t12 0 M110 84 q6 -8 12 0 t12 0', 'b', 2.5), 'b')

    pts = ring_pts(92, 52, 18, 6, 30)
    mo = P(poly(pts), None, 'f', 3) + L(f'M{n(pts[0][0])} {n(pts[0][1])} l14 8 M{n(pts[4][0])} {n(pts[4][1])} l0 -14', 'f', 3)
    mo += C(pts[0][0] + 14, pts[0][1] + 8, 6, 'w', 'f', 2.5) + C(pts[4][0], pts[4][1] - 14, 6, 'w', 'f', 2.5)
    mo += ''.join(C(x, y, 6, 's' if i % 2 else 'u', 'f', 2.5) for i, (x, y) in enumerate(pts))
    fk = P('M28 98 L36 82 V72 H46 V82 L54 98 Z', 'u') + P('M31 92 L37 84 H45 L51 92 L54 98 H28 Z', 's', None)
    fk += L('M41 70 q-5 -8 0 -14 t2 -14 M50 66 q4 -6 10 -6', 's', 2.5, '.7')
    card(g, 'chemical', mo + fk, 's')

    to = ''.join(R(x, y, 10, 44, 5, 'u', 'f', 2.5) for x, y in ((58, 26), (70, 18), (82, 20), (94, 28)))
    to += T(R(-5, 0, 10, 28, 5, 'u', 'f', 2.5), 60, 74, 1, 130) + R(58, 52, 46, 44, 14, 'u', 'f', 2.5)
    to += C(81, 74, 16, 'd', None, o='.3') + C(81, 74, 8, 'd', None, o='.6') + L('M62 26 v6 M74 18 v6 M86 20 v6 M98 28 v6', 'f', 1.5, '.4')
    card(g, 'touch', to + L('M118 60 q5 8 0 16 M126 54 q9 14 0 28 M30 92 l8 -4 M28 80 h8', 'w', 2.5), 'd')

    sg = L('M80 36 L64 100 M80 36 L96 100 M74 56 H86 M70 76 H90 M67 88 H93 M74 56 L90 76 M86 56 L70 76 M70 76 L93 88 M90 76 L67 88', 'f', 2.5)
    sg += C(80, 32, 5, 'd', 'f', 2) + L('M68 24 q-6 8 0 16 M60 18 q-10 14 0 28 M52 12 q-14 20 0 40 M92 24 q6 8 0 16 M100 18 q10 14 0 28 M108 12 q14 20 0 40', 'i', 3)
    card(g, 'signals', sg + dash('M24 98 H56', 'i', 3, '2 5') + dash('M104 98 H136', 'i', 3, '2 5'), 'i')

    sy = dash('M26 60 A54 38 0 1 1 134 60 A54 38 0 1 1 26 60', 'm', 2, '3 5') + P('M50 60 Q80 36 110 60 Q80 84 50 60 Z', 'u')
    sy += C(80, 60, 11, 'g') + C(80, 60, 4.5, 'f', None) + C(42, 33, 7, 'w', 'f', 2.5) + P(poly(((118, 24), (127, 40), (109, 40))), 'w', 'f', 2.5)
    sy += R(127, 53, 14, 14, 2.5, 'w', 'f', 2.5) + T(star('w', 8, 3.5), 42, 87) + P(poly(((107, 85), (114, 93), (107, 101), (100, 93))), 'w', 'f', 2.5)
    card(g, 'symbolic', sy, 'g')


# ---------- mobility ----------
def mobility():
    g = 'mobility'
    fx = P('M22 80 H138 V96 Q138 104 130 104 H30 Q22 104 22 96 Z', 'w', None, o='.25') + L('M22 80 H138', 'f', 3)
    fx += L('M80 80 Q78 92 66 98 M80 80 Q84 94 96 100 M78 84 Q68 86 54 86 M82 84 Q96 86 108 92', 'f', 2.5)
    fx += P('M74 81 Q76 62 76 50 H84 Q84 62 86 81 Z', 'm')
    fx += P('M56 56 Q44 56 46 44 Q48 32 58 32 Q60 18 80 16 Q100 18 102 32 Q114 32 114 44 Q114 58 102 56 Z', 's')
    card(g, 'fixed', fx, 's')

    p, hands = person(60, 100, 'b', (15, 90), sc=.9)
    hx, hy = hands[1]
    ln = C(hx, hy + 22, 18, 'w', None, o='.25') + L(f'M{n(hx)} {n(hy)} v6', 'f', 2) + R(hx - 7, hy + 6, 14, 5, 2, 'm', 'f', 2)
    ln += R(hx - 8, hy + 11, 16, 20, 5, 'w', 'f', 2.5) + C(hx, hy + 21, 3.5, 'u', None)
    card(g, 'carried', p + ln + L('M22 52 h14 M18 64 h16 M24 76 h12', 'm', 3), 'w')

    cp = dash('M18 100 H40', 'm', 2.5, '2 5') + L('M30 100 H140', 'f', 3, '.4')
    cp += ''.join(C(46 + 50 * t, 92 - 28 * math.sin(math.pi * t), 7, 's', 'f', 2) for t in (i / 8 for i in range(9)))
    cp += C(104, 88, 9, 's', 'f', 2) + C(107, 86, 2, 'f', None) + L('M104 80 l1 -8 M109 81 l6 -6', 'f', 2)
    card(g, 'creeping', cp, 'b')

    leg = lambda x, r: T(R(-4.5, 0, 9, 24, 4.5, 'd', 'f', 2), x, 76, 1, r)
    wk = leg(74, 28) + leg(86, -28) + E(62, 98, 7, 3.5, 'f', None) + E(98, 98, 7, 3.5, 'f', None)
    p, _ = person(80, 80, 'd', (40, 40), sc=.8)
    wk += dash('M22 101 H48', 'm', 2.5, '2 6') + L('M32 46 h14 M26 58 h16 M34 70 h12', 'm', 3)
    card(g, 'walking', wk + p, 'd')

    rv = 'M30 36 Q60 30 70 52 Q80 76 112 72 Q130 70 130 90'
    fw = band(rv, 'i', 16)
    for x, y, r in ((44, 34, 0), (72, 58, 65), (110, 72, 0)):
        fw += T(L('M-3 -5 L3 0 L-3 5', 'u', 2.5), x, y, 1, r)
    card(g, 'flowing', fw + T(leaf('s'), 124, 84, .6, 40), 'i')

    bd = dash('M22 96 Q34 66 64 70', 'm', 2.5, '4 6') + T(cloud('u', 'f', 2), 40, 30, .6) + T(cloud('u', 'f', 2), 126, 88, .5)
    bd += P('M68 58 l-12 -6 l2 10 Z', 'b') + E(86, 58, 18, 8, 'b') + P('M80 54 Q70 26 92 14 Q94 36 90 54 Z', 'b')
    bd += C(104, 52, 7, 'b') + P('M110 51 l8 2 l-8 3 Z', 'w', 'f', 2) + C(106, 50, 1.6, 'f', None) + P('M78 62 Q62 70 58 84 Q74 78 88 64 Z', 'b')
    card(g, 'flying', bd, 'b')

    spokes = ''.join(f'M48 54 L{n(48 + 14 * math.cos(a))} {n(54 + 14 * math.sin(a))} ' for a in (math.radians(k * 30) for k in range(12)))
    dd = L('M44 102 Q40 80 48 60', 's', 3) + L(spokes, 'm', 2) + C(48, 54, 4, 'm', None) + dash('M58 46 Q92 24 136 40', 'm', 2, '4 6')
    for x, y, r in ((74, 40, 30), (94, 30, 50), (112, 44, 70), (128, 28, 40), (102, 60, 60)):
        dd += T(L('M0 0 V-10 M0 -10 l-5 -5 M0 -10 V-17 M0 -10 l5 -5', 'f', 2) + C(0, 1, 2, 'f', None), x, y, 1, r)
    card(g, 'drifting', dd + L('M22 68 h16 M28 80 h12', 'm', 2.5, '.6'), 'a')

    ev = C(80, 58, 34, 'u') + E(80, 58, 15, 34, None, 'm', 2) + L('M46 58 H114 M51 40 H109 M51 76 H109', 'm', 2) + C(80, 58, 34, None, 'f', 3)
    for x, y in ((64, 44), (96, 40), (88, 66), (62, 72), (104, 82), (78, 28), (30, 30), (132, 32), (28, 88), (136, 94), (124, 62), (40, 58)):
        ev += C(x, y, 3.5, 'g', 'f', 1.5)
    card(g, 'everywhere', ev + C(96, 40, 8, None, 'g', 2) + C(30, 30, 8, None, 'g', 2) + C(136, 94, 7, None, 'g', 2), 'g')


# ---------- human-likeness ----------
def mug_being(level):
    yb = 88 if level == 3 else 100
    t = yb - 50
    out = ''
    if level == 3:
        out += R(62, 86, 8, 12, 3, 'm', 'f', 2) + R(82, 86, 8, 12, 3, 'm', 'f', 2) + E(64, 100, 8, 3.5, 'f', None) + E(88, 100, 8, 3.5, 'f', None)
    out += L(f'M68 {t - 4} q-4 -6 0 -12 M84 {t - 4} q-4 -6 0 -12', 'm', 2.5) + band(f'M100 {t + 12} q16 0 16 14 q0 14 -16 14', 'w', 4)
    if level >= 2:
        arm_ = lambda x, y, r: T(R(-3.5, 0, 7, 22, 3.5, 'w', 'f', 2) + C(0, 22, 4, 'u', 'f', 2), x, y, 1, r)
        out += arm_(54, t + 26, 140 if level == 3 else 35) + arm_(100, t + 42, -90 if level == 3 else -35)
    out += R(52, t, 48, 50, 8, 'w') + L(f'M56 {t + 8} h40', 'f', 2, '.35')
    if level == 1:
        out += C(68, t + 24, 2.2, 'f', None) + C(84, t + 24, 2.2, 'f', None)
    elif level >= 2:
        out += C(67, t + 23, 5, 'u', 'f', 2) + C(85, t + 23, 5, 'u', 'f', 2) + C(68, t + 24, 2.2, 'f', None) + C(86, t + 24, 2.2, 'f', None)
        out += L(f'M70 {t + 34} q6 5 12 0', 'f', 2.5) if level == 2 else E(76, t + 36, 5, 4, 'f', None)
    if level == 3:
        out += L(f'M61 {t + 13} l8 -2 M83 {t + 11} l8 2', 'f', 2.5) + P(f'M68 {t + 46} l8 -4 l8 4 v-8 l-8 4 l-8 -4 Z', 'd', 'f', 1.5)
        out += bubble(106, 16, 30, 18, 'u') + T(heart('d', 'f', 1.5), 121, 25, .4)
    return out


def anthropomorphism():
    g = 'anthropomorphism'
    for i, (key, hue) in enumerate((('none', 'i'), ('subtle', 'a'), ('partial', 'b'), ('full', 'd'))):
        card(g, key, mug_being(i), hue)


# ---------- relation to humans ----------
def relation():
    g = 'relation_to_humans'
    p, _ = person(52, 100, 'a', (20, 62), sc=.85)
    card(g, 'kin', p + L('M80 76 h8', 'f', 3) + being(108, 100, 1.3), 'a')

    p, _ = person(56, 100, 'a', (20, 40), sc=.85)
    card(g, 'companion', p + being(106, 100, .7) + T(heart('d'), 84, 40, .7) + T(heart('d'), 108, 54, .35), 'd')

    p, _ = person(46, 100, 'a', (20, 20), sc=.85)
    tray = L('M92 82 L78 60', 'f', 3) + R(64, 54, 30, 5, 2, 'm', 'f', 2) + P('M72 44 h12 q0 10 -6 10 q-6 0 -6 -10 Z', 'u', 'f', 2)
    tray += L('M76 38 q-3 -4 0 -8', 'm', 2)
    card(g, 'servant', p + T(being(0, 0, 1.1), 108, 100, 1, -12) + tray, 's')

    p, _ = person(44, 100, 'a', (30, 30), sc=.6)
    wt = being(116, 100, 1.4, eyes='big') + R(90, 72, 52, 28, 4, 'm') + L('M90 86 h52 M106 72 v14 M124 86 v14', 'f', 2, '.4')
    card(g, 'witness', p + wt + dash('M112 66 Q80 52 52 60', 'i', 2, '3 5'), 'i')

    p, _ = person(46, 100, 'a', (20, 70), 'flat', .85)
    bolt = P('M84 34 L72 56 H82 L76 74 L92 48 H82 Z', 'w')
    card(g, 'rival', p + being(114, 100, 1.25, eyes='angry') + bolt, 'w')

    card(g, 'indifferent', being(100, 100, 1.6, eyes='closed') + mini(34, 100, .45, 'a', True) + C(40, 46, 2, 'm', None) + C(46, 40, 2.5, 'm', None), 'b')

    rays = ''.join(L(f'M{n(80 + 30 * math.cos(a))} {n(44 + 30 * math.sin(a))} L{n(80 + 40 * math.cos(a))} {n(44 + 40 * math.sin(a))}', 'w', 3)
                   for a in (math.radians(k) for k in (200, 230, 260, 280, 310, 340)))
    dt = C(80, 44, 28, 'w', None, o='.2') + rays + being(80, 66, 1, sprout=False) + E(80, 18, 13, 3.5, None, 'w', 3)
    card(g, 'deity', dt + mini(80, 100, .4, 'a', True), 'w')


# ---------- withdrawal ----------
def withdrawal():
    g = 'withdrawal'
    rv = P('M80 60 V30 A30 30 0 1 0 110 60 Z', 'a') + L('M60 54 Q62 42 72 38', 'u', 3, '.6')
    wedge = P('M80 60 V30 A30 30 0 0 1 110 60 Z', 'w') + P('M80 60 V42 A18 18 0 0 1 98 60 Z', 'u') + P('M80 60 V51 A9 9 0 0 1 89 60 Z', 'd')
    rv += T(wedge, 7, -7) + L('M122 30 l8 -6 M126 46 h10 M106 16 l4 -8', 'w', 3)
    card(g, 'fully_revealed', rv, 'w')

    op = orb(80, 56, 18) + L('M68 30 l-4 -8 M80 26 v-10 M92 30 l4 -8', 'w', 3)
    op += P('M50 64 L32 50 L42 44 L60 64 Z M110 64 L128 50 L118 44 L100 64 Z', 'm') + R(50, 64, 60, 36, 3, 'm') + L('M50 74 h60', 'f', 2, '.4')
    card(g, 'open', op, 'a')

    card(g, 'guarded', orb(80, 46, 22) + T(shield('i'), 80, 70, 1.9) + L('M68 62 L80 56 L92 62 M70 76 L80 72 L90 76', 'u', 3, '.5'), 'i')

    el = orb(80, 50, 24) + R(30, 56, 72, 13, 6.5, 'm', None, o='.9') + R(62, 70, 74, 13, 6.5, 'm', None, o='.9')
    el += R(36, 84, 70, 12, 6, 'm', None, o='.75') + L('M24 40 h18 M118 36 h16 M112 98 h20', 'm', 3, '.6')
    card(g, 'elusive', el, 'a')

    berg = 'M60 46 L40 70 L48 94 L80 104 L112 92 L120 66 L98 46 Z'
    dw = P('M16 46 H146 V86 Q146 108 118 108 H42 Q14 108 14 86 Z', 'i', None, o='.2') + P(berg, 'u', None, o='.3') + dash(berg, 'f', 2, '4 5', '.5')
    dw += P('M64 46 L74 30 L82 36 L94 46 Z', 'u', 'f', 2.5) + L('M18 46 q6 -4 12 0 t12 0 t12 0 M100 46 q6 -4 12 0 t12 0 t12 0', 'i', 2.5)
    card(g, 'deeply_withdrawn', dw, 'i')


def build():
    machines()
    places()
    abstractions()
    material()
    scale()
    temporality()
    perception()
    mobility()
    anthropomorphism()
    relation()
    withdrawal()
