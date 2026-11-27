# Writing the Simulacra docs

The site is Next.js + [Fumadocs](https://fumadocs.dev), exported as static HTML to GitHub Pages.
Pages are MDX under `content/docs/`; screenshots are WebP under `public/screenshots/`.

## Sections

| Folder | What goes there | Diátaxis type |
|---|---|---|
| `getting-started/` | Install, first launch, the window, first steps | Tutorial (short) |
| `concepts/` | What things *are* and why they work the way they do | Explanation |
| `tutorials/` | One end-to-end outcome, start to finish, with every click | Tutorial |
| `guides/<area>/` | Every feature, view and control of one area of the app | How-to + description |
| `reference/` | Tables to look things up: shortcuts, commands, settings, formats | Reference |

Each folder has a `meta.json` with `title`, optional `icon` (a [Lucide](https://lucide.dev/icons) name in
PascalCase, e.g. `"BookOpen"`) and `pages` (file names without `.mdx`, in reading order).

## Voice

- Second person, present tense, plain words: "Press <Shortcut keys="Ctrl+N" /> to start a thread."
- Lead with what the reader can do and why it matters, then how. One idea per paragraph.
- Compact: short paragraphs (1–3 sentences), tables for option lists, numbered `<Steps>` for procedures.
- Use the app's exact labels (from `locales/en.yml`) wrapped in `<UiLabel>`: <UiLabel>Import file</UiLabel>.
- Never invent behaviour. Every claim must be traceable to the source (`src/`) or `locales/en.yml`.
  When unsure, leave it out rather than guess.
- No marketing superlatives, no emoji, no "simply"/"just"/"easily".
- Say where things are by region: the **title bar**, the **left sidebar** (expanded or the icon **rail**),
  the **centre column**, the **right sidebar**, a **sheet** (a modal panel), a **popover**, a **menu**.

## Page template (guides)

```mdx
---
title: Knowledge Vault
description: One sentence saying what this feature is for (shown under the title and in search).
icon: BookOpen
---

One or two sentences of orientation: what it is, when you would use it.

<Screenshot id="vault-overview" alt="The Knowledge Vault with three documents" caption="The vault of the Stoa network." />

## Open it
How to get here: sidebar item, shortcut, palette command.

## Anatomy
A numbered or bulleted walk through every region and control visible in the screenshot.

## <Task 1> / <Task 2>
<Steps>
<Step>
### Do the first thing
Explanation.
</Step>
</Steps>

## Options
| Option | What it does | Default |

## Tips and limits
Edge cases, what happens on errors, what is not possible.

## Related
Links to concepts, tutorials, reference.
```

## Components available in MDX

| Component | Use |
|---|---|
| `<Screenshot id="…" alt="…" caption="…" size="full\|md\|sm" />` | A capture. `id` must be in a shot list. `size` defaults from the image width. |
| `<ScreenshotGrid cols={2}>…</ScreenshotGrid>` | Small captures side by side. |
| `<Shortcut keys="Ctrl+Shift+P" />` | A chord. Write the Windows form; ⌘ for macOS is added automatically. Use `mac={false}` for keys that are the same everywhere. |
| `<Kbd>Esc</Kbd>` | A single key. |
| `<UiLabel>New Persona</UiLabel>` | A label exactly as shown in the app. |
| `<Callout type="info\|warn\|error\|success\|idea" title="…">…</Callout>` | Notes. Use sparingly. |
| `<Steps><Step>…</Step></Steps>` | Numbered procedures (each step starts with a `###` heading). |
| `<Tabs items={['Windows','macOS','Linux']}><Tab value="Windows">…</Tab></Tabs>` | Per-platform text. |
| `<Cards><Card title="…" href="/docs/…" description="…" /></Cards>` | Link grids on index pages. |
| `<Accordions><Accordion title="…">…</Accordion></Accordions>` | FAQs. |

Links between pages are absolute: `[the vault](/docs/guides/vault)`.

MDX pitfalls: escape `{`, `}`, `<` and `>` in prose (`\{`, `&lt;`), close every tag, and keep a blank
line around components that contain Markdown.

## Screenshots

Every image comes from a **shot list** in `capture/shots/<area>.json`, an array of:

```json
{
  "id": "vault-import-sheet",
  "page": "guides/vault/import.mdx",
  "kind": "window | region | element",
  "subject": "What the image must show, precisely enough to verify it.",
  "region": "For region/element shots: what to crop to (e.g. 'the import sheet only').",
  "setup": "State to prepare first (data to create, settings to change), or empty.",
  "steps": ["Press Ctrl+2", "Click Import file", "…"]
}
```

- `window`: the whole app window. `region`: one panel, sheet or column. `element`: a single control,
  card, chip or menu (the most granular, atomic level).
- Ids are kebab-case, prefixed by area (`vault-`, `forge-`, `review-`…), unique across all lists.
- Steps use exact labels and shortcuts so a person (or a script) can reproduce the image.
- Captures are taken from the seeded first-run data of a fresh profile unless `setup` says otherwise.

`npm run shots` (see `capture/README.md`) converts raw captures into `public/screenshots/` and
regenerates `components/screenshots.json`.
