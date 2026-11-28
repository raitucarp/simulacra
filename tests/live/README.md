# Live model tests

`tests/live_models.rs` runs the app's own clients, engine runtime and parsers against the real
Gemini API. Every case is `#[ignore]`d, so `cargo test` never sends a request.

## Run

```powershell
# GEMINI_API_KEY is already in your environment
cargo live
```

`cargo live` is an alias (`.cargo/config.toml`) for
`cargo test --test live_models -- --ignored --test-threads=1 --nocapture`. One case:
`cargo live a01` (any name filter after the alias).

## Configuration

| Variable | Default | Meaning |
|---|---|---|
| `GEMINI_API_KEY` | (none) | Required by tiers A–C. Without it those cases are recorded as `skip`. Read from the environment only, never printed. |
| `SIMULACRA_LIVE_MODEL` | `gemini-4-hydrogen` | Text model. |
| `SIMULACRA_LIVE_IMAGE_MODEL` | `gemini-nano-banana-2.1` | Image model (tier C). |
| `SIMULACRA_LIVE_EMBED_MODEL` | `gemini-embedding-001` | Embedding model (`gemini-embedding-2` is also tested on its own). |
| `SIMULACRA_LIVE_TIERS` | `a,b,d` | Tiers to run, comma-separated, or `all`. Tier C costs more, so it is opt-in. |
| `SIMULACRA_LIVE_AUDIO` | (none) | Path to an audio file for the transcription case (tier C). |

## Tiers

- **A, text**: plain chat with the default budget, structured output (`responseSchema`) for each
  schema preset, an engine job of each kind (reply, new post, to-do progress, lens), post
  actions with their parsers, the argument map, the tutor, an arena turn and judge, and
  embeddings (similarity, `gemini-embedding-2` task prefixes, 120 texts in two batches).
- **B, tools and transports**: the `vault_search` tool loop (thought signatures), the emit
  tool while tools are offered, native Google Search grounding sources, a custom raw
  `generateContent` request, and Gemini's OpenAI-compatible endpoint (chat and tool loop).
- **C, media (opt-in)**: an image saved as real PNG or JPEG bytes, a post illustrated on every
  post, and transcription when `SIMULACRA_LIVE_AUDIO` is set. Images are kept in
  `target/live-media/`.
- **D, failures**: a fake key and an unknown model, classified as the app does, with the key
  never in the message. The fake-key cases need no key but still reach Google.

## Report

Each case adds a row (tier, case, pass / FAIL / skip, latency, model, a short redacted note)
to `target/live-report.md` (under `CARGO_TARGET_DIR` when that is set). The file is rewritten
after every case, so it is complete when the run ends.

## Safety

Settings and credential stores are built in memory; the app's database is never opened.
Every note and panic message goes through the app's redaction, and `LlmSettings` and
`ImageJob` print no secret when debug-formatted.
