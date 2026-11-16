# Simulacra Illustration Style Guide (Flat Vector)

Every illustration is an SVG **template**. At load time the app replaces `{{token}}` placeholders
with colors from the active theme palette, so each illustration matches all 28 themes.

## Placeholders (palette keys, see `assets/themes/*.json`)
`{{bg}} {{surface}} {{elevated}} {{border}} {{fg}} {{fg_muted}} {{accent}} {{accent_2}}
{{agent}} {{success}} {{warning}} {{danger}} {{info}} {{selection}}`

Never hardcode a hex color. The only literal colors allowed: `none`, `#000` / `#fff` used *only* inside
`stop-opacity`/`fill-opacity` shading layers (e.g. a 12% black shadow).

## Canvas sizes
| Kind | viewBox | Path |
|---|---|---|
| Trait option card | `0 0 160 120` | `illustrations/<group>/<key>.svg` |
| Step banner | `0 0 320 120` | `illustrations/steps/<step>.svg` |
| Class art | `0 0 200 240` | `illustrations/classes/<class>.svg` |
| Guide pose | `0 0 128 128` | `illustrations/guide/<pose>.svg` |
| Empty state | `0 0 240 160` | `illustrations/empty/<name>.svg` |
| Portrait part | `0 0 256 256` (layered, shared origin) | `portrait/<layer>/<variant>.svg` |

## Visual rules
- **Shapes:** soft geometric primitives (circles, rounded rects, arcs). `stroke-linecap="round"`, `stroke-linejoin="round"`.
- **Tones:** at most 3 hues per piece + `{{fg}}` for outlines/details. One `linearGradient` per piece max,
  going from a hue to the same hue at lower opacity (never between two hues).
- **Backdrop:** every option card has a rounded backdrop blob in `{{elevated}}` and one accent hue.
- **Depth:** one soft shadow ellipse (`{{fg}}` at `fill-opacity="0.08"`) under the subject.
- **Stroke widths:** 2 (details), 3 (outlines). No hairlines below 1.5 at 160px.
- **No text** inside illustrations (labels are localized UI). No emoji.
- **IDs:** prefix gradient/clip ids with the file key (`id="g-mbti-intj"`) to avoid collisions.
- **Size:** keep each file under 3 KB; no embedded raster images, no external refs, no `<style>` blocks.
