"""Per-kind catalog data: {{primary}}/{{secondary}} swatches and per-form hints.

Form hints: `anchor` [dx, dy, scale] moves the face/accessory onto the body's face area; `prefer` maps a
layer (or `primary` / `secondary`) to the variants a random roll picks from, so rolls stay coherent.
"""


def f(anchor=None, **prefer):
    out = {}
    if anchor:
        out['anchor'] = list(anchor)
    if prefer:
        out['prefer'] = prefer
    return out


NOFACE = ['none']
BEAK = ['none']

OBJECT = (
    ([('tin', '#b8bec6'), ('copper', '#c27c4e'), ('brass', '#c9a646'), ('porcelain', '#eceff1'),
      ('cherry', '#c0392b'), ('teal', '#2f8f8a'), ('navy', '#34495e'), ('oak', '#a0703c')],
     [('red', '#d9534f'), ('cream', '#f2e6c9'), ('cobalt', '#3b6fb6'), ('mint', '#7fc8a9'),
      ('gold', '#e0b84c'), ('charcoal', '#3a3a40'), ('rose', '#e58fa0'), ('walnut', '#6b4a2f')]),
    {
        'tin_can': f(primary=['tin', 'brass', 'copper'], secondary=['red', 'cobalt', 'cream', 'mint']),
        'book': f(primary=['cherry', 'navy', 'teal', 'oak'], secondary=['gold', 'cream']),
        'chair': f((0, -30, .9), primary=['oak', 'cherry', 'teal'], secondary=['walnut', 'cream', 'red']),
        'lamp': f((0, -26, .9), primary=['porcelain', 'teal', 'cherry', 'brass'], secondary=['gold', 'charcoal', 'walnut']),
        'clock': f((0, -2, .95), primary=['cherry', 'teal', 'navy', 'brass'], secondary=['gold', 'cream', 'charcoal']),
        'mirror': f((0, -16, 1), primary=['porcelain', 'tin'], secondary=['gold', 'walnut', 'rose']),
        'key': f((0, -38, .85), primary=['brass', 'copper', 'tin'], secondary=['gold', 'charcoal']),
        'teacup': f((0, 6, .95), primary=['porcelain', 'teal', 'cherry'], secondary=['rose', 'cobalt', 'gold', 'mint']),
    },
)

PLANT = (
    ([('leaf', '#4caf50'), ('fern', '#2e7d32'), ('sage', '#8fae7e'), ('lime', '#a4c639'),
      ('autumn', '#d2691e'), ('crimson', '#c0392b'), ('sunflower', '#f2c12e'), ('blossom', '#f4a6c1')],
     [('bark', '#7a5230'), ('soil', '#5b3a29'), ('straw', '#d9c27a'), ('cream', '#f3ead3'),
      ('moss', '#556b2f'), ('berry', '#8e3b5e'), ('clay', '#b8613f'), ('sand', '#e2c799')]),
    {
        'oak_tree': f((0, -10, 1), primary=['leaf', 'fern', 'autumn', 'lime'], secondary=['bark', 'soil']),
        'cactus': f((0, 0, .8), primary=['sage', 'leaf', 'fern'], secondary=['clay', 'sand', 'straw']),
        'moss': f((0, 22, .9), primary=['fern', 'lime', 'leaf', 'sage'], secondary=['moss', 'soil', 'berry']),
        'mushroom': f((0, 40, .75), primary=['crimson', 'autumn', 'blossom'], secondary=['cream', 'straw', 'sand']),
        'sunflower': f((0, -26, .8), primary=['sunflower', 'autumn'], secondary=['bark', 'soil']),
        'vine': f((0, -4, .95), primary=['leaf', 'fern', 'lime', 'sage'], secondary=['bark', 'moss', 'berry']),
        'bamboo': f((0, 0, .7), primary=['lime', 'sage', 'leaf'], secondary=['moss', 'straw']),
        'seed': f((0, 16, .95), primary=['autumn', 'sunflower', 'sage'], secondary=['bark', 'moss']),
    },
)

ELEMENT = (
    ([('water', '#3d9be9'), ('aqua', '#4fd1c5'), ('ember', '#f26b1d'), ('gold', '#f5b82e'),
      ('stone', '#8d8a85'), ('frost', '#bfe6f5'), ('storm', '#6c7a89'), ('dune', '#e0c08c')],
     [('foam', '#e6f6ff'), ('flame', '#ffd23f'), ('ash', '#4a4a4f'), ('lichen', '#6f8f4e'),
      ('violet', '#9b7fd1'), ('coral', '#ff8a65'), ('ice', '#dff4ff'), ('umber', '#8a5a2b')]),
    {
        'water': f((0, 18, 1), primary=['water', 'aqua'], secondary=['foam', 'ice']),
        'fire': f((0, 8, 1), primary=['ember'], secondary=['flame', 'coral']),
        'wind': f((-6, 0, .95), primary=['frost', 'aqua', 'storm'], secondary=['foam', 'ice', 'violet'], mouth=['ooh', 'wavy']),
        'stone': f((0, 10, 1), primary=['stone', 'storm'], secondary=['lichen', 'ash', 'umber']),
        'light': f((0, -6, 1), primary=['gold', 'frost'], secondary=['flame', 'foam']),
        'ice': f((-12, 18, .82), primary=['frost', 'aqua'], secondary=['ice', 'foam']),
        'cloud': f((0, 14, 1), primary=['frost', 'storm'], secondary=['foam', 'violet', 'ice']),
        'sand': f((0, 34, .9), primary=['dune', 'gold'], secondary=['umber', 'coral']),
    },
)

CREATURE = (
    ([('ginger', '#e08a3c'), ('ink', '#2b2b33'), ('ash', '#9a9aa3'), ('snow', '#f2f0eb'),
      ('sable', '#6b4a33'), ('honey', '#f2b632'), ('coral', '#e86f6f'), ('ocean', '#3e6fa8')],
     [('cream', '#f5e6cc'), ('blush', '#f4a6b7'), ('gold', '#f2c94c'), ('charcoal', '#3a3a40'),
      ('white', '#fafafa'), ('mint', '#9ad8c0'), ('tan', '#c9a27a'), ('slate', '#5c6b7a')]),
    {
        'cat': f((0, 0, 1), primary=['ginger', 'ink', 'ash', 'snow', 'sable'], secondary=['blush', 'cream']),
        'crow': f((0, -8, 1), mouth=BEAK, primary=['ink', 'ash'], secondary=['gold', 'slate']),
        'octopus': f((0, -6, 1), primary=['coral', 'ocean', 'honey'], secondary=['cream', 'blush', 'mint']),
        'wolf': f((0, 6, 1), mouth=['none', 'smile', 'grin'], primary=['ash', 'sable', 'ink', 'snow'], secondary=['cream', 'white', 'tan']),
        'bee': f((0, -4, .9), primary=['honey'], secondary=['charcoal']),
        'whale': f((18, 6, .85), primary=['ocean', 'ash', 'ink'], secondary=['cream', 'white', 'mint']),
        'snail': f((-46, 14, .62), primary=['honey', 'snow', 'coral'], secondary=['tan', 'gold', 'slate']),
        'owl': f((0, -2, 1), mouth=BEAK, primary=['sable', 'ash', 'snow', 'honey'], secondary=['cream', 'tan', 'gold']),
    },
)

MACHINE = (
    ([('steel', '#9aa5b1'), ('gunmetal', '#4b525c'), ('signal', '#e04f3f'), ('cobalt', '#2f6fd1'),
      ('mint', '#59c3a5'), ('cream', '#efe6d2'), ('amber', '#f0a83a'), ('onyx', '#26282d')],
     [('screen', '#aef0d0'), ('chrome', '#dfe5ea'), ('copper', '#c27c4e'), ('yellow', '#f5c542'),
      ('cyan', '#4fd1e8'), ('red', '#e05050'), ('graphite', '#3c4048'), ('violet', '#9b7fd1')]),
    {
        'robot': f((0, 0, .95), primary=['steel', 'cobalt', 'mint', 'cream', 'signal'], secondary=['screen', 'chrome', 'cyan']),
        'satellite': f((0, 0, .72), primary=['steel', 'cream', 'amber'], secondary=['cyan', 'violet']),
        'server': f((0, 0, .9), primary=['onyx', 'gunmetal', 'steel'], secondary=['screen', 'cyan', 'chrome']),
        'typewriter': f((0, 18, .82), primary=['onyx', 'mint', 'signal', 'cream'], secondary=['graphite', 'chrome', 'copper']),
        'drone': f((0, 4, .78), primary=['onyx', 'steel', 'signal', 'cream'], secondary=['chrome', 'graphite']),
        'radio': f((0, 4, 1), primary=['cream', 'mint', 'signal', 'amber'], secondary=['copper', 'graphite', 'chrome']),
        'calculator': f((0, -54, .8), primary=['gunmetal', 'cream', 'onyx'], secondary=['screen', 'yellow']),
        'train': f((0, -8, 1), primary=['signal', 'cobalt', 'onyx', 'mint'], secondary=['yellow', 'graphite', 'copper']),
    },
)

PLACE = (
    ([('stone', '#b9b2a3'), ('brick', '#b8573e'), ('sandstone', '#d9b77e'), ('slate', '#6e7a87'),
      ('marble', '#ece8e1'), ('jade', '#4f9a7b'), ('dusk', '#7b5ea7'), ('terracotta', '#c8714a')],
     [('roof', '#8e3b3b'), ('sea', '#3d8fd1'), ('pine', '#3f7d4a'), ('gold', '#e2b84b'),
      ('ivory', '#f4efe2'), ('night', '#2c3350'), ('sand', '#e8d3a2'), ('copper', '#b87333')]),
    {
        'city': f((0, 6, .75), primary=['slate', 'dusk', 'brick'], secondary=['night', 'gold', 'ivory']),
        'island': f((0, 38, .82), primary=['sandstone', 'jade'], secondary=['pine', 'sea']),
        'library': f((0, 8, .9), primary=['marble', 'stone', 'sandstone'], secondary=['roof', 'copper', 'gold']),
        'cathedral': f((0, 18, .82), primary=['stone', 'marble', 'sandstone'], secondary=['night', 'copper', 'roof']),
        'desert': f((0, 40, .85), primary=['sandstone', 'terracotta'], secondary=['sand', 'gold']),
        'forest': f((0, 22, .82), primary=['jade'], secondary=['pine', 'night']),
        'harbor': f((0, 14, .7), primary=['marble', 'stone'], secondary=['roof', 'sea']),
        'ruin': f((0, 6, .95), primary=['stone', 'sandstone', 'slate'], secondary=['pine', 'sand']),
    },
)

ABSTRACTION = (
    ([('aether', '#9b8cf2'), ('rose', '#f27aa0'), ('gold', '#f0c05a'), ('azure', '#5aa9f0'),
      ('jade', '#5cc79a'), ('ink', '#34324a'), ('pearl', '#ece9f5'), ('flame', '#f07a4a')],
     [('silver', '#c9ccd6'), ('violet', '#7b5ce0'), ('cream', '#f6ecd2'), ('teal', '#3fb5b0'),
      ('crimson', '#d14b5a'), ('midnight', '#22244a'), ('peach', '#f6b38e'), ('lime', '#b5e05a')]),
    {
        'time': f((0, -40, .8), primary=['azure', 'pearl', 'aether'], secondary=['cream', 'peach']),
        'justice': f((0, -14, .85), primary=['gold', 'pearl'], secondary=['silver', 'midnight']),
        'number': f((2, 14, .7), primary=['azure', 'aether', 'jade', 'flame'], secondary=['violet', 'teal', 'peach']),
        'silence': f((0, -14, 1), mouth=['none', 'flat'], primary=['pearl', 'aether', 'ink'], secondary=['silver', 'violet']),
        'memory': f((0, 4, .95), primary=['rose', 'aether', 'pearl'], secondary=['violet', 'crimson']),
        'chance': f((-16, 22, .82), primary=['pearl', 'flame', 'jade'], secondary=['midnight', 'crimson']),
        'love': f((0, -2, 1), primary=['rose', 'flame'], secondary=['crimson', 'peach']),
        'entropy': f((-14, 0, 1), primary=['ink', 'aether', 'azure'], secondary=['teal', 'lime', 'peach']),
    },
)

KINDS = {'object': OBJECT, 'plant': PLANT, 'element': ELEMENT, 'creature': CREATURE,
         'machine': MACHINE, 'place': PLACE, 'abstraction': ABSTRACTION}
