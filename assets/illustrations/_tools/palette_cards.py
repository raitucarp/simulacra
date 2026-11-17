"""Step palette cards (`palette_steps/<key>.svg`) and gallery category icons (`palette_categories/<key>.svg`),
160x120 option cards."""
from kit import *

STEP = 'palette_steps'
CAT = 'palette_categories'


def bolt(f='w', s=1):
    return T(P('M4 -18 L-8 2 L0 2 L-4 18 L10 -4 L2 -4 Z', f, 'f', 2.5), 0, 0, s)


def qmark(c='a'):
    return L('M-10 -14 Q-10 -26 0 -26 Q10 -26 10 -16 Q10 -9 0 -4 V4', c, 6) + C(0, 15, 3.6, c, None)


def node_graph(c='i'):
    pts = [(-26, -14), (0, -22), (26, -12), (-16, 14), (16, 16)]
    out = L('M-26 -14 L0 -22 L26 -12 M-26 -14 L-16 14 L16 16 L26 -12 M0 -22 L16 16 M0 -22 L-16 14', 'm', 2.5)
    return out + ''.join(C(x, y, 6, c, 'f', 2.5) for x, y in pts)


def calendar(marks=(), f='u', hi='a'):
    out = R(-30, -24, 60, 50, 6, f) + R(-30, -24, 60, 12, 6, 'd', 'f', 3) + L('M-18 -30 v10 M18 -30 v10', 'f', 3)
    for i in range(7):
        x, y = -24 + (i % 4) * 14, -4 + (i // 4) * 14
        out += R(x, y, 10, 9, 2, hi if i in marks else 'e', 'f', 1.5)
    return out


def house(f='w', lit='w'):
    return P('M-24 -2 L0 -24 L24 -2 Z', 'd') + R(-18, -2, 36, 26, 2, f) + R(-6, 6, 12, 18, 2, lit, 'f', 2)


def megaphone(f='d'):
    return P('M-24 -6 L10 -20 L10 20 L-24 6 Z', f) + R(-30, -7, 8, 14, 3, 'm', 'f', 2.5) + L('M18 -12 q8 12 0 24 M24 -18 q12 18 0 36', 'f', 2.5)


def frame_pic(f='u'):
    return R(-26, -20, 52, 40, 4, f) + P('M-20 14 L-6 -4 L4 8 L10 2 L20 14 Z', 's', None) + C(12, -8, 5, 'w', None)


def brush(f='d'):
    return T(R(-3, -30, 6, 34, 3, 'w', 'f', 2) + P('M-5 4 L5 4 L6 12 Q0 22 -6 12 Z', f, 'f', 2), 0, 0, 1, 35)


def globe(f='i'):
    return C(0, 0, 20, f) + L('M-20 0 h40 M0 -20 q-12 20 0 40 M0 -20 q12 20 0 40', 'u', 2.5)


def cairn():
    return E(0, 18, 22, 7, 'm') + E(0, 4, 16, 6, 'u') + E(0, -8, 11, 5, 'm') + E(0, -18, 7, 4, 'u')


def rose(f='d'):
    return L('M0 8 V30', 's', 3) + T(leaf('s'), 10, 20, .6) + C(0, 0, 12, f) + L('M-5 -2 q5 -6 10 0 q-5 6 -10 0 M-8 3 q8 6 16 0', 'f', 2)


def bubbles_stack():
    return bubble(-40, -36, 50, 22, 'u') + bubble(-16, -8, 54, 22, 'u') + bubble(8, 20, 34, 16, 'u') + L('M-26 -26 h22 M-2 2 h26 M16 28 h14', 'm', 3)


def voice_cards():
    card(STEP, 'plain_speaker', bubble(34, 30, 92, 50, 'u') + L('M50 55 h60', 'i', 4), 'i')
    card(STEP, 'lowercase_poet', bubble(26, 28, 84, 52, 'u') + L('M40 46 q8 -5 16 0 t16 0 M40 60 q8 -5 16 0', 'g', 3) + T(qmark_free(), 124, 70), 'g')
    card(STEP, 'academic_hedger', T(book('b'), 56, 76) + bubble(84, 18, 58, 40, 'u', 'r') + lines(94, 30, (36, 28, 36), 8), 'b')
    card(STEP, 'emoji_enthusiast', bubble(26, 26, 108, 54, 'u') + face(56, 52, 11, 'w', 'smile') + face(80, 52, 11, 'w', 'open') + face(104, 52, 11, 'd', 'smile'), 'w')
    card(STEP, 'telegraphic', bubble(30, 30, 100, 44, 'u') + L('M44 52 h4 M56 52 h14 M78 52 h4 M90 52 h14 M112 52 h4', 'f', 4), 'a')


def qmark_free():
    return P('M0 0 Q2 -22 20 -38 Q22 -16 4 -2 Z', 'u', 'f', 2.5) + L('M1 -1 L15 -25', 'f', 2)


def rhetoric_cards():
    card(STEP, 'socratic_questioner', T(qmark('a'), 64, 60, 1.3) + T(qmark('w'), 112, 66, .8), 'a')
    card(STEP, 'storyteller', T(book('s'), 80, 76, 1.3) + P('M58 56 L72 40 L82 50 L92 36 L104 56 Z', 'i', 'f', 2) + C(110, 30, 6, 'w', None), 's')
    card(STEP, 'polemicist', T(megaphone('d'), 76, 60, 1.2) + L('M130 36 l8 -6 M132 60 h10 M130 84 l8 6', 'w', 3), 'd')
    card(STEP, 'aphorist', R(40, 34, 80, 52, 8, 'u') + L('M54 50 h52 M54 70 h52', 'm', 3) + P('M60 60 l-8 -6 v12 Z M100 60 l8 -6 v12 Z', 'w', 'f', 2), 'w')
    card(STEP, 'surrealist_dreamer', E(80, 60, 34, 20, 'u') + C(80, 60, 11, 'g') + C(80, 60, 4, 'f', None) + T(cloud('u'), 120, 30, .6) + T(moon('w'), 40, 34, .8), 'g')


def daily_cards():
    card(STEP, 'early_bird', P('M30 92 A50 50 0 0 1 130 92 Z', 'w', None, o='.5') + T(sun('w', 14, 8, 6), 80, 80) + L('M104 40 q6 -6 12 0 q6 -6 12 0', 'f', 2.5), 'w')
    card(STEP, 'night_owl', T(moon('w'), 70, 58, 2) + T(star('w', 7, 3), 116, 36) + T(star('w', 5, 2), 110, 80) + T(star('w', 4, 2), 40, 34), 'g')
    card(STEP, 'office_hours', T(clock('u', 'f', 'a', .33, 26), 62, 58) + R(96, 58, 40, 30, 5, 'b') + L('M108 58 v-6 h16 v6', 'f', 3), 'a')
    card(STEP, 'weekend_wanderer', T(calendar((5, 6), 'u', 's'), 80, 62, 1.2), 's')
    card(STEP, 'quiet_hermit', T(house('u', 'w'), 80, 66, 1.3) + T(moon('w'), 126, 32, .8), 'b')


def mind_cards():
    card(STEP, 'skeptical_scientist', T(magnifier('u'), 64, 54, 1.2) + P('M104 40 h16 M108 40 V58 L98 84 H126 L116 58 V40', 's', 'f', 2.5), 's')
    card(STEP, 'systems_thinker', T(node_graph('i'), 80, 60, 1.3), 'i')
    card(STEP, 'intuitive_dreamer', T(cloud('u'), 80, 62, 1.4) + T(sparkle('w', 9), 112, 34) + T(sparkle('g', 6), 48, 36), 'g')
    card(STEP, 'quick_wit', T(bolt('w'), 80, 60, 2.2) + L('M40 44 h14 M36 60 h16 M40 76 h14', 'a', 3), 'a')


def disposition_cards():
    card(STEP, 'warm_heart', T(heart('d'), 80, 64, 2.2) + T(sparkle('w', 6), 120, 34), 'd')
    card(STEP, 'stoic_calm', T(cairn(), 80, 70, 1.5), 'i')
    card(STEP, 'brooding_romantic', T(rose('d'), 72, 54, 1.3) + T(cloud('u'), 116, 34, .6) + drop_card(112, 56) + drop_card(122, 66), 'd')
    card(STEP, 'rebel_spark', T(flame('d', 'w'), 80, 62, 2) + T(sparkle('w', 6), 118, 32) + T(sparkle('w', 5), 42, 40), 'w')


def drop_card(x, y):
    return T(P('M0 -6 Q5 0 5 3 A5 5 0 0 1 -5 3 Q-5 0 0 -6 Z', 'i', 'f', 2), x, y)


def engine_cards():
    card(STEP, 'fast_and_cheap', T(bolt('w'), 66, 58, 2) + C(112, 70, 14, 'w', 'f', 2.5) + C(112, 70, 7, None, 'f', 2), 'w')
    card(STEP, 'deep_reasoner', T(gear('m', 14), 64, 54) + T(gear('b', 10), 98, 76) + T(magnifier('u'), 112, 40, .7), 'b')
    card(STEP, 'illustrator', T(frame_pic('u'), 72, 62, 1.2) + T(brush('d'), 120, 62), 'd')
    card(STEP, 'researcher', T(globe('i'), 70, 58, 1.2) + T(magnifier('u'), 110, 72, .8), 'i')
    card(STEP, 'thread_weaver', T(bubbles_stack(), 80, 62), 's')


def category_icons():
    card(CAT, 'all', ''.join(R(x, y, 30, 26, 6, c) for x, y, c in ((46, 24, 'a'), (84, 24, 'w'), (46, 58, 's'), (84, 58, 'a'))), 'a')
    card(CAT, 'thinkers', T(bulb('w'), 80, 60, 2), 'w')
    card(CAT, 'objects', P('M50 44 h50 v24 q0 22 -25 22 q-25 0 -25 -22 Z', 'u') + P('M100 50 q16 0 16 12 q0 12 -16 12', None, 'f', 3) + E(75, 94, 34, 5, 'm') + L('M66 34 q-4 -6 0 -12 M82 34 q-4 -6 0 -12', 'm', 2.5), 'd')
    card(CAT, 'nature', L('M80 96 V66', 'f', 4) + C(80, 50, 26, 's') + C(62, 62, 14, 's') + C(98, 62, 14, 's'), 's')
    card(CAT, 'creatures', E(80, 70, 18, 15, 'w') + C(60, 46, 7, 'w') + C(74, 38, 7, 'w') + C(88, 38, 7, 'w') + C(102, 46, 7, 'w'), 'w')
    card(CAT, 'machines', T(gear('m', 18), 72, 58) + T(gear('i', 11), 106, 78), 'i')
    sky = R(40, 50, 24, 46, 3, 'b') + R(66, 30, 28, 66, 3, 'u') + R(96, 58, 24, 38, 3, 'b') + L('M30 96 h100', 'f', 3)
    sky += ''.join(R(x, y, 6, 6, 1.5, 'w', None) for x, y in ((47, 60), (47, 76), (73, 40), (83, 40), (73, 58), (83, 58), (73, 76), (103, 68)))
    card(CAT, 'places', sky + T(moon('w'), 124, 32, .7), 'b')
    card(CAT, 'concepts', L('M50 60 C50 40 74 40 80 60 C86 80 110 80 110 60 C110 40 86 40 80 60 C74 80 50 80 50 60 Z', 'g', 6), 'g')


def build():
    voice_cards()
    rhetoric_cards()
    daily_cards()
    mind_cards()
    disposition_cards()
    engine_cards()
    category_icons()
