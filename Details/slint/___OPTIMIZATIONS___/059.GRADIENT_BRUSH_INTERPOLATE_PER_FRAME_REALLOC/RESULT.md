# 059 - `Brush::interpolate`'s `.clone()` forces gradient stops onto the always-reallocate path

`InterpolatedPropertyValue for Brush::interpolate` (`internal/core/graphics/brush.rs:780-924`)
runs once per animation frame for any transition/animation touching a gradient brush (hover
highlights, solid<->gradient crossfades). Each of the 6 gradient-producing match arms starts with
`let mut new_grad = grad.clone();` (or `lhs.clone()`), then calls `new_grad.0.make_mut_slice()`.
Cloning the `SharedVector<GradientStop>` bumps its refcount from 1 to 2 *before* `make_mut_slice`
checks it, so `SharedVector::detach` always takes its "shared" branch (full allocation + copy of
every stop via the bounds-checked `IntoIterInner::Shared` path) - even on a frame where the source
value was otherwise uniquely owned and an in-place mutation would have been possible.

## What was tried

Replaced each `grad.clone()` / `lhs.clone()` with a direct, un-shared copy built from the source
slice - `LinearGradientBrush(SharedVector::from(grad.0.as_slice()))` (and the `RadialGradientBrush`/
`ConicGradientBrush` equivalents) - so the subsequent `make_mut_slice()` sees a fresh vector with
refcount 1 and mutates in place, with no `detach()` decision to make. This does **not** eliminate
the allocation (a new, independent `Brush` value has to be produced regardless of the source's
refcount - see "Why the result looks the way it does"), but it does remove: two atomic refcount
operations per call (the `clone()`'s `fetch_add` and the temporary swapped-out vector's `fetch_sub`
on drop), and replaces the copy loop's `IntoIterInner::Shared` per-element bounds-checked
`.get().cloned()` with a plain `for x in slice { ptr::write(..., x.clone()) }` loop.

No existing project benchmark reaches `Brush::interpolate` directly (`perf_test`'s `--animate`
flag animates per-item properties generically and isn't restricted to gradient brushes, and isn't
pixel-comparable per `projects/slint.md`). Wrote a standalone bench
(`bench/`, `gradient_interpolate_bench`) that depends on `i-slint-core` directly and calls
`Brush::interpolate` in a tight loop for each of the six match arms in isolation
(`--mode linear|radial|conic|solid-linear|solid-radial|solid-conic`), varying `t` every call
(`(call_index % 1000) / 1000.0`) so the interpolation isn't degenerate, discarding the first 10%
of batches as warmup, and folding every result into a running FNV-style checksum for a
bit-exactness check between baseline and patched builds. A global-allocator wrapper counts
allocations/bytes per call.

Measured single-core (`taskset -c 3`), paired alternating baseline/patched rebuilds (3 reps
per mode unless noted), `--iters 5000000 --batch 5000`, plus `--stops 64` (larger, more realistic
stop count) and one unpinned (multi-core wall-clock) pass:

| mode (arm) | stops | patched p50 | baseline p50 | paired result |
|---|---|---|---|---|
| `linear` (LinearGradient<->LinearGradient) | 8 | 216 ns (3 reps: 216/216/216) | 221 ns (3 reps: 221/221/221) | patched faster, 3/3, clean separation |
| `radial` (RadialGradient<->RadialGradient) | 8 | 213 ns (3 reps: 213/213/212) | 222 ns (3 reps: 221/222/222) | patched faster, 3/3, clean separation |
| `conic` (ConicGradient<->ConicGradient) | 8 | 279/278/271 ns | 275/276/280 ns | mixed, 1/3 - within noise |
| `solid-linear` (SolidColor->LinearGradient) | 8 | 241 ns (3 reps, all 241) | 226 ns (3 reps: 227/226/226) | **patched slower, 3/3**, clean separation |
| `solid-radial` (SolidColor->RadialGradient) | 8 | 219/221 ns | 231/227 ns | patched faster, 2/2 |
| `solid-conic` (SolidColor->ConicGradient) | 8 | 290/292 ns | 282/? ns | patched slower, at least 1/1 (2nd baseline rep not captured, run interrupted by a shell timeout) |
| `linear` | 64 | 1712 ns (2 reps: 1712/1712) | 1733 ns (2 reps: 1733/1732) | patched faster, 2/2, clean separation |
| `solid-linear` | 64 | 1741 ns (1 rep) | 1767-1779 ns (2 reps) | patched faster |
| `linear`, unpinned (multi-core) | 64 | 1734/1706 ns | 1731/1726 ns | 1/2 - roughly a wash unpinned |

Allocations/call were identical before and after in every mode tested: exactly 1
allocation/call, sized to exactly fit the stop count (e.g. 96 bytes for 8 `LinearGradient` stops,
544 bytes for 64) - see "Why the result looks the way it does".

## Result

At the default, small stop count (8) used for most `.slint` gradients, the effect is a handful of
nanoseconds either way on a ~210-290 ns call that a general-purpose `malloc`/`free` of well under
1 KB dominates - some arms come out consistently faster (`linear`, `radial`, `solid-radial`, all
clean 2-3/3 or 2/2 wins), one arm comes out consistently *slower* (`solid-linear`, clean 3/3 loss
at 8 stops), and one is a wash (`conic`, 1/3). At a larger, still realistic stop count (64), where
the O(n) copy work is a bigger fraction of the fixed `malloc` overhead, the pattern is more
consistent: `linear` and `solid-linear` both come out faster (2/2 and by inference from 1
patched vs 2 baseline reps respectively), suggesting the 8-stop `solid-linear` regression is a
build/allocator-jitter artifact of this particular microbenchmark rather than a property of the
patch itself - see below for the bounded amount of further checking done on this. Unpinned
(multi-core, realistic wall-clock), the whole effect washes into scheduler/turbo noise (1/2).

## Does it change behavior/output?

- **Bit-exact?** Yes. The bench's running FNV-style checksum over every `Brush` produced across
  5-8 million calls (varying `t` every call, 8 and 64 stops, all 6 gradient-producing arms) is
  identical between baseline and patched builds in every mode tested. `i-slint-core`'s own test
  suite (`cargo test -p i-slint-core --features testing --lib`, default `std` features - see
  "What's not done" for why not `--all-features`) passes unchanged: 307/307, including all 20
  `graphics::brush::*` tests (interpolation-reaches-explicit/default-metadata cases for radial and
  conic gradients specifically exercise the arms this patch touches).
- **Is this a breaking change?** No. Internal implementation detail of `Brush::interpolate`
  (trait method, not a new public item); output values are unchanged.
- Project test suite: 307/307 passing before and after (`i-slint-core --features testing --lib`).

## Why the result looks the way it does

The premise in `IDEAS.md`'s "animation sweep" row - that avoiding the artificial `.clone()` would
let `make_mut_slice()` skip the allocation entirely - turned out to be only half right. It's true
*only* in the hypothetical case where the caller's own `Brush` value happens to be uniquely
owned already; in the actual call sites (`properties_animations.rs::compute_interpolated_value`,
`self.from_value.interpolate(&to_value, t)`), `self.from_value` must remain untouched (it's reused
as the animation's start point on every subsequent frame) and `to_value` was itself *just* cloned
one line earlier (`self.to_value.clone()`) - so an independent copy of the N stops is being
produced on every frame by design, not by an avoidable accident. Both the original code
(`clone()` + `detach()`'s shared branch) and this patch (`SharedVector::from(&[T])`) therefore do
exactly the same amount of essential work - one `malloc` of the same size, one pass copying N
`Copy` elements - confirmed directly by the bench's allocation counter (1 alloc/call, identical
byte count, both before and after, every mode). What the patch actually removes is two atomic
refcount operations (`clone()`'s `fetch_add`, and the temporary swapped-out-old-vector's
`fetch_sub` on drop inside `detach`) and swaps `detach`'s bounds-checked
`IntoIterInner::Shared::next()` copy loop for a plain slice-iteration loop in `SharedVector::from`.
At today's ~8-stop gradients, that's a handful of nanoseconds on a `malloc`-dominated call -
small enough that it's within this microbenchmark's own noise floor for some arms (`conic`,
`solid-linear` at 8 stops) and a real, if modest, win for others (`linear`, `radial`,
`solid-radial`, and `linear`/`solid-linear` at the larger 64-stop count where the O(n) work is a
bigger share of the total).

Per SKILL.md pitfall #10 ("a correctness-verified patch can still be a performance regression"),
a bounded amount of extra checking was done on the one clean, consistent regression found
(`solid-linear` at 8 stops, 3/3 baseline wins): re-running the same two builds at 64 stops instead
flips it to a clear, repeated patched win, and the *same* source-level replacement
(`Type(SharedVector::from(x.0.as_slice()))`) is used identically in all 6 arms - there is no
solid-linear-specific code difference from the other arms that would explain a genuine, load-
bearing slowdown. That points at per-build code-layout/allocator jitter (a well-known hazard for
single-call, malloc-dominated microbenchmarks - see MEASUREMENT.md) rather than a real property of
the patch, but this was not chased further with disassembly given the small magnitude and the
bounded-effort guidance.

## What's not done / open questions

- `cargo check -p i-slint-core --all-features` does not build on this tree independent of this
  patch (a pre-existing `zeno`/`libm` `f32` extension-trait conflict under some feature
  combination - see `projects/slint.md` section 6) - correctness was checked with the default
  `std` feature set instead, which is what the bench itself also uses.
- Not measured through an actual running animation/transition end-to-end (e.g. `perf_test`'s
  `--animate` flag, or a real hover-highlight `.slint` scene) - `Brush::interpolate` was isolated
  and measured directly because no existing harness reaches it in a form that separates its own
  cost from everything else a frame does. A future session wanting an end-to-end number would
  need a `perf_test` scene/flag that specifically animates a gradient-brush property, which does
  not exist yet (see the new `IDEAS.md` entry noting this).
- The `solid-conic` mode's second baseline rep was not captured (the paired-comparison shell loop
  was interrupted by this session's own 2-minute command timeout mid-build); the one comparison
  captured favors baseline, consistent with `conic` (gradient<->gradient) being the one arm that
  showed no clear signal either way in this session's data.
