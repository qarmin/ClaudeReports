# UV-colored stroke bounding box without the temporary `Vec<Pos2>` (`stroke_and_fill_path`)

For gradient (`ColorMode::UV`) strokes, `stroke_and_fill_path` collected every path point's position into a temporary `Vec<Pos2>` just to call `Rect::from_points`. Replaced by a direct `extend_with` loop over the path points (identical arithmetic, no allocation).

rustc 1.95.0 (59807616e 2026-04-14), base commit 86099aca1. Metric: wall-clock time only (the saved allocation was not measured separately).

## What was tried

Epaint criterion benches (`thick_uv_line`, `thick_large_uv_line` (1000 points), `thin_uv_line`, `thin_large_uv_line`), interleaved A/B of prebuilt binaries (`tools/ab.sh`, pinned core). Ratios patched/base:

| Benchmark | ratios per round |
|---|---|
| thick_uv_line (3 points) | 0.9845, 0.9035, 0.9634 |
| thin_uv_line (3 points) | 0.9161, 0.9290, 0.8412 |
| thick_large_uv_line (1000 points) | 1.0080, 1.0084, 1.0110 |
| thin_large_uv_line (1000 points) | 1.0054, 1.0862, 1.0803, 1.0817; with A/B order swapped: 1.0869, 1.0866, 1.0805 (inverse of 0.9201, 0.9202, 0.9255) |

## Result

Mixed. Small paths (3 points) get faster, about -2% to -16% (noisy, absolute times are ~70 ns). The 1000-point thin UV stroke is consistently about +8% slower with the patch (verified with the binaries' order swapped, so not an ordering artifact); the thick 1000-point one is about +1%. Likely cause (not verified, no disassembly was compared): `Rect::from_points` over a contiguous `&[Pos2]` auto-vectorizes better than the same loop striding over 16-byte `PathPoint`s, so removing the allocation costs more than it saves once the path is long.

## Does it change behavior/output?

- **Bit-exact?** Yes: same `extend_with` sequence over the same positions in the same order.
- Breaking change: no.
- Tests: `cargo test -p epaint --all-features` 95 + 4 passed (1 ignored) after; same before.
- Real change to the tested project's source: `patch.diff` (+5/-2).

## Why the result looks the way it does

One allocation + collect per UV stroke is a fixed ~tens of ns cost, which dominates for 3-point paths and is noise for 1000-point ones; the per-point loop codegen difference then decides the long-path result.

## What's not done / open questions

- Computing the bounds with a vectorization-friendly loop over `pos` (e.g. separate min/max accumulators like the `Mesh::calc_bounds` change in candidate 002) was not tried here.
- UV-colored strokes are rare in practice (gradients), so neither direction is likely to matter outside the benchmark.
