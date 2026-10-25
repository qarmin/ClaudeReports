# Replace `f32::round` (libm `roundf` call) with a cast-based bit-identical round in `GuiRounding` and `PointScale`

`roundf` showed up as a separate libm symbol at 2.4% (`demo_no_tessellate`), 2.2% (`text_layout_uncached`) and 7.8% (`text_layout_cached_many_lines_modified`) self time in `perf`. LLVM cannot lower ties-away `round` to one baseline x86-64 instruction, so every `round_ui`/`round_to_pixels` is a call. Tried a branch-light replacement using `x as i32 as f32` (exact for |x| < 2^23).

rustc 1.95.0 (59807616e 2026-04-14), base commit 86099aca1. Metric: wall-clock time only.

## What was tried

Interleaved A/B of prebuilt criterion binaries (`tools/ab.sh`, pinned core, 3 rounds). Ratios patched/base (ab.sh absolute values are on a doubled scale, only ratios matter):

| Benchmark | round 1 | round 2 | round 3 |
|---|---|---|---|
| demo_with_tessellate__realistic | 0.9932 | 0.9910 | 1.0082 |
| demo_no_tessellate | 0.9879 | 0.9890 | 1.0070 |
| text_layout_uncached | 0.9526 | 0.9888 | 1.0074 |
| text_layout_cached_many_lines_modified | 0.9995 | 0.9987 | 1.0041 |

## Result

Within noise: ratios straddle 1.0 (-1.2% .. +0.8%, no consistent direction across rounds), including the benchmark where `roundf` was 7.8% of the profile. Time only.

## Does it change behavior/output?

- **Bit-exact?** Yes: `exhaustive_check.rs` (in this folder; `rustc -O exhaustive_check.rs && ./exhaustive_check`) compares the new function with `f32::round` over all 2^32 f32 bit patterns - 0 mismatches (NaN payloads excluded from comparison).
- Breaking change: no.
- Tests: `cargo test -p emath -p epaint --all-features`: emath 13 + 19, epaint 95 + 4 passed (1 ignored) after; same counts on the unpatched tree (earlier in session: epaint 95 + 4).
- Real change to the tested project's source: `patch.diff` (+25/-4).

## Why the result looks the way it does

glibc's `roundf` is cheap (a few integer ops on the bit pattern), so the call overhead recovered by the cast-based version is about equal to the extra float compare/select work; perf attributes the time to `roundf` simply because it is an out-of-line symbol, not because it is slow per call.

## What's not done / open questions

- `floorf`/`ceilf` were not tried (same shape); expected to behave the same way.
- With `-C target-cpu=x86-64-v2` or newer, `floor`/`trunc` inline as `roundss`; `round` still does not.
