# `Color32::hash` as one `u32` write (instead of `[u8; 4]` length prefix + byte-slice write)

`Color32` derived `Hash` over its `[u8; 4]`, which hashes as a slice: a `usize` length write plus a 4-byte `write(&[u8])` (ahash's slow small-slice path). A `TextFormat` hashes up to four colors (color, background, underline, strikethrough), and every `LayoutJob` goes through that hash on each label layout, cached or not.

rustc 1.95.0 (59807616e 2026-04-14), base commit 86099aca1. Metric: wall-clock time only.

## What was tried

Interleaved A/B of prebuilt criterion binaries (`tools/ab.sh`, pinned core). Ratios patched/base (ab.sh absolute values are doubled; only ratios matter):

| Benchmark | ratios per round |
|---|---|
| text_layout_cached (6 extra rounds) | 0.9705, 0.9719, 0.9806, 0.9693, 0.9780, 0.9682 |
| text_layout_cached (first 4-round run) | 0.9712, 0.9698, 0.9727, 0.9974 |
| demo_no_tessellate | 0.9730, 1.0012, 1.0026, 1.0070 |
| demo_with_tessellate__realistic | 0.9757, 1.0081, 1.0072, 1.0131 |
| text_layout_uncached | 1.0074, 0.9859, 0.9993, 1.0053 |

## Result

Time: `text_layout_cached` (one cache-hit layout, 160 ns) about -2% to -3.2%, 9 of 10 rounds faster (about 5 ns saved per layout call). Whole-UI benchmarks (`demo_*`) and uncached layout are within noise, i.e. the saving is real but below the frame-level noise floor of this machine (a few %).

## Does it change behavior/output?

- **Bit-exact rendering/layout?** Yes. The only observable difference is the numeric value of `Hash` output for `Color32` (used by `HashMap`s, the galley cache key, and `Id::new(color)`): hashes of colors change between versions. Nothing persists these hashes besides `Id`s derived from a `Color32`, which are not stable across versions anyway.
- Breaking change: no API change; any code that stored an `Id::new(some_color32)` in persisted memory would lose that entry once.
- Tests: `cargo test -p ecolor -p epaint --all-features`: 13 (+1 ignored) + 95 + 4 + 2 + 4 passed; `cargo test -p egui`: 86 + 3 + 188 passed; unpatched counts identical.
- Real change to the tested project's source: `patch.diff` (+9/-1).

## Why the result looks the way it does

Per-label cost of the job hash is ~25 small ahash writes; replacing 8 slow-path writes (4 colors x 2) by 4 fast `write_u32` removes only a handful of ns per layout call - visible in the 160 ns cache-hit microbenchmark but under 0.1% of the ~300 us demo frame.

## What's not done / open questions

- The earlier perf attribution `core::hash::Hash::hash_slice` ~4% of `demo_no_tessellate` is not fully explained by colors; remaining slice hashing is the text bytes and `sections`. Packing more `TextFormat` fields into fewer writes was not tried.
