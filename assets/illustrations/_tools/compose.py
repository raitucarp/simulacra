"""Preview stacked portraits: python compose.py <out-dir> [layer]

Writes one composite SVG per variant of <layer> (others at defaults), or a mixed gallery when no layer
is given, then render the dir with sheet.py. Stack order is ORDER.
"""
import os
import re
import sys

ROOT = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), 'portrait')
ORDER = ['back', 'face', 'outfit', 'mouth', 'eyes', 'brows', 'hair', 'accessory']
DEFAULT = dict(back='plain', face='oval', outfit='hoodie', mouth='smile', eyes='round', brows='arched', hair='crop', accessory='none')
GALLERY = [
    dict(back='sunburst', face='round', outfit='toga', mouth='grin', eyes='happy', brows='raised', hair='bald', accessory='laurel'),
    dict(back='library', face='square', outfit='suit', mouth='neutral', eyes='almond', brows='straight', hair='side_part', accessory='glasses'),
    dict(back='starfield', face='heart', outfit='robe', mouth='smirk', eyes='wink', brows='thin', hair='long', accessory='none'),
    dict(back='meadow', face='long', outfit='cardigan', mouth='smile', eyes='sleepy', brows='bushy', hair='bun', accessory='scarf'),
    dict(back='geometric', face='diamond', outfit='lab_coat', mouth='open', eyes='wide', brows='furrowed', hair='spiky', accessory='monocle'),
    dict(back='plain', face='oval', outfit='hoodie', mouth='laugh', eyes='dot', brows='arched', hair='curly', accessory='headphones'),
    dict(back='library', face='round', outfit='scholar_gown', mouth='frown', eyes='narrow', brows='straight', hair='braids', accessory='none'),
    dict(back='meadow', face='heart', outfit='tunic', mouth='cat', eyes='round', brows='raised', hair='ponytail', accessory='none'),
    dict(back='sunburst', face='square', outfit='suit', mouth='smile', eyes='almond', brows='bushy', hair='wavy', accessory='scarf'),
    dict(back='starfield', face='oval', outfit='robe', mouth='neutral', eyes='sleepy', brows='thin', hair='bob', accessory='glasses'),
    dict(back='geometric', face='long', outfit='toga', mouth='grin', eyes='happy', brows='arched', hair='buzz', accessory='laurel'),
    dict(back='plain', face='diamond', outfit='cardigan', mouth='smirk', eyes='wink', brows='furrowed', hair='crop', accessory='monocle'),
]


def inner(layer, name):
    text = open(os.path.join(ROOT, layer, name + '.svg'), encoding='utf-8').read()
    body = re.sub(r'^<svg[^>]*>', '', text.strip())
    return '<g fill="none" stroke-linecap="round" stroke-linejoin="round">' + body[:-len('</svg>')] + '</g>'


def compose(sel):
    parts = ''.join(inner(layer, sel[layer]) for layer in ORDER)
    return f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256">{parts}</svg>'


def main():
    out = sys.argv[1]
    os.makedirs(out, exist_ok=True)
    for f in os.listdir(out):
        os.remove(os.path.join(out, f))
    if len(sys.argv) > 2:
        layer = sys.argv[2]
        names = sorted(f[:-4] for f in os.listdir(os.path.join(ROOT, layer)) if f.endswith('.svg'))
        sels = [dict(DEFAULT, **{layer: nm}) for nm in names]
    else:
        sels = GALLERY
    for i, sel in enumerate(sels):
        open(os.path.join(out, f'{i:02d}.svg'), 'w', encoding='utf-8').write(compose(sel))


if __name__ == '__main__':
    main()
