# `Vec2::length` as an inlineable f64 square root instead of a `hypotf` libm call

`Vec2::length` (used by `normalized()` for every path point normal in `Path::add_line_loop` / `add_open_points`, and elsewhere) called `f32::hypot`, an out-of-line libm call (`hypotf` was 1.9% self in the widget-heavy `scene_widget_mix_100` profile, and the call blocks inlining/vectorization of the surrounding loops). Replaced by `((x as f64)^2 + (y as f64)^2).sqrt() as f32`: the squares of two f32 are exact in f64 and the sum of them is rounded once, then once more when narrowing.

rustc 1.95.0 (59807616e 2026-04-14), base commit 86099aca1, glibc libm. Metric: wall-clock time.

## What was tried

Interleaved A/B of prebuilt binaries (`tools/ab.sh`, pinned core). Ratios patched/base:

| Benchmark | ratios |
|---|---|
| demo_only_tessellate (4 rounds) | 0.9628, 0.9740, 0.9762, 0.9773 |
| demo_with_tessellate__realistic | 1.0044, 0.9751, 0.9769, 0.9898 |
| scene_buttons_300 (3 rounds) | 0.9900, 0.9690, 0.9721 |
| scene_widget_mix_100 | 0.9713, 0.9801, 0.9825 |

## Result

Time: about -2% to -3% on tessellation of the demo and on widget-heavy scenes; the realistic full frame is about -1..2% (one round noisy).

## Does it change behavior/output?

- **Bit-exact?** Equal to this platform's `f32::hypot` in every sampled case: `hypot_equivalence_check.rs` (this folder) compared 400,000,000 pairs (raw random bit patterns, small/large magnitudes, grid-aligned coordinates typical for UI) with 0 mismatches. It is not a proof: double rounding could in theory differ from a correctly rounded `hypotf` in rare cases, and other platforms' libm (musl, macOS, wasm) may differ from glibc in the last bit for `f32::hypot` itself, so cross-platform results were never guaranteed to be bit-identical.
- Breaking change: no.
- Tests: `cargo test -p emath -p epaint -p egui --all-features`: emath 13 + 19, epaint 95 + 4 (1 ignored), egui 93 + 3 + 188 (all-features run; the default-feature egui run was 86 + 3 + 188 on the base) passed, 0 failed.
- Real change to the tested project's source: `patch.diff` (+3/-1).

## Why the result looks the way it does

Each rounded rectangle path has 12-20 points and each point's normal needs 2 `length()` calls; removing the libm call overhead per point shows up as a couple of percent of tessellation.

## What's not done / open questions

- `Rot2::length`, bezier `hypot` calls and `Pos2::distance` use the same pattern; not changed here.
