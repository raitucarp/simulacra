"""Rhetoric, style, semiotics, structure, post format, lexical fields and weekdays, plus the Embodiment and Rhetoric step banners."""
from kit import *
from cards_mind import qmark
from scenes import banner


# ---------- local props (drawn around the origin) ----------
def fish(f='i'):
    return (P('M16 0 L30 -11 L28 0 L30 11 Z', f) + E(0, 0, 20, 12, f) + C(-10, -3, 2.2, 'f', None) +
            L('M-1 -7 q4 7 0 14', 'f', 2))


def quill(f='u'):
    return P('M0 0 Q2 -22 20 -38 Q22 -16 4 -2 Z', f, 'f', 2.5) + L('M1 -1 L15 -25', 'f', 2) + L('M0 0 l-4 7', 'f', 3)


def check(x, y, s=1, c='u', w=3):
    return T(L('M-6 0 L-2 4 L6 -4', c, w), x, y, s)


def scales(f='w'):
    out = L('M0 -40 V0', 'f', 3) + R(-14, -2, 28, 6, 3, 'm', 'f', 2) + L('M-26 -38 L26 -42', 'f', 3) + C(0, -40, 4, f, 'f', 2)
    out += L('M-26 -38 L-34 -18 M-26 -38 L-18 -18 M26 -42 L18 -24 M26 -42 L34 -24', 'f', 2)
    return out + P('M-38 -18 h24 q-3 9 -12 9 q-9 0 -12 -9 Z', f, 'f', 2) + P('M14 -24 h24 q-3 9 -12 9 q-9 0 -12 -9 Z', f, 'f', 2)


def bolt(f='w'):
    return P('M4 -18 L-8 2 L0 2 L-4 18 L10 -4 L2 -4 Z', f, 'f', 2.5)


def planet(f='b', ring='w', r=12):
    back = P(f'M{n(-r * 1.8)} 0 A{n(r * 1.8)} {n(r * .5)} 0 0 1 {n(r * 1.8)} 0', None, ring, 3)
    front = P(f'M{n(-r * 1.8)} 0 A{n(r * 1.8)} {n(r * .5)} 0 0 0 {n(r * 1.8)} 0', None, ring, 3)
    return T(back + C(0, 0, r, f, 'f', 2.5) + front, 0, 0, 1, -18)


def postcard(x, y, w, h, av='a', widths=(), gap=8):
    return R(x, y, w, h, 7, 'u') + C(x + 11, y + 11, 5, av, 'f', 2) + L(f'M{n(x + 21)} {n(y + 11)} h{n(w * .3)}', 'm', 3) + lines(x + 8, y + 24, widths, gap)


# ---------- rhetorical mode ----------
def modes():
    g = 'rhetorical_mode'
    st = R(26, 76, 30, 24, 3, 'u') + R(56, 60, 30, 40, 3, 'u') + R(86, 44, 30, 56, 3, 'u')
    st += lines(32, 86, (18,), 0, 'i') + lines(62, 70, (18, 12), 8, 'i') + lines(92, 54, (18, 12, 18), 8, 'i')
    st += L('M30 68 Q46 56 58 54 M62 50 Q76 40 90 38', 'm', 2.5) + C(122, 30, 13, 's') + check(122, 30)
    card(g, 'logos', st, 'i')
    medal = L('M-5 -42 L0 -30 L5 -42', 'i', 3) + C(0, -27, 5, 'w', 'f', 2)
    p, _ = person(62, 100, 's', (20, 20), front=medal)
    sh = T(shield('i') + T(star('w', 7, 3), 0, -1), 116, 54, 1.4)
    card(g, 'ethos', sh + p, 's')
    ht = T(heart('d'), 80, 60, 2.3) + L('M62 48 q6 5 12 0 M86 48 q6 5 12 0', 'u', 3) + P('M68 56 Q72 62 72 66 A4 4 0 0 1 64 66 Q64 62 68 56 Z', 'i', 'f', 2)
    card(g, 'pathos', ht + L('M28 34 q-6 8 0 16 M132 34 q6 8 0 16', 'd', 2.5), 'd')
    pin = P('M0 0 Q-11 -12 -11 -20 A11 11 0 1 1 11 -20 Q11 -12 0 0 Z', 'd') + C(0, -20, 4, 'u', None)
    ck = T(clock('u', 'f', 'w', .14, 28), 72, 68) + T(pin, 86, 44) + T(sparkle('w', 7), 120, 30) + T(sparkle('w', 5), 116, 84)
    card(g, 'kairos', ck, 'w')
    dl = C(40, 82, 14, 'a') + R(106, 68, 28, 28, 6, 'b') + L('M48 68 L66 48 M112 68 L94 48', 'f', 3)
    dl += P('M60 46 l9 -2 l-2 9 Z M100 46 l-9 -2 l2 9 Z', 'f', 'f', 2)
    dl += P('M80 18 A16 16 0 0 0 80 50 Z', 'a', None) + P('M80 18 A16 16 0 0 1 80 50 Z', 'b', None) + C(80, 34, 16, None)
    dl += L('M58 84 l8 -4 M102 84 l-8 -4 M80 76 v8', 'w', 3)
    card(g, 'dialectic', dl, 'a')
    nr = T(book('b'), 80, 88, 1.3) + '<path d="M50 72 Q46 46 70 46 Q96 46 92 30 Q90 22 104 22" stroke="{{fg_muted}}" stroke-width="2.5" stroke-dasharray="4 6"/>'
    nr += C(50, 72, 4, 'f', None) + L('M108 30 V8', 'f', 2.5) + P('M108 8 h16 l-4 5 l4 5 h-16 Z', 'w', 'f', 2)
    card(g, 'narrative', nr, 'b')
    pl = bubble(54, 20, 72, 44, 'u', 'r') + lines(66, 34, (40, 28), 10) + L('M90 22 l-4 10 l6 6 l-4 8 M116 52 l-8 -4', 'f', 2)
    pl += L('M32 94 L104 40', 'd', 5) + P('M100 32 L118 30 L112 46 Z', 'd', 'f', 2) + L('M30 86 l-4 -10 M38 92 l-10 2', 'd', 3)
    card(g, 'polemic', pl + T(bolt('w'), 132, 78, .8), 'd')
    sp = 'M80 50 a3 3 0 0 1 6 0 a6 6 0 0 1 -12 0 a9 9 0 0 1 18 0 a12 12 0 0 1 -24 0 a15 15 0 0 1 30 0 a18 18 0 0 1 -36 0'
    lotus = P('M80 100 Q70 92 74 82 Q80 88 80 100 Z M80 100 Q90 92 86 82 Q80 88 80 100 Z', 'a', 'f', 2) + P('M80 100 Q74 90 80 80 Q86 90 80 100 Z', 'a', 'f', 2)
    card(g, 'meditative', L(sp, 'g', 3.5) + C(83, 50, 3, 'f', None) + T(lotus, -32, -42, 1.4) + T(leaf('s'), 128, 28, .7, -30), 'g')


# ---------- rhetorical device ----------
def devices():
    g = 'rhetorical_device'
    eq = R(68, 50, 22, 6, 3, 'f', None) + R(68, 62, 22, 6, 3, 'f', None)
    wave = L('M100 44 q6 -6 12 0 t12 0 t12 0 M100 58 q6 -6 12 0 t12 0 t12 0 M100 72 q6 -6 12 0 t12 0 t12 0', 'i', 3.5)
    card(g, 'metaphor', T(clock('u', 'f', 'a', .3, 22), 40, 58) + eq + wave, 'a')
    key = C(-16, 0, 9, 'w') + C(-16, 0, 3.5, 'u', 'f', 2) + R(-7, -3, 30, 6, 3, 'w', 'f', 2) + L('M16 3 v6 M22 3 v5', 'f', 3)
    lock = L('M-10 -6 V-14 Q-10 -24 0 -24 Q10 -24 10 -14 V-6', 'f', 4) + R(-15, -7, 30, 26, 5, 'w') + C(0, 3, 3.5, 'f', None) + L('M0 5 v7', 'f', 3)
    an = R(20, 32, 54, 52, 8, 'u') + R(86, 32, 54, 52, 8, 'u') + T(key, 47, 58, .9) + T(lock, 113, 54)
    an += L('M74 52 q3 -4 6 0 t6 0 M74 64 q3 -4 6 0 t6 0', 'b', 3)
    card(g, 'analogy', an, 'b')
    ap = '<rect x="22" y="26" width="42" height="68" rx="10" stroke="{{accent}}" stroke-width="2.5" stroke-dasharray="5 5"/>'
    for y, wd in ((38, 62), (60, 44), (82, 54)):
        ap += R(30, y - 7, 26, 14, 7, 'a') + L(f'M68 {y} h{wd}', 'm', 4)
    card(g, 'anaphora', ap, 'a')
    cx = C(42, 34, 11, 'a') + R(107, 23, 22, 22, 5, 'b') + R(31, 75, 22, 22, 5, 'b') + C(118, 86, 11, 'a')
    cx += L('M56 40 Q80 58 102 78 M104 40 Q80 58 58 78', 'm', 3) + P('M100 72 l8 10 l-12 -1 Z M60 72 l-8 10 l12 -1 Z', 'm', 'm', 2)
    card(g, 'chiasmus', cx, 'b')
    rq = bubble(22, 18, 76, 58, 'w') + qmark(60, 46, 2, 'f', 3.5)
    rq += face(120, 82, 13, 'u') + C(130, 48, 11, 's') + check(130, 48, .9) + L('M106 66 l-6 -4 M134 66 l6 -4', 'm', 2.5)
    card(g, 'rhetorical_question', rq, 'w')
    hy = T(fish('i'), 88, 54, 1.55) + T(fish('i'), 30, 92, .45)
    hy += L('M30 22 l8 8 M30 22 h8 M30 22 v8 M140 22 l-8 8 M140 22 h-8 M140 22 v8 M140 90 l-8 -8 M140 90 h-8 M140 90 v-8', 'w', 3)
    card(g, 'hyperbole', hy, 'w')
    lt = C(74, 58, 22, 'u') + C(66, 52, 2.4, 'f', None) + C(82, 52, 2.4, 'f', None) + L('M64 70 Q74 62 84 70', 'f', 2.5)
    lt += C(74, 58, 32, None, 'd', 4) + L('M51 35 L97 81', 'd', 4) + C(124, 86, 12, 's') + check(124, 86)
    card(g, 'litotes', lt, 's')
    body, tail = 'M55.8 46 A28 28 0 1 0 89.6 33.7', 'M75.1 32.4 A28 28 0 0 0 55.8 46'
    ox = L(tail, 'f', 8) + L(tail, 's', 3.5) + L(body, 'f', 13) + L(body, 's', 8)
    ox += ''.join(C(80 + 28 * math.cos(math.radians(a)), 60 + 28 * math.sin(math.radians(a)), 1.6, 'u', None) for a in (-20, 20, 60, 100, 140, 180))
    ox += P('M75 24 Q84 15 95 20 Q104 25 101 34 Q97 43 86 41 L75 37 L83 31 Z', 's', 'f', 2.5) + C(91, 27, 2.2, 'f', None)
    ox += T(sparkle('w', 7), 80, 58) + T(sparkle('w', 4), 126, 92)
    card(g, 'paradox', ox, 'g')
    ir = T(cloud('m'), 46, 34, .9) + L('M34 48 l-3 8 M44 50 l-3 8 M54 48 l-3 8', 'i', 2.5) + face(44, 84, 13, 'u')
    ir += bubble(76, 22, 64, 44, 'u', 'l') + T(sun('w', 7, 8, 4), 108, 44)
    card(g, 'irony', ir, 'i')
    at = R(24, 28, 112, 62, 10, 'u', None) + P('M80 28 h46 q10 0 10 10 v42 q0 10 -10 10 h-46 Z', 'b', None) + R(24, 28, 112, 62, 10, None)
    at += L('M80 28 v62', 'f', 3) + T(sun('w', 9, 8, 4), 52, 59) + T(moon('u'), 106, 60, 1.2)
    card(g, 'antithesis', at, 'b')
    al = L('M26 74 h110', 'm', 2, '.5')
    for x in (28, 70, 112):
        al += R(x, 48, 11, 24, 4, 'a') + L(f'M{x + 16} 70 h{14}', 'm', 4) + L(f'M{x - 1} 40 q6 -6 13 0 M{x - 4} 32 q9 -9 19 0', 'a', 2.5)
    card(g, 'alliteration', al + L('M40 90 h80', 'm', 2, '.5'), 'a')


# ---------- stylistic school ----------
def schools():
    g = 'stylistic_school'
    pg = R(40, 18, 66, 84, 5, 'u') + lines(50, 34, (46, 46, 46, 30, 46, 22), 10)
    pen = T(R(-5, -30, 10, 46, 2, 'w') + P('M-5 16 L0 26 L5 16 Z', 'u', 'f', 2) + R(-5, -36, 10, 6, 2, 'd', 'f', 2), 118, 64, 1, 24)
    card(g, 'plain', pg + pen, 'w')
    col_ = R(56, 34, 48, 54, 0, 'u') + L('M66 38 v46 M76 38 v46 M84 38 v46 M94 38 v46', 'm', 2)
    col_ += P('M48 28 h64 v6 h-64 Z', 'u') + C(52, 36, 6, 'u') + C(108, 36, 6, 'u') + C(52, 36, 2, 'f', None) + C(108, 36, 2, 'f', None)
    col_ += R(46, 22, 68, 6, 2, 'u') + R(52, 88, 56, 6, 2, 'u') + R(46, 94, 68, 7, 2, 'u')
    col_ += T(leaf('s'), 30, 68, .7, -60) + T(leaf('s'), 34, 50, .7, -90) + T(leaf('s'), 130, 68, .7, 240) + T(leaf('s'), 126, 50, .7, 270)
    card(g, 'classical', col_, 'b')
    br = R(38, 26, 84, 66, 4, 'w') + E(80, 59, 26, 23, 'u') + P('M66 80 Q66 64 80 64 Q94 64 94 80 Z', 'm', None) + C(80, 54, 8, 'm', None)
    for x, y, sx in ((38, 26, 1), (122, 26, -1)):
        br += L(f'M{x} {y + 10} q{-12 * sx} -2 {-10 * sx} -12 q{2 * sx} -8 {10 * sx} -4', 'f', 2.5)
        br += L(f'M{x} {y + 56} q{-12 * sx} 2 {-10 * sx} 12 q{2 * sx} 8 {10 * sx} 4', 'f', 2.5)
    br += P('M66 26 Q66 10 80 10 Q94 10 94 26 Z', 'w') + L('M80 26 V14 M80 26 L72 15 M80 26 L88 15', 'f', 2)
    br += C(60, 92, 3.5, 'd') + C(80, 96, 4, 'd') + C(100, 92, 3.5, 'd')
    card(g, 'baroque', br, 'w')
    rm = P('M18 100 L56 46 L70 62 L96 26 L142 100 Z', 'a') + P('M96 26 L106 42 L98 40 L92 46 L88 38 Z', 'u', 'f', 2) + P('M56 46 L63 56 L54 54 L49 56 Z', 'u', 'f', 2)
    rm += T(moon('w'), 36, 30, 1.1) + L('M114 28 q4 -4 8 0 q4 -4 8 0 M122 42 q3 -3 6 0 q3 -3 6 0', 'f', 2)
    card(g, 'romantic', rm, 'a')
    md = R(34, 22, 92, 78, 3, 'u') + R(36, 24, 46, 40, 0, 'd', None) + R(98, 74, 26, 24, 0, 'i', None) + R(36, 82, 30, 16, 0, 'w', None)
    md += L('M82 22 v78 M34 64 h92 M98 64 v36 M34 82 h48 M66 82 v18 M98 74 h28', 'f', 4) + R(34, 22, 92, 78, 3, None, 'f', 4)
    card(g, 'modernist', md, 'd')
    card(g, 'minimalist', R(30, 20, 100, 80, 6, 'u') + L('M60 62 h22', 'g', 4) + C(102, 62, 3, 'f', None), 'g')
    sr = L('M22 50 Q70 46 140 56 M112 54 l14 -14', 'm', 5)
    sr += P('M52 50 Q50 22 76 22 Q102 22 102 44 L102 56 Q106 72 102 86 Q99 94 95 86 Q93 70 86 60 Q70 56 52 50 Z', 'w')
    sr += E(77, 38, 17, 11, 'u', 'f', 2) + L('M77 38 v-7 M77 38 l8 3', 'f', 2.5) + C(77, 38, 2, 'f', None)
    sr += P('M98 96 q3 6 0 8 q-3 -2 0 -8 Z', 'w', 'f', 2) + P('M114 18 Q130 8 146 18 Q130 28 114 18 Z', 'u', 'f', 2) + C(130, 18, 4, 'i', None)
    card(g, 'surrealist', sr, 'a')
    gz = P(poly(star_pts(80, 58, 50, 36, 12, -90)), 'w', None, o='.55') + C(80, 60, 25, 'u')
    gz += P('M54 46 Q56 30 80 30 Q104 30 106 46 Z', 'd') + R(46, 44, 68, 6, 3, 'd')
    gz += P('M58 56 h18 q0 14 -9 14 q-9 0 -9 -14 Z M84 56 h18 q0 14 -9 14 q-9 0 -9 -14 Z', 'f', 'f', 2) + L('M76 58 h8', 'f', 2.5)
    gz += L('M62 60 l4 -2 M88 60 l4 -2', 'u', 2) + L('M74 78 q6 3 12 0', 'f', 2.5) + L('M86 78 l26 6', 'm', 3) + L('M116 80 q4 -6 0 -10 q-4 -6 2 -12', 'm', 2)
    card(g, 'gonzo', gz, 'd')
    ac = R(34, 14, 74, 88, 4, 'u') + lines(44, 28, (48, 54, 40), 9) + C(88, 26, 2.5, 'i', None) + C(98, 44, 2.5, 'i', None)
    ac += L('M44 74 h22', 'f', 2) + lines(44, 82, (40, 30), 7, 'm', 2) + C(41, 82, 1.5, 'i', None) + C(41, 89, 1.5, 'i', None)
    ac += C(106, 86, 10, 'u', 'f', 3) + C(130, 86, 10, 'u', 'f', 3) + L('M116 84 q2 -3 4 0 M96 84 l-8 -6', 'f', 2.5)
    card(g, 'academic', ac, 'i')


# ---------- semiotic mode ----------
def tree(f='s'):
    return R(-4, -18, 8, 20, 2, 'w') + C(0, -30, 14, f) + C(-4, -34, 3, 'u', None, o='.5')


def semiotics():
    g = 'semiotic_mode'
    arrow = '<path d="M86 52 h20" stroke="{{fg_muted}}" stroke-width="3" stroke-dasharray="3 6"/>' + P('M106 46 l8 6 l-8 6 Z', 'm', 'm', 2)
    sy = bubble(18, 24, 66, 52) + lines(28, 40, (44, 32, 40), 10) + arrow + T(tree(), 130, 86)
    card(g, 'symbolic', sy, 'a')
    ic = bubble(18, 24, 66, 52) + T(tree(), 51, 70, .8) + arrow + T(tree(), 130, 86)
    card(g, 'iconic', ic, 'b')
    ix = T(flame('d', 'w'), 70, 62, .9) + P('M18 100 Q50 64 96 74 Q126 80 144 100 Z', 's')
    ix += C(76, 38, 6, 'm', None, o='.7') + C(86, 27, 8, 'm', None, o='.55') + C(100, 19, 9, 'm', None, o='.4')
    ix += T(magnifier('u'), 120, 52, .7)
    card(g, 'indexical', ix, 'w')
    gl = bubble(22, 22, 116, 60) + T(heart('d', 'f', 2), 48, 52, .9) + T(star('w', 13, 5.5), 80, 51) + face(112, 52, 13, 'w')
    card(g, 'glyph', gl, 'd')
    mm = R(30, 16, 100, 88, 6, 'u') + R(38, 24, 84, 72, 3, 'm', 'f', 2, o='.3')
    mm += P('M60 64 L58 40 L70 50 Q80 46 90 50 L102 40 L100 64 Q100 84 80 84 Q60 84 60 64 Z', 'w') + C(72, 62, 2.4, 'f', None) + C(88, 62, 2.4, 'f', None)
    mm += P('M76 70 h8 l-4 4 Z', 'f', 'f', 2) + L('M72 78 q8 3 14 -2', 'f', 2)
    mm += R(46, 28, 68, 9, 4.5, 'u', 'f', 2) + R(50, 85, 60, 9, 4.5, 'u', 'f', 2)
    card(g, 'meme', mm, 'w')
    ty = R(26, 26, 32, 38, 4, 'a') + lines(66, 32, (66, 50, 60), 11, 'm', 4)
    ty += ''.join(R(x, 72, 9, 22, 2, 'f', None) for x in (30, 52, 74, 96, 118))
    ty += '<path d="M26 18 v84 M134 18 v84" stroke="{{fg_muted}}" stroke-width="1.5" stroke-dasharray="3 5"/>'
    card(g, 'typographic', ty, 'a')
    el = bubble(20, 26, 70, 46) + lines(32, 42, (44,), 0) + C(44, 56, 4, 'm', None)
    el += C(98, 58, 7, 'b') + C(118, 58, 7, 'b', None, o='.55') + C(138, 58, 7, 'b', None, o='.25')
    card(g, 'ellipsis', el, 'b')
    hs = bubble(30, 16, 100, 76) + L('M74 26 L64 80 M100 26 L90 80 M50 42 h60 M46 64 h60', 'i', 8)
    card(g, 'hashtag', hs, 'i')


# ---------- sentence structure ----------
def structures():
    g = 'sentence_structure'
    card(g, 'simple', R(30, 50, 80, 20, 10, 'a') + L('M44 60 h18', 'u', 3) + C(126, 64, 6, 'f', None), 'a')
    cp = R(18, 50, 48, 18, 9, 'a') + R(94, 50, 48, 18, 9, 'a') + L('M66 59 h28', 'f', 3) + C(80, 59, 10, 'b')
    cp += L('M80 54 v10 M75 59 h10', 'u', 3) + C(146, 66, 3, 'f', None)
    card(g, 'compound', cp, 'b')
    cx = R(26, 24, 108, 18, 9, 'a') + L('M50 42 V60 M50 50 H96 V60 M96 78 V88 H114', 'm', 3)
    cx += R(30, 60, 40, 14, 7, 'b') + R(80, 60, 42, 14, 7, 'b') + R(110, 82, 26, 12, 6, 'b', 'f', 2, o='.7')
    card(g, 'complex', cx, 'a')
    pr = R(22, 72, 12, 10, 5, 'm') + R(40, 68, 18, 14, 7, 'm') + R(64, 62, 22, 20, 8, 'm') + R(92, 40, 44, 42, 10, 'w')
    pr += T(sparkle('u', 9), 114, 61) + L('M24 96 h96', 'f', 3) + P('M120 90 l10 6 l-10 6 Z', 'f', 'f', 2)
    card(g, 'periodic', pr, 'w')
    cu = R(22, 36, 44, 44, 10, 's') + check(44, 58, 1.2) + L('M66 64 h60', 'm', 2, '.6')
    cu += R(72, 54, 24, 16, 7, 'm') + R(102, 57, 18, 11, 5, 'm') + R(126, 59, 12, 8, 4, 'm') + C(146, 66, 2.5, 'm', None)
    cu += L('M24 96 h96', 'f', 3) + P('M120 90 l10 6 l-10 6 Z', 'f', 'f', 2)
    card(g, 'cumulative', cu, 's')
    pa = ''
    for y in (36, 60, 84):
        pa += R(28, y - 7, 22, 14, 7, 'i') + L(f'M58 {y} h28', 'm', 4) + R(94, y - 7, 22, 14, 7, 'i') + L(f'M124 {y} h10', 'm', 4)
    pa += '<path d="M24 22 v76 M90 22 v76" stroke="{{fg_muted}}" stroke-width="1.5" stroke-dasharray="2 5"/>'
    card(g, 'parallel', pa, 'i')
    fr = ''
    for x, y, wd, r in ((26, 34, 26, -10), (82, 26, 18, 8), (40, 64, 16, 14), (92, 58, 30, -6), (60, 90, 22, 4)):
        fr += T(R(0, -6, wd, 12, 6, 'd') + C(wd + 7, 4, 3, 'f', None), x, y, 1, r)
    fr += L('M128 82 l-6 8 l6 4 l-6 8', 'm', 2.5) + L('M130 28 l8 -6 M134 40 h10', 'm', 2.5)
    card(g, 'fragmented', fr, 'd')
    sc = 'M20 30 C52 4 66 64 86 36 S124 14 128 46 S96 92 70 76 S56 52 84 60 S128 98 142 92'
    card(g, 'stream_of_consciousness', L(sc, 'f', 12) + L(sc, 'g', 7) + C(142, 92, 4, 'g', None) + T(sparkle('w', 6), 28, 88), 'g')


# ---------- post format ----------
def formats():
    g = 'post_format'
    sp = postcard(30, 20, 100, 74, 'a', (78, 70, 50), 10)
    sp += T(heart('d', 'f', 2), 46, 82, .35) + C(70, 82, 4, None, 'm', 2) + L('M90 78 l4 4 l-4 4 M98 78 l4 4 l-4 4', 'm', 2)
    card(g, 'single_post', sp, 'a')
    th = L('M41 30 V86', 'b', 3)
    for y in (16, 46, 76):
        th += R(30, y, 102, 24, 6, 'u') + C(41, y + 12, 6, 'b', 'f', 2) + lines(54, y + 8, (50, 34), 8, 'm', 3)
    card(g, 'thread', th, 'b')
    ls = R(34, 18, 92, 82, 7, 'u')
    for y, wd in ((36, 52), (56, 40), (76, 48)):
        ls += R(44, y - 6, 12, 12, 3, 's', 'f', 2) + check(50, y, .55, 'u', 3) + L(f'M64 {y} h{wd}', 'm', 4)
    card(g, 'list', ls, 's')
    dg = C(26, 52, 10, 'a') + bubble(42, 18, 74, 30, 'u') + lines(54, 30, (46,), 0)
    dg += C(134, 92, 10, 'w') + bubble(44, 58, 74, 30, 'w', 'r') + lines(56, 70, (48,), 0, 'f')
    card(g, 'dialogue', dg, 'w')
    pm = R(42, 14, 76, 90, 6, 'u')
    for y, wd in ((28, 40), (37, 30), (46, 44), (55, 26), (72, 36), (81, 24), (90, 32)):
        pm += L(f'M{n(80 - wd / 2)} {y} h{wd}', 'm', 3)
    card(g, 'poem', pm + T(quill('a'), 122, 96, .75), 'a')
    qs = R(30, 16, 100, 60, 8, 'u') + qmark(80, 44, 1.9, 'i', 3.5)
    for x in (44, 80, 116):
        qs += L(f'M{80 + (x - 80) * .4} 76 L{x} 88', 'm', 2) + C(x, 94, 6, 'b', 'f', 2)
    card(g, 'question', qs, 'i')
    ap = R(26, 34, 108, 50, 8, 'u') + R(42, 54, 76, 10, 5, 'w', 'f', 2)
    ap += L('M42 72 h48', 'w', 3, '.5')
    card(g, 'aphorism', ap + T(sparkle('w', 8), 128, 26) + T(sparkle('w', 5), 138, 92), 'w')
    fn = R(36, 20, 82, 84, 5, 'u') + ''.join(C(x, 20, 3.5, 'e', 'f', 2) for x in (48, 62, 76, 90, 104))
    fn += T(leaf('s'), 62, 50, 1.4, -10) + lines(46, 76, (60, 44, 52), 8, 'm', 2.5)
    fn += R(94, 32, 30, 26, 4, 'w') + L('M94 40 h30', 'f', 2) + C(109, 49, 3, 'f', None)
    card(g, 'field_note', fn, 's')


# ---------- lexical field ----------
def fields():
    g = 'lexical_field'
    nt = E(80, 96, 34, 7, 'w', 'f', 2) + L('M80 92 Q78 70 82 50', 's', 3) + T(leaf('s'), 66, 70, 1.3, 200) + T(leaf('s'), 96, 54, 1.3, -20)
    nt += T(sun('w', 9, 8, 4), 122, 30) + T(cloud('u', 'f', 2.5), 40, 34, .7)
    card(g, 'nature', nt, 's')
    tp = T(R(-30, -7, 52, 14, 5, 'u') + R(22, -9, 8, 18, 3, 'b') + R(-36, -4, 6, 8, 2, 'f', None), 52, 58, 1, -30)
    tp += L('M50 66 L36 100 M50 66 L52 100 M50 66 L66 100', 'f', 3)
    tp += T(planet('b', 'w', 13), 112, 44) + T(star('w', 6, 2.5), 128, 86) + T(sparkle('w', 6), 82, 22) + T(sparkle('w', 4), 140, 66)
    card(g, 'astronomy', tp, 'b')
    fl = P('M70 22 h20 M74 22 V46 L50 90 Q48 98 56 98 H104 Q112 98 110 90 L86 46 V22', 'u')
    fl += P('M61 70 H99 L109 90 Q110 95 104 95 H56 Q50 95 51 90 Z', 'i', None) + P('M74 22 V46 L50 90 Q48 98 56 98 H104 Q112 98 110 90 L86 46 V22', None)
    fl += L('M68 22 h24', 'f', 3) + C(70, 84, 3, 'u', None) + C(86, 78, 4, 'u', None) + C(96, 88, 2.5, 'u', None)
    fl += C(88, 12, 3, 'i', 'f', 2) + C(100, 18, 4, 'i', 'f', 2) + L('M112 46 l10 -4 M114 58 h10', 'm', 2.5)
    card(g, 'science', fl, 'i')
    tc = L('M80 40 V24 M66 40 V30 H40 M94 40 V30 H120 M60 60 H36 M60 74 H44 V92 M100 60 H124 M100 74 H112 V92 M80 94 V100', 'f', 2.5)
    tc += ''.join(C(x, y, 4.5, 'a', 'f', 2) for x, y in ((80, 20), (36, 30), (124, 30), (32, 60), (44, 96), (128, 60), (112, 96)))
    tc += R(58, 38, 44, 56, 6, 'u') + R(68, 50, 24, 30, 3, 'a') + L('M74 58 h12 M74 66 h8', 'u', 2.5)
    card(g, 'technology', tc, 'a')
    mt = P('M38 96 A42 42 0 0 1 122 96 Z', 'u') + P('M58 96 A22 22 0 0 1 102 96', None, 'f', 2)
    for a in range(15, 180, 15):
        t = math.radians(a)
        r1 = 36 if a % 45 else 30
        mt += L(f'M{n(80 - 40 * math.cos(t))} {n(96 - 40 * math.sin(t))} L{n(80 - r1 * math.cos(t))} {n(96 - r1 * math.sin(t))}', 'f', 2)
    mt += L('M112 14 L96 56 M112 14 L130 52', 'f', 3.5) + C(112, 14, 5, 'w') + R(124, 44, 7, 12, 2, 'w', 'f', 2)
    mt += '<path d="M96 60 Q112 70 128 58" stroke="{{warning}}" stroke-width="2.5" stroke-dasharray="3 5"/>'
    card(g, 'mathematics', mt, 'w')
    gv = R(36, 86, 60, 12, 4, 'w') + T(R(-6, 0, 12, 52, 5, 'w') + R(-22, -14, 44, 20, 6, 'w') + R(-26, -12, 6, 16, 2, 'f', None) + R(20, -12, 6, 16, 2, 'f', None), 92, 44, 1, -40)
    gv += L('M64 76 l-8 -6 M70 70 l-4 -10 M58 82 l-12 -2', 'd', 3)
    card(g, 'law', gv, 'd')
    win = P('M78 104 V44 Q78 16 106 16 Q134 16 134 44 V104 Z', 'u')
    win += P('M84 98 V46 Q84 22 106 22 V98 Z', 'a', None, o='.8') + P('M106 22 Q128 22 128 46 V98 H106 Z', 'w', None, o='.8')
    win += L('M106 22 V98 M84 58 H128 M84 78 H128', 'f', 2.5) + C(106, 44, 7, 'u', 'f', 2) + P('M78 104 V44 Q78 16 106 16 Q134 16 134 44 V104 Z', None)
    win += R(36, 60, 20, 40, 3, 'u') + L('M46 60 v-6', 'f', 2) + T(flame('w', 'u'), 46, 42, .6) + R(28, 98, 36, 6, 3, 'm', 'f', 2)
    card(g, 'theology', win, 'b')
    ms = L('M22 38 H138 M22 48 H138 M22 58 H138 M22 68 H138 M22 78 H138', 'm', 2)
    for x, y in ((50, 70), (76, 64), (114, 54)):
        ms += T(E(0, 0, 7, 5, 'f', None), x, y, 1, -20)
    ms += L('M56 68 V30 M82 62 V24', 'f', 3) + P('M56 30 L82 24 L82 32 L56 38 Z', 'f', 'f', 2) + L('M120 52 V20 q8 4 10 14', 'f', 3)
    card(g, 'music', ms + T(sparkle('a', 7), 26, 96) + T(sparkle('a', 5), 138, 96), 'a')
    wh = C(46, 42, 18, None, 'm', 5) + ''.join(L(f'M{n(46 + 10 * math.cos(math.radians(a)))} {n(42 + 10 * math.sin(math.radians(a)))} L{n(46 + 26 * math.cos(math.radians(a)))} {n(42 + 26 * math.sin(math.radians(a)))}', 'm', 4) for a in range(0, 360, 60))
    wh += C(46, 42, 6, 'm', None)
    an = C(96, 26, 7, None, 'f', 4) + L('M96 33 V88 M82 44 H110', 'f', 6) + L('M96 33 V88 M82 44 H110', 'u', 2)
    an += L('M70 70 Q74 90 96 92 Q118 90 122 70', 'f', 6) + P('M64 74 l6 -10 l6 8 Z M128 74 l-6 -10 l-6 8 Z', 'f', None)
    an += L('M20 104 q8 -6 16 0 t16 0 M108 104 q8 -6 16 0 t16 0', 'i', 3)
    card(g, 'seafaring', wh + an, 'i')
    pt = R(34, 54, 70, 42, 10, 'd') + R(28, 46, 82, 10, 5, 'd') + R(62, 38, 14, 8, 4, 'f', None) + L('M34 66 h-10 M104 66 h10', 'f', 4)
    pt += L('M52 30 q-6 -6 0 -12 q6 -6 0 -12 M72 28 q-6 -6 0 -12 M90 30 q-6 -6 0 -12 q6 -6 0 -12', 'm', 2.5)
    pt += T(R(-3, -40, 6, 46, 3, 'w') + E(0, 12, 8, 11, 'w'), 128, 64, 1, 20)
    card(g, 'cuisine', pt, 'w')
    saw = L('M28 92 ' + ' '.join('l3.5 5 l3.5 -5' for _ in range(11)), 'f', 2) + P('M26 78 L104 66 V92 H26 Z', 'u')
    saw += P('M100 60 Q130 56 132 76 Q132 96 100 96 Z', 'w') + E(116, 78, 7, 7, 'e', 'f', 2)
    hm = T(R(-5, -6, 10, 56, 4, 'w') + R(-24, -18, 48, 16, 4, 'm') + P('M-24 -18 q-10 8 0 16 Z', 'm', 'f', 2), 58, 34, .8, -30)
    card(g, 'craft', saw + hm, 'b')
    cm = ''
    for i in range(4):
        cm += E(54, 92 - i * 9, 22, 7, 'w')
    cm += E(54, 65, 22, 7, 'w') + E(54, 65, 12, 3.5, None, 'f', 2) + C(96, 84, 12, 'w') + C(96, 84, 6, None, 'f', 2)
    tag = P('M0 -14 H30 V14 H0 L-14 0 Z', 's') + C(-4, 0, 3.5, 'e', 'f', 2) + L('M8 -4 h14 M8 5 h9', 'u', 3)
    cm += T(tag, 106, 40, 1, -25) + L('M86 26 Q80 16 70 18', 'f', 2)
    card(g, 'commerce', cm, 's')


# ---------- weekday ----------
def weekdays():
    g = 'weekday'
    mon = T(moon('w'), 128, 38, 1.4) + T(sparkle('w', 5), 144, 64)
    tue = T(R(-2, -34, 4, 64, 2, 'm', 'f', 2) + P('M0 -44 l6 10 h-12 Z', 'm', 'f', 2), 140, 54, 1, 12) + T(shield('d'), 126, 46, 1.2)
    wing = P('M0 0 C-4 -20 10 -34 32 -36 C28 -32 30 -29 32 -26 C24 -25 25 -20 27 -16 C19 -16 19 -10 21 -6 C14 -6 8 -2 0 0 Z', 's', 'f', 2.5)
    wing += L('M5 -9 Q14 -18 28 -20 M4 -20 Q14 -28 30 -30', 'f', 2)
    wed = T(wing, 114, 60, 1.2)
    thu = T(bolt('w'), 130, 40, 1.4) + L('M114 22 l-4 -6 M148 34 l6 -2', 'w', 2.5)
    fri = T(heart('d'), 130, 40, .9) + T(sparkle('w', 6), 146, 18) + T(sparkle('w', 4), 114, 20)
    sat = T(planet('g', 'w', 11), 130, 38) + C(146, 66, 2, 'w', None)
    sun_ = T(sun('w', 11, 8, 5), 128, 38)
    days = (('monday', 'i', mon), ('tuesday', 'd', tue), ('wednesday', 's', wed), ('thursday', 'b', thu),
            ('friday', 'a', fri), ('saturday', 'g', sat), ('sunday', 'w', sun_))
    for i, (key, hue, motif) in enumerate(days):
        x = 36 + i * 11
        cal = R(22, 24, 92, 78, 8, 'u') + R(x - 6, 49, 12, 48, 6, hue, None, o='.35')
        cal += P('M22 44 V32 Q22 24 30 24 H106 Q114 24 114 32 V44 Z', hue)
        cal += L('M44 16 v14 M92 16 v14', 'f', 3)
        dots = ''.join(f'M{36 + k * 11} {y} h0' for y in (70, 84) for k in range(7) if k != i)
        cal += L(dots, 'm', 4.5) + L(' '.join(f'M{36 + k * 11} 56 h0' for k in range(7) if k != i), 'f', 5)
        cal += C(x, 56, 4.5, hue, 'f', 2) + L(f'M{x} 70 h0 M{x} 84 h0', hue, 5)
        card(g, key, cal + motif, hue)


# ---------- step banners ----------
def cat(f='w'):
    out = P('M14 -4 Q30 -6 26 -24', None, f, 5) + P('M-15 0 Q-17 -26 0 -28 Q17 -26 15 0 Z', f)
    out += P('M-11 -42 L-12 -56 L-2 -48 Z M11 -42 L12 -56 L2 -48 Z', f, 'f', 2.5) + C(0, -40, 12, f)
    out += C(-4.5, -42, 1.8, 'f', None) + C(4.5, -42, 1.8, 'f', None) + P('M-2 -37 h4 l-2 2 Z', 'f', 'f', 1.5)
    return out + L('M-6 -35 l-10 -1 M6 -35 l10 -1', 'f', 1.5)


def can():
    return R(-13, -36, 26, 36, 4, 'u') + E(0, -36, 13, 4, 'm', 'f', 2.5) + R(-13, -26, 26, 14, 0, 'w', 'f', 2) + L('M-13 -6 h26', 'f', 2)


def robot(f='u', e='a'):
    out = L('M0 -54 v-8', 'f', 2.5) + C(0, -64, 3.5, e, 'f', 2) + R(-14, -54, 28, 20, 5, f) + C(-6, -44, 3, e, None) + C(6, -44, 3, e, None)
    out += R(-12, -32, 24, 26, 4, f) + R(-6, -26, 12, 8, 2, e, 'f', 2) + L('M-12 -26 l-8 12 M12 -26 l8 12', 'f', 3)
    return out + R(-10, -6, 7, 6, 2, 'm', 'f', 2) + R(3, -6, 7, 6, 2, 'm', 'f', 2)


def banners():
    em = L('M22 100 H298', 'm', 3)
    p, _ = person(44, 100, 'a', (25, 25), sc=.86)
    em += p + T(cat('w'), 92, 100) + T(can(), 138, 100)
    em += T(R(-4, -30, 8, 30, 2, 'm') + C(-12, -38, 11, 's') + C(12, -38, 11, 's') + C(0, -50, 16, 's'), 186, 100)
    em += T(P('M0 -36 Q16 -16 16 -10 A16 16 0 0 1 -16 -10 Q-16 -16 0 -36 Z', 'a') + C(-6, -12, 3, 'u', None, o='.6'), 228, 100, .9)
    em += T(flame('w', 'u'), 262, 84, 1.05) + T(robot(), 292, 100)
    em += T(sparkle('w', 6), 120, 24) + T(sparkle('w', 4), 236, 26)
    banner('embodiment', em, 's')

    rh = T(scales('w'), 62, 96)
    rh += T(quill('u'), 108, 62, .8, 0) + L('M100 70 q6 2 12 0', 'm', 2)
    orator, _ = person(160, 86, 'a', (20, 150), mouth='open', sc=.8)
    rh += orator + P('M136 74 H184 L178 104 H142 Z', 'b') + R(130, 68, 60, 8, 3, 'b') + C(160, 88, 5, 'u', 'f', 2)
    rh += bubble(194, 14, 44, 34, 'u', 'l') + qmark(216, 31, 1.1, 'a')
    rh += C(250, 62, 9, 'a') + R(277, 53, 18, 18, 4, 'b') + R(241, 84, 18, 18, 4, 'b') + C(286, 93, 9, 'a')
    rh += L('M260 66 Q268 77 278 87 M276 66 Q268 77 260 87', 'm', 2.5) + P('M274 82 l6 7 l-9 0 Z M262 82 l-6 7 l9 0 Z', 'm', 'm', 2)
    rh += T(sparkle('w', 5), 126, 24)
    banner('rhetoric', rh, 'a')


def build():
    modes()
    devices()
    schools()
    semiotics()
    structures()
    formats()
    fields()
    weekdays()
    banners()
