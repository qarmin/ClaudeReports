# hayro-jbig2 `Bitmap::combine` word-loop optimization

`hayro-jbig2/src/bitmap.rs`'s `Bitmap::combine()` carries the maintainers' own comment - "TODO:
Figure out how we can optimize this, especially since this is a bottleneck for symbols/patterns."
It blits one bitmap into another word-by-word, and is called for every symbol placement in JBIG2
text regions (`decode/text.rs:197`) and every pattern placement in halftone regions
(`decode/halftone.rs:412`). `rustc 1.98.0 (88d9e12ae 2026-08-18)` / `cargo 1.98.0` throughout (no
`rust-toolchain.toml` pinned in this repo - see `projects/hayro.md`).

## What was tried

Three variants were built and measured, in order (each replacing the previous):

1. **Naive "reuse previous word" rewrite.** Per-word `get_word()` calls (each doing its own
   `row >= height || word_idx >= stride` bounds check plus a `row*stride+word_idx` multiply) were
   replaced with a hoisted-once source-row slice, and the observation that for every word after
   the first, the source bit-offset is constant and the source word index advances by exactly one
   per destination word - so the second half of one word's source load can be reused as the first
   half of the next, roughly halving source loads for wide (multi-word) placements. **This variant
   had a real, reproducible bug**: the very first -> second destination word transition does not
   follow the same "+32 bits" arithmetic progression as later transitions (the first word's
   `px_start` is clipped to `dest_x_start`, not aligned to `word_start_x`) - caught by a 20,000-case
   randomized differential test against a naive per-pixel reference (`combine_naive`, temporary,
   not part of the final patch) before this ever reached the asset-corpus tests. Wrong pixels
   appeared specifically on the `-45` (rotated-pattern) power_jbig2 files.
2. **Fixed by recomputing fresh for the first *two* words, reusing from the 3rd word onward.**
   Passed the 20,000-case differential test. Correctness-verified end to end, but callgrind showed
   `combine()`'s own instruction share *increased* on halftone-heavy files (see below) - the extra
   `anchored`/`Option` state-machine bookkeeping was pure overhead whenever a placement never
   reaches a 3rd word (i.e. spans <=2 destination words), which turned out to be common for
   halftone pattern cells.
3. **Final: short-span (<=2 words) fast path + reuse only for 3rd+ words (this patch).** Placements
   spanning at most 2 destination words use a plain, always-fresh per-word loop (functionally
   identical to the original algorithm, just with the hoisted source-row slice instead of
   `get_word()` calls); the reuse state machine only runs for placements that actually reach a 3rd
   word, where it has something to amortize against.

### Instruction counts (`valgrind --tool=callgrind`, deterministic `Ir`, `decode_only` example -
NullDecoder, no PNG encode, 2 decode iterations - 1 warmup + 1 timed - per invocation)

| File (workload) | Variant | `combine()` Ir | `combine()` share | Total Ir |
|---|---|---|---|---|
| `042_11.jb2` (text/symbol-heavy) | baseline | 6,448,700 *(1 iter)* | 12.53% | 51,471,913 *(1 iter)* |
| `042_11.jb2` | final (this patch) | 12,295,416 *(2 iter)* | 12.04% | 102,089,204 *(2 iter)* |
| `042_11.jb2` | baseline (2 iter, matched) | 12,897,400 | 12.56% | 102,691,623 |
| `600-6-45.jb2` (halftone/pattern-heavy) | baseline (1 iter) | 118,223,543 | 4.90% | 2,411,082,394 |
| `600-6-45.jb2` | variant 2 (reuse-from-2nd-word, 1 iter) | 166,004,559 | 6.75% | 2,458,865,192 |
| `600-6-45.jb2` | final (this patch, 2 iter matched) | 255,980,216 | 5.29% | 4,842,831,938 |
| `600-6-45.jb2` | baseline (2 iter, matched) | 236,447,086 | 4.90% | 4,823,298,701 |

### Wall-clock (`decode_only` example, `Instant`-based p50 over N iterations after a 10% warmup
discard, paired alternating baseline/patched runs per MEASUREMENT.md, 9 pairs each; single-core =
`taskset -c 3`, multi-core = unpinned)

| File | single-core median baseline | single-core median patched | delta | wins (patched faster / 9) | multi-core delta |
|---|---|---|---|---|---|
| `042_11.jb2` (symbol-heavy) | 2.876ms | 2.803ms | **-2.54%** | 9/9 | -1.90% (9/9) |
| `042_17.jb2` (symbol-heavy) | 6.621ms | 6.632ms | +0.17% (noise) | 2/9 | not measured |
| `200-5-45.jb2` (halftone) | 17.137ms | 17.163ms | +0.15% (noise) | 3/9 | not measured |
| `600-6-45.jb2` (halftone) | 125.891ms | 126.156ms | +0.21% (noise) | 3/9 | +0.41% (noise, 2/9) |

## Result

Consistent, clean (9/9 paired-run) wall-clock win on the text/symbol-region workload
(`042_11.jb2`, `042_17.jb2` - JBIG2 symbol dictionaries whose glyph placements commonly span 3+
destination words): **-2.5% single-core / -1.9% multi-core** on `042_11.jb2`; `042_17.jb2` is
smaller and within this session's noise floor (2/9). No measurable change - within noise both
directions, 2-3 wins out of 9 - on the two halftone/pattern-heavy files tested (`200-5-45.jb2`,
`600-6-45.jb2`), where individual `combine()` calls are one destination word wide most of the
time and so never reach the reuse fast path at all. Net: a real win on the workload the
maintainers' own comment called out (`text.rs`'s symbol placement), no regression on the other
workload the same comment called out (`halftone.rs`'s pattern placement) once the short-span fast
path was added - the intermediate "reuse from the 2nd word" variant (dropped, not in `patch.diff`)
*did* regress the halftone files by ~1.5-2% and is documented above as a dead end, not silently
discarded.

## Does it change behavior/output?

- **Bit-exact?** Yes. `cargo test --release -p hayro-jbig2 --test asset_suite` (full corpus, both
  manifests - 65 `power_jbig2` + 101 `serenity` assets with `render: true`, snapshot-diffed against
  PNG references generated from this session's own unpatched baseline run) - **166 passed, 0
  failed** both before and after the patch, byte-for-byte (the harness's own pixel-diff check
  reports 0 differing pixels on every asset). Confirmed twice: once during development (worktree
  session, `power_jbig2` only - 65/65 both ways) and again end-to-end via this candidate's own
  `justfile` (`just clone && just asset-suite-baseline && just apply && just asset-suite-patched`)
  against a fresh clone that also downloaded `serenity` - 166/166 both ways. A 20,000-iteration
  randomized differential test (`combine()` vs. a naive per-pixel reference implementation,
  covering random destination/source sizes, positions including negative x/y and out-of-bounds
  overlap, and all 5 `CombinationOperator` variants) also passed on the final patch - this test was
  written and used to catch and fix the bug described above, then removed from `bitmap.rs` before
  capturing `patch.diff` (per this skill's own rule against adding permanent unit tests to the
  tested project; it is preserved as `combine_diff_test.patch` in this candidate's own folder,
  applied on top of `patch.diff` by the `just correctness-check` recipe, so it can be re-run).
- `cargo test --release -p hayro-jbig2 --lib`: 3 passed, 0 failed (2 pre-existing segment-header
  tests + the temporary differential test, both before capturing the final patch and after removing
  it).
- **Is this a breaking change?** No - internal, private (`pub(crate)`) function, same signature,
  same observable output on every input tested.

## Why the result looks the way it does

`combine()`'s per-word loop recomputes a division and modulo (`src_x_for_range / WORD_BITS`,
`% WORD_BITS`) and makes two bounds-checked `get_word()` calls on every iteration, even though - as
proven by construction and confirmed by the 20,000-case differential test - the source bit offset
is constant and the source word index advances by exactly one word per destination word for every
word after the *second* one in a placement. The maintainers' comment about this being a
"bottleneck for symbols/patterns" holds most clearly for the *symbols* half: JBIG2 text regions
place variable-width glyph bitmaps that, for real scanned-text documents, often span several
32-bit destination words, so removing redundant loads/divisions from that inner loop pays off.
Pattern placements in halftone regions are typically narrow (commonly <=1-2 destination words per
row for the pattern cell sizes seen in the `power_jbig2` corpus), so the extra state (`anchored`
flag, `Option`-wrapped carried word) needed to know *when* it's safe to reuse a load pays for
itself only on wider placements - hence the short-span fast path in the final patch, and the
still-flat (within-noise) result on halftone-heavy files rather than an additional win there.

## What's not done / open questions

- Not benchmarked: JBIG2 files exercising `lib.rs`'s three other `combine()` call sites (page
  compositing of generic/refinement regions) - only the `text.rs` (symbol) and `halftone.rs`
  (pattern) call sites were targeted, per the maintainers' own comment.
- A SIMD rewrite (the repo already has a `fearless_simd`-backed `u32x8` abstraction in
  `hayro-jbig2/src/simd.rs`, currently only wired into `gray_scale.rs`) was considered but not
  attempted - bit-level shift-and-merge across word boundaries doesn't map cleanly onto that
  abstraction's current API (integer comparison/select, not arbitrary bit shifts), and the
  narrow/short-span common case found here would likely see little benefit from it regardless.
