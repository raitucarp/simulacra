"""Layered non-human bodies ("forms"), 256x256 on the shared portrait origin, plus their catalogs.

Layout: `forms/<kind>/<layer>/<variant>.svg` per kind and `forms/shared/<layer>/<variant>.svg` for the
backdrops and the non-human face set. `forms/<kind>/catalog.json` lists every layer's variants in paint
order, the {{primary}}/{{secondary}} swatches and per-form hints (face anchor, preferred parts/colors).

Anchors: face centre (128, 136); eyes at y=124 (x=110 / x=146); mouth at y=152. A body keeps
x 92-164, y 106-166 solid, or declares an `anchor` [dx, dy, scale] that moves the face (and the
accessory) onto its own face area. Bodies define `<clipPath id="form-silhouette">`; the composer clips
`finish` and `markings` to it, so textures stay on the body whatever its shape.
`none` is a reserved variant: it has no file and draws nothing.
"""
import json
import os
import re

from kit import *
import kit

kit.TOK.update(P='primary', Q='secondary')
FX, FY = 128, 136
EYE_Y, EYE_DX, MOUTH_Y = 124, 18, 152
CLIP = 'form-silhouette'
NONE = 'none'
SHARED = 'shared'
STACK = ['back', 'aura', 'body', 'finish', 'markings', 'mouth', 'eyes', 'accessory']
CLIPPED = {'finish', 'markings'}
ANCHORED = {'eyes', 'mouth', 'accessory'}
SHARED_LAYERS = {'back', 'eyes', 'mouth'}
BARE = {'aura': [1, 3], 'finish': [1, 3], 'markings': [1, 2], 'accessory': [1, 2]}
WROTE = {}  # (dir, layer) -> [variant, ...] in write order
SHADOW = E(128, 234, 66, 7, 'f', None, o='.1')
INK = ' fill="#000" fill-opacity=".82"'
WHITE = ' fill="#fff" fill-opacity=".94"'


def lay(dir_, layer, name, body, grad=None):
    write(f'forms/{dir_}/{layer}/{name}.svg', 256, 256, body, grad)
    WROTE.setdefault((dir_, layer), []).append(name)


def S(el):
    """Marks a shape as part of the body silhouette (clip path for finish and markings)."""
    return '\0' + el


def body(kind, form, *parts):
    """Writes a body: the shadow, then `parts` in order; `S(...)` shapes also form the clip path."""
    ids = []

    def tag(m):
        ids.append(f'fs{len(ids)}')
        return f'<{m.group(1)} id="{ids[-1]}"'
    out = SHADOW + re.sub(r'\0<(\w+)', tag, ''.join(p for p in parts if p))
    refs = ''.join(f'<use href="#{i}"/>' for i in ids)
    assert ids, f'{kind}/{form}: no silhouette'
    lay(kind, 'body', form, f'<defs><clipPath id="{CLIP}">{refs}</clipPath></defs>' + out)


def blob(circles, f, w=5):
    """Union of circles with a clean outer outline: stroked set first, unstroked fill set on top."""
    under = ''.join(S(C(x, y, r, f, 'f', w)) for x, y, r in circles)
    return under + ''.join(C(x, y, r, f, None) for x, y, r in circles)


def ink(el):
    return el.replace('/>', INK + '/>')


def white(el, o=None):
    return el.replace('/>', (WHITE if o is None else f' fill="#fff" fill-opacity="{o}"') + '/>')


def stroke_ink(d, w=4, o='.8'):
    return f'<path d="{d}" stroke="#000" stroke-opacity="{o}" stroke-width="{w}"/>'


def eye_pair(fn):
    return fn(FX - EYE_DX) + fn(FX + EYE_DX)


# ---------- shared: backdrops ----------
def backs():
    full = 'M0 0 H256 V256 H0 Z'
    lay(SHARED, 'back', 'plain', P(full, 'G', None) + C(128, 132, 100, 'u', None, o='.35'), 'a')
    sun = C(128, 150, 70, 'w', None, o='.35') + C(128, 150, 46, 'w', None, o='.4')
    lay(SHARED, 'back', 'dawn', P(full, 'G', None) + sun + P('M0 196 Q128 176 256 196 V256 H0 Z', 'u', None, o='.4'), 'd')
    stars = ''.join(T(sparkle('w', r), x, y) for x, y, r in ((34, 40, 7), (214, 34, 6), (228, 126, 8), (26, 150, 5), (52, 222, 6), (206, 214, 7)))
    lay(SHARED, 'back', 'night', P(full, 'G', None) + stars + T(moon('w', None), 206, 74, 1.2), 'l')
    grid = ''.join(L(f'M{v} 0 V256 M0 {v} H256', 'i', 1.5, '.35') for v in range(16, 256, 32))
    lay(SHARED, 'back', 'grid', P(full, 'e', None) + grid + C(128, 132, 96, 'i', None, o='.12'))
    waves = ''.join(P(f'M0 {y} Q32 {y - 10} 64 {y} T128 {y} T192 {y} T256 {y} V256 H0 Z', 'i', None, o='.25') for y in (150, 186, 222))
    lay(SHARED, 'back', 'waves', P(full, 'G', None) + waves, 'i')
    spot = P('M100 0 H156 L232 256 H24 Z', 'u', None, o='.18')
    lay(SHARED, 'back', 'studio', P(full, 'e', None) + spot + E(128, 232, 110, 18, 'k', None, o='.25'))


# ---------- shared: face set (eyes, mouth) ----------
def eyes():
    dot = lambda x: white(C(x, EYE_Y, 9.5, None, None), '.6') + ink(C(x, EYE_Y, 7, None, None)) + white(C(x + 2, EYE_Y - 2, 2, None, None))
    lay(SHARED, 'eyes', 'dot', eye_pair(dot))
    rnd = lambda x: white(E(x, EYE_Y, 11, 12, None, 'f', 2.5)) + ink(C(x, EYE_Y + 1, 6.5, None, None)) + white(C(x + 2.5, EYE_Y - 2, 2.2, None, None))
    lay(SHARED, 'eyes', 'round', eye_pair(rnd))
    lay(SHARED, 'eyes', 'happy', eye_pair(lambda x: stroke_ink(f'M{x - 10} {EYE_Y + 4} Q{x} {EYE_Y - 9} {x + 10} {EYE_Y + 4}', 4.5)))
    sleepy = lambda x: ink(P(f'M{x - 10} {EYE_Y} Q{x} {EYE_Y + 9} {x + 10} {EYE_Y} Z', None, None)) + stroke_ink(f'M{x - 11} {EYE_Y} h22', 4)
    lay(SHARED, 'eyes', 'sleepy', eye_pair(sleepy))
    wide = lambda x: white(C(x, EYE_Y, 14, None, 'f', 2.5)) + ink(C(x, EYE_Y, 4.5, None, None))
    lay(SHARED, 'eyes', 'wide', eye_pair(wide))
    lay(SHARED, 'eyes', 'wink', rnd(FX - EYE_DX) + stroke_ink(f'M{FX + EYE_DX - 10} {EYE_Y + 2} Q{FX + EYE_DX} {EYE_Y - 7} {FX + EYE_DX + 10} {EYE_Y + 2}', 4.5))
    glow = lambda x: C(x, EYE_Y, 13, 'a', None, o='.3') + C(x, EYE_Y, 8, 'a', 'f', 2) + white(C(x + 2, EYE_Y - 2, 2.5, None, None))
    lay(SHARED, 'eyes', 'glow', eye_pair(glow))


def mouths():
    y = MOUTH_Y
    lay(SHARED, 'mouth', 'smile', stroke_ink(f'M{FX - 14} {y - 4} Q{FX} {y + 10} {FX + 14} {y - 4}'))
    lay(SHARED, 'mouth', 'flat', stroke_ink(f'M{FX - 10} {y} h20'))
    lay(SHARED, 'mouth', 'open', ink(P(f'M{FX - 13} {y - 5} Q{FX} {y + 16} {FX + 13} {y - 5} Z', None, None)) + E(FX, y + 4, 6, 3, 'd', None))
    grin = ink(P(f'M{FX - 16} {y - 6} Q{FX} {y + 14} {FX + 16} {y - 6} Z', None, None)) + white(P(f'M{FX - 13} {y - 5} h26 l-2 5 h-22 Z', None, None))
    lay(SHARED, 'mouth', 'grin', grin)
    lay(SHARED, 'mouth', 'ooh', ink(E(FX, y + 1, 6, 7.5, None, None)))
    lay(SHARED, 'mouth', 'wavy', stroke_ink(f'M{FX - 15} {y} q5 -6 10 0 t10 0 t10 0', 3.5))


# ---------- catalogs ----------
def catalog(kind, swatches, forms):
    layers = {}
    for layer in STACK:
        dir_ = SHARED if layer in SHARED_LAYERS else kind
        names = list(WROTE[(dir_, layer)])
        if layer in BARE:
            names = [NONE] + names
        entry = {'variants': names}
        if dir_ == SHARED:
            entry['dir'] = SHARED
        if layer in CLIPPED:
            entry['clip'] = True
        if layer in ANCHORED:
            entry['anchor'] = True
        if layer in BARE:
            entry['bare'] = BARE[layer]
        layers[layer] = entry
    for layer in ('eyes', 'mouth'):
        layers[layer]['variants'] = list(WROTE[(SHARED, layer)]) + [NONE]
    primary, secondary = swatches
    known = dict(primary=[k for k, _ in primary], secondary=[k for k, _ in secondary], **{l: layers[l]['variants'] for l in layers})
    for form, hint in forms.items():
        for key, picks in hint.get('prefer', {}).items():
            bad = set(picks) - set(known[key])
            assert not bad, f'{kind}/{form}: unknown {key} {bad}'
    doc = {'kind': kind, 'stack': STACK, 'layers': layers,
           'primary': [{'key': k, 'hex': h} for k, h in primary],
           'secondary': [{'key': k, 'hex': h} for k, h in secondary],
           'forms': forms}
    path = os.path.join(kit.ROOT, 'forms', kind, 'catalog.json')
    with open(path, 'w', newline='\n') as fh:
        fh.write(compact_json(doc) + '\n')


def compact_json(doc):
    """Indented JSON with scalar lists and swatches kept on one line."""
    text = json.dumps(doc, indent=2)
    text = re.sub(r'\[\s*([^\[\]{}]*?)\s*\]', lambda m: '[' + re.sub(r'\s*\n\s*', ' ', m.group(1)) + ']', text)
    return re.sub(r'\{\s*("key": "[^"]*"),\s*("hex": "[^"]*")\s*\}', r'{\1, \2}', text)


def build():
    import forms_bodies_a
    import forms_bodies_b
    import forms_parts
    from forms_data import KINDS
    backs()
    eyes()
    mouths()
    forms_bodies_a.build(body)
    forms_bodies_b.build(body)
    forms_parts.build(lay)
    for kind, (swatches, forms) in KINDS.items():
        missing = set(forms) ^ set(WROTE[(kind, 'body')])
        assert not missing, f'{kind}: body/catalog mismatch {missing}'
        catalog(kind, swatches, forms)
