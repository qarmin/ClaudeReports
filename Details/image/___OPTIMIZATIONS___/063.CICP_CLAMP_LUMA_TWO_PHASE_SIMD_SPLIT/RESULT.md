# `CicpTransform::clamp_rgb_luma`/`clamp_rgba_luma`: split the per-pixel reduction from the clamp/cast into two loops over a scratch buffer

`cicp::CicpRgb::cast_pixels_by_layout`'s color-transform fallback path (used for any conversion
that isn't a same-layout identity cast, e.g. `Rgb8->Luma8`, `Rgba8->LumaA16`) was measured at
93-99% of self-time of the `convert` benches that hit it (candidate 059, `IDEAS.md` row for
2026-08-29). That row explicitly named the `STEP=256`-batched loop/SIMD shape of
`CicpTransform::expand_rgb`/`clamp_rgb_luma` as the open, untried angle (candidate 059 had already
ruled out `#[inline]` annotations as the fix - everything in this path is already fully inlined).
This candidate follows that lead for `clamp_rgb_luma`/`clamp_rgba_luma` specifically: both fuse a
per-pixel horizontal reduction (`rgb`/`rgba` -> `luma`, a dot product against 3 coefficients) with
a per-pixel elementwise clamp+saturating-cast (`f32` -> `P`) in one loop body.

## What was tried

Base commit: `44ce9226c541dd2b11897b9ef07156a28871cdb7`. `perf record -F 999 --call-graph dwarf`
(`taskset -c 2`) against the existing `benches/convert.rs` `cast_dynamic_rgba8_l8` case (`Rgb8 ->
Luma8`, exercises `clamp_rgb_luma<u8>`) confirmed candidate 059's own finding still holds: 64.85%
of the whole benchmark process's cycles are inside `cast_pixels_by_layout::<u8, u8>` (`perf
annotate` full report: `perf_reports.txt`). Baseline disassembly of the hot loop
(`objdump_baseline_clamp_rgb_luma.txt`) shows the reduction is already SLP-vectorized by LLVM (4
pixels' worth of `r*cr+g*cg+b*cb` computed via `unpcklps`/`movlhps`/`mulps`/`addps` on packed
`xmm` registers), but the immediately-following clamp+saturating-cast+byte-pack step is fully
scalar: each of the 4 lanes is extracted one at a time (`shufps`/`unpckhpd`), clamped
(`xorps`/`maxss`; `movaps`+`minss`), converted (`cvttss2si`, scalar), zero-extended, shifted, and
`or`-ed together into one `u32` store - roughly 9 scalar instructions per lane, dwarfing the ~20
vector instructions of the reduction itself.

The change: split each function's single fused loop into two loops over a `[f32; 256]` stack
scratch buffer - first loop computes all `luma` values for a chunk (up to 256, matching the
`STEP`/`N` constant every call site already batches by) and stores them into the buffer; second
loop reads the buffer and does only the clamp/cast/store, with nothing else mixed into that loop
body. Applied identically to both `clamp_rgb_luma` (full patch: `patch.diff`) and
`clamp_rgba_luma` (same shape, plus the pass-through alpha channel copied from the original
`rgba_chunk` slice in the second loop).

Compared prebuilt `--release` `convert` bench binaries (`convert_baseline`, `convert_patched`) via
`paired_convert.sh` (adapted from candidate 060's `paired_overlay.sh` - same alternating-runs
shape, `--warm-up-time 1 --measurement-time 1.2`), 8-12 pairs per case/mode. A same-binary
self-check (`convert_baseline` vs. an independently-recopied instance of itself) established the
noise floor first.

| Bench case (function exercised) | Variant | single-core p50 | multi-core p50 | notes |
|---|---|---|---|---|
| `cast_dynamic_rgba8_l8` (`clamp_rgb_luma<u8>`) | noise floor: baseline vs. baseline (same binary) | 0.0719 -> 0.0715 ms (-0.6%, 7/10 pairs) | not separately re-checked | md5-identical; establishes the noise floor for this bench |
| `cast_dynamic_rgba8_l8` (`clamp_rgb_luma<u8>`) | baseline vs. patched | 0.0715 -> 0.0711 ms (-0.5%, 9/12 pairs) | 0.0712 -> 0.0705 ms (-1.0%, 9/10 pairs) | both deltas are within/at the noise floor measured above - not a real, distinguishable change |
| `cast_dynamic_rgba8_la16` (`clamp_rgba_luma<u16>`) | baseline vs. patched | 0.1659 -> 0.2078 ms (**+25.3%**, 0/10 pairs favor patch) | 0.1683 -> 0.2066 ms (**+22.8%**, 0/8 pairs favor patch) | decisive, consistent regression - every single paired run favors the unpatched baseline |

Full raw paired-run output: `single_l8.txt`, `noise_l8.txt`, `multi_l8.txt`, `single_la16.txt`,
`multi_la16.txt`. Full `perf`/`objdump` evidence: `perf_reports.txt`,
`objdump_baseline_clamp_rgb_luma.txt`, `objdump_patched_clamp_rgb_luma.txt`.

## Result

**Mixed - no measurable change for the `Rgb->Luma` path (`clamp_rgb_luma`), a decisive ~23-25%
regression for the `Rgba->LumaAlpha` path (`clamp_rgba_luma`).** Time was the only dimension
measured; the mechanism (loop restructuring, no allocation involved - the scratch buffer is a
fixed-size stack array) has no plausible effect on allocation count or peak RSS, so none was taken.

- `clamp_rgb_luma<u8>` (`cast_dynamic_rgba8_l8`): the disassembly does show a real codegen change
  (the "multiply by 255, add 0.5" step, previously scalar-per-lane, is now a single packed
  `mulps`+`addps` across 4 lanes at once - see `objdump_patched_clamp_rgb_luma.txt`), but the
  saturating float->u8 cast itself (`maxss`/`minss`/`cvttss2si` per lane) and the final byte-pack
  (`movzx`+`shl`+`or`) remain scalar either way, so the change is not large enough to clear this
  bench's own noise floor (measured at ~0.6-1% via the same-binary self-check).
- `clamp_rgba_luma<u16>` (`cast_dynamic_rgba8_la16`): a real, large, reproducible slowdown. Not
  root-caused down to the exact instruction (see "Why the result looks the way it does" below for
  the leading hypothesis) - out of scope for this candidate's bounded budget once the regression
  was confirmed decisive and consistent across both single- and multi-core, several pair counts.

## Does it change behavior/output?

- **Bit-exact?** Yes, as far as this session's checks go. The crate's own unit tests in
  `src/metadata/cicp.rs` (`transform_pixels_srgb`, `transform_pixels_srgb_16`,
  `transform_pixels_srgb_luma_alpha`) exercise `cast_pixels` through exactly the two functions
  this patch touches (`Rgba<u8>->Luma<u8>`, `Luma<u16>->Luma<u8>`, `LumaA<u16>->LumaA<u8>`, etc.)
  against concrete expected numeric output, and pass identically before and after the patch (see
  test-suite counts below) - no new float-rounding/rebalancing was introduced by moving the same
  arithmetic across two loops instead of one.
- **Is this a breaking change?** No - no public API, signature, or documented behavior changed;
  both functions are `pub(crate)`, only their internal loop structure changed.
- Project test suite (`cargo test --release`): **550 passed / 1 failed, identically before and
  after** (`test_baseline.txt`, `test_patched.txt`). The one failure is the same pre-existing,
  unrelated `tests/regression.rs::resizing_with_catmul` every prior BMP/`pic-scale-safe`-adjacent
  candidate in this workspace has already documented, confirmed independently on both this
  session's own baseline and patched builds.

## Why the result looks the way it does

The reduction (`rgb`/`rgba` dot product) and the saturating float-to-int cast are two operations
LLVM's auto-vectorizer treats very differently: the reduction is ordinary FP arithmetic (freely
vectorizable via SLP once decoupled from the store), but `f32 as u8`/`f32 as u16` in Rust lowers
to a *saturating* conversion (NaN -> 0, out-of-range clamped, then truncated) that does not have an
efficient packed lowering on this target for narrow integer widths - LLVM keeps it scalar
(`cvttss2si` + explicit `min`/`max` per lane) regardless of how the surrounding loop is shaped.
Splitting the loop only ever had a chance to help the *arithmetic before* the cast (confirmed: the
`* 255 + 0.5` step did become packed for the `u8` case) - it could not touch the cast itself, which
is why `clamp_rgb_luma`'s own bench case moved by an amount indistinguishable from noise.

The `clamp_rgba_luma<u16>` regression is a real, different effect from the `clamp_rgb_luma<u8>`
non-change, and was not root-caused within this candidate's scope. The most likely mechanism,
un-confirmed: the rewritten `clamp_rgba_luma` reads `rgba_chunk` (the 4-wide input buffer) twice -
once in the reduction loop (channels 0-2, to compute `luma`) and again in the second loop (channel
3, for the pass-through alpha) - where the original single-loop version read each `rgba` element
exactly once. For the `u16` output type in particular (not `u8`), this doubled traversal of a
256-`f32`-wide (1KiB) working set, combined with reading back the freshly-written 256-`f32`
`luma_buf` scratch array from L1, plausibly costs more than whatever vectorization benefit the
split bought - especially since (per the previous paragraph) the split bought *no* benefit for the
cast itself, only for the pre-cast arithmetic. This is a plausible explanation, not a confirmed
one; a definitive answer would need a second disassembly/`perf annotate` pass specifically on the
`<u8, u16>` (or `<u8, u16>`-shaped) monomorphization of `cast_pixels_by_layout`, which this
candidate's bounded time did not reach once the regression was already decisively confirmed via
paired runs (10/10 and 8/8 unanimous).

## What's not done / open questions

- The `clamp_rgba_luma<u16>` regression's exact mechanism (double-read of the input chunk vs.
  scratch-buffer round-trip vs. something else in that specific monomorphization's register
  allocation) was not disassembled/root-caused - see above. A future session could `perf annotate`
  the patched `cast_pixels_by_layout::<u8, u16>` symbol directly to settle this.
- The underlying limitation this candidate ran into - LLVM not vectorizing Rust's saturating
  `f32 as u8`/`f32 as u16` cast even when the surrounding arithmetic is packed - was not worked
  around. Doing so would need explicit `unsafe` SIMD intrinsics (e.g. `_mm_cvttps_epi32` +
  `_mm_packs_epi32`/`_mm_packus_epi16` with an explicit `_mm_min_ps`/`_mm_max_ps` clamp beforehand,
  x86-only, needing a scalar fallback for other architectures) - a substantially larger, riskier,
  architecture-specific change than this candidate's scope, and not attempted here. Added to
  `IDEAS.md` as its own follow-up, explicitly flagged as `unsafe`/non-portable so a future session
  goes in with the right expectations.
- Only `u8`- and `u16`-output monomorphizations were benched (the two cases `benches/convert.rs`
  already covers for this path); the `f32`-output monomorphization (`clamp_from_f32` for `f32` is
  just `val` - no cast at all) was not separately checked but is expected to be unaffected either
  way since there is no saturating-cast bottleneck for it to hit.
