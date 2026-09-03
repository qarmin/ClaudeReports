# imageops::resize (free function) - extend pic-scale-safe delegation to Rgba<u8>

Extends candidate 013's `Rgb<u8>`-only prototype (which delegates `imageops::resize`'s
fully-packed, contiguous case to `pic_scale_safe::resize_rgb8`) with a second `TypeId` arm for
`Rgba<u8>` (`RgbaImage`), delegating to `pic_scale_safe::resize_rgba8` and mirroring
`src/imageops/resize.rs`'s `DynamicImage::resize` premultiply/`has_constant_alpha`/unpremultiply
handling (none of which 013's prototype touched, since it only covered the no-alpha case).

## What was tried

Base commit: `44ce9226c541dd2b11897b9ef07156a28871cdb7` (same as candidate 013 - no upstream
changes to the touched files since). Since candidate 013's own patch was reverted at the end of
its session (per this skill's "the one rule that matters most"), this candidate's `patch.diff`
re-includes 013's `Rgb<u8>` arm plus this session's new `Rgba<u8>` arm, so it applies standalone
against the pinned base commit.

Benchmark: the crate's own pre-existing `benches/imageops.rs::bench_resize`, extended this
session with two new `RgbaImage` sources at the same 1920x1080 size as the existing `RgbImage`
one - `src_rgba` (non-constant alpha, `(x + y*7) % 256`, exercising the premultiply/unpremultiply
path) and `src_rgba_opaque` (constant alpha = 255, exercising the `has_constant_alpha` skip) -
each benched at "400x300" and (for the non-opaque source) "2000x2000", same as the existing
`RgbImage` cases.

All numbers are alternating A/B runs of two prebuilt `--release` criterion binaries (baseline:
original algorithm + the extended bench; patched: both fast-path arms + the extended bench),
`--warm-up-time 1 --measurement-time 2` (`--measurement-time 3` for the 2000x2000 case), seconds
apart on the same machine. `uptime` load varied 2-12 across the session (this machine is known
noisy per `projects/image.md`/`IDEAS.md`'s existing notes) - paired alternation is what the
conclusion rests on, not the absolute numbers, which drifted between sub-sessions.

**`resize rgba 400x300 Triangle`, non-constant alpha, single-core (`taskset -c 3`):**

| Variant | run 1 | run 2 | run 3 | run 4 | run 5 |
|---|---|---|---|---|---|
| baseline | 8.4238 ms | 8.1216 ms | 8.1971 ms | 8.6977 ms | 8.1564 ms |
| patched | 9.8396 ms | 9.9033 ms | 10.079 ms | 9.7994 ms | 9.8883 ms |

Patched *slower* in 5/5 pairs (median 8.1971 -> 9.8883 ms, ~+21%).

**`resize rgba 400x300 Lanczos3`, non-constant alpha, single-core:**

| Variant | run 1 | run 2 | run 3 | run 4 | run 5 |
|---|---|---|---|---|---|
| baseline | 14.306 ms | 10.530 ms | 10.537 ms | 11.153 ms | 20.944 ms |
| patched | 8.2522 ms | 7.7418 ms | 7.8130 ms | 15.454 ms | 16.222 ms |

Patched faster in 4/5 pairs (the last pair coincides with a machine-wide slow episode affecting
both binaries - see `IDEAS.md`'s existing "mode lottery" notes). Excluding that pair, patched is
consistently ~25-45% faster.

**`resize rgba 400x300 Triangle`, non-constant alpha, multi-core (unpinned):**

| Variant | run 1 | run 2 | run 3 | run 4 | run 5 |
|---|---|---|---|---|---|
| baseline | 4.2225 ms | 4.1913 ms | 4.2428 ms | 5.0546 ms | 4.2546 ms |
| patched | 5.0354 ms | 5.5634 ms | 5.2205 ms | 5.5783 ms | 5.1936 ms |

Patched slower in 5/5 pairs - same direction and similar magnitude as single-core (as expected:
candidate 013 already found `pic_scale_safe`'s `rayon` feature isn't forwarded from this crate's
`Cargo.toml` yet, so neither variant parallelizes here).

**`resize rgba opaque 400x300 Triangle`, constant alpha, single-core:**

| Variant | run 1 | run 2 | run 3 | run 4 | run 5 |
|---|---|---|---|---|---|
| baseline | 4.0815 ms | 7.9286 ms | 4.0800 ms | 4.0857 ms | 4.0833 ms |
| patched | 4.4405 ms | 2.1856 ms | 2.2226 ms | 2.1807 ms | 2.2560 ms |

Patched faster in 4/5 pairs (run 1's baseline/patched pair looks like a cold-start/warm-up
artifact, consistent with a previously-documented noise shape in this workspace - see
`IDEAS.md`'s "single anomalous pair" entry from candidate 010's session); every other pair shows
patched roughly 1.8-3.6x faster.

**`resize rgba opaque 400x300 Lanczos3`, constant alpha, single-core:**

| Variant | run 1 | run 2 | run 3 |
|---|---|---|---|
| baseline | 12.888 ms | 10.524 ms | 12.443 ms |
| patched | 5.0622 ms | 5.1958 ms | 5.2633 ms |

Patched faster in 3/3 pairs, ~2.3x.

**`resize rgba opaque 400x300 Triangle`, constant alpha, multi-core (unpinned):**

| Variant | run 1 | run 2 | run 3 | run 4 | run 5 |
|---|---|---|---|---|---|
| baseline | 6.5281 ms | 4.2012 ms | 4.2615 ms | 4.2276 ms | 4.1134 ms |
| patched | 2.2749 ms | 2.3558 ms | 2.2233 ms | 2.2144 ms | 2.2315 ms |

Patched faster in 5/5 pairs, ~1.9x.

**`large/resize rgba 2000x2000 Triangle`, non-constant alpha, single-core (`--measurement-time 3`):**

| Variant | run 1 | run 2 | run 3 |
|---|---|---|---|
| baseline | 67.460 ms | 48.110 ms | 48.866 ms |
| patched | 15.236 ms | 14.937 ms | 15.000 ms |

Patched faster in 3/3 pairs, ~3.2x.

**`large/resize rgba 2000x2000 Lanczos3`, non-constant alpha, single-core (`--measurement-time 3`):**

| Variant | run 1 | run 2 | run 3 |
|---|---|---|---|
| baseline | 64.017 ms | 64.031 ms | 64.081 ms |
| patched | 22.291 ms | 22.374 ms | 22.186 ms |

Patched faster in 3/3 pairs, ~2.9x.

## Result

The direction of the delta depends on both the alpha content and the output size, not just on
whether the fast path fires at all - a genuinely different shape from candidate 013's uniform
~3.9-4.2x win for `Rgb<u8>`:

- **Constant-alpha (opaque) `RgbaImage`, any size tried**: a consistent win, ~1.8-3.6x faster
  across both filters and both single-/multi-core, matching candidate 013's `Rgb<u8>` magnitude
  (unsurprising - the `has_constant_alpha` check skips premultiply/unpremultiply entirely here,
  leaving the same `pic_scale_safe::resize_rgba8` call that plain `Rgb<u8>` gets, just with a
  46th channel along for the ride).
- **Non-constant-alpha `RgbaImage`, small downscale (1920x1080 -> 400x300)**: a consistent
  *regression*, ~18-25% slower for `Triangle` (5/5 paired runs), single- and multi-core alike.
  `Lanczos3` at the same size trended faster (4/5 pairs) but with much more spread than the
  opaque/Rgb cases.
- **Non-constant-alpha `RgbaImage`, large output (1920x1080 -> 2000x2000)**: a consistent win,
  ~2.9-3.2x faster for both filters tried (3/3 pairs each).

## Why the result looks the way it does

`premultiply_rgba8` (called on the *full source* buffer - 1920x1080x4 = ~8 MiB here - whenever
alpha isn't constant and the filter isn't `Nearest`) is itself expensive: isolated in a standalone
release binary, `pic_scale_safe::premultiply_rgba8` alone measured ~4.4-7.5ms for this exact
buffer size on this machine, comparable to or larger than the entire baseline `resize` call at the
400x300 output size (~8.1ms for `Triangle`). Deliberately forcing the fast path to always
premultiply (bypassing the `has_constant_alpha` check) reproduced the same ~9-10ms regression seen
in the real patch, and forcing it to *never* premultiply dropped the same call to ~2.7ms -
confirming `premultiply_rgba8`'s own cost, not `resize_rgba8` itself, as the direct cause of the
400x300 regression.

This cost is **fixed per source-buffer size**, not per output-pixel count, while both the
baseline algorithm's cost and `pic_scale_safe::resize_rgba8`'s own cost scale with how much
resampling work there is (roughly with output size for a fixed source, since a smaller target here
means fewer taps/writes). At 400x300 (a small target, cheap resampling), the fixed premultiply tax
dominates and erases the algorithmic win the delegation would otherwise provide. At 2000x2000 (an
upscale from 1920x1080, four times the destination pixel count of the source), the resampling
work is large enough that the same fixed premultiply tax becomes a small fraction of the total,
and the delegation's algorithmic advantage (candidate 013's SIMD/no-dynamic-closure argument)
dominates again.

## Does it change behavior/output?

- **Bit-exact? No**, for every `RgbaImage` case (fast path fires whenever the buffer is fully
  contiguous). Characterized via a temporary standalone example
  (`examples/_resize_rgba_diff_tmp.rs`, written and run this session, removed before capturing
  `patch.diff`) resizing a deterministic 1920x1080 `RgbaImage` 1920x1080 -> 400x300, for both a
  non-constant-alpha source and a constant/opaque-alpha source, across all five `FilterType`
  variants, comparing raw output bytes between a baseline and a patched build:

  **Opaque (constant alpha) source** - RGB channels behave exactly like candidate 013's `Rgb<u8>`
  characterization (same max-diff values per filter: Triangle 2, CatmullRom 3, Gaussian 123,
  Lanczos3 5), and the **alpha channel is byte-identical between baseline and patched (0/120000
  bytes differ)** for every filter - confirming the constant-alpha fast path is a pure
  RGB-channel-only change here, same known "+1 accumulator bug" root cause as 013 (see
  `___OTHER___/001.RESIZE_VERTICAL_SAMPLE_OFF_BY_ONE/`).

  **Non-constant-alpha source** - a materially larger and qualitatively different change, because
  **the original `imageops::resize` free function does not premultiply alpha at all** (its own
  doc comment already states it "assumes alpha pre-multiplication for images that contain
  non-constant alpha" - i.e. the caller is expected to have premultiplied already, or accept color
  bleed). This patch's fast path *does* premultiply/unpremultiply automatically (mirroring
  `DynamicImage::resize`), so for a non-premultiplied `RgbaImage` with varying alpha, this is a
  real semantic change, not just rounding:

  | Filter | RGB bytes differing | RGB max abs diff | alpha bytes differing (of 120000) | alpha max abs diff |
  |---|---|---|---|---|
  | Nearest | 99.75% | 254 | (Nearest never premultiplies either way; same `+1`/adjacent-pixel-selection differences as 013's Rgb8 Nearest case) | - |
  | Triangle | 82.43% overall | 97 | 108525 (90.4%) | 9 |
  | CatmullRom | 81.66% overall | 106 | not separately isolated | - |
  | Gaussian | 81.64% overall | 136 | not separately isolated | - |
  | Lanczos3 | 81.97% overall | 255 | 101546 (84.6%) | 2 |

  The alpha channel itself differing (up to 9/255, not just the known `+1` bug) for `Triangle`
  confirms this is a genuine algorithm-level behavior change in how transparent/semi-transparent
  pixels blend, not a rounding artifact - a plausible, largely expected consequence of adding
  premultiply/unpremultiply where none existed before, and arguably a *more correct* default (it
  avoids the "color bleed from fully transparent pixels" that `DynamicImage::resize`'s own doc
  comment for `resize_impl` names as the reason it premultiplies), but a behavior change either way
  for any code relying on the free function's current (non-premultiplying) contract.
- **Is this a breaking change?** Yes, same two ways candidate 013 already noted for `Rgb<u8>`
  (numeric output changes for any `RgbImage`/`RgbaImage` input; the new `where I::Pixel: 'static`
  bound on `resize`'s signature), **plus a third, specific to this extension**: for `RgbaImage`
  inputs with non-constant alpha, `imageops::resize`'s documented behavior itself changes (it
  starts premultiplying/unpremultiplying automatically, where it previously did neither) - this is
  a behavior change beyond "different valid rounding", since it changes what contract the function
  follows for alpha-bearing images, not just the numeric precision of an existing contract.
- **Project test suite:** 331 passed, 0 failed, both before and after (same count, no new
  failures) - `cargo test --release --lib`. Same result candidate 013 found; the existing suite
  does not pin `imageops::resize`'s exact numeric output for `RgbaImage` inputs either.

## What's not done / open questions

- **The premultiply cost itself was not optimized or worked around** - e.g. `pic_scale_safe` also
  exposes `premultiplied_rgba8` (allocates and premultiplies in one fused pass, rather than this
  patch's `to_vec()` then `premultiply_rgba8` in place) which would save the extra ~0.6ms this
  session measured for the separate copy, but would not address the ~6.5ms `premultiply_rgba8`
  cost itself, which dominates. Not attempted as a fix in this session (see next bullet).
- **Whether `pic_scale_safe::premultiply_rgba8`'s own cost (measured standalone: ~4.4-7.5ms for an
  8 MiB / 1920x1080x4 buffer on this machine) is expected/inherent to correct premultiplication
  arithmetic, or a missed-vectorization issue in that specific dependency function, was not
  investigated with `perf`/disassembly** (unavailable in this environment - see `README.md`). The
  function's own doc comment ("Almost all loops are not auto-vectorised without doing anything
  dirty. So everywhere is just added something beautiful.") suggests its authors were already
  fighting this, which is circumstantial evidence toward "known limitation" rather than "easy
  local fix", but this wasn't confirmed by reading past this session's black-box timing.
  Worth flagging upstream or investigating further if this delegation direction is pursued for
  real.
- **Only the plain, downscale-to-400x300 and upscale-to-2000x2000 shapes were measured** for the
  non-constant-alpha case - the crossover output size where the fixed premultiply cost stops
  dominating (somewhere between the two) was not located; a maintainer deciding whether to ship
  this would want that threshold, or a source-size-aware/output-size-aware heuristic for whether
  to take the fast path at all, rather than always delegating whenever the `TypeId`/contiguity
  checks pass.
- **A `SubImage` crop and other non-contiguous-layout `RgbaImage` cases were not re-verified for
  this arm** specifically (013 confirmed the `Rgb8` arm falls back correctly for these; this
  arm's contiguity guard is structurally identical, so the same fallback logic applies, but this
  session didn't run a byte-identical-output check for it the way 013 did).
- **16-bit/f32 `Rgba` variants, `Luma<u8>`, `LumaA<u8>`** remain untouched, same as 013 left them.
- Did not sweep whether the regression's magnitude changes with a different non-constant-alpha
  pattern (this session's synthetic alpha, `(x + y*7) % 256`, is a fairly "busy" pattern - a more
  realistic photo-like alpha channel, e.g. a soft circular vignette, might hit `has_constant_alpha`
  less predictably but wasn't tried).
