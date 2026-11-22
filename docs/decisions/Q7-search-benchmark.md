# Q7: search at 100 000 posts

**Status:** measured, budget test in place (issue #274).

## The test

`src/state/search/bench.rs`, `one_hundred_thousand_posts_search_within_budget`. It is
`#[ignore]`d, so a plain `cargo test` never builds the 100 000 post vault. Run it with:

```
cargo test --lib state::search::bench -- --ignored --nocapture
```

It builds a scratch vault (`bench.db`) in a fresh folder under the system temp folder and
removes it afterwards. The posts are 16 made-up words each, drawn unevenly from 20 000 words
built from syllables (so prefixes spread like they do in prose); every 20th post holds a to-do,
a code block or the phrase "quiet harbour", every 7th is tinted, every 11th is a task. One in ten
posts has an earlier version (10 000 versions). Every query asks for 100 results through the
executor (`AppState::search`), filters included; the slowest must stay under 400 ms (8x that in
an unoptimised build, as in the graph bench).

## Measured (optimised test build, Windows 11, one run)

| What | Time |
|---|---|
| Write 100 000 posts + 10 000 versions, both indexes kept by triggers (one transaction each) | 32.6 s |
| Load the vault into the store | 2.1 s |

| Query | Hits | Total | Full-text index |
|---|---|---|---|
| `nuso` (common word) | 100 | 27.6 ms | 17.0 ms |
| `kapolo` (mid word) | 100 | 3.2 ms | 1.8 ms |
| `zenudure` (rare word) | 4 | 0.4 ms | 0.5 ms |
| `nuso kapolo` | 4 | 0.5 ms | 0.5 ms |
| `"quiet harbour"` (phrase in 5 % of posts) | 100 | 18.9 ms | 13.3 ms |
| `nus` (short prefix) | 100 | 19.3 ms | 16.5 ms |
| `kapolo has:todo` | 26 | 2.4 ms | 2.4 ms |
| `nuso -kapolo tint:amber` | 100 | 20.5 ms | 15.4 ms |
| `has:code is:open` (filters only) | 100 | 21.8 ms | none |
| `tint:amber after:2026-01-01` (filters only) | 100 | 12.7 ms | none |
| `oldword7`, old versions on | 100 | 4.4 ms | 0.3 ms |
| `kapolo`, old versions on | 100 | 2.5 ms | 1.8 ms |

## What the first runs changed

- **Only the last word is a prefix.** With every word a prefix, `word1 word2` over a vocabulary
  where many words share the prefix took 750 ms in the index alone: each prefix expanded to
  thousands of terms. The query now reads every word whole except the last, the one still being
  typed (`query_match_expression`; the in-memory fallback reads them the same way).
- **`posts.status` is indexed.** Leaving scheduled posts out (`NOT IN (… WHERE status = …)`)
  scanned every post on every query: a rare word cost 20 ms. With `idx_posts_status` it reads
  only the scheduled rows, and a rare word costs under 1 ms.

## Known limit

A prefix that matches nearly every post (in an earlier run, three letters that began every word
of the made-up vocabulary) makes bm25 rank all 100 000 rows: about 0.8 s. Real prose does not
share one prefix across every post, and the search runs once per change of the field, so this
was left as is; a later fix could rank short prefixes by recency instead of bm25.
