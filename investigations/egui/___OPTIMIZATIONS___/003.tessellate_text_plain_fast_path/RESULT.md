# `Tessellator::tessellate_text`: branch-free per-vertex loop for the common (no override color / color glyph / opacity / rotation) case

perf of `demo_only_tessellate` put 33% in the `Map::fold` that transforms each glyph vertex in `tessellate_text` (src lines in the closure at tessellator.rs:2297-2350). The closure re-tests `override_text_color`, `glyph_vertex_range.contains`, `is_color_glyph_vertex`, `opacity_factor < 1.0` and `angle == 0.0` for every vertex although all but the color placeholder check are invariant per row.

This matches open upstream issue emilk/egui#8731 ("Text tessellation ~60% slower in 0.37", reporter measured +22 instructions/vertex from the color-glyph conditional inside the closure and proposes the same per-row split). rustc 1.95.0 (59807616e 2026-04-14), base commit 86099aca1. Metric: wall-clock time only.

## What was tried

Interleaved A/B of two prebuilt criterion binaries (`___OPTIMIZATIONS___/tools/ab.sh`, pinned `taskset -c 4`, 4 rounds, 3 s each; ab.sh's printed absolute values are ~2x the real criterion numbers because of its unit handling, only the ratio is meaningful). Load average 4-7 during this session.

| Variant | demo_only_tessellate B/A per round | tessellate_text B/A per round |
|---|---|---|
| baseline | 1.0 | 1.0 |
| fast path when `override_text_color.is_none() && !row_has_color_glyphs && opacity_factor >= 1.0 && angle == 0.0` | 0.8231, 0.7774, 0.8248, 0.8231 | 0.5374, 0.5348, 0.5319, 0.5325 |

Absolute (criterion, from the baseline run earlier in the session): `tessellate_text` 10.48 us -> about 5.6 us; `demo_only_tessellate` 63.8 us -> about 52 us.

## Result

`just test` from this folder was run end to end (clone, baseline bench, apply, patched bench; clone pointed at the local checkout via `--set repo_url`): `tessellate_text` -47.9%, `demo_only_tessellate` -17.5% (criterion's own comparison; the absolute demo_only_tessellate time in that run, ~12 us, differs from the ~64 us of the in-session benchmark run because only the filtered benchmarks ran in a fresh process, so the scene was smaller).

Time: `tessellate_text` about -46%, `demo_only_tessellate` (egui demo's whole tessellation) about -18% to -22%, 4/4 rounds, tight spread. Other paths (override color, color emoji rows, fading, rotated text) still use the original closure, unchanged.

## Does it change behavior/output?

- **Bit-exact?** Yes by construction: the fast path executes the same arithmetic as the general closure restricted to the case where the skipped branches are all false (`final_row_pos + pos.to_vec2()`, `(uv.to_vec2() * uv_normalizer).to_pos2()`, placeholder -> fallback color).
- Breaking change: no.
- Tests: `cargo test -p epaint --all-features` 95 + 4 passed before and after; `cargo test -p egui` 86 + 3 + 188 passed after; `cargo test -p egui_demo_lib`: 11 passed / 2 failed (`demo::demo_app_windows::tests::demos_should_match_snapshot`, `rendering_test::tests::rendering_test`) both before and after - identical failing set, pre-existing in this environment.
- Real fix to the tested project's source: `patch.diff`.

## Why the result looks the way it does

Text is the bulk of the vertices in a typical UI frame, and the original closure's invariant branches defeat unswitching/vectorization (it also made `Vec::extend` go through a non-trivial `fold`). Removing them lets LLVM generate a straight copy/transform loop.

## What's not done / open questions

- The patch deliberately does not re-indent the original (unchanged) general path, to keep the diff small - rustfmt cannot format that statement anyway because of the long `debug_assert!` message line, so a maintainer may prefer restructuring the whole function.
- The general path could be improved the same way per condition (e.g. hoist `angle == 0.0`), not done here.
