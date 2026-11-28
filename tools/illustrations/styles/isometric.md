# Isometric pack: style guide

A downloadable raster pack. Every Classic illustration is re-made as a clean isometric illustration
with flat colour planes and thin dark outlines, like a tidy tabletop diorama. Images are generated in
Google Flow (Nano Banana) with two ingredients, the pack anchor and the Classic reference render, on a
solid magenta background, then keyed out by `process.py`.

This guide is written for the image model and its operator: the "Look" sections say what to draw,
the two fenced blocks at the end are the prompt templates the tooling fills in.

## Anchor

`anchors/isometric-1.jpg` is the canonical anchor of this pack: a 2048×2048 JPEG sheet of four
subjects (Mnemo, an empty desk, books with a bulb, linked speech bubbles) chosen by the user. It is
always the **first** ingredient. `anchors/isometric-2.jpg` and `anchors/isometric-3.jpg` may follow
later; when they disagree with `-1`, `-1` wins.

What the anchor shows, and what to copy from it:

- **Mnemo:** a plump lavender owl facing the viewer, with teal spectacles (described below).
- **Empty desk:** an isometric table with a pale lavender top and dark navy side faces and legs, an
  open cream book on a teal cover, a lavender mug with a coral drink, a navy flower pot with coral
  soil and a teal-and-yellow sprout, and a coral pencil with a teal end.
- **Books with a bulb:** three stacked isometric hardbacks (teal, lavender and navy covers, cream page
  edges) with a yellow bulb on a coral ringed base standing upright on top; a small dark footprint
  under the base on the book is shading on the subject and allowed.
- **Speech bubbles:** two extruded speech-bubble slabs, lavender and teal, each with a darker side
  face, linked by two yellow arcs with yellow dot ends.

Do **not** copy the dark ground-shadow ellipse under Mnemo in the anchor (shared rule 3).

## Look

**Overall.** Clean isometric illustration. Objects are drawn in true isometric projection: the
three axes at 30° to the horizontal and vertical, parallel lines, no vanishing points, no perspective
shrink. Forms are tidy geometric slabs and blocks with slightly rounded corners, like a miniature
model on a desk. Characters such as Mnemo face the viewer straight on.

**Line policy: thin dark-navy outlines.** Every shape has a thin outline of even weight in dark navy
(about `#1C1450`, measured on the anchor), along its silhouette and along the edges between planes.
Inner details (page lines, chevron feathers, filaments) use the same line.

**Shading: flat planes with three-tone face shading.** Each colour comes in three flat values: the
top face lightest, the left face the base value, the right face darkest. No gradients except a very
soft one on a character's round body, no textures, no reflections. Round glossy things (pupils, the
bulb) get one small white highlight.

**Palette.** Pastel violet, teal, coral, yellow, cream and navy; base values measured on the anchor,
face steps where it shows them. They read on both a near-black and a white app background.

| Role | Colour | Hex | Light / dark face |
| --- | --- | --- | --- |
| Mnemo, desk tops, bubbles, books | Pastel violet | `#B395E1` | `#D9B5F5` / `#8A5FC5` |
| Spectacles, covers, bubbles | Teal | `#58C2B8` | `#7FD3CB` / `#338D90` |
| Beak, feet, drinks, pencils | Coral | `#FF7462` | — / `#F36C5B` |
| Bulbs, eye discs, connectors | Yellow | `#FECD65` | — |
| Pages, bellies | Cream | `#FAEBD1` | `#F9EAD1` / `#F3D1B5` |
| Side faces, pots, dark covers | Navy | `#2B356B` | `#3A4590` / `#1C1450` |
| Outlines | Dark navy | `#1C1450` | — |

Violet is the brand colour and is welcome. Magenta, fuchsia, hot pink and pink-leaning purples are
not (shared rule 4).

**Perspective exceptions** that keep readability or alignment:
- **Characters** (Mnemo, people, creatures) face the viewer straight on, with flat three-tone shading
  and outlines; only objects and furniture take the isometric angle.
- **Symbols and garden stages** are round tokens seen straight on from the front (not tilted), with a
  slightly extruded rim and the glyph raised as a bevelled shape.
- **Palace rooms** are front-facing cutaway dioramas: the back wall stays parallel to the image, the
  floor recedes gently, and the furniture has isometric depth, so the landmark positions hold.
- **Layered families** (portraits, forms) stay front-facing.

**Proportions and characters.** Rounded, simple figures with big heads, round eyes with a highlight
dot, mitten hands, standing on their own feet (no floor tile under single objects). People are
simple, inclusive and non-stereotyped. Flat things (pages, screens, cards, documents) become thin
slabs or tiles with visible thickness.

## Mnemo, the guide owl

Mnemo is the app's guide and appears in the guide poses, in some empty states and step banners. He
looks the same in every image (one character sheet), as in the anchor, facing the viewer:

- **Body:** a plump egg shape, wider at the bottom, in pastel violet with a darker violet edge on his
  right side, outlined in dark navy. Two short pointed ear tufts on top, left and right.
- **Wings:** pastel violet with a darker violet shade, rounded, held against his sides.
- **Belly:** a large cream patch with six small chevron feathers in rows of three, two and one, in
  mixed colours (violet, teal, yellow, coral).
- **Eyes:** two large round yellow eye discs in a cream face mask, wearing round teal spectacle rims
  joined by a short teal bridge; navy pupils with a white highlight dot.
- **Beak:** a small coral triangle pointing down, between and just below the eyes.
- **Feet:** two coral feet with three rounded toes each.
- **Poses** (128×128, front-facing, standing):
  - `idle`: both wings resting at his sides, looking straight ahead.
  - `point`: the wing on the viewer's left rests down; the wing on the viewer's right sweeps out and
    up to the right, pointing at something beside him. Looking ahead.
  - `think`: the wing on the viewer's left rests down; the wing on the viewer's right curls up
    under his beak like a hand on the chin. Pupils glance up and to the right, towards a small cream
    thought bubble (one round bubble with a short violet dash inside and a tiny bubble below it) that
    touches his head at the upper right.
  - `read`: both wings hold an open teal book in front of his belly; the pages show blank lines.
    Pupils look down at the book.
  - `cheer`: both wings thrown up high in a V above his shoulders; eyes closed as happy upward arcs;
    a large yellow four-point sparkle at the upper right and a smaller teal sparkle at the upper
    left, both touching the tips of his wings.

## Composition

- Centred subject with generous, even padding on all sides; the subject (or scene) fills about
  70–80 % of the canvas in its aspect ratio and never touches the image edges.
- Keep the reference's layout: the same objects, in the same places, at the same relative sizes.
- Wide banners (8:3) spread their objects from left to right across the width; tall class and
  palette art (5:6) is one full-height figure; small round badges fill their circle.

## Canvas families

| Family | Canvas | Aspect | What it is |
| --- | --- | --- | --- |
| Trait cards and most groups | 160×120 | 4:3 | one symbolic subject |
| Step banners, vault banner | 320×120 | 8:3 | a wide scene, objects spread left to right |
| Classes, full palettes | 200×240 | 5:6 | one full-height character or being |
| Guide poses | 128×128 | 1:1 | Mnemo |
| Empty states | 240×160 | 3:2 | a small calm scene |
| Symbols, garden stages | 64×64 | 1:1 | a round badge with one bold glyph, readable at 24 px |
| Palace rooms | 160×120 | 4:3 | a full room scene filling the card, landmarks fixed |
| Portrait and form layers | 256×256 | 1:1 | one aligned layer in key colours |

## Flow notes

- Ingredients, in this order: the pack anchor `anchors/isometric-1.jpg` first, then the item's Classic
  reference render (or the composed reference sheet).
- Flow serves only JPEG and only the aspects 16:9, 4:3, 1:1, 3:4 and 9:16: pick the closest of those
  five to the canvas or sheet aspect (4:3 → 4:3, 3:2 → 4:3, 8:3 → 16:9, 5:6 → 3:4, 1:1 → 1:1; for a
  sheet, the aspect `sheets.py` records).

## Shared rules (every pack, every image)

1. No text of any kind: no letters, words, numbers, logos, brand marks, signatures, UI chrome
   (window bars with titles, labelled buttons, tooltips) or watermarks. Writing on objects is shown
   as blank bars or lines. Where the reference shows a simple symbol (a question mark, the bar and
   dot of a warning sign, a play triangle, a tick), it is drawn as a bold graphic shape, never as a
   letter or a word.
2. Background: solid, flat, pure magenta `#FF00FF` from edge to edge. No gradient, vignette, texture,
   noise, floor, horizon line or frame on it.
3. No light effects and nothing on the background: no glow, bloom, halos, light rays, lens flares or
   lens effects, and no drop, cast, contact or ground shadows on the background. A glowing bulb (or
   lamp, orb or screen) is drawn as a solid lit shape without a halo. Subjects may have internal
   shading only. A sun or star may keep short solid ray shapes attached to it; "light rays" means
   beams and glow streaks. (In testing, a glowing bulb keyed into a pink halo.)
4. Keep magenta out of the subject: no magenta, fuchsia, hot pink or purple-pink close to `#FF00FF`
   (hue roughly 285–330°). The keyer measures magenta-ness as min(R, B) − G, so a blue-leaning violet
   such as `#7C6CF0` is safe, while a pink-leaning purple is not.
5. One subject (or one scene) per image, centred, with generous even padding; it fills about 70–80 %
   of the canvas in its aspect ratio and never touches the edges.
6. Clean silhouettes: the subject is solid and compact, its parts touch or overlap, and nothing small
   floats free of it (no stray sparkles, dust, particles, motion lines or debris). Small accents from
   the reference (sparkles, dots, stars) are either left out or drawn touching the subject. In a scene
   of several objects, each object is solid and free of floating specks.
7. Match the references: the first reference image (the pack anchor) sets the style; the second (the
   Classic render or reference sheet) sets the subject, composition, object placement and
   proportions.
8. Drop the reference's vector furniture: its rounded backdrop card, the faint halo disc behind the
   subject and the grey shadow ellipse are not drawn. Exceptions: palace rooms are full scenes that
   fill their card, and symbols and garden stages keep their round badge disc.

## Layered families (portrait/\*\*, forms/\*\*)

Portraits and non-human forms are composed at runtime from layers that share one 256×256 square
canvas. Each layer is generated on its own (one layer per image, or per sheet cell):

- **Alignment.** Draw the layer exactly where the reference draws it: the same position, size and
  outline on the square canvas, nothing else in the image. Do not recentre, rescale or crop it; the
  usual padding rule does not apply to layers.
- **Key colours.** The recolourable fills are drawn in key colours that the app recolours at runtime:
  - neutral mid-grey `#808080` for the main recolourable fill: the skin on `portrait/face` (face,
    ears, neck), the hair on `portrait/hair` and `portrait/brows`, and the main body colour on
    `forms/*/body`;
  - pure cyan `#00FFFF` for the second recolourable fill: the secondary regions of `forms/*/body`
    (a label, a trunk, an inner flame, a face panel) and the pattern of `forms/*/markings`.
  Shading in those regions uses only lighter and darker values of the same grey or the same cyan; no
  other hue, no warm or cool tint. Eye whites stay white; pupils, lash lines and other ink stay dark
  navy. Everything else (outfits, accessories, auras, back layers) uses the pack palette normally,
  but never mid-grey or cyan.
- **Front-facing.** Layers are always front-facing, whatever the pack; only the rendering changes. In
  this pack they keep the dark-navy outline and the three-tone flat shading but drop the isometric
  angle.
- **Back layers** (`portrait/back`, `forms/shared/back`) fill the whole square canvas edge to edge,
  as in the reference.
- **Auras** are solid shapes behind the body with internal shading only, never glows that fade into
  the background.
- **Finish and markings** cover the area the reference covers; the app clips them to the body.
- Empty variants (`portrait/accessory/none`, form `none` parts) are never generated.

## Prompt templates

The tooling fills `{subject}`, `{constraints}`, `{canvas}` and `{aspect}` from `catalog.yaml`.

```prompt
Match the illustration style of the first reference image exactly; redraw the subject(s) of the second reference image (same layout, same positions) in that style; {aspect} image for a {canvas} app canvas. Subject: {subject}. {constraints} Style: true 30° isometric, flat colour planes, three-tone faces (top light, left mid, right dark), thin dark-navy outlines; pastel violet, teal, coral, yellow, cream, navy. Leave out the second image's backdrop card, halo disc and shadow ellipse. Centred, even padding, nothing floating free. Background: solid flat pure magenta #FF00FF edge to edge. No glow, bloom, halos, light rays, lens flares or lens effects; no drop, cast, contact or ground shadows; a glowing bulb is a solid lit shape without a halo; internal shading only. No magenta, fuchsia or hot pink in the subject. No text, letters, numbers, logos, UI chrome or watermarks.
```

The sheet template is used by `sheets.py`; `{cells}` becomes one line per cell
(`Row 1, cell 2: <subject>`).

```sheet-prompt
Match the illustration style of the first reference image exactly; redraw the subject(s) of the second reference image (same layout, same positions) in that style; {aspect} sprite sheet, {rows} rows by {cols} columns: one separate subject per cell, centred in its cell, nothing crossing a cell border or touching a neighbour, solid flat pure magenta #FF00FF gutters and background everywhere; all cells in one consistent style, scale and palette. Cells, row by row:
{cells}
Style: true 30° isometric, flat colour planes, three-tone faces (top light, left mid, right dark), thin dark-navy outlines; pastel violet, teal, coral, yellow, cream, navy. Leave out the reference cells' backdrop cards, halo discs and shadow ellipses. No glow, bloom, halos, light rays, lens flares or lens effects; no drop, cast, contact or ground shadows; a glowing bulb is a solid lit shape without a halo; internal shading only. No magenta, fuchsia or hot pink in any subject. No text, letters, numbers, logos, UI chrome or watermarks.
```
