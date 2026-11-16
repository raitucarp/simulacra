"""MBTI (16 role figures) and identity variant cards."""
from kit import *

NT, NF, SJ, SP = 'a', 's', 'i', 'w'


def fig(f, arms=(15, 55), x=56, mouth='smile', **kw):
    return person(x, 100, f, arms, mouth, **kw)


def mbti():
    g = 'mbti'
    # INTJ: architect unrolling a blueprint of a tower
    p, _ = fig(NT, (15, 60))
    bp = (R(88, 30, 50, 52, 4, 'b') + L('M96 74 L96 52 L113 40 L130 52 L130 74 Z M106 74 L106 62 L120 62 L120 74', 'u', 2) +
          L('M94 36 h38', 'u', 2, '.6') + E(88, 82, 4, 6, 'b'))
    card(g, 'intj', bp + p, NT)
    # INTP: logician with thought cloud holding gear + bulb
    p, _ = fig(NT, (15, 25), mouth='flat')
    th = C(78, 46, 3, 'u', 'f', 2) + C(86, 38, 4.5, 'u', 'f', 2) + T(cloud('u', 'f', 2.5) + T(gear('b', 5, 7), -9, -4) + T(bulb('w'), 10, -4, .42), 116, 40, 1.4)
    card(g, 'intp', th + p, NT)
    # ENTJ: commander pointing at a planted flag
    p, _ = fig(NT, (15, 125))
    fl = L('M118 100 L118 22', 'f', 3) + P('M118 24 Q130 18 142 26 Q134 34 142 44 Q130 36 118 42 Z', 'b') + E(118, 101, 10, 3, 'f', None, o='.2')
    card(g, 'entj', fl + p, NT)
    # ENTP: debater with clashing exclamation + question bubbles
    p, _ = fig(NT, (15, 95), mouth='open')
    bb = (bubble(84, 22, 30, 26, 'b') + R(97, 27, 4, 11, 2, 'f', None) + C(99, 42, 2.2, 'f', None) +
          bubble(112, 50, 32, 26, 'u', 'r') + L('M123 58 Q123 54 128 54 Q133 54 133 59 Q133 63 128 64 L128 67', 'f', 2.5) + C(128, 71, 1.6, 'f', None))
    card(g, 'entp', p + bb, NT)
    # INFJ: advocate holding a glowing lantern
    p, (_, h) = fig(NF, (15, 50))
    lx, ly = h
    ln = T(C(0, 16, 24, 'w', None, o='.25') + L('M0 -4 L0 2', 'f', 2) + R(-11, 2, 22, 28, 6, 'u') +
           T(flame('w', 'd'), 0, 17, .6) + R(-8, 30, 16, 5, 2, 'f', None) + P('M-7 2 Q0 -6 7 2 Z', 'f', None), lx + 4, ly + 2)
    card(g, 'infj', p + ln, NF)
    # INFP: mediator offering a flower, butterfly nearby
    p, (_, h) = fig(NF, (15, 60))
    hx, hy = h
    fl = L(f'M{n(hx)} {n(hy)} Q{n(hx + 10)} {n(hy - 14)} {n(hx + 12)} {n(hy - 30)}', 's', 3)
    cx, cy = hx + 12, hy - 34
    fl += ''.join(C(px, py, 5.5, 'd', 'f', 2) for px, py in ring_pts(cx, cy, 7, 5, -90)) + C(cx, cy, 4, 'w', 'f', 2)
    bf = T(P('M0 0 Q-10 -12 -12 -2 Q-10 6 0 0 Z M0 0 Q10 -12 12 -2 Q10 6 0 0 Z', 'w', 'f', 2) + L('M0 -6 L0 4', 'f', 2), 130, 30, 1, 15)
    card(g, 'infp', fl + bf + p, NF)
    # ENFJ: protagonist raising a star, two followers
    p, _ = person(80, 100, NF, (25, 150), 'open')
    f1, _ = person(38, 102, 'm', (None, None), sc=.55)
    f2, _ = person(122, 102, 'm', (None, None), sc=.55)
    card(g, 'enfj', f1 + f2 + p + T(star('w', 9, 4), 103, 16), NF)
    # ENFP: campaigner cheering in confetti
    p, _ = person(80, 100, NF, (150, 150), 'open')
    conf = ''.join(T(R(-3, -2, 6, 4, 1, c, None), x, y, 1, r) for x, y, c, r in
                   ((40, 30, 'w', 20), (120, 26, 'd', -30), (34, 64, 'd', 50), (128, 60, 'w', 10), (58, 18, 'd', -15), (104, 14, 'w', 40)))
    card(g, 'enfp', conf + p + T(sparkle('w', 6), 132, 40) + T(sparkle('w', 5), 28, 44), NF)
    # ISTJ: logistician with checklist clipboard
    p, _ = fig(SJ, (15, 55))
    cb = R(86, 32, 40, 52, 5, 'u') + R(98, 28, 16, 8, 3, 'm', 'f', 2)
    cb += ''.join(L(f'M93 {y} l3 3 l6 -6', 's', 2.5) + L(f'M106 {y} h13', 'm', 2.5) for y in (46, 58, 70))
    card(g, 'istj', cb + p, SJ)
    # ISFJ: defender sheltering under an umbrella from rain
    p, (_, h) = fig(SJ, (15, 30))
    hx, hy = h
    um = (L(f'M{n(hx)} {n(hy)} L{n(hx)} 26', 'f', 3) + P(f'M{n(hx - 40)} 30 Q{n(hx)} -6 {n(hx + 40)} 30 Q{n(hx + 27)} 24 {n(hx + 13)} 30 Q{n(hx)} 24 {n(hx - 13)} 30 Q{n(hx - 27)} 24 {n(hx - 40)} 30 Z', 'd'))
    rain = ''.join(L(f'M{x} {y} l-3 7', 'i', 2) for x, y in ((24, 20), (30, 40), (136, 18), (140, 42), (128, 60)))
    card(g, 'isfj', rain + p + um, SJ)
    # ESTJ: executive with tie and briefcase
    tie = P('M-3 -43 L3 -43 L5 -18 L0 -12 L-5 -18 Z', 'd', 'f', 2)
    p, (_, h) = fig(SJ, (15, 12), front=tie)
    hx, hy = h
    bc = R(hx - 4, hy - 3, 32, 24, 4, 'm') + L(f'M{n(hx + 6)} {n(hy - 3)} v-5 h12 v5', 'f', 2.5) + L(f'M{n(hx - 4)} {n(hy + 7)} h32', 'f', 2)
    card(g, 'estj', p + bc, SJ)
    # ESFJ: consul presenting a gift with bow
    p, _ = person(66, 100, SJ, (40, 40))
    gb = R(54, 62, 26, 22, 3, 'd') + R(64, 62, 6, 22, 0, 'w', None) + P('M67 62 Q56 50 60 58 Q62 62 67 62 Q78 50 74 58 Q72 62 67 62 Z', 'w', 'f', 2)
    card(g, 'esfj', p + gb + T(heart('d'), 118, 38, .6) + T(sparkle('w', 6), 130, 64), SJ)
    # ISTP: virtuoso with wrench and floating gear
    p, (_, h) = fig(SP, (15, 70), mouth='flat')
    hx, hy = h
    wr = T(R(-3.5, 0, 7, 30, 3.5, 'm', 'f', 2) + P('M-8 -2 Q-8 -14 0 -14 L0 -6 L6 -6 L6 -12 Q10 -8 8 -2 Q4 4 -4 3 Z', 'm', 'f', 2), hx + 2, hy - 6, 1, -150)
    card(g, 'istp', p + wr + T(gear('i', 11), 124, 40), SP)
    # ISFP: adventurer with paint palette and brush
    p, (h1, h2) = fig(SP, (40, 60))
    pl = P('M88 58 Q86 42 104 40 Q128 38 132 56 Q134 70 118 72 Q110 72 110 78 Q106 84 96 80 Q88 74 88 58 Z', 'u')
    pl += C(100, 50, 4, 'd', None) + C(112, 48, 4, 's', None) + C(123, 54, 4, 'w', None) + E(102, 66, 4, 3, 'k', 'f', 2)
    br = T(R(-2, 0, 4, 26, 2, 'f', None) + P('M-3 0 Q0 -10 3 0 Z', 'd', 'f', 2), 136, 26, 1, 20)
    card(g, 'isfp', pl + p + br, SP)
    # ESTP: entrepreneur rolling dice
    p, _ = fig(SP, (15, 80), mouth='open')
    def die(x, y, r, pips):
        return T(R(-11, -11, 22, 22, 5, 'u') + ''.join(C(px, py, 2.2, 'f', None) for px, py in pips), x, y, 1, r)
    dc = die(102, 52, -15, ((-5, -5), (0, 0), (5, 5))) + die(128, 72, 20, ((-5, -5), (5, -5), (-5, 5), (5, 5)))
    card(g, 'estp', p + dc + L('M96 30 q6 -6 12 0 M120 42 q6 -6 12 0', 'm', 2), SP)
    # ESFP: entertainer singing into a microphone
    p, (_, h) = fig(SP, (15, 125), mouth='open')
    hx, hy = h
    mic = T(R(-3, 0, 6, 16, 3, 'f', None) + C(0, -4, 7, 'm', 'f', 2.5), hx - 2, hy - 4, 1, -30)
    notes = (T(C(0, 0, 4, 'd', None) + L('M3.5 0 L3.5 -14 L10 -11', 'd', 2.5), 116, 40) +
             T(C(0, 0, 4, 'd', None) + L('M3.5 0 L3.5 -14 L10 -11', 'd', 2.5), 132, 64))
    card(g, 'esfp', notes + p + mic + T(sparkle('w', 6), 112, 76), SP)


def identity():
    g = 'identity_variant'
    boat = (P('M-26 0 L26 0 L18 12 L-18 12 Z', 'm') + L('M0 0 L0 -40', 'f', 3) +
            P('M2 -38 L24 -4 L2 -4 Z', 'u', 'f', 2.5) + P('M-2 -32 L-18 -4 L-2 -4 Z', 'b', 'f', 2.5))
    calm = L('M20 96 q10 -4 20 0 t20 0 t20 0 t20 0 t20 0 t20 0', 'i', 3)
    card(g, 'assertive', T(sun('w', 10), 122, 30) + T(boat, 74, 86) + calm, 'i')
    storm = L('M20 98 q10 -12 20 0 t20 0 t20 0 t20 0 t20 0 t20 0', 'i', 3)
    bolt = P('M126 34 L118 50 L126 50 L120 66 L136 44 L128 44 L134 34 Z', 'w', 'f', 2)
    rain = ''.join(L(f'M{x} {y} l-3 8', 'i', 2) for x, y in ((32, 44), (44, 56), (24, 64)))
    card(g, 'turbulent', T(cloud('m'), 46, 32) + T(cloud('m'), 112, 26) + bolt + rain + T(boat, 74, 88, 1, -14) + storm, 'i')


def build():
    mbti()
    identity()
