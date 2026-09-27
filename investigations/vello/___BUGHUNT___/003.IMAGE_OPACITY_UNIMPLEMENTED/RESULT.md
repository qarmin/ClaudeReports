# Image opacity panics via unimplemented!() in Image::encode_into

`sparse_strips/vello_common/src/encode.rs`'s `impl EncodeExt for Image { fn encode_into }` does
`unimplemented!("Applying opacity to image commands")` whenever `self.sampler.alpha != 1.0`, so
any real caller (of `vello_cpu`, `vello_hybrid`, or any other consumer of `vello_common`'s public
`Image`/`ImageSampler` types) that draws an image with a less-than-fully-opaque
`ImageSampler.alpha` panics the renderer. `ImageSampler.alpha` is a plain public `f32` field with
no validation anywhere between the caller and this code, so any value other than exactly `1.0`
(the `Default` value) triggers it. An open upstream PR (linebender/vello#1736, not yet merged as of
this session) proposes folding `sampler.alpha` into the paint's tint instead of panicking; this
candidate independently reproduces the panic, root-causes it, and verifies a fix using the same
general approach (folding alpha into the tint), implemented from scratch rather than copied from
that PR's diff.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction: standalone repro crate calling the real public `vello_cpu::RenderContext` API (`set_paint`/`fill_rect`/`render`) with `ImageSampler { alpha: 0.5, .. }` | `cargo run --release` against pristine `HEAD` (75168314), 5 runs (1 plain + 4 with `RUST_BACKTRACE=1`) | Panics deterministically every run, identical location/message/backtrace (`encode.rs:492`, `unimplemented!("Applying opacity to image commands")`) |
| Same repro, debug build | `cargo run` (no `--release`), 1 run during initial triage | Panics identically (this is a plain `unimplemented!()`, not a `debug_assertions`-gated condition, so debug vs release makes no difference) |
| Additional boundary values against the *patched* tree (ad hoc scratch crate, not archived) | `alpha=0.0` no tint; `alpha=0.5` + existing `Multiply` tint; `alpha=0.3` + existing `AlphaMask` tint | All three render successfully post-fix, no panic |

No `miri`/ASAN/fuzzing was used - this is a deterministic, unconditional `unimplemented!()` on any
input in a documented-but-unhandled range of a public struct field, not a memory-safety or
input-space question; manual boundary-value construction was sufficient to fully characterize it
on the first attempt, so the soft cap of 3 escalating approaches was not needed.

## Status

**Confirmed**. Reproduces 5/5 runs (release build) plus 1/1 in a debug build, deterministically -
this is not layout/timing/seed-dependent, it is an unconditional `unimplemented!()` reached
whenever the input condition (`sampler.alpha != 1.0`) holds.

**Fully reproducible**: **Yes**. The repro drives the real, public `vello_cpu` API end-to-end
(`RenderContext::set_paint` -> `fill_rect` -> `render`, the same sequence any real caller of
`vello_cpu` uses) with a genuine `ImageSampler` value a real caller would construct - it does not
call `Image::encode_into` or any other private/internal function directly. This is a
single-participant case (`CONFIRMATION.md` section 11's "single-process, single-participant repro
remains completely fine" carve-out applies): the input is a struct field a caller sets directly on
a value it owns, not something produced by a second participant across a boundary that needed
faking.

## Class

crash

## Root cause

`Image::encode_into` (sparse_strips/vello_common/src/encode.rs, `impl EncodeExt for Image`) reads
`self.sampler.alpha`, which is a plain public `f32` on `peniko::ImageSampler` (re-exported through
`vello_common`/`vello_cpu`'s public API) that any caller can set to any value via
`ImageSampler { alpha: <anything>, ..ImageSampler::default() }`. The code was clearly written with
opacity support planned but not yet implemented: guarded by
`if sampler.alpha != 1.0 { unimplemented!(...) }` right at the top of the function, before any
other encoding work happens. There is no validation, clamping, or fallback anywhere between a
caller setting this field and this code running it - `set_paint`/`fill_rect` (the public
`vello_cpu::RenderContext` API) pass the `Image` straight through to `encode_into` during
`render()`/rasterization. A few lines further down, the `has_opacity` computation already
special-cases `sampler.alpha != 1.0` with the comment "Not supported yet, but just to
future-proof", confirming this was a known, deliberately-unimplemented gap rather than an
oversight in a code path nobody expected to be reached.

## Introduced by

bf4962a0 "sparse_strips: Use peniko::Brush (#1253)" (2025-10-02) - found via `git log -S
'Applying opacity to image commands' -- sparse_strips/vello_common/src/encode.rs`, which returns
exactly this one commit. This is when `Image::encode_into` was rewritten to consume
`peniko::Brush`/`ImageSampler` (replacing an earlier internal type that presumably didn't expose a
freestanding alpha field the same way), and the `unimplemented!()` guard was added at that point
rather than actually implementing opacity support.

## The fix

Folds `sampler.alpha` into the paint's `Tint` instead of panicking, then always resets
`sampler.alpha` to `1.0` before the rest of `encode_into` runs:

- If a `Tint` is already set, its alpha channel is multiplied by `sampler.alpha` (mode/RGB
  untouched) - because both `TintMode`s (`AlphaMask`: `tint_premul * sample.alpha`; `Multiply`:
  `sample_premul * tint_premul`) scale linearly with the tint's alpha channel, pre-scaling that
  alpha reproduces exactly what a dedicated image-opacity path would compute.
- If no `Tint` is set, a white `Tint { color: [1,1,1,alpha], mode: Multiply }` is synthesized,
  which is the identity tint color-wise and applies only the opacity scaling.
- The now-redundant `sampler.alpha != 1.0` arm of the `has_opacity` computation further down is
  removed, since `sampler.alpha` is unconditionally `1.0` by the time that line runs.

This independently arrives at the same general approach as the open upstream PR #1736 (folding
alpha into the tint) but is implemented from scratch against this session's own reading of the
`TintMode` semantics in `paint.rs`, not copied from that PR's diff.

Verified against the standalone `repro/`:
- Before the patch: `repro/` panics (`unimplemented!("Applying opacity to image commands")`,
  `encode.rs:492`), reproduced 5/5 runs.
- After the patch: `repro/` (same code, `alpha = 0.5`, unchanged) runs to completion and prints
  `rendered successfully (bug is fixed)`, 3/3 runs.
- Additional boundary-value inputs, all against the patched tree (per `CONFIRMATION.md` section
  7, checked beyond just the one triggering `alpha = 0.5`): `alpha = 0.0` with no tint, `alpha =
  0.5` with an existing `Multiply` tint, and `alpha = 0.3` with an existing `AlphaMask` tint all
  render successfully with no panic.

See `output.txt` for the actual captured before/after terminal output.

- Project test suite (`cargo test -p vello_common -p vello_cpu`): **159 passed / 0 failed / 1
  ignored** (vello_common) and **81 passed / 0 failed** (vello_cpu, plus 1 passing doctest) -
  identical counts before and after the patch. No test in either crate's existing suite exercised
  `ImageSampler.alpha != 1.0` before this session (consistent with the path being entirely
  unimplemented on `main`), and the fix introduces no regressions.

## What's not done / open questions

- This candidate only patches `vello_common`'s CPU-facing encoder (`Image::encode_into`). It does
  not check whether `vello_hybrid`'s own encoding/shader path (which has its own image-paint
  handling in `sparse_strips/vello_hybrid/src/paint.rs`/`scene.rs`) shares the same
  not-yet-implemented gap or a different one - see "new suspects" below, one was found there.
  Upstream PR #1736's description mentions covering both CPU and hybrid renderers via a snapshot
  test; that wasn't independently re-verified here.
- The upstream PR (#1736) additionally adds 4 unit tests + 1 snapshot integration test directly in
  the project's own test files; this session's fix was verified via the standalone `repro/` plus
  the existing test suite, but no new test was added to the project's own test files (per the
  bug-hunt workflow, the main tree is always reverted after verification - a maintainer picking up
  `patch.diff` may want to add project-native regression tests, which this patch does not include).
- Didn't check whether a similarly-shaped `unimplemented!()`/panic exists for other not-yet-
  supported *gradient* paint combinations (the candidate description asked to check "sibling
  paint/gradient code paths") - `EncodeExt for Gradient` in the same file was read but no
  `unimplemented!()`/`todo!()` was found in it. Other `unimplemented!()` sites found nearby (not
  investigated as part of this candidate, see new suspects) are in image/filter code, not
  gradients.
