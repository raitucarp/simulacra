"""Engine step cards: model source, providers, request mode and HTTP verbs, answer shape and schema presets,
image engines, local server flavours, image sizes and triggers, tools and search backends, plus the Engine banner.
Every provider and service is drawn as a generic idea (a cloud model, a local server, a custom wire), never a logo."""
from kit import *
from cards_rhetoric import postcard, check, bolt
from cards_voice import globe
from scenes import banner


# ---------- local props (drawn around the origin unless noted) ----------
def frame(x, y, w, h, hue='s', f='u'):
    """A picture frame with hills and a sun (sun in warning)."""
    hills = P(f'M{n(x + 4)} {n(y + h - 4)} L{n(x + w * .38)} {n(y + h * .42)} L{n(x + w * .58)} {n(y + h * .68)} '
              f'L{n(x + w * .74)} {n(y + h * .52)} L{n(x + w - 4)} {n(y + h - 4)} Z', hue, None)
    return R(x, y, w, h, 5, f) + hills + C(x + w * .74, y + h * .28, min(w, h) * .1, 'w', None)


def chip_ic(f='b', s=1):
    """A processor chip with pins."""
    pins = ''.join(L(f'M{d} -16 v-6 M{d} 16 v6 M-16 {d} h-6 M16 {d} h6', 'f', 2.5) for d in (-8, 0, 8))
    return T(pins + R(-16, -16, 32, 32, 6, f) + R(-8, -8, 16, 16, 3, 'u', 'f', 2), 0, 0, s)


def server(f='u', led='s'):
    """A small server tower, bottom at the origin."""
    out = R(-16, -46, 32, 46, 5, f)
    for y in (-38, -26, -14):
        out += L(f'M-9 {y} h12', 'm', 3) + C(9, y, 2.5, led, None)
    return out


def plug(f='m'):
    """A two-pin plug facing right, centre at the origin."""
    return R(-10, -9, 16, 18, 4, f) + L('M6 -4 h8 M6 4 h8', 'f', 3) + L('M-10 0 h-8', 'f', 3)


def trash(f='d'):
    return R(-12, -14, 24, 28, 4, f) + R(-16, -20, 32, 6, 3, 'u', 'f', 2.5) + L('M-4 -24 h8 M-5 -7 v14 M0 -7 v14 M5 -7 v14', 'f', 2.5)


def braces(x, y, h=20, c='f', w=3):
    """A pair of curly braces around (x, y), 30 apart."""
    a, b = h / 2, h / 4
    left = f'M{n(x - 10)} {n(y - a)} q-5 0 -5 {n(b)} q0 {n(b)} -5 {n(b)} q5 0 5 {n(b)} q0 {n(b)} 5 {n(b)}'
    right = f'M{n(x + 10)} {n(y - a)} q5 0 5 {n(b)} q0 {n(b)} 5 {n(b)} q-5 0 -5 {n(b)} q0 {n(b)} -5 {n(b)}'
    return L(left, c, w) + L(right, c, w)


def die(x, y, f='u', pips=((0, 0),)):
    return R(x - 13, y - 13, 26, 26, 6, f) + ''.join(C(x + px * 6, y + py * 6, 2.6, 'f', None) for px, py in pips)


def rows(x, y, k, w=46, c='m', mark=None):
    out = ''
    for i in range(k):
        yy = y + i * 14
        out += R(x, yy, w, 10, 5, 'u', 'f', 2)
        if mark:
            out += T(mark, x + w + 8, yy + 5)
    return out


# ---------- model source and providers ----------
def sources():
    g = 'engine_source'
    hub = T(cloud('u'), 80, 40, 1.4) + L('M58 54 L40 80 M80 56 V82 M102 54 L120 80', 'm', 2.5)
    hub += ''.join(C(x, 88, 9, 'g') + C(x, 86, 3, 'u', None) for x in (40, 80, 120))
    card(g, 'use_global', hub, 'a')
    p, _ = person(56, 100, 'g', (25, 70))
    card(g, 'own', p + T(chip_ic('b'), 108, 56, 1) + T(sparkle('w', 6), 132, 26), 'g')


def providers():
    g = 'engine_provider'
    orbit = E(80, 58, 44, 16, None, 'm', 2.5) + C(80, 58, 13, 'i') + C(46, 50, 9, 'b') + C(114, 66, 9, 'b')
    card(g, 'gemini', orbit + T(sparkle('w', 5), 128, 30), 'i')
    talk = bubble(36, 26, 64, 44, 'u', 'l') + lines(46, 40, (40, 30), 10) + T(cloud('s'), 112, 80, .9)
    card(g, 'open_ai', talk, 's')
    scroll = R(46, 30, 56, 66, 6, 'u') + E(74, 30, 28, 6, 'w', 'f', 2.5) + E(74, 96, 28, 6, 'w', 'f', 2.5) + lines(56, 46, (36, 28, 36, 22), 10)
    card(g, 'anthropic', scroll + T(sparkle('w', 7), 118, 40) + T(sparkle('a', 4), 124, 64), 'w')
    hub = R(64, 46, 32, 26, 6, 'a') + C(80, 59, 4, 'u', None)
    hub += L('M40 32 Q56 34 64 50 M40 86 Q56 84 64 68 M96 59 H124', 'm', 2.5) + C(36, 30, 7, 'u') + C(36, 88, 7, 'u') + C(128, 59, 7, 'u')
    card(g, 'open_router', hub + P('M118 54 l8 5 l-8 5 Z', 'f', None), 'a')
    speed = T(chip_ic('d'), 92, 58, .95) + L('M26 44 h26 M20 58 h32 M28 72 h24', 'm', 3) + T(bolt('w'), 92, 58, .8)
    card(g, 'groq', speed, 'd')
    wire = T(plug('m'), 48, 62) + L('M30 62 Q20 62 20 78 Q20 94 40 94 H70', 'f', 3) + R(92, 36, 36, 50, 8, 'u')
    wire += L('M104 54 v8 M116 54 v8', 'f', 3) + C(110, 74, 4, 'b', None)
    card(g, 'custom', wire + L('M58 58 L86 58', 'b', 3) + L('M58 66 L86 66', 'b', 3), 'b')


# ---------- request ----------
def requests():
    g = 'engine_request'
    app = R(26, 40, 40, 36, 7, 'u') + lines(34, 52, (24, 16), 9)
    card(g, 'native', app + L('M70 58 H98', 's', 4) + P('M96 50 l10 8 l-10 8 Z', 's', None) + T(cloud('u'), 126, 62, .9), 's')
    env = R(30, 34, 74, 50, 7, 'u') + L('M30 38 L67 62 L104 38', 'f', 2.5) + braces(67, 70, 14, 'w', 2.5)
    card(g, 'custom', env + T(gear('w', 11, 7, 'u'), 122, 36) + T(sparkle('w', 5), 126, 84), 'w')


def methods():
    g = 'engine_method'
    tray = lambda c: R(46, 74, 68, 20, 6, 'u') + L('M46 80 H64 Q66 88 80 88 Q94 88 96 80 H114', 'f', 2.5)
    card(g, 'get', tray('i') + L('M80 26 V66', 'i', 5) + P('M68 54 L80 68 L92 54 Z', 'i', None) + T(cloud('u'), 120, 32, .6), 'i')
    card(g, 'post', tray('s') + L('M80 68 V30', 's', 5) + P('M68 40 L80 26 L92 40 Z', 's', None) + T(cloud('u'), 40, 32, .6), 's')
    shelf = R(36, 30, 88, 60, 7, 'u') + L('M36 60 H124', 'f', 2.5) + R(46, 40, 22, 16, 3, 'm', 'f', 2)
    card(g, 'put', shelf + R(92, 66, 24, 18, 3, 'a') + L('M104 18 V36', 'a', 4) + P('M96 30 L104 40 L112 30 Z', 'a', None), 'a')
    doc = R(48, 26, 52, 68, 6, 'u') + lines(58, 40, (32, 22, 32, 26), 10)
    patch = T(R(-20, -8, 40, 16, 8, 'w') + C(-6, 0, 1.6, 'f', None) + C(6, 0, 1.6, 'f', None) + C(0, -3, 1.6, 'f', None) + C(0, 3, 1.6, 'f', None), 96, 76, 1, -30)
    card(g, 'patch', doc + patch, 'w')
    card(g, 'delete', T(trash('d'), 80, 62, 1.4) + L('M34 40 l8 8 M126 40 l-8 8', 'm', 2.5), 'd')


# ---------- answer shape and schema presets ----------
def outputs():
    g = 'engine_output'
    card(g, 'free_text', bubble(34, 24, 92, 58, 'u', 'l') + L('M46 42 q8 -5 16 0 t16 0 t16 0 t16 0 M46 56 q8 -5 16 0 t16 0 t16 0 M46 70 q8 -5 16 0', 'a', 2.5), 'a')
    doc = R(42, 22, 76, 76, 7, 'u') + braces(80, 60, 52, 'i', 3)
    doc += ''.join(R(76, y, 8, 6, 2, 'i', None) + R(88, y, 18, 6, 3, 'm', None) for y in (40, 54, 68))
    card(g, 'json_schema', doc + C(122, 28, 10, 's') + check(122, 28), 'i')


def schemas():
    g = 'engine_schema'
    hashmark = L('M110 66 l-4 22 M120 66 l-4 22 M102 72 h22 M100 82 h22', 'a', 3)
    card(g, 'post', postcard(30, 24, 76, 56, 'a', (56, 40)) + hashmark, 'a')
    card(g, 'illustrated', postcard(26, 22, 70, 58, 's', (48,)) + frame(80, 50, 52, 40, 's'), 's')
    thread = L('M44 34 V86', 'm', 3) + postcard(34, 18, 90, 26, 'i', ()) + postcard(48, 48, 76, 24, 'i', ()) + postcard(48, 78, 76, 24, 'i', ())
    card(g, 'thread', thread, 'i')
    mood = postcard(26, 26, 74, 56, 'd', (52, 36)) + C(116, 52, 16, 'w', 'f', 2.5) + C(110, 48, 2, 'f', None) + C(122, 48, 2, 'f', None)
    card(g, 'expressive', mood + L('M108 56 Q116 64 124 56', 'f', 2.5) + T(heart('d'), 126, 84, .6), 'd')
    grid = R(30, 24, 100, 70, 6, 'u') + ''.join(L(f'M{x} 28 V90', 'b', 1.5, '.4') for x in (50, 70, 90, 110)) + ''.join(L(f'M34 {y} H126', 'b', 1.5, '.4') for y in (44, 64))
    pencil = T(R(-4, -30, 8, 40, 2, 'w') + P('M-4 10 L0 18 L4 10 Z', 'f', None), 108, 62, 1, 40)
    card(g, 'custom', grid + L('M40 80 L64 42 L88 70', 'b', 3) + pencil, 'b')


# ---------- image engines, local flavours, sizes, triggers ----------
def images():
    g = 'engine_image'
    card(g, 'none', frame(44, 30, 72, 56, 'm') + L('M36 96 L124 22', 'd', 5), 'd')
    brush = T(R(-3, -26, 6, 30, 3, 'w') + P('M-5 4 Q0 14 5 4 Z', 's', 'f', 2), 120, 60, 1, 30)
    card(g, 'open_ai', T(cloud('u'), 64, 40, 1.1) + frame(38, 56, 56, 40, 's') + brush, 's')
    prism = P('M110 32 L128 64 L92 64 Z', 'u', 'f', 2.5) + L('M128 52 l14 -6 M130 58 l14 0 M128 64 l14 6', 'i', 2.5)
    card(g, 'google', T(cloud('u'), 58, 40, 1.1) + frame(34, 56, 56, 40, 'i') + prism, 'i')
    screen = R(30, 26, 72, 52, 6, 'u') + frame(36, 32, 60, 40, 'g', 'k') + L('M66 78 v10 M52 90 h28', 'f', 3)
    card(g, 'local_sd', screen + T(server('u', 'g'), 124, 96), 'g')
    wire = T(plug('m'), 34, 60) + L('M52 56 H70 M52 64 H70', 'w', 3)
    card(g, 'custom', wire + frame(74, 30, 60, 50, 'w') + braces(40, 90, 12, 'w', 2.5), 'w')


def flavours():
    g = 'engine_sd'
    nodes = R(26, 30, 34, 24, 6, 'u') + R(26, 70, 34, 24, 6, 'u') + R(100, 48, 34, 28, 6, 'i')
    nodes += L('M60 42 C80 42 80 58 100 58 M60 82 C80 82 80 66 100 66', 'i', 2.5) + C(60, 42, 3, 'f', None) + C(60, 82, 3, 'f', None)
    card(g, 'comfy_ui', nodes + lines(32, 40, (20,), 0) + lines(32, 80, (20,), 0) + frame(106, 54, 22, 16, 'i', 'u'), 'i')
    panel = R(32, 22, 96, 76, 7, 'u') + R(32, 22, 96, 12, 6, 'a', None, o='.5')
    for i, at in enumerate((.3, .7, .5)):
        y = 50 + i * 16
        panel += L(f'M44 {y} H116', 'm', 3) + C(44 + 72 * at, y, 5, 'a', 'f', 2)
    card(g, 'a1111', panel, 'a')


def sizes():
    g = 'engine_size'
    card(g, 'square', frame(50, 28, 60, 60, 'a'), 'a')
    card(g, 'portrait', frame(58, 20, 44, 76, 'b'), 'b')
    card(g, 'landscape', frame(30, 34, 100, 54, 's'), 's')
    small = frame(64, 42, 32, 32, 'i') + L('M40 30 l12 12 M120 30 l-12 12 M40 86 l12 -12 M120 86 l-12 -12', 'm', 2.5)
    card(g, 'small', small, 'i')


def triggers():
    g = 'engine_trigger'
    cam = R(42, 40, 76, 50, 9, 'u') + R(60, 32, 22, 10, 3, 'u', 'f', 2.5) + C(80, 65, 16, 'm') + C(80, 65, 10, 'd', 'f', 2)
    card(g, 'never', cam + L('M70 65 h20', 'u', 3) + T(moon('w', 'f'), 126, 30, .7), 'd')
    dice = die(64, 66, 'u', ((-1, -1), (0, 0), (1, 1))) + T(die(0, 0, 'w', ((-1, -1), (1, -1), (-1, 1), (1, 1))), 102, 56, 1, 18)
    card(g, 'chance', dice + T(sparkle('w', 5), 128, 28), 'w')
    loop = L('M40 60 A20 20 0 1 1 80 60 A20 20 0 1 0 120 60 A20 20 0 1 0 80 60 A20 20 0 1 1 40 60', 's', 4)
    card(g, 'always', loop + frame(64, 76, 32, 24, 's') + P('M114 40 l8 -6 l2 10 Z', 's', None), 's')
    ask = bubble(30, 22, 70, 52, 'u', 'l') + frame(42, 32, 46, 32, 'a') + T(sparkle('w', 6), 112, 30)
    p, _ = person(122, 104, 'a', (20, 160), sc=.62)
    card(g, 'when_model_asks', ask + p, 'a')


# ---------- tools and search backends ----------
def tools():
    g = 'engine_tool'
    books = R(30, 66, 56, 12, 3, 'b') + R(34, 54, 52, 12, 3, 'u') + R(28, 42, 58, 12, 3, 'b')
    card(g, 'vault_search', books + T(magnifier('u'), 104, 50, 1.1), 'b')
    net = L('M50 40 L110 44 L80 88 Z', 'm', 2.5) + face(50, 40, 12, 'g') + face(110, 44, 12, 'g') + face(80, 88, 12, 'g')
    card(g, 'read_network', net, 'g')
    card(g, 'web_search', T(globe('i'), 66, 54, 2.3) + T(magnifier('u'), 112, 70, .9), 'i')
    page = R(84, 40, 44, 54, 6, 'u') + lines(92, 54, (28, 20, 28), 10)
    card(g, 'web_fetch', T(globe('s'), 48, 46, 1.8) + L('M66 64 Q76 76 82 74', 's', 3) + P('M78 66 l8 8 l-10 4 Z', 's', None) + page, 's')
    tool = T(plug('m'), 42, 60) + L('M60 56 H80 M60 64 H80', 'w', 3) + T(gear('w', 16, 8, 'u'), 108, 60)
    card(g, 'custom_http', tool + T(sparkle('w', 5), 126, 28), 'w')


def searches():
    g = 'engine_search'
    card(g, 'provider_native', T(cloud('u'), 72, 58, 1.8) + T(magnifier('a'), 76, 58, .75), 'a')
    medal = C(0, 0, 4, 'w', 'f', 2)
    card(g, 'brave', rows(34, 32, 4, 50, mark=medal) + T(magnifier('u'), 118, 76, .8), 'w')
    linked = rows(34, 32, 4, 50) + L('M92 37 Q104 51 92 65 Q104 79 92 93', 'i', 2.5) + C(92, 37, 3.5, 'i', None) + C(92, 65, 3.5, 'i', None) + C(92, 93, 3.5, 'i', None)
    card(g, 'tavily', linked + T(magnifier('u'), 122, 46, .8), 'i')


# ---------- step banner ----------
def banners():
    en = T(gear('m', 22, 10, 'u'), 160, 60) + T(chip_ic('a'), 160, 60, .55)
    en += T(cloud('u'), 70, 42, 1.3) + L('M92 50 Q120 54 132 58', 'm', 2.5, '.8')
    en += bubble(204, 20, 58, 40, 'u', 'l') + braces(233, 40, 18, 'a', 2.5) + L('M186 62 Q196 56 204 52', 'm', 2.5)
    en += frame(212, 70, 46, 34, 's') + T(magnifier('u'), 98, 86, .7) + T(server('u', 's'), 280, 104, .8)
    en += T(sparkle('w', 6), 128, 24) + T(sparkle('w', 4), 196, 96) + T(bolt('w'), 46, 88, .6)
    banner('engine', en, 'a')


def build():
    sources()
    providers()
    requests()
    methods()
    outputs()
    schemas()
    images()
    flavours()
    sizes()
    triggers()
    tools()
    searches()
    banners()
