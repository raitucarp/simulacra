"""Tiny flat-vector SVG kit used to generate the illustration templates.

Colors are one-letter palette shorthands that expand to `{{token}}` placeholders
(see STYLE.md). `G` expands to the file's single gradient.
Run `python assets/illustrations/_tools/build.py` to regenerate every file.
"""
import math
import os
import re

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
TOK = dict(a='accent', b='accent_2', g='agent', s='success', w='warning', d='danger',
           i='info', f='fg', m='fg_muted', e='elevated', u='surface', o='border', k='bg',
           l='selection', S='skin', H='hair')
HUES = {'accent', 'accent_2', 'agent', 'success', 'warning', 'danger', 'info'}
LIMITS = {'card': 3072, 'wide': 6144}
WRITTEN = []


def n(v):
    v = round(v, 1)
    return str(int(v)) if v == int(v) else str(v)


def col(c):
    if c in TOK:
        return '{{%s}}' % TOK[c]
    if c == 'G':
        return 'url(#@G)'
    return c


def _paint(f, s, w, o):
    out = ''
    if f is not None:
        out += f' fill="{col(f)}"'
    if o is not None:
        out += f' fill-opacity="{o}"'
    if s is not None:
        out += f' stroke="{col(s)}" stroke-width="{n(w)}"'
    return out


def C(x, y, r, f, s='f', w=3, o=None):
    return f'<circle cx="{n(x)}" cy="{n(y)}" r="{n(r)}"{_paint(f, s, w, o)}/>'


def E(x, y, rx, ry, f, s='f', w=3, o=None):
    return f'<ellipse cx="{n(x)}" cy="{n(y)}" rx="{n(rx)}" ry="{n(ry)}"{_paint(f, s, w, o)}/>'


def R(x, y, wd, h, rx, f, s='f', w=3, o=None):
    r = f' rx="{n(rx)}"' if rx else ''
    return f'<rect x="{n(x)}" y="{n(y)}" width="{n(wd)}" height="{n(h)}"{r}{_paint(f, s, w, o)}/>'


def P(d, f, s='f', w=3, o=None):
    return f'<path d="{d}"{_paint(f, s, w, o)}/>'


def L(d, s='f', w=3, o=None):
    op = f' stroke-opacity="{o}"' if o is not None else ''
    return f'<path d="{d}" stroke="{col(s)}" stroke-width="{n(w)}"{op}/>'


def T(body, x=0, y=0, sc=1, rot=0):
    t = f'translate({n(x)} {n(y)})'
    if rot:
        t += f' rotate({n(rot)})'
    if sc != 1:
        t += f' scale({n(sc)})'
    return f'<g transform="{t}">{body}</g>'


def MX(body, cx=80):
    """Mirror horizontally around x = cx."""
    return f'<g transform="matrix(-1 0 0 1 {n(2 * cx)} 0)">{body}</g>'


def poly(pts):
    return 'M' + ' L'.join(f'{n(x)} {n(y)}' for x, y in pts) + ' Z'


def star_pts(cx, cy, ro, ri, k=5, rot=-90):
    pts = []
    for i in range(2 * k):
        r = ro if i % 2 == 0 else ri
        a = math.radians(rot + i * 180 / k)
        pts.append((cx + r * math.cos(a), cy + r * math.sin(a)))
    return pts


def ring_pts(cx, cy, r, k, rot=0):
    return [(cx + r * math.cos(math.radians(rot + i * 360 / k)),
             cy + r * math.sin(math.radians(rot + i * 360 / k))) for i in range(k)]


def svg(w, h, body, grad=None, gid='g'):
    defs = ''
    if grad:
        c = col(grad)
        defs = (f'<defs><linearGradient id="{gid}" x1="0" y1="0" x2="0" y2="1">'
                f'<stop offset="0" stop-color="{c}"/><stop offset="1" stop-color="{c}" stop-opacity=".7"/>'
                '</linearGradient></defs>')
    body = body.replace('@G', gid)
    return (f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {w} {h}" fill="none" '
            f'stroke-linecap="round" stroke-linejoin="round">{defs}{body}</svg>\n')


def write(rel, w, h, body, grad=None, limit='card'):
    gid = 'g-' + re.sub(r'[^a-z0-9]+', '-', rel[:-4].lower())
    text = svg(w, h, body, grad, gid)
    hues = set(re.findall(r'\{\{(\w+)\}\}', text)) & HUES
    if len(hues) > 3:
        print(f'WARN {rel}: {len(hues)} hues {sorted(hues)}')
    if len(text.encode()) >= LIMITS[limit]:
        print(f'WARN {rel}: {len(text.encode())} bytes')
    path = os.path.join(ROOT, rel)
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, 'w', newline='\n') as fh:
        fh.write(text)
    WRITTEN.append(rel)


# ---------- card frame ----------
BLOB = 'M20 28 Q22 10 48 10 L116 12 Q148 12 148 40 L146 86 Q146 110 118 110 L42 108 Q12 108 14 80 Z'


def card(group, key, body, hue='a', grad=None, sx=80, sw=42):
    frame = P(BLOB, 'e', None) + C(80, 56, 38, hue, None, o='.16') + E(sx, 104, sw, 5, 'f', None, o='.08')
    write(f'{group}/{key}.svg', 160, 120, frame + body, grad)


# ---------- reusable props (drawn around the origin) ----------
def heart(f='d', s='f', w=3):
    return P('M0 12 C-18 1 -17 -14 -7 -14 C-3 -14 0 -11 0 -8 C0 -11 3 -14 7 -14 C17 -14 18 1 0 12 Z', f, s, w)


def star(f='w', ro=12, ri=5, s='f', w=2):
    return P(poly(star_pts(0, 0, ro, ri)), f, s, w)


def sparkle(f='w', r=6):
    h = r * .3
    return P(f'M0 {-r} L{h} {-h} L{r} 0 L{h} {h} L0 {r} L{-h} {h} L{-r} 0 L{-h} {-h} Z', f, None)


def sun(f='w', r=10, rays=8, rl=6):
    out = ''.join(L(f'M{n(math.cos(a) * (r + 4))} {n(math.sin(a) * (r + 4))} L{n(math.cos(a) * (r + 4 + rl))} {n(math.sin(a) * (r + 4 + rl))}', f, 3)
                  for a in (math.radians(i * 360 / rays) for i in range(rays)))
    return out + C(0, 0, r, f)


def moon(f='w', s='f'):
    return P('M2 -14 A14 14 0 1 0 14 2 A11 11 0 0 1 2 -14 Z', f, s, 2)


def cloud(f='u', s='f', w=3):
    return P('M-18 8 Q-27 8 -27 0 Q-27 -9 -17 -8 Q-15 -19 -3 -19 Q8 -19 10 -10 Q21 -12 23 -2 Q26 8 15 8 Z', f, s, w)


def gear(f='m', r=14, k=8, hole='u'):
    pts = []
    for i in range(k):
        a = 360 / k
        for da, rr in ((-a * .3, r), (-a * .18, r + 5), (a * .18, r + 5), (a * .3, r)):
            t = math.radians(i * a + da)
            pts.append((rr * math.cos(t), rr * math.sin(t)))
    return P(poly(pts), f, 'f', 2.5) + C(0, 0, r * .38, hole, 'f', 2)


def magnifier(f='u', h='f'):
    return T(R(-4, 12, 8, 20, 4, h, None), 0, 0, 1, -45) + C(0, 0, 13, f, 'f', 3) + L('M-6 -5 Q-4 -8 0 -9', 'f', 2, '.5')


def bulb(f='w'):
    return (P('M-8 10 Q-15 3 -15 -5 Q-15 -19 0 -19 Q15 -19 15 -5 Q15 3 8 10 Z', f) +
            R(-7, 11, 14, 8, 2, 'm', 'f', 2) + L('M-4 -2 L0 3 L4 -2 M0 3 L0 9', 'f', 2))


def clock(face='u', hand='f', wedge=None, frac=0.0, r=24):
    out = C(0, 0, r, face)
    if wedge:
        a = math.radians(-90 + 360 * frac)
        big = 1 if frac > .5 else 0
        rr = r - 4
        out += P(f'M0 0 L0 {-rr} A{rr} {rr} 0 {big} 1 {n(rr * math.cos(a))} {n(rr * math.sin(a))} Z', wedge, None)
    for i in range(12):
        t = math.radians(i * 30)
        if i % 3 == 0:
            out += L(f'M{n((r - 3) * math.cos(t))} {n((r - 3) * math.sin(t))} L{n((r - 7) * math.cos(t))} {n((r - 7) * math.sin(t))}', 'f', 2)
    return out + L('M0 0 L0 -14 M0 0 L9 4', hand, 3) + C(0, 0, 2.5, 'f', None)


def bubble(x, y, w, h, f='u', tail='l', s='f'):
    tx = x + 14 if tail == 'l' else x + w - 14
    d = 1 if tail == 'l' else -1
    path = (f'M{n(x + 10)} {n(y)} L{n(x + w - 10)} {n(y)} Q{n(x + w)} {n(y)} {n(x + w)} {n(y + 10)} L{n(x + w)} {n(y + h - 10)} '
            f'Q{n(x + w)} {n(y + h)} {n(x + w - 10)} {n(y + h)} L{n(tx + 8 * d)} {n(y + h)} L{n(tx - 4 * d)} {n(y + h + 9)} L{n(tx)} {n(y + h)} '
            f'L{n(x + 10)} {n(y + h)} Q{n(x)} {n(y + h)} {n(x)} {n(y + h - 10)} L{n(x)} {n(y + 10)} Q{n(x)} {n(y)} {n(x + 10)} {n(y)} Z')
    return P(path, f, s, 3)


def lines(x, y, widths, gap=7, c='m', w=3):
    return ''.join(L(f'M{n(x)} {n(y + i * gap)} h{n(wd)}', c, w) for i, wd in enumerate(widths) if wd)


def flame(f='d', inner='w'):
    return (P('M0 16 Q-13 14 -13 1 Q-13 -9 -3 -19 Q-1 -9 6 -7 Q13 -3 13 3 Q13 14 0 16 Z', f) +
            P('M0 12 Q-6 11 -6 5 Q-6 0 0 -5 Q1 0 4 2 Q6 4 5 7 Q4 12 0 12 Z', inner, None))


def leaf(f='s', s='f'):
    return P('M-12 8 Q-12 -10 12 -10 Q12 8 -12 8 Z', f, s, 2) + L('M-9 5 L6 -5', s, 2)


def shield(f='i', s='f'):
    return P('M0 -18 L15 -12 Q15 6 0 18 Q-15 6 -15 -12 Z', f, s, 3)


def book(f='b', page='u'):
    return (P('M-26 -12 L0 -6 L26 -12 L26 12 L0 18 L-26 12 Z', f, 'f', 2.5) + L('M0 -6 L0 18', 'f', 2) +
            L('M-20 -3 L-6 0 M-20 4 L-6 7 M6 0 L20 -3 M6 7 L20 4', page, 2))


def face(x, y, r=12, f='u', mouth='smile'):
    m = {'smile': f'M{n(x - 4)} {n(y + 3)} Q{n(x)} {n(y + 7)} {n(x + 4)} {n(y + 3)}',
         'flat': f'M{n(x - 4)} {n(y + 5)} L{n(x + 4)} {n(y + 5)}',
         'open': None, 'none': None}[mouth]
    out = C(x, y, r, f, 'f', 3) + C(x - 4, y - 2, 1.8, 'f', None) + C(x + 4, y - 2, 1.8, 'f', None)
    if m:
        out += L(m, 'f', 2)
    if mouth == 'open':
        out += E(x, y + 5, 3, 2.5, 'f', None)
    return out


# ---------- the bean person (feet at origin, ~76 tall) ----------
def arm(sx, sy, ang, ln, f):
    """Capsule arm from shoulder (sx,sy); ang 0 = straight down, positive = toward +x. Returns svg, hand pos."""
    t = math.radians(ang)
    hx, hy = sx + ln * math.sin(t), sy + ln * math.cos(t)
    body = T(R(-4.5, 0, 9, ln, 4.5, f, 'f', 2), sx, sy, 1, -ang)
    return body + C(hx, hy, 4.5, 'u', 'f', 2), (hx, hy)


def person(x, y, f='a', arms=(20, 20), mouth='smile', sc=1, hat='', front=''):
    """arms: (left, right) outward angle in degrees (0 = hanging down, 90 = level, 160 = raised); None hides it."""
    parts, hands = [], []
    for side, ang in ((-1, arms[0]), (1, arms[1])):
        if ang is None:
            hands.append(None)
            continue
        svg_, hand = arm(side * 12, -38, side * ang, 22, f)
        parts.append(svg_)
        hands.append((x + hand[0] * sc, y + hand[1] * sc))
    out = P('M-16 0 Q-19 -40 0 -44 Q19 -40 16 0 Z', f, 'f', 3) + ''.join(parts)
    out += face(0, -56, 12, 'u', mouth) + hat + front
    return T(out, x, y, sc), hands
