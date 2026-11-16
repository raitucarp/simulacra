"""Layered portrait parts (256x256, shared origin).

Stack order: back -> face -> outfit -> mouth -> eyes -> brows -> hair -> accessory.
Anchors: head centre (128, 112); eyes y=116 at x=108/148; brows y~98; mouth y~148; shoulders y~190.
Skin uses {{skin}}, hair/brows use {{hair}}. Facial "ink" (pupils, lips) is #000/#fff with an opacity
so it reads on every skin tone and theme.
"""
from kit import *

INK = ' fill="#000" fill-opacity=".8"'
WHITE = ' fill="#fff" fill-opacity=".92"'
INK_S = ' stroke="#000" stroke-opacity=".75"'


def ink_path(d, w=4):
    return f'<path d="{d}"{INK_S} stroke-width="{w}"/>'


def ink_fill(shape):
    return shape.replace('/>', INK + '/>')


def part(layer, name, body, grad=None):
    write(f'portrait/{layer}/{name}.svg', 256, 256, body, grad)


# ---------- back ----------
def backs():
    full = 'M0 0 H256 V256 H0 Z'
    part('back', 'plain', P(full, 'G', None) + C(128, 120, 96, 'u', None, o='.35'), 'a')
    rays = ''.join(P(f'M128 128 L{n(128 + 200 * math.cos(math.radians(a)))} {n(128 + 200 * math.sin(math.radians(a)))} L{n(128 + 200 * math.cos(math.radians(a + 12)))} {n(128 + 200 * math.sin(math.radians(a + 12)))} Z', 'u', None, o='.25') for a in range(0, 360, 24))
    part('back', 'sunburst', P(full, 'w', None) + rays + C(128, 128, 70, 'u', None, o='.25'))
    stars = ''.join(T(sparkle('w', r), x, y) for x, y, r in ((30, 40, 8), (210, 30, 6), (230, 120, 9), (24, 150, 5), (60, 220, 6), (200, 210, 7), (120, 20, 5)))
    dots = ''.join(C(x, y, 2, 'u', None, o='.6') for x, y in ((80, 50), (170, 70), (40, 100), (220, 170), (150, 30), (100, 230)))
    part('back', 'starfield', P(full, 'G', None) + dots + stars + T(moon('w', None), 200, 70, 1.4), 'l')
    shelves = ''
    for row, y in enumerate((44, 120, 196)):
        shelves += R(0, y + 52, 256, 8, 0, 'm', None)
        x = 6
        for i, (w, h) in enumerate(((22, 44), (16, 38), (26, 48), (18, 40), (24, 46), (16, 36), (28, 44), (20, 50), (18, 40), (24, 44), (20, 38), (22, 46))):
            if x + w > 252:
                break
            c = ('a', 'i', 'w')[(i + row) % 3]
            shelves += R(x, y + 52 - h, w, h, 2, c, None)
            x += w + 2
    part('back', 'library', P(full, 'e', None) + shelves + P(full, 'k', None, o='.25'))
    hills = P('M0 170 Q70 130 140 168 Q200 140 256 160 V256 H0 Z', 's', None, o='.6') + P('M0 210 Q90 180 170 206 Q220 196 256 204 V256 H0 Z', 's', None)
    part('back', 'meadow', P(full, 'G', None) + T(sun('w', 22, 10, 8), 196, 62) + T(cloud('u', None), 60, 70, 1.3) + hills, 'i')
    geo = ''.join(P(poly(ring_pts(x, y, 26, 6, 30)), c, None, o='.35') for x, y, c in
                  ((30, 40, 'a'), (75, 66, 'b'), (30, 92, 'b'), (226, 40, 'b'), (181, 66, 'a'), (226, 92, 'a'), (30, 196, 'a'), (226, 196, 'b'), (181, 222, 'b'), (75, 222, 'a')))
    part('back', 'geometric', P(full, 'e', None) + geo)


# ---------- face (neck, ears, face shape) ----------
FACES = {
    'oval': ('E', (128, 112, 46, 56), 46),
    'round': ('E', (128, 116, 56, 50), 55),
    'square': ('R', (76, 60, 104, 108, 26), 52),
    'heart': ('P', 'M80 100 Q80 56 128 56 Q176 56 176 100 Q176 136 150 158 Q138 170 128 170 Q118 170 106 158 Q80 136 80 100 Z', 47),
    'long': ('E', (128, 114, 40, 62), 40),
    'diamond': ('P', 'M128 50 Q156 58 178 108 Q170 140 140 168 Q128 178 116 168 Q86 140 78 108 Q100 58 128 50 Z', 48),
}


def faces():
    for name, (kind, geo, half) in FACES.items():
        out = R(108, 150, 40, 52, 14, 'S') + E(128, 160, 22, 10, '#000', None, o='.14')
        for sx in (-1, 1):
            out += E(128 + sx * (half + 2), 120, 10, 14, 'S')
        if kind == 'E':
            out += E(*geo, 'S')
        elif kind == 'R':
            out += R(*geo, 'S')
        else:
            out += P(geo, 'S')
        out += L('M126 122 q-6 10 2 12', '#000', 3, '.3') + E(98, 138, 9, 5, 'd', None, o='.18') + E(158, 138, 9, 5, 'd', None, o='.18')
        part('face', name, out)


# ---------- outfit ----------
BUST = 'M34 256 Q34 202 94 188 L162 188 Q222 202 222 256 Z'


def outfits():
    toga = P(BUST, 'w') + P('M96 188 Q132 214 174 194 L210 230 Q206 250 200 256 L120 256 Q140 228 96 188 Z', 'a')
    toga += L('M60 220 q20 10 40 34 M150 208 q20 20 30 48', '#000', 3, '.2') + C(176, 200, 6, 'u', 'f', 2)
    part('outfit', 'toga', toga)
    lab = P(BUST, None, None).replace('/>', WHITE.replace('.92', '.97') + ' stroke="{{fg}}" stroke-width="3"/>') + P('M104 188 L128 236 L152 188 Z', 'i') + L('M128 236 V256 M104 188 L120 256 M152 188 L136 256', 'f', 3)
    lab += R(160, 222, 26, 22, 3, None, 'f', 2.5) + L('M166 222 v-8 M174 222 v-6', 'd', 3)
    part('outfit', 'lab_coat', lab)
    robe = P(BUST, 'G') + P('M100 188 Q128 232 156 188', None, 'w', 6) + P('M100 188 Q128 232 156 188', None, 'f', 2) + C(128, 226, 7, 'w', 'f', 2)
    part('outfit', 'robe', robe, 'a')
    hood = P('M50 220 Q46 188 84 180 Q128 214 172 180 Q210 188 206 220 Z', 's') + P(BUST, 's')
    hood += P('M88 184 Q128 222 168 184', None, 'f', 3) + L('M114 206 v26 M142 206 v26', 'u', 3) + R(98, 232, 60, 24, 8, 's', 'f', 2.5)
    part('outfit', 'hoodie', hood)
    suit = P(BUST, 'm') + P('M104 188 L128 220 L152 188 Z', 'u') + P('M122 198 h12 l3 40 l-9 12 l-9 -12 Z', 'd', 'f', 2)
    suit += P('M104 188 L96 210 L114 214 L120 240 Z M152 188 L160 210 L142 214 L136 240 Z', 'm', 'f', 2.5)
    part('outfit', 'suit', suit)
    arm = P(BUST, 'i') + P('M34 256 Q34 214 70 198 L84 214 Q60 228 60 256 Z M222 256 Q222 214 186 198 L172 214 Q196 228 196 256 Z', 'm')
    arm += P('M100 192 h56 l-6 40 h-44 Z', 'm', 'f', 2.5) + T(shield('w'), 128, 212, .8) + C(76, 208, 3, 'f', None) + C(180, 208, 3, 'f', None)
    part('outfit', 'tunic', arm)
    gown = P(BUST, None, None).replace('/>', INK.replace('.8', '.94') + ' stroke="{{fg}}" stroke-width="3"/>') + P('M96 188 L84 256 M160 188 L172 256', None, 'b', 8) + P('M96 188 Q128 212 160 188 Q140 200 128 202 Q116 200 96 188 Z', 'u', 'f', 2)
    gown += P('M84 256 L96 188 M172 256 L160 188', None, 'f', 2)
    part('outfit', 'scholar_gown', gown)
    card_ = P(BUST, 'w') + P('M104 188 L128 224 L152 188 Z', 'u') + L('M128 224 V256', 'f', 3)
    card_ += C(122, 234, 3, 'f', None) + C(122, 248, 3, 'f', None) + L('M60 230 v26 M196 230 v26', '#000', 3, '.2') + R(150, 224, 22, 18, 4, 'w', 'f', 2.5)
    part('outfit', 'cardigan', card_)


# ---------- mouth ----------
def mouths():
    part('mouth', 'smile', ink_path('M112 144 Q128 158 144 144'))
    part('mouth', 'grin', ink_fill(P('M108 142 Q128 168 148 142 Z', None, None)) + P('M112 144 h32 l-2 6 h-28 Z', None, None).replace('/>', WHITE + '/>'))
    part('mouth', 'neutral', ink_path('M114 148 h28'))
    part('mouth', 'smirk', ink_path('M114 150 Q132 152 144 140'))
    part('mouth', 'open', ink_fill(E(128, 150, 8, 10, None, None)))
    part('mouth', 'laugh', ink_fill(P('M106 140 Q128 176 150 140 Q128 146 106 140 Z', None, None)) + E(128, 158, 9, 5, 'd', None))
    part('mouth', 'frown', ink_path('M114 152 Q128 142 142 152'))
    part('mouth', 'cat', ink_path('M112 144 q8 8 16 0 q8 8 16 0', 3.5))


# ---------- eyes ----------
def pair(fn):
    return fn(108) + fn(148)


def eyes():
    part('eyes', 'dot', pair(lambda x: ink_fill(C(x, 116, 5, None, None))))
    part('eyes', 'round', pair(lambda x: E(x, 116, 10, 11, None, None).replace('/>', WHITE + '/>') + ink_fill(C(x, 117, 6, None, None)) + C(x + 2, 114, 2, None, None).replace('/>', WHITE + '/>')))
    part('eyes', 'almond', pair(lambda x: P(f'M{x - 12} 117 Q{x} 104 {x + 12} 117 Q{x} 126 {x - 12} 117 Z', None, None).replace('/>', WHITE + '/>') +
                                ink_fill(C(x, 116, 5.5, None, None)) + ink_path(f'M{x - 13} 116 Q{x} 102 {x + 13} 116', 3)))
    part('eyes', 'sleepy', pair(lambda x: ink_fill(P(f'M{x - 10} 116 Q{x} 124 {x + 10} 116 Z', None, None)) + ink_path(f'M{x - 11} 116 h22', 3.5)))
    part('eyes', 'happy', pair(lambda x: ink_path(f'M{x - 9} 119 Q{x} 108 {x + 9} 119')))
    part('eyes', 'wink', ink_fill(C(108, 116, 5.5, None, None)) + ink_path('M139 117 Q148 110 157 117'))
    part('eyes', 'wide', pair(lambda x: C(x, 116, 12, None, None).replace('/>', WHITE + '/>') + ink_fill(C(x, 116, 4, None, None)) + f'<circle cx="{x}" cy="116" r="12" fill="none"{INK_S} stroke-width="2.5"/>'))
    part('eyes', 'narrow', pair(lambda x: ink_path(f'M{x - 11} 117 Q{x} 112 {x + 11} 117', 4.5)))


# ---------- brows ----------
def brows():
    def b(d, w=0):
        return f'<path d="{d}" fill="{{{{hair}}}}"{INK_S.replace(".75", ".45")} stroke-width="2"/>'
    part('brows', 'straight', b('M95 97 h26 v5 h-26 Z') + b('M135 97 h26 v5 h-26 Z'))
    part('brows', 'arched', b('M94 102 Q106 90 122 98 L121 102 Q106 96 95 106 Z') + b('M162 102 Q150 90 134 98 L135 102 Q150 96 161 106 Z'))
    part('brows', 'raised', b('M95 92 Q108 82 121 90 L120 94 Q108 88 96 96 Z') + b('M161 92 Q148 82 135 90 L136 94 Q148 88 160 96 Z'))
    part('brows', 'furrowed', b('M94 94 L122 102 L120 107 L93 99 Z') + b('M162 94 L134 102 L136 107 L163 99 Z'))
    part('brows', 'bushy', b('M92 100 Q94 88 108 90 Q120 90 124 98 Q116 96 108 100 Q98 100 92 104 Z') + b('M164 100 Q162 88 148 90 Q136 90 132 98 Q140 96 148 100 Q158 100 164 104 Z'))
    part('brows', 'thin', ink_path('M96 99 Q108 93 120 98', 2.5).replace(INK_S, ' stroke="{{hair}}"') + ink_path('M136 98 Q148 93 160 99', 2.5).replace(INK_S, ' stroke="{{hair}}"'))


# ---------- hair ----------
def hair_shape(d, extra=''):
    return P(d, 'H') + extra


SHINE = ' fill="#fff" fill-opacity=".18"'


def hairs():
    part('hair', 'bald', P('M100 70 Q112 58 130 58 Q120 64 108 76 Z', None, None).replace('/>', SHINE + '/>'))
    part('hair', 'buzz', P('M80 104 Q78 56 128 54 Q178 56 176 104 Q170 80 128 76 Q86 80 80 104 Z', 'H', 'f', 2.5))
    part('hair', 'crop', hair_shape('M76 108 Q70 48 128 46 Q186 48 180 108 Q172 92 170 78 Q150 88 120 82 Q96 80 86 76 Q82 92 76 108 Z'))
    part('hair', 'side_part', hair_shape('M76 112 Q66 46 124 44 Q186 44 182 112 Q176 86 172 74 Q140 72 116 62 Q110 80 84 84 Q80 96 76 112 Z', L('M116 50 Q116 58 118 62', 'f', 2)))
    part('hair', 'bob', hair_shape('M72 160 Q58 100 74 70 Q96 40 132 44 Q178 46 186 90 Q194 130 184 160 L168 160 Q172 120 168 90 Q140 88 112 72 Q98 88 88 92 Q84 124 90 160 Z'))
    part('hair', 'long', hair_shape('M60 230 Q56 150 70 90 Q84 44 130 44 Q178 46 188 92 Q200 150 196 230 L172 230 Q174 160 170 96 Q140 84 116 66 Q100 88 86 96 Q82 160 84 230 Z'))
    curls = ''.join(C(x, y, r, 'H') for x, y, r in ((84, 92, 18), (100, 68, 20), (128, 58, 22), (156, 68, 20), (172, 92, 18), (76, 122, 14), (180, 122, 14), (112, 54, 14), (146, 54, 14)))
    part('hair', 'curly', curls)
    bun = C(128, 34, 20, 'H') + P('M78 110 Q72 52 128 50 Q184 52 178 110 Q170 82 128 76 Q86 82 78 110 Z', 'H') + L('M110 46 q18 -8 36 0', 'f', 2)
    part('hair', 'bun', bun)
    pony = P('M170 80 Q206 100 200 160 Q196 196 180 206 Q190 160 176 120 Z', 'H') + P('M78 108 Q72 50 128 48 Q184 50 178 108 Q170 80 128 74 Q86 80 78 108 Z', 'H') + R(170, 84, 12, 14, 4, 'a', 'f', 2)
    part('hair', 'ponytail', pony)
    spikes = P('M80 100 L72 64 L92 72 L90 40 L110 58 L118 26 L132 54 L148 28 L152 58 L172 42 L168 72 L186 66 L176 100 Q160 78 128 76 Q96 78 80 100 Z', 'H')
    part('hair', 'spiky', spikes)
    wavy = hair_shape('M64 190 Q48 170 64 150 Q52 128 68 108 Q60 60 100 48 Q140 36 172 56 Q196 76 190 110 Q204 130 192 150 Q206 170 190 190 L172 188 Q180 160 172 140 Q178 110 170 92 Q140 90 120 70 Q100 90 86 96 Q78 120 84 140 Q76 160 84 188 Z')
    part('hair', 'wavy', wavy)
    braids = P('M78 108 Q72 50 128 48 Q184 50 178 108 Q170 80 128 74 Q86 80 78 108 Z', 'H')
    for x in (78, 178):
        braids += ''.join(E(x, 120 + i * 18, 10, 11, 'H') for i in range(5)) + C(x, 212, 5, 'a', 'f', 2)
    part('hair', 'braids', braids)


# ---------- accessory ----------
def accessories():
    part('accessory', 'none', '')
    gl = C(108, 116, 17, 'u', 'f', 3, o='.25') + C(148, 116, 17, 'u', 'f', 3, o='.25') + L('M125 114 q3 -3 6 0 M91 112 L76 108 M165 112 L180 108', 'f', 3)
    part('accessory', 'glasses', gl)
    laurel = ''
    for d in range(160, 381, 20):
        if 250 < d < 290:
            continue
        t = math.radians(d)
        laurel += T(P('M-9 0 Q0 -7 9 0 Q0 7 -9 0 Z', 's', 'f', 2), 128 + 58 * math.cos(t), 104 + 54 * math.sin(t), 1, d + 90)
    part('accessory', 'laurel', laurel + C(128, 50, 5, 'w', 'f', 2))
    scarf = P('M80 186 Q128 214 176 186 L182 202 Q128 232 74 202 Z', 'd') + P('M150 204 L160 252 L140 254 L134 210 Z', 'd')
    scarf += L('M96 198 l-2 10 M116 206 v10 M140 206 v10 M160 198 l2 10', '#000', 2.5, '.2')
    part('accessory', 'scarf', scarf)
    mono = C(148, 116, 16, 'u', 'w', 3.5, o='.25') + L('M160 128 Q170 160 156 190', 'w', 2)
    part('accessory', 'monocle', mono)
    hp = P('M70 120 Q66 44 128 40 Q190 44 186 120', None, 'f', 9) + P('M70 120 Q66 44 128 40 Q190 44 186 120', None, 'a', 5)
    hp += R(60, 104, 22, 38, 10, 'a') + R(174, 104, 22, 38, 10, 'a')
    part('accessory', 'headphones', hp)


def build():
    backs()
    faces()
    outfits()
    mouths()
    eyes()
    brows()
    hairs()
    accessories()
