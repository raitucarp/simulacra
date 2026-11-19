"""Notices (160x120, sb D3): a missing key, being offline, a failed request and work in
progress, shown next to the message that explains them."""
from kit import *
from scenes import dashed

G = 'notice'


def no_key():
    ring = C(56, 60, 16, None, 'm', 4)
    ghost = dashed('M70 60 H118 M104 60 v12 M114 60 v8', 'm', 4)
    lock = R(98, 30, 30, 24, 6, 'w') + P('M104 30 v-6 q0 -9 9 -9 q9 0 9 9 v6', None, 'f', 3) + C(113, 42, 3, 'f', None)
    card(G, 'no_key', ring + ghost + lock + T(sparkle('a', 6), 36, 32), 'w')


def offline():
    cloud_ = T(cloud('u', 'f', 3), 80, 56, 1.6)
    slash = L('M50 92 L112 26', 'd', 5) + L('M50 92 L112 26', 'u', 1.5)
    drops = C(46, 30, 3, 'm', None) + C(124, 88, 3, 'm', None)
    card(G, 'offline', cloud_ + slash + drops, 'i')


def error():
    sheet = R(46, 24, 52, 66, 6, 'u') + lines(56, 40, (30, 22, 30), 9, 'm', 3)
    sign = P('M104 46 L126 86 H82 Z', 'd') + L('M104 60 v12', 'u', 4) + C(104, 79, 2.5, 'u', None)
    card(G, 'error', sheet + sign, 'd')


def working():
    glass = P('M64 26 H96 Q96 46 80 60 Q96 74 96 94 H64 Q64 74 80 60 Q64 46 64 26 Z', 'u')
    sand = P('M70 34 H90 Q88 46 80 52 Q72 46 70 34 Z', 'w', None) + P('M68 92 Q72 78 80 74 Q88 78 92 92 Z', 'w', None)
    caps = R(58, 20, 44, 8, 4, 'm') + R(58, 92, 44, 8, 4, 'm')
    card(G, 'working', glass + sand + caps + T(sparkle('a', 7), 118, 36) + T(sparkle('a', 5), 42, 80), 'a')


def build():
    for make in (no_key, offline, error, working):
        make()
