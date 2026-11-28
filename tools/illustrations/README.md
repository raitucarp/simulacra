# Illustration packs: the generation pipeline

> **Tooling is being rewritten in Rust.** The repository stays pure Rust, so the commands below
> become `--features codegen` binaries under `src/bin/`. The data files here (`styles/`,
> `subjects.json`, `packs.json`, `anchors/`) and the pipeline steps stay the same.

Simulacra draws every illustration from a **pack**. **Classic** (the palette-tinted SVG templates in
`assets/illustrations/`) is always built in and is the fallback for any key a pack does not cover
or that failed QA. **Flat** is a built-in raster pack (`assets/packs/flat/`, the default); **3D
clay**, **Cozy** and **Isometric** are downloadable `.simpack` files. All raster packs are made
with the tools in this folder from images generated in Google Flow, one pack at a time.

Everything here needs only Python 3 and Pillow. Run commands from the repository root.

## Keys and files

- A **key** is the Classic asset path without `illustrations/` and `.svg`: `empty/no_posts`,
  `guide/cheer`, `portrait/face/round`, `forms/object/body/tin_can`.
- New art that has no Classic template yet may only use the prefixes **`journey/`** and
  **`guides/`**. A Rust guard test checks that every key of every `assets/packs/*/pack.json` is a
  Classic key or starts with one of these.
- A pack folder holds `pack.json` and one `<key>.webp` (WebP with alpha) per covered key:

  ```json
  {"id": "flat", "name": "Flat", "version": "1.0.0", "description": "...", "style": "flat",
   "samples": ["guide/cheer", "spot/ideas", "empty/no_posts"],
   "coverage": {"empty/no_posts": {"file": "empty/no_posts.webp", "sha256": "<hex>", "width": 480, "height": 320}}}
  ```

- **Stored size:** twice the canvas width, at most 1024 px (`160x120` -> `320x240`); layers
  `512x512`.
- A **`.simpack`** is a zip with `pack.json` at its root and every coverage file at its `file`
  path. The app refuses an archive whose files do not match their sha256 in `pack.json`.

## Families and fit

| Kind | Fit | Batch |
|---|---|---|
| Splash, palace rooms | splash: trim; palace: **canvas** (whole scene, landmarks fixed) | single |
| Canvases up to 160x120 and 64x64 (trait cards, spot, notice, settings, import, symbols, garden, palette cards, body cards) | trim | 3x3 sheet |
| Guide 128x128, empty states, classes, palettes, step banners | trim | 2x2 sheet |
| Layers `portrait/**`, `forms/**` (256x256) | **canvas** | 3x3 sheet |
| A group of one (vault) or the last lone cell of a group | as above | single |

- **trim**: the keyed subject is cropped to its alpha and padded, centred, to the canvas aspect
  (it fills 80 % of the limiting side).
- **canvas**: the whole image is scaled to the canvas (centre-cropped if its aspect is a little
  off); nothing moves. Palace rooms keep the five landmark positions of
  `src/domain/learning/palace/rooms.rs` (the contact sheet marks them); layers keep the exact
  position of their reference layer.
- **Layered families** (portraits and forms) are stacked at runtime, so each layer must line up
  with its Classic layer: `process.py` compares its alpha with the reference render (IoU at least
  0.80 for back, face, hair, outfit, body, aura; 0.60 for eyes, mouth, brows, accessory, finish,
  markings). A failing layer is left out of `pack.json`, and the app then draws the whole
  avatar in Classic. Layers are generated in **key colours** that the app recolours with the
  persona's choices: neutral grey **`#808080`** for skin (`portrait/face`), hair
  (`portrait/hair`, `portrait/brows`) and a form body's main colour, pure cyan **`#00FFFF`** for a
  form's second colour (`forms/*/body`, `forms/*/markings`). Whites and blacks stay as they are.
- `portrait/accessory/none` is intentionally empty (`generate: false`).

## Workflow

1. **Reference renders** (Classic SVGs filled with the default dark palette, layers in the key
   colours, at 2x canvas):

   ```
   cargo run --features codegen --bin illustration-refs -- tools/illustrations/refs
   cargo run --features codegen --bin illustration-refs -- tools/illustrations/refs-magenta --matte ff00ff
   ```

   `refs/` (transparent) feeds the sheets and the layer alignment check; `refs-magenta/` holds the
   single references to upload to Flow.

2. **Catalog.** `catalog.yaml` (generated and git-ignored; about 5 MB) lists every key with its canvas, aspect, stored size, fit, batch,
   sheet id, subject, constraints and the full prompt of each pack. Rebuild it after editing
   `subjects.json` (hand-written subjects and group sentences), `styles/<pack>.md` (each has a
   ```` ```prompt ```` and a ```` ```sheet-prompt ```` block) or the SVGs:

   ```
   python tools/illustrations/build_catalog.py
   ```

   The file is a strict YAML subset (every scalar JSON-encoded) read by `catalog.py`; it ends with
   commented placeholders for the `journey/` and `guides/` art that comes later.

3. **Style session.** Before a pack starts, try its look in Flow with the style guide
   (`styles/<pack>.md`) and keep the chosen anchor images as `anchors/<pack>-1.jpg`, `-2.jpg`,
   `-3.jpg` (tracked in git); every later generation of that pack starts from them.

4. **Sheets.** Compose the reference sheets and their prompts:

   ```
   python tools/illustrations/sheets.py flat                 # every sheet of the pack
   python tools/illustrations/sheets.py flat --group mbti    # one group
   python tools/illustrations/sheets.py flat --batch trait-mbti-01
   ```

   Each sheet gives `sheets/<pack>/<id>.png` (the references on solid `#FF00FF`, magenta
   gutters, longest side 2048), `<id>.txt` (the prompt, row by row and cell by cell) and
   `<id>.json` (grid, the Flow **aspect** to pick, each cell's key, box and slot). Sheet aspects
   are only ever Flow's five: 16:9, 4:3, 1:1, 3:4 and 9:16.

5. **Generate in Flow.** `flow/flow.py` does this whole step in your own signed-in Chrome,
   through Chrome's remote debugging on 127.0.0.1 (turn it on at `chrome://inspect/#remote-debugging`;
   Chrome asks you to allow the connection):

   ```
   node tools/illustrations/flow/flowctl.mjs           # keep running; writes flow/flowctl.json
   python tools/illustrations/flow/flow.py jobs flat   # what is still to do
   python tools/illustrations/flow/flow.py run flat [--group empty] [--limit N]
   ```

   Open a Flow project first, then attach flowctl to that tab (or let it open one). For each job
   it uploads the reference (once), sets the aspect and 2 outputs, and refuses to generate unless a
   Nano Banana model costs **0 credits**. It then attaches the anchor and the reference, generates,
   and reads both variants' 1K originals into `inbox/<pack>/_variants/`. The variant whose cells
   pass the most checks in a scratch run of `process.py` is downloaded at 2K to
   `inbox/<pack>/<id>.jpg`. References are uploaded before the first job, ten at a time. Choices
   and scores go to `inbox/<pack>/_variants/log.jsonl`; failed jobs are retried on the next `run`.

   By hand: add the ingredients **in this order**:
   1. the style anchor `anchors/<pack>-1.jpg`;
   2. the reference: `refs-magenta/<key>.png` for a single, `sheets/<pack>/<id>.png` for a sheet.

   Paste the prompt (a single: the entry's `prompts.<pack>` in `catalog.yaml`; a sheet:
   `sheets/<pack>/<id>.txt`) and pick the **closest of Flow's five aspects** (16:9, 4:3, 1:1,
   3:4, 9:16; a sheet's json names it). Download the chosen image as **"1K Original" or
   "2K Upscaled"** (Flow serves JPEG; 1K is 1024 px and 2K is 2048 px on the long side of a
   1:1 image) into the pack's inbox; `.jpg`, `.jpeg`, `.png` and `.webp` are all accepted:

   ```
   tools/illustrations/inbox/<pack>/<key>.jpg        # a single, e.g. inbox/flat/empty/no_posts.jpg
   tools/illustrations/inbox/<pack>/<batch-id>.jpg   # a sheet, e.g. inbox/flat/trait-mbti-01.jpg
   ```

   Folders whose name starts with `_` (e.g. `inbox/_raw/` for untouched downloads) are ignored
   by the tools, and the whole `inbox/` is git-ignored.

6. **Process.** Keys the magenta out, fits, resizes and writes WebP plus `pack.json` (flat goes
   straight to `assets/packs/flat/`, other packs to `tools/illustrations/out/packs/<pack>/`).
   Keying: magentaness `m = min(R, B) - G` relative to the backdrop colour estimated from the
   image border (so a drifted magenta keys the same), a soft alpha ramp from 0.35 (opaque) to
   0.75 (transparent), interior pixels forced opaque; colours unmixed from the backdrop; edge
   pixels tinted by the backdrop (anti-aliasing and JPEG 4:2:0 chroma fringes, up to 2 px into
   the opaque edge) take the subject's own colour from just inside; any leftover cast is clamped
   (`min(R, B)` at most 6 above `G`); a 1 px feather.

   ```
   python tools/illustrations/process.py flat --key empty/no_posts
   python tools/illustrations/process.py flat --sheet trait-mbti-01
   python tools/illustrations/process.py flat --all          # everything in inbox/flat/
   ```

   A sheet is sliced by its json, its boxes scaled to whatever size came back (1024, 2048 or any
   other, as long as the aspect matches); each cell must hold exactly
   one subject that does not touch its slot edge, or it is reported as `REGENERATE <key> [row r,
   cell c]: <reason>` and left to Classic; regenerate that cell alone as a single. Runs are
   idempotent and resumable (`out/state/<pack>.json`); `--force` reprocesses unchanged inputs.

7. **Check.** Automated QA of the whole pack and one contact sheet per group:

   ```
   python tools/illustrations/process.py flat --check
   ```

   It checks each file's sha256 and stored size, the aspect (within 2 %), transparent corners and
   alpha coverage (0.08-0.92) for trimmed art, and leftover magenta (at most 0.2 % of opaque
   pixels), lists the layers that failed alignment, writes `out/qa/<pack>.json` and
   `out/contact-<pack>-<group>.png` (each image on a checkerboard, a light and a dark backdrop;
   failures framed red; palace landmarks marked) and exits 1 on any failure. Review the contact
   sheets and regenerate rejects.

8. **Downloadable packs.** Build the archive and note its checksum:

   ```
   python tools/illustrations/make_simpack.py cozy
   ```

   It verifies every file, writes `tools/illustrations/out/cozy-1.0.0.simpack` and prints the
   entry for `assets/packs/downloadable.json` (`id`, `name`, `description`, `version`, `url`,
   `sha256`, `bytes`). The list ships empty; add the entry, with the release asset URL, only once
   the archive is published.

## Tests

```
python -m unittest discover -s tools/illustrations/test -p "test_*.py"
```

They use synthetic images only (keying, including a 4:2:0 JPEG of cream and coral shapes that
must key without a pink fringe; fitting; sheets, including a 2048 sheet returned as a 1024 JPEG;
layer alignment; QA; catalog; archives).
The Rust tests use the fixture pack `tests/fixtures/packs/sample/` (and `sample.simpack`), made
by running this pipeline on synthetic inbox images.
