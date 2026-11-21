# R4: colour emoji rendering in the UI toolkit on Windows, macOS and Linux

Status: decided (issue #162). Used by the emoji picker (#164), the reaction bar (#166) and
the clap button (#169).

## Question

Reactions show emoji as text: in chips on every post card, in the picker grid and on the
clap button. Do colour emoji render through the toolkit's ordinary text path on all three
desktop platforms, or do we need our own emoji images?

## What the toolkit does (read from its platform text systems, version 0.3.10)

| | Windows | macOS | Linux (X11 and Wayland) |
| --- | --- | --- | --- |
| Text system | DirectWrite (`direct_write.rs`) | Core Text (`gpui_apple::text_system`) | cosmic-text shaping, swash rasterising (`cosmic_text_system.rs`) |
| Emoji font reached through | the system font fallback (`GetSystemFontFallback`), which maps emoji to Segoe UI Emoji | Core Text cascade to Apple Color Emoji | cosmic-text's fallback list, which names Noto Color Emoji |
| How a glyph is marked as colour | the font has colour tables and `TranslateColorGlyphRun` succeeds for COLR, SVG, PNG, JPEG or BGRA | the face's PostScript name is `AppleColorEmoji` or `.AppleColorEmojiUI` | the face's PostScript name is `NotoColorEmoji` (a `TODO` notes that other emoji fonts are not recognised) |
| Colour formats painted | COLR layers only (a `todo` notes that the other formats are not rasterised) | the font's bitmaps (sbix), drawn into an RGB context | COLR outlines first, then colour bitmaps (CBDT), then plain outlines |
| Tinted by the text colour | no: colour glyphs go through the emoji path, which ignores the text colour | no | no, when the face is Noto Color Emoji |

ZWJ sequences, skin-tone modifiers and flags are shaped by the platform shaper (DirectWrite,
Core Text, HarfRust inside cosmic-text), so a sequence the font knows comes out as one
glyph. The Linux text system keeps a ZWJ inside its emoji cluster when it splits runs by
font. It also skips glyph id 3 in emoji runs to avoid a crash on variation selectors.

## Findings per platform

- **Windows 10 and 11.** Segoe UI Emoji has COLR tables, so emoji render in colour with no
  extra work. Coverage follows the Windows release: current Windows 11 covers Emoji 15.0, while
  Windows 10 no longer receives new emoji and lacks the recent ones. An emoji the font lacks falls back to its pieces (a ZWJ
  sequence shows as two or three emoji side by side) or to an empty box.
- **macOS 11 and later.** Apple Color Emoji renders in colour. macOS updates its emoji font
  every year, so coverage is the newest of the three.
- **Linux.** Colour needs the Noto Color Emoji font. With it, emoji render in colour (COLR
  or CBDT build). Without it, cosmic-text falls back to whatever face has the code point:
  another colour font (such as Twemoji) is not recognised as emoji and is drawn as a
  one-colour shape, and with no emoji font at all the user sees empty boxes. Distribution
  packages of Noto Color Emoji are usually recent (Emoji 15 or later).

## Decision

1. **Render emoji as text through the toolkit**, in the theme's ordinary text sizes. No
   emoji images and no emoji font are bundled: they would add megabytes and a second
   licence, and every platform already ships a colour emoji font the toolkit can draw.
2. **The picker offers emoji up to Emoji 15.0** (`PICKER_MAX_EMOJI_VERSION`), the newest
   version every supported platform's own font covers (Windows 11, current macOS, current
   Noto Color Emoji). Newer emoji stay in the generated table, so a reaction that arrives
   with one (an import, or a newer version of the app) is still kept and shown; agents may
   only use emoji the picker offers.
3. **Mixed skin tones are not offered**: the picker applies one tone to everything, which
   matches the generated table (#159).
4. **Linux packages should depend on Noto Color Emoji** (`fonts-noto-color-emoji` on
   Debian and Ubuntu). That is a packaging change outside this spike; it is listed here so
   the packaging task can pick it up.

## Screenshots

Not attached. This spike was done without access to a macOS or Linux machine, and the
Windows run could not be captured from the agent session (no screen capture tool is
available to it, and the repository allows no helper scripts). To capture them, run the
app with a scratch vault (`SIMULACRA_DB_PATH=<scratch file>`), react to a post with a few
emoji including a ZWJ sequence (for example the family emoji), a skin-tone variant and a
flag, open the picker, and screenshot the post card and the picker on each platform.

## Revisit when

- The toolkit rasterises the other colour formats on Windows (SVG, PNG) or recognises
  emoji fonts other than Noto Color Emoji on Linux: the Linux packaging note can then be
  relaxed.
- Windows ships an emoji font past 15.0: raise `PICKER_MAX_EMOJI_VERSION`.
