"""Settings window category headers (`settings/<category>.svg`), 160x120 option cards: one small scene per
category of the settings rail (General, Appearance, Reading & Writing, ... Advanced)."""
from kit import *
from cards_voice import globe
from cards_engine import chip_ic, frame, server
from palette_cards import node_graph

G = 'settings'


def page(x, y, w=44, h=56, f='u', widths=(30, 24, 30, 18)):
    """A sheet of paper with text lines."""
    return R(x, y, w, h, 5, f) + lines(x + 7, y + 12, widths, 9)


def pencil(f='w'):
    """A pencil pointing down-left, centre at the origin."""
    return T(R(-5, -22, 10, 34, 3, f) + P('M-5 12 L0 22 L5 12 Z', 'u', 'f', 2.5) + R(-5, -26, 10, 6, 2, 'd', 'f', 2), 0, 0, 1, 40)


def key_cap(x, y, w=18, f='u'):
    return R(x, y, w, 16, 4, f) + R(x + 3, y + 3, w - 6, 6, 2, 'e', None)


def bell(f='w'):
    return (P('M-14 8 Q-14 -14 0 -16 Q14 -14 14 8 L18 12 L-18 12 Z', f) + C(0, 17, 4, 'f', None) +
            C(0, -18, 3, f, 'f', 2))


def lock(f='w'):
    return L('M-7 -4 V-11 Q-7 -18 0 -18 Q7 -18 7 -11 V-4', 'f', 3) + R(-11, -5, 22, 18, 4, f) + C(0, 4, 2.5, 'f', None)


def general():
    card(G, 'general', T(gear('a', 18), 64, 54) + T(globe('i'), 108, 40, 1.1) + T(clock('u', 'f', 'b', .3, 14), 110, 80), 'a')


def appearance():
    swatches = ''.join(C(x, y, 9, c) for x, y, c in ((58, 42, 'a'), (80, 34, 'b'), (102, 44, 'w')))
    palette = P('M40 66 Q40 30 80 30 Q122 30 122 62 Q122 84 98 80 Q86 78 84 88 Q80 100 64 96 Q40 90 40 66 Z', 'u')
    card(G, 'appearance', palette + swatches + T(sun('w', 7, 8, 4), 120, 24) + T(moon('w'), 40, 26, .8), 'b')


def reading_writing():
    card(G, 'reading_writing', page(44, 24) + page(62, 32, widths=(30, 30, 22, 26)) + T(pencil('w'), 116, 58), 'w')


def ai_models():
    card(G, 'ai_models', T(chip_ic('a'), 70, 58, 1.3) + T(sparkle('w', 9), 118, 34) + T(sparkle('i', 5), 126, 62), 'a')


def embeddings():
    card(G, 'embeddings', T(node_graph('i'), 72, 58, 1.1) + T(magnifier('u'), 114, 74, .8), 'i')


def images():
    card(G, 'images', frame(36, 28, 64, 48, 's') + T(sparkle('w', 7), 116, 30) + face(112, 78, 14, 'b'), 's')


def agents():
    left, _ = person(58, 100, 'g', (25, 60), sc=.8)
    right, _ = person(104, 100, 'b', (60, 25), sc=.8)
    talk = bubble(62, 18, 38, 22, 'u', 'l') + lines(70, 29, (20,), 8)
    card(G, 'agents', left + right + talk, 'g')


def learning():
    card(G, 'learning', T(book('b'), 72, 70, 1.2) + T(bulb('w'), 112, 38, .9), 'w')


def importing():
    box = R(84, 58, 44, 34, 5, 'u') + L('M84 70 h44', 'f', 2.5)
    arrow = L('M66 44 Q92 34 104 50', 's', 4) + P('M97 50 L111 50 L104 60 Z', 's', 'f', 2)
    card(G, 'import', page(34, 30, 36, 46, widths=(22, 18, 22)) + arrow + box, 's')


def privacy_data():
    card(G, 'privacy_data', T(server('u', 's'), 62, 96) + T(shield('i'), 104, 54, 1.4) + T(lock('w'), 104, 56, .8), 'i')


def notifications():
    dots = C(118, 34, 7, 'd') + C(118, 34, 2, 'u', None)
    card(G, 'notifications', T(bell('w'), 78, 56, 1.4) + dots + L('M44 40 q-6 14 0 28 M116 62 q6 10 0 20', 'm', 2.5), 'w')


def shortcuts():
    keys = ''.join(key_cap(38 + col * 22, 40 + row * 20, 18, 'a' if (row, col) == (1, 2) else 'u')
                   for row in range(2) for col in range(4))
    card(G, 'shortcuts', R(32, 32, 96, 52, 8, 'e') + keys + key_cap(60, 80, 40), 'a')


def advanced():
    window = R(36, 28, 76, 56, 7, 'u') + R(36, 28, 76, 12, 7, 'm', 'f', 3) + L('M48 56 L58 62 L48 68 M64 70 h16', 'f', 3)
    card(G, 'advanced', window + T(gear('b', 12), 118, 78), 'b')


def build():
    for draw in (general, appearance, reading_writing, ai_models, embeddings, images, agents, learning, importing,
                 privacy_data, notifications, shortcuts, advanced):
        draw()
