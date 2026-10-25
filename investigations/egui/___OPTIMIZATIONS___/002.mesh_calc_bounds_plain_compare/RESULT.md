# `Mesh::calc_bounds` with plain comparisons instead of `Rect::extend_with` (f32::min/max)

`tessellate_row` calls `Mesh::calc_bounds` for every text row after glyph quads are built; perf annotate of `text_layout::layout` (calc_bounds inlined) showed `f32::min/max` NaN-handling sequences (`cmpunordss`/`andps`/`orps`, src lines f32.rs:992/1019) among the hottest lines.

rustc 1.95.0 (59807616e 2026-04-14), base commit 86099aca1. Metric: wall-clock time only.

## What was tried

Interleaved A/B of two prebuilt criterion binaries (`___OPTIMIZATIONS___/tools/ab.sh`, pinned `taskset -c 4`, 4 rounds, 3 s measurement each; load average was ~4-7 during this session, so absolute numbers are not comparable to the other candidates; the printed absolute values in ab.sh output are on a different scale, only the ratio is meaningful).

| Variant | text_layout_uncached B/A ratio per round | tessellate_text B/A |
|---|---|---|
| baseline | 1.0 | 1.0 |
| plain-comparison calc_bounds | 0.9685, 0.9757, 0.9799, 0.9814 | 1.0028, 0.9994, 1.0032, 1.0034 |

## Result

Time: about -2.0% to -3.1% on `text_layout_uncached`, 4/4 rounds faster. `tessellate_text` (does not call `calc_bounds`) unchanged, as expected. A first attempt using criterion's own `--baseline` comparison was unusable (the saved baseline moved between runs under load); the interleaved A/B replaced it.

## Does it change behavior/output?

- **Bit-exact?** Yes except for one theoretical case: ties between `0.0` and `-0.0` (`f32::min` may return either; the new code keeps the first-seen value). NaN vertex positions are skipped in both versions. Bounds of an empty mesh stay `Rect::NOTHING` (INF, -INF).
- Breaking change: no.
- Tests: `cargo test -p epaint --all-features` 95 + 4 passed before and after; `cargo test -p egui` 86 + 3 + 188 passed after (unpatched baseline: identical counts).
- Real fix to the tested project's source: `patch.diff`.

## Why the result looks the way it does

The `f32::min/max` implementation must propagate the non-NaN operand, which costs ~5 instructions per call; `if p < m {p} else {m}` has the same NaN semantics and maps to one `minss`/`maxss`. Gain is small because bounds are one of many per-glyph costs in layout.

## What's not done / open questions

- `Rect::extend_with` / `Pos2::min/max` in emath have the same pattern and are used elsewhere (e.g. tessellator paths); only `Mesh::calc_bounds` was changed here.
