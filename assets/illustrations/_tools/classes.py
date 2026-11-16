"""Archetype class art (200x240), full-body flat characters with distinct silhouettes."""
from kit import *


def limb(sx, sy, ang, ln, wd, f, hand='u'):
    t = math.radians(ang)
    hx, hy = sx + ln * math.sin(t), sy + ln * math.cos(t)
    out = T(R(-wd / 2, 0, wd, ln, wd / 2, f, 'f', 2.5), sx, sy, 1, -ang)
    if hand:
        out += C(hx, hy, wd * .55, hand, 'f', 2.5)
    return out, (hx, hy)


def head(y=58, r=24, eyes='open', mouth='smile'):
    out = C(100, y, r, 'u')
    if eyes == 'open':
        out += C(91, y, 2.8, 'f', None) + C(109, y, 2.8, 'f', None)
    elif eyes == 'closed':
        out += L(f'M86 {y} q5 4 10 0 M104 {y} q5 4 10 0', 'f', 2.5)
    elif eyes == 'lidded':
        out += C(91, y + 1, 2.8, 'f', None) + C(109, y + 1, 2.8, 'f', None) + L(f'M86 {y - 2} h10 M104 {y - 2} h10', 'f', 2.5)
    if mouth == 'smile':
        out += L(f'M93 {y + 11} q7 5 14 0', 'f', 2.5)
    elif mouth == 'flat':
        out += L(f'M94 {y + 12} h12', 'f', 2.5)
    elif mouth == 'grin':
        out += P(f'M90 {y + 9} q10 12 20 0 Z', 'f', 'f', 2)
    return out


def klass(name, body, hue, grad=None):
    frame = R(8, 8, 184, 224, 28, 'e', None) + C(100, 100, 72, hue, None, o='.14') + E(100, 222, 58, 7, 'f', None, o='.08')
    write(f'classes/{name}.svg', 200, 240, frame + body, grad, 'wide')


def socratic_gadfly():
    out = E(84, 214, 12, 5, 'm') + E(116, 214, 12, 5, 'm')
    out += P('M70 92 Q58 150 62 212 L138 212 Q142 150 130 92 Q100 82 70 92 Z', 'u')
    out += P('M72 92 Q94 84 106 98 L140 176 Q132 184 122 178 L84 112 Q74 104 72 92 Z', 'w')
    a, _ = limb(76, 98, -12, 52, 14, 'u')
    b, (hx, hy) = limb(126, 98, 150, 40, 14, 'u', None)
    out += a + b + R(hx - 3, hy - 18, 7, 18, 3.5, 'u', 'f', 2.5) + C(hx, hy, 8, 'u', 'f', 2.5)
    out += head(58, 24, 'open', 'none') + P('M78 64 Q80 100 100 104 Q120 100 122 64 Q112 76 100 74 Q88 76 78 64 Z', 'm')
    out += L('M92 72 q8 -4 16 0', 'f', 2.5) + L('M84 50 l10 -2 M106 48 l10 2', 'f', 3) + L('M76 56 q-4 0 -4 6 M124 56 q4 0 4 6', 'f', 2.5)
    fly = '<path d="M170 32 q-16 -12 -26 6 q-6 14 10 14" stroke="{{fg_muted}}" stroke-width="2" stroke-dasharray="3 5"/>'
    fly += E(164, 30, 7, 4, 'u', 'f', 2, o='.8') + E(172, 26, 7, 4, 'u', 'f', 2, o='.8') + E(170, 34, 6, 4, 'f', None)
    klass('socratic_gadfly', out + fly, 'w')


def rationalist_architect():
    out = R(84, 178, 12, 34, 4, 'f', None) + R(104, 178, 12, 34, 4, 'f', None) + E(88, 214, 12, 5, 'f', None) + E(112, 214, 12, 5, 'f', None)
    out += P('M74 90 L68 194 L132 194 L126 90 Q100 82 74 90 Z', 'a') + L('M100 92 V192 M88 90 L100 112 L112 90', 'u', 2.5)
    out += C(106, 130, 2.5, 'u', None) + C(106, 150, 2.5, 'u', None)
    out += T(R(-34, -8, 68, 16, 8, 'b') + E(34, 0, 4, 8, 'u', 'f', 2), 70, 122, 1, -15)
    a, _ = limb(78, 96, -14, 40, 14, 'a')
    b, (hx, hy) = limb(122, 96, 40, 46, 14, 'a')
    comp = L(f'M{n(hx + 6)} {n(hy - 6)} L{n(hx - 8)} 206 M{n(hx + 6)} {n(hy - 6)} L{n(hx + 34)} 200', 'f', 3) + C(hx + 6, hy - 6, 6, 'w')
    out += comp + a + b
    out += head(58, 24, 'open', 'flat') + P('M76 54 Q74 30 100 30 Q126 30 124 54 Q114 40 100 42 Q84 40 76 54 Z', 'f', 'f', 2)
    out += C(91, 59, 8, None, 'f', 2) + C(109, 59, 8, None, 'f', 2) + L('M99 59 h2', 'f', 2)
    out += P('M150 30 l12 20 h-24 Z', None, 'b', 2.5) + R(34, 34, 18, 18, 2, None, 'b', 2.5) + C(160, 168, 9, None, 'b', 2.5)
    klass('rationalist_architect', out, 'a')


def existential_poet():
    out = R(86, 186, 10, 26, 4, 'f', None) + R(104, 186, 10, 26, 4, 'f', None) + E(88, 214, 12, 5, 'f', None) + E(112, 214, 12, 5, 'f', None)
    out += P('M76 92 Q70 150 64 196 L136 196 Q130 150 124 92 Q100 84 76 92 Z', 'm')
    out += P('M112 94 Q150 98 178 74 Q172 92 184 102 Q150 114 114 106 Z', 'd') + P('M106 100 L98 150 L110 150 L116 102 Z', 'd')
    a, _ = limb(78, 98, 30, 36, 14, 'm')
    b, _ = limb(122, 98, -30, 36, 14, 'm')
    nb = P('M80 122 L100 128 L120 122 L120 144 L100 150 L80 144 Z', 'u', 'f', 2.5) + L('M100 128 v22 M86 132 l9 2 M86 138 l9 2', 'f', 2)
    out += a + b + nb + P('M78 86 Q100 100 122 86 L124 96 Q100 110 76 96 Z', 'd')
    out += head(60, 24, 'lidded', 'flat') + L('M78 52 q-4 12 0 22 M122 52 q4 12 0 22', 'f', 3)
    out += P('M72 46 Q78 26 104 28 Q132 30 130 46 Q100 52 72 46 Z', 'f') + C(104, 26, 4, 'f', None)
    out += T(moon('w'), 160, 36, 1.2) + C(40, 60, 2.5, 'm', None) + C(46, 44, 2, 'm', None)
    klass('existential_poet', out, 'd')


def stoic_sage():
    out = R(52, 206, 96, 16, 4, 'm') + P('M76 92 Q70 150 70 206 L130 206 Q130 150 124 92 Q100 84 76 92 Z', 'u')
    out += P('M84 92 L96 90 L118 206 L104 206 Z', 's')
    a, _ = limb(78, 98, 18, 46, 14, 'u')
    b, _ = limb(122, 98, -18, 46, 14, 'u')
    out += a + b + head(58, 24, 'closed', 'none') + P('M80 66 Q80 112 100 118 Q120 112 120 66 Q110 78 100 76 Q90 78 80 66 Z', 'm')
    out += L('M92 74 q8 -4 16 0', 'f', 2.5)
    for d in range(200, 345, 24):
        t = math.radians(d)
        out += T(P('M-6 0 Q0 -5 6 0 Q0 5 -6 0 Z', 's', 'f', 1.5), 100 + 26 * math.cos(t), 58 + 26 * math.sin(t), 1, d + 90)
    out += T(sun('w', 10, 8, 5), 156, 40)
    klass('stoic_sage', out, 's')


def empirical_scientist():
    out = R(86, 186, 10, 26, 4, 'f', None) + R(104, 186, 10, 26, 4, 'f', None) + E(88, 214, 12, 5, 'f', None) + E(112, 214, 12, 5, 'f', None)
    out += R(84, 90, 32, 100, 4, 'i', None)
    out += P('M76 92 Q68 150 68 196 L96 196 L92 96 Z', 'u') + P('M124 92 Q132 150 132 196 L104 196 L108 96 Z', 'u')
    out += R(112, 120, 12, 14, 2, 'u', 'f', 2) + L('M115 120 v-6 M120 120 v-4', 'w', 2.5)
    a, (lx, ly) = limb(78, 98, -10, 46, 14, 'u')
    b, (hx, hy) = limb(122, 98, 140, 40, 14, 'u')
    clip = R(lx - 18, ly - 6, 26, 34, 3, 'm') + L(f'M{n(lx - 13)} {n(ly + 6)} h16 M{n(lx - 13)} {n(ly + 13)} h12', 'u', 2)
    fl = P(f'M{n(hx - 5)} {n(hy - 34)} v12 l-12 20 q-2 6 4 6 h26 q6 0 4 -6 l-12 -20 v-12 Z', 'u', 'f', 2.5)
    fl += P(f'M{n(hx - 13)} {n(hy - 10)} h26 l3 6 q1 4 -3 4 h-26 q-4 0 -3 -4 Z', 's', None) + C(hx - 2, hy - 42, 3, 's', 'f', 2) + C(hx + 6, hy - 50, 2.2, 's', 'f', 2)
    out += a + clip + b + fl
    out += head(60, 24, 'open', 'grin') + P('M76 52 L80 34 L88 44 L96 28 L104 42 L114 30 L118 44 L126 40 L124 54 Q100 42 76 52 Z', 'f')
    out += L('M76 46 h48', 'm', 3) + C(90, 44, 7, 'w') + C(110, 44, 7, 'w')
    klass('empirical_scientist', out, 'i')


def contrarian_trickster():
    l1, _ = limb(90, 158, 4, 50, 13, 'a', None)
    l2, (fx, fy) = limb(110, 156, -60, 44, 13, 'd', None)
    out = l1 + l2 + P('M80 206 Q90 200 96 208 Q104 214 112 206 L110 214 Q96 220 80 214 Z', 'f')
    out += T(P('M-6 -6 Q10 -10 18 -2 Q22 -10 28 -6 Q24 4 10 6 L-6 6 Z', 'f'), fx, fy, 1, -60)
    out += P('M78 92 Q72 130 70 162 L130 162 Q128 130 122 92 Q100 84 78 92 Z', 'd')
    out += P('M100 96 L112 116 L100 136 L88 116 Z M100 136 L110 152 L100 162 L90 152 Z', 'a', 'f', 2)
    out += P('M70 162 l10 10 l10 -10 l10 10 l10 -10 l10 10 l10 -10 Z', 'a', 'f', 2)
    a, _ = limb(78, 98, -140, 36, 13, 'a')
    b, _ = limb(122, 98, 140, 36, 13, 'd')
    out += a + b + head(64, 24, 'open', 'grin') + L('M84 54 l10 -4 M106 52 l10 0', 'f', 2.5)
    hat = P('M76 52 Q60 22 32 30 Q52 36 64 54 Z', 'a') + P('M124 52 Q140 22 168 30 Q148 36 136 54 Z', 'a')
    hat += P('M78 52 Q84 20 100 6 Q116 20 122 52 Q100 44 78 52 Z', 'd') + R(74, 46, 52, 10, 5, 'a')
    hat += C(32, 30, 6, 'w') + C(100, 6, 6, 'w') + C(168, 30, 6, 'w')
    balls = C(42, 92, 7, 'w') + C(158, 92, 7, 'w') + C(150, 64, 6, 'w') + L('M50 84 Q60 66 66 62 M142 72 Q136 60 130 60', 'm', 2)
    klass('contrarian_trickster', out + hat + balls, 'd')


def compassionate_mentor():
    out = R(84, 184, 12, 28, 4, 'm', None) + R(104, 184, 12, 28, 4, 'm', None) + E(88, 214, 12, 5, 'f', None) + E(112, 214, 12, 5, 'f', None)
    out += R(84, 92, 32, 96, 4, 'u', None)
    out += P('M72 94 Q62 150 70 192 L96 192 L94 96 Z', 'w') + P('M128 94 Q138 150 130 192 L104 192 L106 96 Z', 'w')
    out += C(100, 120, 2.5, 'f', None) + C(100, 140, 2.5, 'f', None) + C(100, 160, 2.5, 'f', None)
    a, (lx, ly) = limb(76, 100, -24, 44, 15, 'w')
    b, (hx, hy) = limb(124, 100, 76, 40, 15, 'w')
    lan = C(lx - 2, ly + 26, 22, 'w', None, o='.25') + L(f'M{n(lx)} {n(ly)} v6', 'f', 2) + R(lx - 12, ly + 8, 22, 30, 6, 'u')
    lan += T(flame('d', 'w'), lx - 1, ly + 24, .6) + R(lx - 10, ly + 38, 18, 5, 2, 'f', None)
    out += b + a + lan + T(sparkle('w', 9), hx + 4, hy - 20)
    out += head(58, 24, 'open', 'smile') + P('M76 56 Q74 30 100 30 Q126 30 124 56 Q120 44 100 42 Q80 44 76 56 Z', 'm', 'f', 2) + C(100, 28, 9, 'm', 'f', 2)
    out += C(91, 58, 7, None, 'f', 2) + C(109, 58, 7, None, 'f', 2)
    pot = P('M142 196 h28 l-4 18 h-20 Z', 'd') + L('M156 196 v-16', 's', 3) + T(leaf('s'), 148, 180, .7, 200) + T(leaf('s'), 164, 176, .7, -20)
    klass('compassionate_mentor', out + pot, 'w')


def mystic_visionary():
    out = P('M68 84 Q52 150 40 216 L160 216 Q148 150 132 84 Z', 'G')
    out += ''.join(T(sparkle('w', r), x, y) for x, y, r in ((66, 180, 6), (130, 196, 5), (120, 150, 4), (76, 140, 4)))
    out += C(100, 112, 32, 'b', None, o='.25')
    a, _ = limb(76, 96, 28, 34, 15, 'a')
    b, _ = limb(124, 96, -28, 34, 15, 'a')
    out += a + b + C(100, 108, 17, 'b') + L('M92 102 q4 -6 10 -6', 'u', 2.5)
    out += P('M70 78 Q64 24 100 14 Q136 24 130 78 Q118 92 100 92 Q82 92 70 78 Z', 'a')
    out += C(100, 62, 19, 'u') + C(92, 64, 2.6, 'b', None) + C(108, 64, 2.6, 'b', None) + P('M100 48 l3 4 l-3 4 l-3 -4 Z', 'w', None)
    out += L('M94 72 q6 3 12 0', 'f', 2) + T(moon('w'), 158, 34, 1.1) + T(star('w', 5, 2.2), 40, 44)
    klass('mystic_visionary', out, 'a', grad='a')


def build():
    socratic_gadfly()
    rationalist_architect()
    existential_poet()
    stoic_sage()
    empirical_scientist()
    contrarian_trickster()
    compassionate_mentor()
    mystic_visionary()
