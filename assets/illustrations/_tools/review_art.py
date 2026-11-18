"""Review page empty states (240x160, sb C1): no flashcards yet (a fanned deck waiting for
its first card) and everything reviewed for today (the deck squared away, a check and the
next sunrise)."""
from kit import *
from scenes import empty, dashed


def deck_card(x, y, rot, f='u', rows=2, ink='m'):
    """One flashcard, rotated about its own corner."""
    face = R(0, 0, 70, 48, 7, f, 'f', 2.5) + ''.join(L(f'M12 {16 + i * 12} h{46 - i * 16}', ink, 3, '.6') for i in range(rows))
    return T(face, x, y, 1, rot)


def question_mark(x, y, c='a'):
    return L(f'M{x - 6} {y - 6} q0 -9 7 -9 q8 0 8 7 q0 6 -8 8 v4', c, 3.5) + C(x + 1, y + 13, 2.2, c, None)


def review_empty():
    fan = deck_card(56, 70, -14) + deck_card(70, 56, -4) + deck_card(86, 50, 8, 'e')
    top = question_mark(122, 72)
    slot = dashed('M150 92 h44 a6 6 0 0 1 6 6 v30 a6 6 0 0 1 -6 6 h-44 a6 6 0 0 1 -6 -6 v-30 a6 6 0 0 1 6 -6 Z', 'm', 2.5)
    plus = C(196, 48, 15, 'a') + L('M189 48 h14 M196 41 v14', 'u', 3)
    empty('review_empty', fan + top + slot + plus + T(sparkle('w', 7), 172, 30), 'a')


def review_done():
    stack = ''.join(R(64 - i * 3, 96 - i * 10, 80, 34, 7, ('m', 'u', 'u')[i], 'f', 2.5) for i in range(3))
    stack += L('M74 74 h40 M74 84 h26', 'm', 3, '.6')
    check = C(156, 70, 22, 's') + L('M145 70 l8 8 l15 -16', 'u', 4)
    horizon = L('M150 124 h56', 'f', 2.5, '.5') + T(sun('w', 9, 8, 5), 190, 118)
    dots = C(54, 42, 3, 'm', None) + C(66, 34, 2, 'm', None)
    empty('review_done', stack + check + horizon + dots + T(sparkle('w', 6), 120, 36), 's')


def build():
    review_empty()
    review_done()
