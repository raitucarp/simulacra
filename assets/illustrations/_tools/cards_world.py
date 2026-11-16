"""Relational, social and prompt cards."""
from kit import *
from cards_mind import qmark


def owl(f='a', eyes='open'):
    """Mini Mnemo, guide-pose coordinates (128 canvas, centre 64)."""
    out = P('M30 62 Q30 30 64 28 Q98 30 98 62 L98 88 Q98 112 64 114 Q30 112 30 88 Z', f)
    out += P('M38 34 L32 18 L50 30 Z M90 34 L96 18 L78 30 Z', f, None) + P('M44 84 Q64 74 84 84 Q84 106 64 108 Q44 106 44 84 Z', 'e', None)
    out += C(50, 56, 15, 'u') + C(78, 56, 15, 'u')
    out += (C(50, 56, 5, 'f', None) + C(78, 56, 5, 'f', None)) if eyes == 'open' else L('M43 58 Q50 64 57 58 M71 58 Q78 64 85 58', 'f', 3)
    return out + P('M59 68 L64 76 L69 68 Z', 'w', 'f', 2)


def prompt_cards():
    g = 'assembly_strategy'
    st = T(R(-30, -24, 60, 50, 4, 'u'), 84, 60, 1, 8) + T(R(-30, -24, 60, 50, 4, 'u'), 80, 58, 1, -4)
    st += P('M40 40 h22 l6 6 h44 v52 h-72 Z', 'w') + R(48, 52, 22, 26, 3, 'u', 'f', 2) + C(59, 62, 5, 'm', None) + P('M51 78 q8 -12 16 0 Z', 'm', None)
    st += lines(78, 58, (26, 26, 18), 8, 'f', 2.5) + L('M100 34 v16 q0 5 -4 5 q-4 0 -4 -5 v-12', 'f', 2.5)
    card(g, 'full_human_synthesis', st, 'w')
    hd = P('M48 100 L48 82 Q32 72 32 52 Q32 22 64 22 Q94 22 98 46 L106 62 L98 64 Q100 80 84 82 L82 100 Z', 'a')
    hd += T(cloud('u', 'f', 2), 64, 50, .75) + L('M58 44 q4 4 8 0 M62 52 q4 -4 8 0', 'a', 2)
    hd += L('M114 52 q6 8 0 16 M122 46 q10 14 0 28', 'b', 3)
    card(g, 'psychology_and_voice_focused', hd, 'a')
    pg = R(40, 18, 70, 84, 5, 'u') + lines(50, 32, (44, 30), 9, 'm')
    pg += '<rect x="50" y="52" width="50" height="14" rx="4" stroke="{{accent}}" stroke-width="2.5" stroke-dasharray="5 5"/>'
    pg += '<rect x="50" y="74" width="34" height="14" rx="4" stroke="{{accent}}" stroke-width="2.5" stroke-dasharray="5 5"/>'
    pg += L('M124 36 q-6 0 -6 6 v8 q0 4 -4 6 q4 2 4 6 v8 q0 6 6 6', 'w', 3) + T(sparkle('w', 7), 128, 90)
    card(g, 'custom_template', pg, 'a')

    g = 'pre_post_reasoning'
    glow = C(0, -26, 9, 'd', None, o='.3') + T(heart('d', 'f', 2), 0, -26, .45)
    p, _ = person(72, 100, 's', (20, 20), front=glow)
    card(g, 'embodied_human_perspective', p + C(98, 34, 3, 'u', 'f', 2) + C(106, 26, 4.5, 'u', 'f', 2) + T(sparkle('w', 7), 120, 20), 's')
    nodes = ((40, 36), (80, 24), (120, 40), (48, 84), (112, 86), (80, 60))
    web = ''.join(L(f'M{a[0]} {a[1]} L{b[0]} {b[1]}', 'm', 2) for i, a in enumerate(nodes) for b in nodes[i + 1:] if (a[0] - b[0]) ** 2 + (a[1] - b[1]) ** 2 < 2700)
    for x, y in nodes[:5]:
        web += R(x - 9, y - 11, 18, 22, 3, 'u', 'f', 2) + L(f'M{x - 4} {y - 3} h8 M{x - 4} {y + 3} h6', 'i', 2)
    card(g, 'dialectical_network_weaver', web + C(80, 60, 10, 'a') + L('M76 60 h8', 'u', 2.5), 'a')
    pl = P('M30 70 L130 30 L96 96 L82 74 Z', 'u') + P('M82 74 L130 30 L74 90 Z', 'm', 'f', 2.5)
    trail = '<path d="M28 98 Q14 86 30 78" stroke="{{fg_muted}}" stroke-width="2.5" stroke-dasharray="4 6"/>'
    card(g, 'direct_spontaneous', trail + pl + T(sparkle('w', 7), 128, 76), 'i')


def hands_clasp():
    out = R(14, 50, 46, 22, 6, 'a') + R(100, 52, 46, 22, 6, 'b')
    out += R(76, 46, 34, 26, 12, 'u') + R(52, 50, 36, 24, 12, 'u') + L('M66 58 h18 M64 66 h20', 'f', 2) + E(80, 46, 6, 9, 'u', 'f', 2.5)
    return out


def attachment():
    g = 'attachment_style'
    card(g, 'secure', hands_clasp() + T(heart('d'), 80, 26, .6), 'a')
    ph = R(52, 16, 56, 88, 10, 'u') + bubble(60, 30, 40, 22, 'i') + C(70, 41, 2.5, 'u', None) + C(80, 41, 2.5, 'u', None) + C(90, 41, 2.5, 'u', None)
    ph += T(heart('d'), 80, 74, .7) + P('M122 30 Q128 38 122 42 Q116 38 122 30 Z', 'i', 'f', 2) + L('M120 66 l10 -4 M122 80 l10 2', 'm', 2.5)
    card(g, 'anxious_preoccupied', ph, 'd')
    isl = E(80, 92, 40, 10, 'w') + L('M80 88 Q78 60 88 42', 'f', 3)
    isl += P('M88 42 Q70 30 58 44 Q72 38 88 42 Q86 24 72 18 Q90 22 88 42 Q104 26 118 32 Q100 34 88 42 Q108 42 112 58 Q98 46 88 42 Z', 's')
    isl += L('M20 100 q8 -5 16 0 M124 100 q8 -5 16 0', 'i', 2.5)
    card(g, 'dismissive_avoidant', isl, 'i')
    vine = L('M44 30 Q120 40 104 70 Q90 96 50 80 Q30 70 58 52 Q92 32 116 90', 's', 3)
    thorns = ''.join(L(f'M{x} {y} l{dx} {dy}', 's', 2.5) for x, y, dx, dy in ((70, 34, 2, -6), (110, 50, 6, -2), (72, 89, -2, 6), (40, 66, -6, 0), (100, 52, 4, 4)))
    card(g, 'fearful_avoidant', T(heart('d'), 80, 62, 1.9) + vine + thorns, 'd')


def moral():
    g = 'moral_foundation'
    eye = P('M30 60 Q80 18 130 60 Q80 102 30 60 Z', 'u') + C(80, 60, 16, 'b') + C(80, 60, 7, 'f', None) + C(84, 56, 2.5, 'u', None)
    eye += L('M80 22 v-8 M52 28 l-5 -6 M108 28 l5 -6', 'w', 3)
    card(g, 'epistemic_truth', eye, 'b')
    bd = T(R(-30, -9, 60, 18, 7, 'w') + R(-9, -9, 18, 18, 0, 'u', 'f', 2) + C(-3, -3, 1.4, 'f', None) + C(3, 3, 1.4, 'f', None), 80, 60, 1, -35)
    card(g, 'care_harm', T(heart('d'), 80, 62, 2.4) + bd, 'd')
    sc = L('M80 24 v66 M40 36 h80', 'f', 3) + R(62, 88, 36, 8, 3, 'm') + C(80, 22, 4, 'w')
    sc += L('M40 36 L28 64 M40 36 L52 64 M120 36 L108 64 M120 36 L132 64', 'f', 2)
    sc += P('M24 64 h32 q-4 12 -16 12 q-12 0 -16 -12 Z', 'w') + P('M104 64 h32 q-4 12 -16 12 q-12 0 -16 -12 Z', 'w')
    card(g, 'fairness_cheating', sc, 'w')
    ch = T(R(-14, -8, 28, 16, 8, None, 'm', 5), 44, 78, 1, -20) + T(P('M-14 -8 h20 M-14 8 h20 M6 -8 a8 8 0 0 1 0 16', None, 'm', 5), 72, 70, 1, -20)
    bird = E(112, 54, 16, 8, 'i') + C(126, 48, 6, 'i') + P('M131 47 l7 2 l-7 3 Z', 'w', 'f', 2) + C(127, 46, 1.4, 'f', None)
    bird += P('M104 50 Q100 24 118 16 Q118 36 116 50 Z', 'i') + P('M96 54 l-10 -6 l2 10 Z', 'i', 'f', 2)
    card(g, 'liberty_oppression', ch + bird + L('M84 62 l6 -4 M86 74 l8 0', 'w', 2.5), 'i')
    dog = P('M50 30 Q36 30 38 56 Q40 70 50 66 Z M110 30 Q124 30 122 56 Q120 70 110 66 Z', 'm')
    dog += P('M52 40 Q52 24 80 24 Q108 24 108 40 L108 66 Q108 92 80 92 Q52 92 52 66 Z', 'w') + E(80, 76, 14, 11, 'u', 'f', 2.5)
    dog += E(80, 70, 6, 4, 'f', None) + C(68, 52, 3.5, 'f', None) + C(92, 52, 3.5, 'f', None) + L('M80 74 v4 M74 80 q6 4 12 0', 'f', 2)
    card(g, 'loyalty_betrayal', dog + T(heart('d'), 126, 86, .45), 'w')
    tp = P('M30 46 L80 18 L130 46 Z', 'i') + R(32, 46, 96, 8, 2, 'u') + ''.join(R(x, 56, 10, 30, 2, 'u', 'f', 2.5) for x in (40, 62, 88, 110))
    tp += R(28, 86, 104, 6, 2, 'm') + R(22, 92, 116, 6, 2, 'm') + C(80, 36, 4, 'w', None)
    card(g, 'authority_subversion', tp, 'i')
    halo = E(80, 30, 26, 6, None, 'w', 3) + C(80, 66, 34, 'w', None, o='.15')
    lotus = P('M80 90 Q62 70 80 44 Q98 70 80 90 Z', 's') + P('M80 90 Q48 86 44 62 Q66 64 80 90 Z M80 90 Q112 86 116 62 Q94 64 80 90 Z', 's')
    lotus += P('M80 90 Q56 96 36 84 M80 90 Q104 96 124 84', None, 'f', 2) + L('M50 98 h60', 'i', 2.5)
    card(g, 'sanctity_degradation', halo + lotus, 's')


def temperament():
    g = 'temperament'
    mel = T(cloud('m'), 78, 42, 1.4) + ''.join(L(f'M{x} {y} l-3 8', 'i', 2.5) for x, y in ((60, 62), (74, 68), (88, 62), (102, 68)))
    card(g, 'melancholic', mel + T(leaf('w'), 112, 90, .8, 30), 'i')
    card(g, 'choleric', T(flame('d', 'w'), 80, 60, 2.6) + T(sparkle('w', 6), 120, 28) + T(sparkle('d', 5), 40, 40), 'd')
    sun_ = T(sun('w', 22, 10, 8), 80, 56) + C(72, 52, 2.5, 'f', None) + C(88, 52, 2.5, 'f', None) + L('M72 62 q8 6 16 0', 'f', 2.5)
    card(g, 'sanguine', sun_ + L('M20 86 q10 -6 20 0 M118 92 q10 -6 20 0', 'b', 2.5), 'w')
    ph = E(80, 92, 44, 8, None, 'b', 2.5) + E(80, 92, 28, 5, None, 'b', 2.5) + P('M80 26 Q104 56 96 72 Q90 84 80 84 Q70 84 64 72 Q56 56 80 26 Z', 'b')
    card(g, 'phlegmatic', ph + L('M72 60 q-2 8 4 12', 'u', 2.5), 'b')
    mc = P('M34 96 L60 60 L80 70 L100 56 L128 96 Z', 'm') + T(flame('d', 'w'), 80, 40, 1.3) + T(star('w', 6, 2.5), 122, 24)
    card(g, 'melancholic_choleric', mc, 'd')
    ps = E(80, 88, 50, 12, 'b', None, o='.4') + E(72, 84, 26, 9, 's') + L('M72 84 L86 78', 'f', 2) + T(sun('w', 9, 8, 4), 128, 28)
    bf = T(P('M0 0 Q-12 -14 -14 -2 Q-12 8 0 0 Z M0 0 Q12 -14 14 -2 Q12 8 0 0 Z', 'w', 'f', 2) + L('M0 -7 L0 5', 'f', 2), 82, 48, 1.1, -10)
    card(g, 'phlegmatic_sanguine', ps + bf, 's')


def locus():
    g = 'locus_of_control'
    wh = ''.join(T(R(-3, -40, 6, 80, 3, 'w', 'f', 2) + C(0, -40, 5, 'w', 'f', 2) + C(0, 40, 5, 'w', 'f', 2), 80, 60, 1, a) for a in (0, 45, 90, 135))
    wh += C(80, 60, 26, None, 'f', 9) + C(80, 60, 26, None, 'w', 5) + C(80, 60, 8, 'w')
    card(g, 'strong_internal', wh, 'w')
    mp = P('M28 34 L56 26 L84 34 L112 26 L112 92 L84 100 L56 92 L28 100 Z', 'u') + L('M56 26 v66 M84 34 v66', 'm', 2)
    mp += '<path d="M38 80 Q56 60 70 74 T100 50" stroke="{{danger}}" stroke-width="2.5" stroke-dasharray="4 5"/>'
    cp = C(118, 76, 20, 's') + C(118, 76, 14, 'u', 'f', 2) + P('M118 64 L122 76 L118 88 L114 76 Z', 'd', None) + P('M118 64 L122 76 L114 76 Z', 'f', None)
    card(g, 'moderate_internal', mp + cp, 's')
    kite = P('M110 14 L132 34 L110 64 L90 34 Z', 'a') + L('M110 14 V64 M90 34 H132', 'u', 2)
    kite += L('M110 64 Q100 80 108 92 M110 64 Q96 86 62 78', 'f', 2) + C(104, 76, 3, 'w', None) + C(106, 86, 3, 'w', None)
    p, _ = person(52, 100, 'b', (15, 110), sc=.7)
    card(g, 'balanced_dialectical', kite + p + L('M20 30 q10 -6 20 0 M30 44 q10 -6 20 0', 'm', 2.5), 'a')
    orb = E(80, 60, 56, 22, None, 'm', 2) + E(80, 60, 34, 13, None, 'm', 2) + T(sun('w', 10, 8, 4), 80, 60)
    orb += C(126, 72, 7, 'i') + C(52, 50, 5, 'a') + T(star('w', 4, 1.8), 36, 24) + T(star('w', 4, 1.8), 130, 26) + C(104, 22, 2, 'f', None)
    card(g, 'external_cosmological', orb, 'a')


def cadence():
    g = 'posting_cadence'
    bell = P('M58 80 Q60 66 60 52 Q60 32 80 30 Q100 32 100 52 Q100 66 102 80 Z', 'w') + R(52, 78, 56, 8, 4, 'w') + C(80, 92, 5, 'f', None) + C(80, 27, 3, 'w', 'f', 2)
    bell += C(106, 34, 9, 'd') + L('M20 60 h14 l5 -10 l6 20 l5 -10 h4 M116 60 h4 l5 -10 l6 20 l5 -10 h4', 'm', 2.5)
    card(g, 'on_every_new_post', bell, 'w')
    card(g, 'every15_minutes', T(clock('u', 'f', 'i', .25, 30), 80, 60), 'i')
    card(g, 'every30_minutes', T(clock('u', 'f', 'a', .5, 30), 80, 60), 'a')
    hg = R(54, 18, 52, 8, 3, 'm') + R(54, 94, 52, 8, 3, 'm') + P('M60 26 h40 Q100 48 82 60 Q100 72 100 94 h-40 Q60 72 78 60 Q60 48 60 26 Z', 'u')
    hg += P('M68 38 h24 Q88 50 80 56 Q72 50 68 38 Z M66 92 Q72 76 80 74 Q88 76 94 92 Z', 'w', None) + L('M80 60 v12', 'w', 2)
    card(g, 'hourly', hg, 'w')
    me = T(sun('w', 14, 8, 5), 54, 52) + T(moon('a'), 108, 64, 1.5) + L('M120 24 L40 100', 'm', 2, '.5') + T(star('w', 4, 1.8), 128, 92)
    card(g, 'morning_and_evening', me, 'w')
    branch = L('M24 100 Q80 86 140 98', 'f', 3) + T(leaf('s'), 128, 88, .7, 20)
    card(g, 'night_owl_hours', T(moon('w'), 126, 28, 1.2) + branch + T(owl('a'), 50, 12, .62) + T(star('w', 4, 1.8), 30, 30), 'a')
    btn = E(80, 90, 32, 8, 'm') + R(56, 76, 48, 14, 6, 'd')
    hand = R(74, 26, 12, 40, 6, 'u') + R(70, 46, 28, 30, 10, 'u') + L('M78 56 h16 M78 64 h16', 'f', 2)
    card(g, 'manual_only', btn + hand + L('M50 66 l-8 -4 M110 66 l8 -4', 'w', 3), 'd')


def chronotype():
    g = 'chronotype'
    sky = R(20, 20, 120, 64, 12, 'G', None) + P('M60 84 a20 20 0 0 1 40 0 Z', 'w') + L('M18 84 h124', 'f', 3) + L('M30 52 h30 M100 62 h28 M86 44 h20', 'u', 2.5, '.5')
    p, _ = person(40, 100, 'm', (None, None), 'none', .45)
    card(g, 'twilight_contemplative', sky + p + T(star('u', 5, 2.2), 118, 34), 'a', grad='a')
    rays = ''.join(L(f'M{n(80 + 24 * math.cos(a))} {n(84 + 24 * math.sin(a))} L{n(80 + 32 * math.cos(a))} {n(84 + 32 * math.sin(a))}', 'w', 3)
                   for a in (math.radians(d) for d in range(195, 360, 30)))
    sr = rays + P('M60 84 a20 20 0 0 1 40 0 Z', 'w') + L('M18 84 h124', 'f', 3) + L('M40 94 h24 M96 94 h30', 'w', 2.5, '.6')
    bird = P('M-14 0 Q-12 -12 2 -10 Q12 -8 14 0 Q4 8 -8 6 Z', 'i') + P('M-4 -6 Q4 -18 10 -4 Z', 'u', 'f', 2) + P('M14 -4 l7 2 l-7 2 Z', 'w', 'f', 2) + C(7, -5, 1.6, 'f', None) + P('M-14 0 l-8 -6 l2 8 Z', 'i', 'f', 2)
    card(g, 'early_bird_lark', sr + L('M84 62 Q112 58 142 52', 'f', 3) + T(bird, 112, 48, 1.2), 'w')
    dd = P('M14 96 Q60 64 110 80 Q130 86 146 80 L146 102 L14 102 Z', 's') + T(sun('w', 14, 10, 5), 80, 34) + T(cloud('u'), 124, 46, .7) + T(cloud('u'), 36, 52, .6)
    card(g, 'daytime_diurnal', dd, 'w')
    lamp = L('M46 96 L58 60 L78 46', 'f', 3) + P('M74 36 L98 52 L84 62 Z', 'b') + P('M90 60 L120 98 L74 98 L84 64 Z', 'w', None, o='.25') + R(36, 94, 24, 6, 3, 'm')
    mug = R(106, 80, 18, 18, 4, 'u') + L('M124 84 q6 0 6 5 q0 5 -6 5', 'f', 2.5) + L('M112 74 q-3 -4 0 -8', 'm', 2)
    card(g, 'night_owl_insomniac', T(moon('w'), 34, 30, 1.1) + lamp + mug + T(star('w', 4, 1.8), 128, 24), 'b')


def mention():
    g = 'mention_attitude'
    p, _ = person(56, 100, 'a', (15, 150))
    card(g, 'socratic_questioning', p + bubble(88, 18, 46, 40, 'u') + qmark(111, 36, 1.1, 'a'), 'a')
    p, _ = person(80, 100, 's', (115, 115))
    card(g, 'warm_and_receptive', p + T(heart('d'), 40, 30, .5) + T(heart('d'), 124, 26, .6), 's')
    kn = P('M56 96 L60 78 Q48 66 54 50 Q60 30 80 24 L84 16 L90 26 Q112 34 108 60 L104 78 L108 96 Z', 'b')
    kn += P('M80 24 Q66 46 70 56 Q58 62 52 58', None, 'f', 2) + C(86, 40, 2.5, 'f', None) + R(48, 94, 64, 8, 3, 'm')
    card(g, 'analytical_debater', kn + L('M120 40 h14 M127 33 v14', 'w', 3), 'b')
    pw = ''.join(T(P('M0 0 L0 -28 Q20 -24 0 0 Z', c), 80, 50, 1, a) for a, c in ((0, 'd'), (90, 'w'), (180, 'd'), (270, 'w')))
    pw += C(80, 50, 3.5, 'f', None) + L('M80 54 V100', 'f', 3) + L('M40 30 q-8 10 0 20 M120 70 q8 10 0 20', 'm', 2.5)
    card(g, 'playful_witty', pw, 'w')
    cat = P('M56 96 Q48 64 66 50 L62 32 L74 42 Q80 40 86 42 L98 32 L94 50 Q112 64 104 96 Z', 'm')
    cat += L('M104 92 Q130 92 128 70 Q126 58 118 62', 'm', 6) + L('M104 92 Q130 92 128 70 Q126 58 118 62', 'f', 2) + L('M70 60 q10 4 20 0', 'f', 2, '.5')
    card(g, 'aloof_selective', cat + L('M30 30 h14 M36 40 h12', 'm', 2.5), 'i')
    ear = P('M84 96 Q66 96 68 80 Q70 70 62 62 Q52 50 58 36 Q66 20 86 22 Q106 26 106 48 Q106 64 94 72 Q86 80 84 96 Z', 'w')
    ear += L('M72 44 Q76 32 88 34 Q96 38 94 50 Q92 58 84 58', 'f', 2.5)
    ear += L('M30 50 q-6 10 0 20 M40 54 q-4 6 0 12', 'i', 2.5) + T(heart('d'), 124, 34, .55)
    card(g, 'empathetic_listener', ear, 'w')
    dv = bubble(36, 34, 80, 54, 'd') + P('M48 36 Q44 18 54 14 Q52 26 60 34 Z M104 36 Q108 18 98 14 Q100 26 92 34 Z', 'd')
    dv += L('M116 70 Q136 70 132 90', 'f', 3) + P('M128 88 L136 86 L134 96 Z', 'd', 'f', 2) + C(64, 58, 3, 'u', None) + C(88, 58, 3, 'u', None) + L('M66 70 Q76 76 86 70', 'u', 2.5)
    card(g, 'devil_advocate', dv, 'd')


def conflict():
    g = 'conflict_handling'
    yy = C(80, 58, 34, 'u') + P('M80 24 A34 34 0 0 1 80 92 A17 17 0 0 1 80 58 A17 17 0 0 0 80 24 Z', 'a', None)
    yy += C(80, 41, 5, 'a', None) + C(80, 75, 5, 'u', None) + C(80, 58, 34, None, 'f', 3)
    card(g, 'dialectical_synthesis', yy, 'a')
    cn = E(80, 92, 30, 9, 'm') + E(80, 74, 24, 9, 'b') + E(80, 57, 18, 8, 'm') + E(80, 42, 12, 7, 'b') + E(80, 30, 7, 5, 'm')
    card(g, 'calm_rational_refutation', cn + L('M30 50 q8 -6 16 0 M114 60 q8 -6 16 0', 'm', 2), 'b')
    dove = P('M38 66 Q52 44 80 50 Q96 34 118 42 L126 46 L118 50 Q112 76 84 80 Q60 82 38 66 Z', 'u')
    dove += P('M62 56 Q70 26 100 22 Q92 46 76 60 Z', 'u') + C(114, 46, 1.8, 'f', None)
    dove += L('M126 48 Q134 58 128 68', 's', 2.5) + T(leaf('s'), 132, 62, .5, 70) + T(leaf('s'), 126, 72, .5, 120)
    card(g, 'deescalate_harmonize', dove, 's')
    sw = C(80, 58, 30, 'w') + L('M62 42 L74 38 M88 36 Q96 30 102 38', 'f', 3) + C(74, 52, 3.5, 'f', None) + C(98, 52, 3.5, 'f', None)
    sw += L('M66 70 Q80 74 96 64', 'f', 3) + L('M68 52 h10 M92 52 h10', 'f', 2)
    card(g, 'sardonic_wit', sw + T(sparkle('a', 6), 124, 28), 'w')
    tc = P('M72 64 L88 64 L84 100 L76 100 Z', 'm') + R(66, 56, 28, 10, 3, 'w') + T(flame('d', 'w'), 80, 36, 1.4)
    tc += L('M44 34 l-8 -4 M46 54 h-10 M116 34 l8 -4 M114 54 h10', 'w', 3)
    card(g, 'passionate_conviction', tc, 'd')


def build():
    prompt_cards()
    attachment()
    moral()
    temperament()
    locus()
    cadence()
    chronotype()
    mention()
    conflict()
