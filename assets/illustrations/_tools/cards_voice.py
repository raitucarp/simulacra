"""Linguistic voice cards: code switching, grammar, diction, capitalization, punctuation, emoji, sentence length."""
from kit import *
from cards_mind import qmark


def globe(f='i'):
    return C(0, 0, 12, f) + L('M-12 0 h24 M0 -12 q-8 12 0 24 q8 -12 0 -24', 'u', 2)


def smiley(x, y, r=7, f='w', kind='smile'):
    out = C(x, y, r, f, 'f', 2)
    if kind == 'heart':
        out += T(heart('d', None), x - r * .38, y - r * .2, r / 50) + T(heart('d', None), x + r * .38, y - r * .2, r / 50)
    else:
        out += C(x - r * .35, y - r * .2, r * .14, 'f', None) + C(x + r * .35, y - r * .2, r * .14, 'f', None)
    return out + L(f'M{n(x - r * .45)} {n(y + r * .25)} Q{n(x)} {n(y + r * .7)} {n(x + r * .45)} {n(y + r * .25)}', 'f', 2)


def code_switching():
    g = 'code_switching'
    b = bubble(26, 30, 82, 50, 'u') + lines(36, 44, (40, 0, 30), 11) + R(36, 51, 18, 8, 4, 'w', None) + L('M58 55 h38', 'm', 3)
    card(g, 'natural_loanwords', b + T(globe('i'), 124, 32) + L('M114 42 Q100 52 58 52', 'w', 2, '.7'), 'w')
    b = bubble(32, 26, 96, 56, 'i') + lines(46, 42, (68, 56, 40), 11, 'u')
    card(g, 'monolingual_pure', b, 'i')
    b = bubble(20, 22, 70, 44, 'a') + lines(30, 36, (46, 34), 10, 'u') + bubble(70, 50, 70, 40, 'w', 'r') + lines(80, 62, (46, 30), 10, 'f')
    b += L('M100 24 q14 0 16 14 M60 98 q-14 0 -16 -14', 'm', 2.5) + P('M112 36 l4 4 l4 -5', None, 'm', 2.5)
    card(g, 'bilingual_fluid', b, 'a')
    tree = L('M80 76 Q70 92 54 98 M80 76 Q80 92 76 100 M80 76 Q92 92 108 98', 'w', 3) + L('M30 80 h100', 'm', 2)
    tree += R(74, 52, 12, 28, 3, 'w') + C(80, 38, 24, 's') + C(62, 48, 12, 's') + C(98, 48, 12, 's') + C(72, 34, 4, 'u', None, o='.5')
    card(g, 'classical_etymology', tree, 's')


def grammar():
    g = 'grammar_style'
    doc = R(42, 18, 58, 80, 5, 'u') + lines(52, 34, (38, 38, 38, 38, 24), 10)
    nib = T(P('M0 -26 Q10 -10 6 8 L-6 8 Q-10 -10 0 -26 Z', 'a') + L('M0 -14 v12', 'u', 2) + R(-7, 8, 14, 18, 3, 'f', None), 116, 72, 1, 30)
    card(g, 'standard_formal', doc + nib, 'a')
    b = bubble(22, 20, 62, 34, 's') + lines(32, 33, (36, 22), 9, 'u') + bubble(66, 50, 50, 28, 'u', 'r') + lines(76, 60, (28, 0), 9)
    mug = R(112, 74, 22, 22, 5, 'w') + L('M134 79 q7 0 7 6 q0 6 -7 6', 'f', 2.5) + L('M118 66 q-3 -4 0 -8 M126 66 q-3 -4 0 -8', 'm', 2)
    card(g, 'relaxed_conversational', b + mug, 's')
    notes = ''
    for x, y, r, c, ls in ((54, 40, -12, 'w', (16, 10)), (102, 36, 10, 'i', (18,)), (62, 80, 8, 'i', (12, 18)), (108, 80, -6, 'w', (10,))):
        notes += T(R(-16, -14, 32, 28, 3, c) + lines(-10, -5, ls, 8, 'f', 2), x, y, 1, r)
    card(g, 'fragmented_stream', notes + L('M80 56 l3 3 M84 52 l2 -2', 'm', 2), 'w')
    horn = 'M66 88 Q40 78 44 52 Q46 36 34 22 Q48 24 54 40 Q58 56 56 66 Q60 78 72 82 Z'
    lyre = P(horn, 'w') + MX(P(horn, 'w'))
    lyre += R(42, 30, 76, 7, 3, 'm', 'f', 2) + L('M70 37 v46 M80 37 v46 M90 37 v46', 'f', 2) + E(80, 88, 20, 7, 'w')
    card(g, 'poetic_classical', lyre + T(leaf('s'), 34, 92, .7, 20) + T(leaf('s'), 126, 92, .7, 70), 'w')
    ph = R(56, 16, 48, 86, 9, 'u') + R(72, 22, 16, 4, 2, 'm', None) + L('M66 52 h18', 'm', 3) + C(90, 52, 2, 'i', None)
    card(g, 'minimalist_lowercase', ph + C(80, 92, 3.5, 'm', None), 'i')
    jag = poly(star_pts(76, 58, 46, 34, 12, -90))
    sq = L('M54 52 q6 -8 12 0 t12 0 t12 0 t12 0 M60 66 q6 -8 12 0 t12 0 t12 0', 'u', 3)
    card(g, 'colloquial_expressive', P(jag, 'd') + sq + L('M126 30 l8 -6 M130 46 h10 M124 16 l2 -8', 'w', 3), 'd')


def diction():
    g = 'diction_register'
    bk = R(40, 72, 80, 12, 3, 'i') + R(46, 84, 72, 12, 3, 'b') + R(44, 60, 74, 12, 3, 'u')
    cap = P('M80 22 L118 34 L80 46 L42 34 Z', 'f', 'f', 2) + P('M62 40 v10 q18 8 36 0 v-10 L80 46 Z', 'f', None) + L('M118 34 v14', 'w', 2.5) + C(118, 50, 3, 'w', None)
    card(g, 'academic_philosophical', bk + cap, 'i')
    bag = P('M44 52 h72 l-6 46 h-60 Z', 'w') + L('M62 52 q0 -12 8 -12 M98 52 q0 -12 -8 -12', 'f', 3)
    bag = T(R(-6, -26, 12, 34, 6, 'u'), 70, 52, 1, -15) + T(leaf('s'), 98, 40, 1.1, -40) + bag
    card(g, 'everyday_natural', bag + L('M56 64 h48', 'f', 2, '.4'), 's')
    bk = P('M30 76 L80 84 L130 76 L130 98 L80 106 L30 98 Z', 'u') + L('M80 84 v22', 'f', 2)
    vine = L('M80 82 Q74 60 86 46 Q96 34 88 20', 's', 3) + T(leaf('s'), 70, 60, .8, 200) + T(leaf('s'), 98, 40, .8, -20)
    vine += C(88, 18, 6, 'd') + C(64, 40, 5, 'a') + C(108, 62, 5, 'd')
    card(g, 'literary_poetic', vine + bk, 'a')
    chip = ''.join(L(f'M{x} 26 v10 M{x} 84 v10', 'f', 3) for x in (64, 80, 96)) + ''.join(L(f'M46 {y} h10 M104 {y} h10', 'f', 3) for y in (44, 60, 76))
    chip += R(54, 34, 52, 52, 6, 'b') + R(68, 48, 24, 24, 3, 'u', 'f', 2) + C(80, 60, 4, 'b', None)
    card(g, 'technical_precision', chip, 'b')
    bust = P('M40 98 Q40 74 62 72 L82 72 Q104 74 104 98 Z', 'u') + C(72, 48, 18, 'u') + P('M56 54 Q60 78 72 80 Q84 78 88 54 Q78 62 72 60 Q66 62 56 54 Z', 'm')
    bust += C(66, 46, 2, 'f', None) + C(78, 46, 2, 'f', None)
    card(g, 'socratic_ironic', bust + bubble(98, 18, 40, 32, 'a', 'l') + qmark(118, 32, .9, 'u'), 'a')
    env = R(36, 34, 88, 58, 6, 'u') + L('M36 40 L80 70 L124 40', 'f', 3) + C(80, 68, 13, 'd') + T(heart('u', None), 80, 69, .5)
    card(g, 'warm_empathetic', env + T(heart('d'), 128, 28, .45) + T(heart('d'), 34, 26, .35), 'd')
    can = R(44, 46, 30, 54, 6, 'i') + R(52, 34, 14, 12, 3, 'm') + R(56, 28, 6, 6, 2, 'f', None) + L('M44 62 h30 M44 86 h30', 'u', 2)
    spray = L('M66 30 l10 -4 M68 26 l10 -10 M64 22 l4 -10', 'w', 2) + L('M86 82 Q94 50 108 74 T128 46 L136 62', 'a', 4)
    card(g, 'slang_colloquial', spray + can + T(star('w', 6, 2.5), 132, 30), 'i')


def capitalization():
    g = 'capitalization'
    base = R(26, 34, 108, 56, 8, 'u') + L('M36 78 h88', 'm', 2, '.6')
    card(g, 'standard_sentence', base + R(38, 46, 12, 32, 4, 'a', None) + ''.join(R(x, 62, 10, 16, 4, 'm', None) for x in (54, 68, 82, 96)) + C(114, 74, 4, 'a', None), 'a')
    card(g, 'all_lowercase', base + ''.join(R(x, 62, 10, 16, 5, 's', None) for x in (38, 52, 66, 84, 98, 112)), 's')
    loud = base + ''.join(R(x, 44, 13, 34, 3, 'd', None) for x in (38, 55, 72, 89)) + R(108, 44, 8, 22, 3, 'd', None) + C(112, 73, 4, 'd', None)
    card(g, 'expressive_emphasis', loud + L('M24 26 l-6 -6 M80 26 v-10 M136 26 l6 -6', 'w', 3), 'd')


def punctuation():
    g = 'punctuation_habit'
    em = R(24, 54, 62, 10, 5, 'a') + C(112, 40, 8, 'a') + P('M104 70 a8 8 0 1 1 15 4 Q116 90 100 96 Q108 86 104 76 Z', 'a')
    card(g, 'em_dash_and_semicolon', em, 'a')
    cm = lines(30, 58, (52,), 0, 'm', 4) + C(96, 60, 9, 'f', None) + P('M118 54 a7 7 0 1 1 12 4 Q128 72 114 78 Q122 70 118 62 Z', 'f', None)
    card(g, 'crisp_minimal', cm + T(sparkle('w', 8), 104, 34), 'w')
    el = C(48, 64, 9, 'i') + C(76, 64, 9, 'i', o='.7') + C(104, 64, 9, 'i', o='.4')
    card(g, 'ellipsis_contemplative', el + C(116, 38, 4, 'u', 'f', 2) + T(cloud('u'), 130, 22, .55), 'i')
    card(g, 'question_driven', qmark(76, 56, 3.1, 'w', 3.5) + qmark(124, 40, 1.3, 'b') + qmark(36, 34, 1, 'b'), 'w')
    zp = lines(28, 50, (60, 42), 18, 'm', 4) + C(112, 70, 16, 'u') + C(112, 70, 7, 'f', None) + L('M98 84 L126 56', 'd', 3)
    card(g, 'zero_trailing_period', zp, 'd')


def emoji():
    g = 'emoji_usage'
    def bub(c='u'):
        return bubble(24, 24, 92, 56, c)
    nv = bub() + lines(36, 40, (68, 60, 40), 11) + C(124, 76, 14, 'u') + T(smiley(0, 0, 8, 'm'), 124, 76) + L('M114 66 L134 86', 'd', 3) + C(124, 76, 14, None, 'd', 3)
    card(g, 'never', nv, 'd')
    card(g, 'rare_subtle', bub() + lines(36, 40, (68, 60, 30), 11) + smiley(80, 62, 6, 'w'), 'w')
    md = bub() + lines(36, 40, (44, 30), 11) + smiley(96, 42, 9, 'w') + smiley(60, 64, 9, 'w', 'heart') + smiley(132, 30, 8, 'w')
    card(g, 'moderate_expressive', md, 'w')
    ko = bub('b') + C(44, 52, 4, 'f', None) + R(56, 49, 26, 6, 3, 'f', None) + L('M92 46 h14 v12', 'f', 4)
    card(g, 'kaomoji_text_only', ko, 'b')


def sentence_length():
    g = 'sentence_length'
    bal = bubble(34, 14, 92, 52, 'u') + R(46, 27, 50, 6, 3, 'i', None) + lines(46, 42, (66, 56), 10, 'm')
    bal += L('M44 88 h72', 'f', 3) + P('M72 98 L80 88 L88 98 Z', 'm', 'f', 2) + C(52, 80, 7, 'i') + C(108, 80, 7, 'i')
    card(g, 'balanced_cadence', bal, 'i')
    tab = P('M50 98 L50 44 Q50 22 80 22 Q110 22 110 44 L110 98 Z', 'm') + R(66, 56, 28, 7, 3, 'w', None) + L('M58 98 v-46 Q58 30 80 30', 'u', 2, '.4')
    card(g, 'ultra_concise_aphorism', tab + T(sparkle('w', 8), 124, 34) + R(40, 96, 80, 6, 3, 'f', None), 'w')
    gl = P('M44 44 Q44 24 66 24 L82 24 Q104 24 104 48 Q104 70 84 72 L58 72 Q44 72 44 58 Z', 'd') + R(46, 72, 34, 18, 4, 'u') + L('M64 44 q-10 0 -10 10', 'f', 2.5)
    card(g, 'short_punchy', gl + L('M114 32 l10 -6 M118 50 h12 M114 66 l10 6', 'w', 3) + R(110, 86, 14, 6, 3, 'm', None), 'd')
    sc = R(22, 30, 116, 58, 4, 'u') + R(16, 24, 12, 70, 6, 's') + R(132, 24, 12, 70, 6, 's') + lines(36, 42, (90, 88, 92, 70), 10, 'm', 3)
    card(g, 'long_periodic', sc, 's')


def build():
    code_switching()
    grammar()
    diction()
    capitalization()
    punctuation()
    emoji()
    sentence_length()
