# Documentation screenshots

Every image in the guide is captured from the real app by replayable scripts, so the whole set can
be re-shot after a UI change. Windows only (it drives the window with SendInput, captures it with
`PrintWindow` and finds labels with the built-in Windows OCR engine).

| Path | What it is |
|---|---|
| `shots/<area>.json` | The shot lists: every screenshot id, the page that uses it, what it must show and how to get there. |
| `scenes/<area>.ps1` | The scripts that perform those steps and save each capture. |
| `run.ps1` | Starts Simulacra on a fresh throw-away profile at a 1440×900 client area and runs the scenes. |
| `scripts/Simulacra.psm1` | Window, keyboard, mouse, capture and OCR helpers the scenes use. |
| `scripts/ocr.ps1` | Windows OCR of one image (runs under Windows PowerShell 5.1). |
| `stub-ai.mjs` | A local stand-in for an OpenAI-compatible engine, so AI features can be shown without keys. |
| `convert.py` | `raw/<theme>/*.png` → `public/screenshots/**/*.webp` and `components/screenshots.json`. |
| `contact_sheet.py` | Tiles captures into sheets for reviewing layout at a glance. |

## Re-shooting everything

```powershell
cargo build                                    # from the repository root
node docs/capture/stub-ai.mjs                  # in a second terminal, for the AI scenes
pwsh docs/capture/run.ps1                      # dark theme (about 20 minutes)
pwsh docs/capture/run.ps1 -Theme light         # optional light-theme twins
python docs/capture/convert.py                 # WebP + manifest (needs Pillow)
node docs/scripts/check-mdx.mjs                # every <Screenshot id> resolves
```

Runs happen in the background, so the computer stays usable: Simulacra waits off-screen below the
primary monitor, clicks, keys and wheel turns are posted to its window (`scripts/SimPost.cs`), and
captures read the window itself. Simulacra takes the foreground once as it starts; the run hands it
straight back. A paste of a sample file borrows the clipboard for a moment and puts it back after.
`-Foreground` drives the real pointer and keyboard on a topmost window instead (needed only for the
vault's drag-from-Explorer overlay); leave the mouse and keyboard alone during such a run.
`pwsh docs/capture/run.ps1 -Scenes vault` re-shoots one area; a failed step is reported at the end
and the run carries on.

AI answers shown in screenshots come from `stub-ai.mjs`, which stitches canned sentences around the
words of each request. They illustrate where answers appear, not what a real model would say.
