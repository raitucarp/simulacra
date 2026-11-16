"""Regenerate every illustration template: python assets/illustrations/_tools/build.py [module ...]"""
import importlib
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import kit  # noqa: E402

MODULES = ['cards_mbti', 'cards_mind', 'cards_self', 'cards_voice', 'cards_world', 'scenes', 'classes', 'portrait']

if __name__ == '__main__':
    for name in sys.argv[1:] or MODULES:
        importlib.import_module(name).build()
    print(f'wrote {len(kit.WRITTEN)} files')
