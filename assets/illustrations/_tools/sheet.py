"""Render a contact sheet: fills {{skin}}/{{hair}}/{{primary}}/{{secondary}} in a temp copy, then runs the render_illustrations example.

usage: python sheet.py <theme-id> <out.png> <svg-dir>...   (run from the repo root; build the example first)
"""
import os
import shutil
import subprocess
import sys
import tempfile

SAMPLE = {'skin': '#e0ac7e', 'hair': '#5b3a29', 'primary': '#b8bec6', 'secondary': '#d9534f'}
EXE = os.environ.get('SHEET_EXE') or os.path.join(os.environ.get('CARGO_TARGET_DIR', 'target'), 'debug', 'examples', 'render_illustrations')


def fill_dir(src, dst, prefix):
    for name in sorted(os.listdir(src)):
        if name.endswith('.svg'):
            text = open(os.path.join(src, name), encoding='utf-8').read()
            for k, v in SAMPLE.items():
                text = text.replace('{{%s}}' % k, v)
            open(os.path.join(dst, prefix + name), 'w', encoding='utf-8').write(text)


def main():
    theme, out, dirs = sys.argv[1], sys.argv[2], sys.argv[3:]
    tmp = tempfile.mkdtemp()
    for i, src in enumerate(dirs):
        fill_dir(src, tmp, f'{i:02d}_')
    subprocess.run([EXE, tmp, theme, out], check=True)
    shutil.rmtree(tmp)


if __name__ == '__main__':
    main()
