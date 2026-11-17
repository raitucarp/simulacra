# Simulacra Illustration Style Guide (Flat Vector)

Every illustration is an SVG **template**. At load time the app replaces `{{token}}` placeholders
with colors from the active theme palette, so each illustration matches all 28 themes.

## Placeholders (palette keys, see `assets/themes/*.json`)
`{{bg}} {{surface}} {{elevated}} {{border}} {{fg}} {{fg_muted}} {{accent}} {{accent_2}}
{{agent}} {{success}} {{warning}} {{danger}} {{info}} {{selection}}`

Portrait parts add two non-theme placeholders, filled from the agent's chosen look:
- `{{skin}}`: skin fill (face, ears, neck) in `portrait/face/*`.
- `{{hair}}`: hair and eyebrow fill in `portrait/hair/*` and `portrait/brows/*`.

Form parts (`forms/**`) use two others instead, filled from the form's swatches (never `{{skin}}`/`{{hair}}`):
- `{{primary}}`: the body's main fill (the can, the crown of the tree, the water drop).
- `{{secondary}}`: its second fill (label, trunk, inner flame, face panel).

Never hardcode a hex color. The only literal colors allowed: `none`, `#000` / `#fff` used *only* together
with an opacity attribute (`fill-opacity`, `stroke-opacity`, `stop-opacity`): shading layers (e.g. a 12%
black shadow) and, in portrait parts only, facial "ink" (pupils, lips, mouth interior), eye whites, teeth,
and garments that must stay white/black on every theme (lab coat, scholar gown).

## Canvas sizes
| Kind | viewBox | Path |
|---|---|---|
| Trait option card | `0 0 160 120` | `illustrations/<group>/<key>.svg` |
| Step banner | `0 0 320 120` | `illustrations/steps/<step>.svg` |
| Class art | `0 0 200 240` | `illustrations/classes/<class>.svg` |
| Full palette art | `0 0 200 240` | `illustrations/palettes/<palette>.svg` |
| Step palette card, category icon | `0 0 160 120` | `illustrations/palette_steps/<palette>.svg`, `illustrations/palette_categories/<category>.svg` |
| Guide pose | `0 0 128 128` | `illustrations/guide/<pose>.svg` |
| Empty state | `0 0 240 160` | `illustrations/empty/<name>.svg` |
| Splash brand mark | `0 0 160 120` (hand-drawn, like the guide owl) | `illustrations/splash/brand.svg` |
| Portrait part | `0 0 256 256` (layered, shared origin) | `illustrations/portrait/<layer>/<variant>.svg` |
| Form part | `0 0 256 256` (layered, shared origin) | `illustrations/forms/<kind or shared>/<layer>/<variant>.svg` |

## Portrait layers
Stack order (bottom to top): `back, face, outfit, mouth, eyes, brows, hair, accessory`.
Shared anchors: head centre `(128,112)`, eyes at `y=116` (`x=108` / `x=148`), brows `y~98`, mouth `y~148`,
shoulders from `y~188`. Every face shape covers `x 92-164, y 90-160`, so features align on all of them.
`accessory/none.svg` is intentionally empty; `back/*` fill the whole canvas.

## Forms (non-human bodies)
Kinds: `creature plant object element machine place abstraction`, eight forms each (the `body` layer). Each
kind's `forms/<kind>/catalog.json` lists, in paint order (`stack`), every layer's `variants` (the first is
the default), the `primary`/`secondary` swatches (`{key, hex}`) and per-form hints (`anchor`, `prefer`).
- **Stack:** `back, aura, body, finish, markings, mouth, eyes, accessory`. `back`, `eyes` and `mouth` come
  from `forms/shared/` (`"dir": "shared"`), so one face set fits every non-human body.
- **`none`** is a reserved variant with no file: it draws nothing (eyes/mouth `none` for pure objects).
- **Face anchors:** face centre `(128,136)`, eyes at `y=124` (`x=110` / `x=146`), mouth at `y=152`. A body
  keeps `x 92-164, y 106-166` solid, or its catalog hint `anchor: [dx, dy, scale]` moves the eyes, mouth
  and accessory (`"anchor": true` layers) onto its own face area. Accessories are drawn on the default face.
- **Silhouette:** every body defines `<clipPath id="form-silhouette">` (`<use>` of its outline shapes,
  ids `fs0, fs1, …`; the one id exception, since a composition holds a single body). `finish` and
  `markings` (`"clip": true`) are canvas-wide textures the composer clips to it.
- **Aura** paints behind the body over the whole canvas; **finish** is a translucent overlay (gloss, rust,
  frost), **markings** a pattern (stripes, spots, windows). A tiled `<pattern id="g-…">` is fine.
- Organic "ink" on bodies (a nose, pupils) is `#000`/`#fff` with an opacity, never `{{fg}}` (light on dark
  themes).

## Visual rules
- **Shapes:** soft geometric primitives (circles, rounded rects, arcs). `stroke-linecap="round"`, `stroke-linejoin="round"`.
- **Tones:** at most 3 hues per piece + `{{fg}}` for outlines/details. One `linearGradient` per piece max,
  going from a hue to the same hue at lower opacity (never between two hues).
- **Backdrop:** every option card has a rounded backdrop blob in `{{elevated}}` and one accent hue.
- **Depth:** one soft shadow ellipse (`{{fg}}` at `fill-opacity="0.08"`) under the subject.
- **Stroke widths:** 2 (details), 3 (outlines). No hairlines below 1.5 at 160px. Wider strokes (4-9) only
  draw solid shapes (a chain link, a headband, a symbol bar), never outlines.
- **No text** inside illustrations (labels are localized UI). No emoji.
- **IDs:** prefix gradient/clip ids with the file key (`id="g-mbti-intj"`) to avoid collisions.
- **Size:** keep each file under 3 KB (class art, palette art and step banners may use up to 6 KB); no embedded raster
  images, no external refs, no `<style>` blocks. `tests/illustrations.rs` enforces sizes, palette-only
  colors, parseability, and a card for every trait variant.

## Tooling
Every template except the guide owl is generated by `_tools/build.py` (Python 3, no dependencies) from the
small kit in `_tools/kit.py`. Edit the generator rather than the SVGs, then run
`python assets/illustrations/_tools/build.py`.
Preview with `python assets/illustrations/_tools/sheet.py <theme-id> <out.png> <svg-dir>...`, which wraps the
`render_illustrations` example and fills `{{skin}}`/`{{hair}}` with samples (set `CARGO_TARGET_DIR` or
`SHEET_EXE`). `_tools/compose.py <out-dir> [layer]` writes stacked portrait composites for previewing.
Palette art is generated by `_tools/palettes.py` (`python build.py palettes`, after `forms`): each full palette
draws its own form body, face and accessory (read from `forms/**`, recoloured with two theme hues) plus a few
props; `_tools/palette_cards.py` draws the step palette cards and the gallery category icons.
Forms are generated by `_tools/forms*.py` (`python build.py forms`, which also writes the catalogs from
`forms_data.py`); `python _tools/forms_sheet.py <out-dir> [theme]` composes every form and every
finish/markings/accessory/aura variant like the app does and renders one contact sheet per kind and layer.
