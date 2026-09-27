# DynamicImage tile/replace/vertical_gradient/horizontal_gradient: same #[inline]/#[inline(always)] fix as candidate 060, now benched and timed for the three call sites 060 only correctness-checked

Candidate 060 added `#[inline(always)]`/`#[inline]` to `DynamicImage`'s `get_pixel`/`put_pixel`
and measured a -6.8%/-6.7% win for `imageops::overlay`, but `tile` (which calls `overlay`
directly), `replace`, `vertical_gradient`, and `horizontal_gradient` (which call the same two
accessors directly) were only exercised for correctness in that candidate, never timed on their
own. This candidate adds a dedicated `benches/imageops.rs` case for each of the four and measures
whether the same already-identified fix transfers.

## What was tried

Base commit: `44ce9226c541dd2b11897b9ef07156a28871cdb7`. Four new criterion cases added to
`benches/imageops.rs`, reusing candidate 060's own fixture images (1920x1080
`DynamicImage::ImageRgba8` bottom, 300x300 `DynamicImage::ImageRgba8` top with partial alpha
180/255):

- `"dyn image tile"` - `imageops::tile(&mut bottom, &top)` (steps the 300x300 top across the full
  1920x1080 canvas via repeated `overlay` calls), `iter_batched(.., BatchSize::LargeInput)` so
  each iteration starts from a fresh clone of `bottom` (blending is not idempotent across
  iterations since the top has partial alpha).
- `"dyn image replace"` - a single `imageops::replace(&mut bottom, &top, 100, 100)` call per
  iteration on a reused buffer (no clone needed - `replace` unconditionally overwrites the target
  region, so repeated calls are idempotent).
- `"dyn image vertical gradient"` / `"dyn image horizontal gradient"` - `imageops::vertical_gradient`/
  `horizontal_gradient(&mut img, &start, &stop)` on a reused 1920x1080 buffer per iteration (also
  idempotent - every pixel is unconditionally overwritten every call).

Then applied candidate 060's exact source patch (`#[inline(always)]` on
`<DynamicImage as GenericImageView>::get_pixel`, `#[inline]` on
`<DynamicImage as GenericImage>::put_pixel`, `src/images/dynimage.rs`) and compared two prebuilt
`--release` `imageops` bench binaries (baseline vs. patched source) via 10 alternating paired runs
per bench id per mode (`taskset -c 3` for single-core; unpinned for multi-core),
`--warm-up-time 1 --measurement-time 1.2`, using candidate 060's own `paired_overlay.sh` script
(copied here unmodified as `paired.sh`). Same-binary self-checks (baseline vs. an independently
rebuilt, byte-identical copy) were run first for `"dyn image tile"` (n=8, single-core) and
`"dyn image horizontal gradient"` (n=5 then a matched n=10, single-core) to establish this host's
noise floor for these specific bench shapes.

| Variant | single-core (`taskset -c 3`) p50 | multi-core (unpinned) p50 | notes |
|---|---|---|---|
| noise floor: `dyn image tile`, baseline vs. rebuilt-identical baseline (n=8, single-core) | ratio 1.016, 0/8 pairs favor either build meaningfully | not separately checked | md5-identical binaries; ~1.6% spread |
| noise floor: `dyn image replace`, baseline vs. rebuilt-identical baseline (n=5, single-core) | ratio 0.997, 2/5 | not separately checked | ~0.3-7% spread across individual pairs |
| noise floor: `dyn image vertical gradient`, baseline vs. rebuilt-identical baseline (n=5, single-core) | ratio 1.059, 2/5 | not separately checked | wide per-pair spread (up to ~15%) on this fast (~4-5ms), memory-bound bench shape |
| noise floor: `dyn image horizontal gradient`, baseline vs. rebuilt-identical baseline (n=5 then n=10, single-core) | n=5: ratio 0.980, 3/5; n=10: ratio 1.030, 5/10 | not separately checked | same wide spread as vertical gradient; the n=10 self-check landed almost exactly at 1.000/50-50, i.e. no real median offset between identical builds, but individual pairs swing +-10% |
| baseline: `dyn image tile` | 21.95 ms | 22.14 ms | `tile` calls `overlay` once per 300x300 step across the 1920x1080 canvas (7x4=28 calls/iteration) |
| patched: `dyn image tile` | **19.65 ms (-10.5%)**, 9/10 pairs favor patch | **19.85 ms (-10.3%)**, 9/10 pairs favor patch | |
| baseline: `dyn image replace` | 0.283 ms | 0.282 ms | single `replace` call, 300x300 region, no blend |
| patched: `dyn image replace` | **0.212 ms (-25.1%)**, 10/10 pairs favor patch | **0.211 ms (-25.2%)**, 10/10 pairs favor patch | |
| baseline: `dyn image vertical gradient` | 4.763 ms | 4.657 ms | row-major write order (`for y { for x { put_pixel } } }`), no `get_pixel` calls at all |
| patched: `dyn image vertical gradient` | **3.649 ms (-23.4%)**, 7/10 pairs favor patch | **3.637 ms (-21.9%)**, 8/10 pairs favor patch | clear median shift despite the wider per-pair noise measured above |
| baseline: `dyn image horizontal gradient` | 5.158 ms | 4.689 ms | column-major write order (`for x { for y { put_pixel } } }`) |
| patched: `dyn image horizontal gradient` | 4.823 ms (-6.5%), 6/10 pairs favor patch | 4.541 ms (-3.2%), 6/10 pairs favor patch | both deltas are inside/at the edge of this bench's own ~+-10%-per-pair, ~1.030-ratio-at-n=10 same-binary noise floor measured above - not distinguishable from noise with the runs done here |

## Result

**Real, clearly-outside-noise wins for `tile` (-10.5%/-10.3%), `replace` (-25.1%/-25.2%), and
`vertical_gradient` (-23.4%/-21.9%)**, single-/multi-core respectively, all with a majority
(7-10 of 10) of paired runs favoring the patch and medians well outside each bench's own measured
same-binary noise floor. `horizontal_gradient`'s measured delta (-6.5% single-core, -3.2%
multi-core, 6/10 pairs) sits inside the noise floor measured for that specific bench shape
(same-binary self-check ratio 1.030 at n=10, individual pairs swinging +-10%) and is **not
distinguishable from noise with the runs done here** - see "Why the result looks the way it does"
for the likely structural reason. Time was the only dimension measured; none of these four
functions allocate per-pixel, so no RSS/allocation measurement was taken.

`replace`'s and `vertical_gradient`'s magnitude (-21% to -25%) is larger than `overlay`'s own
-6.8%/-6.7% (candidate 060) and `tile`'s -10.5%/-10.3% here - both `replace` and the gradient
functions have a much simpler per-pixel body than `overlay` (`replace`: one `get_pixel` + one
`put_pixel`, no blend arithmetic at all; the gradients: one `put_pixel`, no `get_pixel` at all;
`overlay`/`tile`: two `get_pixel` calls, an alpha blend, and one `put_pixel`), so the two
accessor calls this patch removes make up a proportionally larger share of each per-pixel
iteration's total cost in `replace`/`vertical_gradient` than in `overlay`/`tile`.

## Does it change behavior/output?

- **Bit-exact?** Yes. A temporary standalone example (`examples/_correctness_check_064.rs`, not
  part of `patch.diff`, removed before reverting) ran `tile` (bottom clone + top), `replace` at
  three offsets (fully in-bounds, negative-offset-clipped, out-of-bounds-clipped), and both
  gradient functions on the same 1920x1080/300x300 fixtures, reading every output pixel back via
  `DynamicImage::get_pixel` and combining an FNV-1a hash of every buffer produced. The combined
  hash (`962ab8d09835d4ce`) matched byte-for-byte between an unpatched-source build and this
  candidate's patched-source build.
- **Is this a breaking change?** No - same as candidate 060, only `#[inline]`/`#[inline(always)]`
  attributes added to two existing trait-impl methods; no public signature, panic behavior, or
  documented result changed.
- Project test suite (`cargo test --release`): **550 passed / 1 failed, identically before and
  after.** The one failure is the same pre-existing, unrelated `tests/regression.rs::
  resizing_with_catmul` every prior candidate in this workspace touching this area has already
  documented (confirmed on this session's own unpatched baseline too).

## Why the result looks the way it does

`tile`'s result (-10.5%/-10.3%) is close to, but somewhat larger than, `overlay`'s own -6.8%/-6.7%
from candidate 060 - expected, since `tile` calls `overlay` directly with no other per-call
overhead beyond `overlay`'s own bounds computation, so the same removed-call-count benefit applies
per pixel; the extra few points of improvement is plausibly just this bench's different image/tile
size ratio (28 `overlay` invocations per `tile` call vs. one direct `overlay` call in 060's bench)
amortizing 060's overlay-entry fixed cost differently, not a different mechanism.

`replace` and `vertical_gradient` show a substantially bigger relative win (-21% to -25%) than
`overlay`/`tile` because their per-pixel body is doing much less non-accessor work: `replace` does
one `get_pixel` + one `put_pixel` and nothing else (no blend), and `vertical_gradient` does one
`put_pixel` and a cheap `lerp` computed once per row (not per pixel, since the interpolated value
only depends on `y`) - so removing the two accessor calls' dispatch overhead removes a
proportionally much larger fraction of each function's already-small per-pixel cost.

`horizontal_gradient`'s much weaker/noise-floor-indistinguishable result, despite being the
identical accessor-removal mechanism and an almost identical per-pixel body to
`vertical_gradient`, is most plausibly explained by its different loop nesting:
`horizontal_gradient` iterates `for x in 0..width { for y in 0..height { img.put_pixel(x, y, ..) } }`
- a column-major write pattern against a row-major-stored `ImageBuffer`, so consecutive writes
within the inner loop land on different, widely-separated cache lines (`width` bytes-per-channel
apart), unlike `vertical_gradient`'s `for y { for x { .. } }` row-major pattern, which writes
consecutive addresses. This was inferred from the loop shapes in `src/imageops/mod.rs` (both
functions are otherwise structurally identical, differing only in which axis is outer/inner) and
from the noise-floor measurements above, not independently confirmed via `perf`/cache-miss
counters (not available in this environment, consistent with every earlier candidate in this
workspace's own notes on the same limitation) - if the per-pixel loop is memory-bandwidth/cache-
miss-bound rather than call-dispatch-bound for this specific access pattern, removing the call
overhead would plausibly have much less effect on the observed wall-clock time, which matches what
was measured.

## What's not done / open questions

- `horizontal_gradient`'s cache-miss hypothesis above was not independently confirmed via
  `perf stat`/cache-counter measurement (unavailable in this environment) - a future session with
  `perf` available could confirm whether `L1-dcache-load-misses` is materially higher for
  `horizontal_gradient` than `vertical_gradient` on an equivalent-sized image, which would support
  or refute the explanation given here.
- `SubImageInner`'s own `get_pixel`/`put_pixel` (candidates 061/062) were not re-examined via
  `tile`/`replace`/gradient call sites specifically - those candidates already found no benefit
  there for `overlay`, and the underlying accessor mechanism is identical, so a materially
  different result via these other call sites seems unlikely, but wasn't directly checked.
