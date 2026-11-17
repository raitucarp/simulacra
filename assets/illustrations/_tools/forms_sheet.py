"""Preview composed forms: python forms_sheet.py <out-dir> [theme-id]

Composes every form of every kind (mirroring `compose_form_template` in Rust: catalog paint order,
silhouette clip for clipped layers, face anchor transform for anchored layers), plus one sheet per
kind and layer that cycles that layer's variants on the kind's first forms. Then renders each folder
with the render_illustrations example into <out-dir>/<name>.png (set CARGO_TARGET_DIR or SHEET_EXE).
"""
import json
import os
import re
import shutil
import subprocess
import sys

ROOT = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), 'forms')
EXE = os.environ.get('SHEET_EXE') or os.path.join(os.environ.get('CARGO_TARGET_DIR', 'target'), 'debug', 'examples', 'render_illustrations')
GROUP = '<g fill="none" stroke-linecap="round" stroke-linejoin="round"{}>'
FACE = (128, 136)
SAMPLE = {'eyes': 'round', 'mouth': 'smile', 'back': 'plain'}


def inner(path):
    text = open(path, encoding='utf-8').read()
    return re.sub(r'^<svg[^>]*>', '', text.strip())[:-len('</svg>')]


def compose(kind, cat, parts):
    form = parts['body']
    dx, dy, s = cat['forms'].get(form, {}).get('anchor', [0, 0, 1])
    out = ''
    for layer in cat['stack']:
        spec = cat['layers'][layer]
        variant = parts.get(layer, spec['variants'][0])
        if variant == 'none':
            continue
        attrs = ''
        if spec.get('clip'):
            attrs = ' clip-path="url(#form-silhouette)"'
        if spec.get('anchor'):
            attrs = f' transform="matrix({s} 0 0 {s} {FACE[0] * (1 - s) + dx} {FACE[1] * (1 - s) + dy})"'
        out += GROUP.format(attrs) + inner(os.path.join(ROOT, spec.get('dir', kind), layer, variant + '.svg')) + '</g>'
    hexes = {k: cat[k][0]['hex'] for k in ('primary', 'secondary')}
    prefer = cat['forms'].get(form, {}).get('prefer', {})
    for key in hexes:
        if key in prefer:
            hexes[key] = next(sw['hex'] for sw in cat[key] if sw['key'] == prefer[key][0])
    svg = f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256">{out}</svg>'
    return svg.replace('{{primary}}', hexes['primary']).replace('{{secondary}}', hexes['secondary'])


def write_dir(path, svgs):
    os.makedirs(path, exist_ok=True)
    for i, svg in enumerate(svgs):
        open(os.path.join(path, f'{i:02d}.svg'), 'w', encoding='utf-8').write(svg)


def main():
    out, theme = sys.argv[1], (sys.argv[2] if len(sys.argv) > 2 else 'simulacra-midnight')
    tmp = os.path.join(out, '_svg')
    shutil.rmtree(tmp, ignore_errors=True)
    sheets = {}
    for kind in sorted(os.listdir(ROOT)):
        cat_path = os.path.join(ROOT, kind, 'catalog.json')
        if not os.path.isfile(cat_path):
            continue
        cat = json.load(open(cat_path))
        forms = cat['layers']['body']['variants']
        sheets[f'{kind}_forms'] = [compose(kind, cat, dict(SAMPLE, body=f)) for f in forms]
        for layer in ('finish', 'markings', 'accessory', 'aura'):
            variants = [v for v in cat['layers'][layer]['variants'] if v != 'none']
            sheets[f'{kind}_{layer}'] = [compose(kind, cat, dict(SAMPLE, body=f, **{layer: v})) for v in variants for f in forms[:3]]
    cat = json.load(open(os.path.join(ROOT, 'object', 'catalog.json')))
    for layer in ('eyes', 'mouth', 'back'):
        sheets[f'shared_{layer}'] = [compose('object', cat, dict(SAMPLE, body='tin_can', **{layer: v})) for v in cat['layers'][layer]['variants']]
    for name, svgs in sheets.items():
        write_dir(os.path.join(tmp, name), svgs)
        subprocess.run([EXE, os.path.join(tmp, name), theme, os.path.join(out, name + '.png')], check=True)


if __name__ == '__main__':
    main()
