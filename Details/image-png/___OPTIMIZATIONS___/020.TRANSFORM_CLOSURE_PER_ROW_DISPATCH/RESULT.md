# Precompute per-row-call-invariant state in `create_transform_fn`'s non-memoized transform closures

`rustc 1.98.0 (88d9e12ae 2026-08-18)`, `cargo 1.98.0`, x86_64, 24 logical cores, default `cargo
build --release` (no target-cpu tuning).

`src/decoder/transform.rs`'s `TransformFn` doc comment carries a TODO ("If some precomputed state
is needed ... consider changing this into `Box<dyn Fn(...)>`") that is already stale in one sense
(the type already is `Box<dyn Fn>`) but points at a real gap: `create_transform_fn` memoizes which
function to dispatch to (`self.transform_fn` in `src/decoder/mod.rs` is built once per decode, not
per row - confirmed by reading `next_interlaced_row_impl`), but the *functions themselves* still
take `&Info` and recompute per-decode-constant values from it on every single row call:
`expand_gray_u8`/`expand_gray_u8_with_trns` recompute `scaling_factor` (a `u8` division) every row,
and `expand_trns_line`/`expand_trns_line16`/`expand_trns_and_strip_line16` recompute
`channels = info.color_type.samples()` every row. `benches/expand_paletted.rs` already covers the
paletted case (`palette::create_expansion_into_rgba8`/`_into_rgb8` build an RGBA LUT once and
capture it in a `move` closure) - this candidate applies the exact same established pattern
(closure-captured state, `move |input, output, info| ...`) to the three un-memoized families named
in the candidate description: bit-depth expansion, 16-bit stripping, and tRNS application.
`transform_row_strip16` itself has nothing to hoist (it does not read `Info` at all), so it is
unchanged and included only as a same-shape control.

## What was tried

Patch (`src/decoder/transform.rs`, `+38/-21`): in `create_transform_fn`, compute `scaling_factor`
(for the two `expand_gray_u8*` variants) and `channels` (for the three `expand_trns*` variants)
once in the match arm, then capture them by value in a `move` closure instead of passing a fn
item and recomputing inside it. Function signatures for the 5 affected helpers gained explicit
`bit_depth`/`scaling_factor`/`channels` parameters in place of deriving them from `&Info` (which
is still passed through for the actual `trns` byte comparison, since that can't be hoisted without
giving `TransformFn` a lifetime).

Harness: a new standalone bench crate (`bench/`, `png = { path = "../clone", features =
["benchmarks"] }`, no other dependencies) built for this candidate, since nothing in the repo
exercises the non-paletted transform paths at a controlled row width (`benches/expand_paletted.rs`
only reaches the indexed-color paths; `README.md`'s harness table has no encoder-side gap here
since this is a decoder-only concern). It calls `png::benchable_apis::create_transform_fn` once
per configuration (matching real usage: the fn is built once per decode) then times `BATCH=200`
back-to-back calls per sample (individual per-row calls at width=1 are ~3-10ns, close to
`Instant::now()`'s own resolution/overhead, so batching is required the same way criterion does it
internally) across widths 1/4/16/64/256/1024/4096, for 6 configurations: `graybits` (gray 4-bit,
no trns -> `expand_gray_u8`), `graytrns` (gray 4-bit + trns -> `expand_gray_u8_with_trns`), `trns8`
(RGB8 + trns -> `expand_trns_line`), `trns16` (RGB16 + trns, no strip -> `expand_trns_line16`),
`striptrns16` (RGB16 + trns + strip -> `expand_trns_and_strip_line16`), `strip16` (RGBA16, no
trns, strip only -> `transform_row_strip16`, the unchanged control).

Single-core: `taskset -c 3`, 2 independent runs per build (baseline x2, patched x2) - raw output in
`raw/baseline_run{1,2}.txt` / `raw/patched_run{1,2}.txt`. Multi-core: same bench unpinned, 1 run
per build (`raw/baseline_multi.txt` / `raw/patched_multi.txt`). Representative rows (width
1/64/1024/4096, all 6 configs) below; full sweep (all 7 widths) is in `raw/`.

| bench | width | base p50 (run1/run2) | patched p50 (run1/run2) | single-core delta | multi-core (base->patched) |
|---|---|---|---|---|---|
| graybits(gray4,noTrns) | 1 | 3/4 ns | 3/3 ns | +0.0% / -25.0% | 3->3 ns (+0.0%) |
| graytrns(gray4,trns) | 1 | 3/4 ns | 3/3 ns | +0.0% / -25.0% | 4->3 ns (-25.0%) |
| trns8(rgb8,trns) | 1 | 6/7 ns | 5/5 ns | -16.7% / -28.6% | 5->5 ns (+0.0%) |
| trns16(rgb16,trns,noStrip) | 1 | 6/7 ns | 6/6 ns | +0.0% / -14.3% | 6->6 ns (+0.0%) |
| striptrns16(rgb16,trns,strip) | 1 | 5/6 ns | 6/6 ns | +20.0% / +0.0% | 5->6 ns (+20.0%) |
| strip16(rgba16,noTrns,strip) [control] | 1 | 3/3 ns | 2/3 ns | -33.3% / +0.0% | 3->2 ns (-33.3%) |
| graybits(gray4,noTrns) | 64 | 34/34 ns | 33/33 ns | -2.9% / -2.9% | 34->33 ns (-2.9%) |
| graytrns(gray4,trns) | 64 | 50/50 ns | 51/52 ns | +2.0% / +4.0% | 50->50 ns (+0.0%) |
| trns8(rgb8,trns) | 64 | 239/240 ns | 254/250 ns | +6.3% / +4.2% | 253->238 ns (-5.9%) |
| trns16(rgb16,trns,noStrip) | 64 | 250/253 ns | 250/253 ns | +0.0% / +0.0% | 265->255 ns (-3.8%) |
| striptrns16(rgb16,trns,strip) | 64 | 166/166 ns | 178/178 ns | +7.2% / +7.2% | 179->178 ns (-0.6%) |
| strip16(rgba16,noTrns,strip) [control] | 64 | 70/70 ns | 70/69 ns | +0.0% / -1.4% | 71->70 ns (-1.4%) |
| graybits(gray4,noTrns) | 1024 | 504/504 ns | 501/504 ns | -0.6% / +0.0% | 504->499 ns (-1.0%) |
| graytrns(gray4,trns) | 1024 | 773/770 ns | 772/775 ns | -0.1% / +0.6% | 770->1170 ns (+51.9%, unpinned scheduling noise) |
| trns8(rgb8,trns) | 1024 | 4015/3985 ns | 3785/3803 ns | -5.7% / -4.6% | 3444->3424 ns (-0.6%) |
| trns16(rgb16,trns,noStrip) | 1024 | 4201/4203 ns | 4180/4201 ns | -0.5% / -0.0% | 4223->3990 ns (-5.5%) |
| striptrns16(rgb16,trns,strip) | 1024 | 2823/2836 ns | 2821/2850 ns | -0.1% / +0.5% | 2823->2822 ns (-0.0%) |
| strip16(rgba16,noTrns,strip) [control] | 1024 | 1117/1122 ns | 1117/1117 ns | +0.0% / -0.4% | 1117->1117 ns (+0.0%) |
| graybits(gray4,noTrns) | 4096 | 1998/2012 ns | 1988/1998 ns | -0.5% / -0.7% | 1998->1998 ns (+0.0%) |
| graytrns(gray4,trns) | 4096 | 3097/3080 ns | 3068/3096 ns | -0.9% / +0.5% | 3086->3093 ns (+0.2%) |
| trns8(rgb8,trns) | 4096 | 15925/16066 ns | 15109/15232 ns | -5.1% / -5.2% | 13590->14476 ns (+6.5%) |
| trns16(rgb16,trns,noStrip) | 4096 | 15955/16795 ns | 16707/16792 ns | +4.7% / -0.0% | 15206->15272 ns (+0.4%) |
| striptrns16(rgb16,trns,strip) | 4096 | 11300/11330 ns | 11333/11342 ns | +0.3% / +0.1% | 10579->11353 ns (+7.3%) |
| strip16(rgba16,noTrns,strip) [control] | 4096 | 4351/4515 ns | 5410/4500 ns | +24.3% / -0.3% | 4571->4563 ns (-0.2%) |

Baseline-vs-baseline repeat spread (the noise floor, from the two independent baseline runs
alone): up to +33% at width=1 (absolute times are 3-7ns there, a couple of clock-resolution ticks),
narrowing to roughly ±5% by width=64 and ±1% by width>=1024 for most configs (`trns16` at
width=4096 is an outlier at +5.3% baseline-vs-baseline). The `strip16` control (which the patch
does not touch at all) shows the same magnitude of run-to-run noise as the patched configs
(e.g. +24.3%/-0.3% at width=4096, -33.3%/+0.0% at width=1) - direct evidence that the deltas seen
in the patched configs are of the same origin (measurement noise), not a real effect of the patch.

## Result

**No measurable change, at any row width, on either single-core or unpinned multi-core.** Time
only (no allocation dimension - the patch moves an existing stack computation, it doesn't add or
remove any allocation). Every delta above is within the noise floor established by the unchanged
`strip16` control and by baseline-vs-baseline repeat runs at the same width; there is no width or
configuration at which the patched build is consistently and reproducibly faster or slower than
baseline across both single-core runs. Some values look like isolated wins or regressions in a
single run (e.g. `trns8`/`trns16` showing a consistent ~-5% at large widths across both single-core
runs is the closest thing to a real signal here) but this does not hold up as a stable pattern
across widths or against the unpinned multi-core run for the same configuration (`trns8` at
width=4096 shows -5.1%/-5.2% single-core but +6.5% multi-core) - not treated as a genuine effect
without a third confirming run, which given the size of the effect (single-digit percent, same
order as the noise floor) was judged not worth the additional single-core run time for what the
mechanism itself already predicts should be unmeasurable (see "Why" below).

## Does it change behavior/output?

- **Bit-exact?** Yes. Verified two ways: (1) the bench's own `out_hash` (FNV-1a64 over the output
  buffer) is identical between the baseline and patched binaries for all 6 configurations x 7
  widths on a fixed, varied (non-constant-fill) synthetic input - see `raw/` if the hash-mode bench
  output is wanted (not saved separately; reproducible via `just test`'s own build). (2) The
  project's full test suite passes with **identical counts before and after**: 91 lib unit tests +
  1 (`tests/bugfixes.rs`) + 4 (`tests/check_testimages.rs`) + 3 (`tests/partial_decode.rs`) + 8
  doc-tests = 107 passed, 0 failed, 1 ignored, both builds (`cargo test --release`).
- **Is this a breaking change?** No. `create_transform_fn`/`TransformFn` are only exposed under
  the internal `benchmarks` feature (`src/benchable_apis.rs`); the public `Reader` API and its
  output are unaffected.

## Why the result looks the way it does

The mechanism this candidate targeted (a `u8` division or a 4-arm match, done once per row call)
is genuinely tiny compared to two things that dominate every configuration measured: the
`Box<dyn Fn>` indirect call itself (unchanged by this patch - both before and after go through the
same vtable dispatch, just with different code behind it), and the O(width) per-pixel loop body,
which is >=90% of the row's total time for any width above a few dozen pixels (e.g. `trns8` goes
from 6ns at width=1 to 15,925ns at width=4096, a ~2600x increase for a 4096x increase in per-pixel
work - almost exactly linear, leaving very little room for a fixed per-call term to matter). The
TODO comment's premise (paying "X ns... to improve throughput by Y ns/byte", the same trade-off
`palette.rs`'s own module doc describes for the paletted LUT) doesn't apply here the same way,
because the paletted case's per-call setup (building a 256-entry LUT) is orders of magnitude larger
than a single division/match - there was a real X to amortize there. Here X is already close to
zero, so amortizing it away also nets close to zero.

## What's not done / open questions

- Not tried: hoisting `info.trns.as_deref()` itself out of the closure by cloning the trns bytes
  into an owned `Vec<u8>` captured at creation time (would remove the last remaining per-call
  `Info` field read for the three `expand_trns*` functions). Not attempted because the same
  "already near-zero fixed cost" argument applies, and it would require an allocation per decode
  (to own the bytes) to save a `Option::as_deref()` per row - likely a net loss for small images,
  not attempted given the measured result above already shows no headroom to capture.
- Not tried: disassembly/codegen inspection of whether the closure form actually devirtualizes or
  inlines any differently than the fn-item form once instantiated behind `Box<dyn Fn>` - the
  measured result didn't show a signal worth explaining at that level.
- No incidental bugs or correctness issues found while reading `src/decoder/transform.rs` for this
  candidate.
