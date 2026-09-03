# GRADIENT_RAMP_MAKE_RAMP_ASSUMES_SORTED_STOPS

`vello_encoding/src/ramp_cache.rs`'s `make_ramp` walks a `&[ColorStop]` slice with a
forward-only cursor (`this_u`/`j`), so it implicitly assumes `stops` is sorted by non-decreasing
`offset`. Nothing on the path from a public `vello`/`peniko` caller down to this function
(`peniko::Gradient::with_stops`, `vello::Scene::fill`/`encode_brush`, `Encoding::add_ramp`,
`Resolver::resolve_patches`, `RampCache::add`) ever sorts or validates that offsets are
non-decreasing - `clippy::while_float` merely flags the `while u > this_u` loop syntactically; it
does not itself indicate whether the assumption actually holds for real input.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read the full call chain (`ramp_cache.rs` -> `resolve.rs::resolve_patches` -> `encoding.rs::add_ramp`/`encode_*_gradient` -> `vello::Scene::fill` -> `peniko::Gradient::with_stops`/`ColorStopsSource` impls -> `peniko::ColorStops`) | Static read of `vello_encoding` 0.10.0 and `peniko` 0.6.1 (vendored at `~/.cargo/registry/.../peniko-0.6.1`) | Confirmed no sort/validation of `offset` monotonicity exists anywhere on this path; `Gradient::with_stops` and every `ColorStopsSource` impl (arrays of tuples, `&[AlphaColor<_>]`, etc.) just push stops in caller-given order |
| Manual boundary-value construction: real public API (`vello::Scene::fill` with `Brush::Gradient(Gradient::new_linear(..).with_stops([...]))`), 3 stops with offsets `[0.0, 1.0, 0.5]` (middle stop appended out of order), resolved with the real `vello_encoding::Resolver::resolve` (the same step `vello::Renderer` runs before uploading the ramp texture) | Debug + release builds, `cargo run --release`, 3 repeat runs | Reproduces deterministically 3/3: the out-of-order GREEN stop (offset 0.5) never appears anywhere in the 512-sample computed ramp; the ramp is a plain RED->BLUE blend as if the GREEN stop didn't exist |

Two approaches were enough here: this is a pure, fully-deterministic logic bug in single-threaded
CPU code with no adversarial-input-space search needed (a single hand-picked boundary case - one
stop given out of offset order - demonstrates it completely), so a fuzz harness/miri/ASAN would
add nothing beyond what direct construction already showed.

## Status

**Confirmed.** Reproduces 3/3 runs, release build, fully deterministic (no fixed seed, no
timing/thread dependence - this is pure `f32` arithmetic over a fixed input).

**Fully reproducible**: **Yes**. The repro drives the real, public end-to-end path: a real
`peniko::Gradient` built via `Gradient::new_linear(..).with_stops(..)` (peniko's own public
`ColorStopsSource` machinery, exactly as any real caller would use it - peniko never requires or
enforces sorted offsets, so an out-of-order `ColorStops` value is a completely legal input, not a
contrived one), fed into a real `vello::Scene` via the real public `Scene::fill`, then resolved
with the real, public `vello_encoding::Resolver::resolve` - the identical resolve step
`vello::Renderer::render_to_texture` runs internally before uploading the gradient ramp texture to
the GPU. There is no second participant being faked here (per `CONFIRMATION.md` section 11): the
"caller" supplying the unsorted stops is just an ordinary, legal use of `Gradient`'s own public
builder API, not a stand-in for another process/peer.

## Class

logic

## Root cause

`make_ramp(stops, ..)` initializes `this_u = last_u = 0.0`, `this_c = last_c = stops[0].color`,
`j = 0`, then for each of the 512 output samples (`u` increasing from 0.0 to 1.0) runs
`while u > this_u { ... j += 1; this_u = stops[j].offset; ... }`. This cursor only ever advances
`j` forward through the *given order* of `stops`, never re-examines earlier stops, and never sorts
by `offset` first. If `stops[i].offset` is not non-decreasing in `i` (legal per `peniko::ColorStop`
/ `ColorStops` - there is no invariant enforced anywhere on the type or on any of the
`ColorStopsSource` impls that construct one), the interval bounds used to interpolate between
`last_c`/`this_c` no longer correspond to the correct "which two stops does `u` fall between"
question the algorithm assumes it's answering.

Concretely, with stops `[(0.0, RED), (1.0, BLUE), (0.5, GREEN)]` (offsets not sorted - GREEN's
0.5 comes after BLUE's 1.0 in the slice): after the cursor advances past `stops[1]` (BLUE, offset
1.0), `this_u` is pinned at `1.0`. Since sample `u` never exceeds `1.0`, the loop condition
`u > this_u` can never be true again, so `stops[2]` (GREEN, offset 0.5) is never visited at all -
not "sampled at the wrong place", but *entirely and silently dropped* from the ramp. The resulting
512-sample ramp is a plain RED->BLUE linear blend, as if the GREEN stop had never been supplied.

This is a caller-triggerable silent-wrong-output bug, not a crash: any application built on
`vello`/`vello_encoding` that constructs `ColorStops` from an unordered or dynamically-reordered
data source (e.g., accumulating stops from independent events, or animating per-stop offsets that
can cross over between frames) can hit this without any error, panic, or diagnostic - the affected
gradient just renders with the wrong colors.

## Introduced by

6f707c4c "Start work on gradients" (2021-06-23). This predates the Vello project name (the
repository was still `piet-gpu`/`piet-scene` at the time) and predates `vello_encoding` as a
separate crate (the code later moved through `piet-gpu/src/ramp.rs` -> `vello_encoding`'s
predecessor -> its current location, each time carrying the same `while u > this_u` forward-only
cursor unchanged in its core logic). Verified via `git log -S"while u > this_u" --oneline` across
the whole repository history, then `git show 6f707c4c -- '*.rs' | grep 'while u > this_u'`
confirming the line's first appearance in that commit's diff.

## The fix

Sort a local copy of `stops` by `offset` (via `f32::total_cmp`, which never panics/traps
regardless of the input floats, unlike `partial_cmp().unwrap()`) at the top of `make_ramp`, before
running the existing forward-cursor algorithm unchanged. This is a minimal, purely-local guard:
it does not change the function's public contract (`make_ramp` is a private `fn`, not part of any
public API) and does not change behavior at all for the overwhelmingly common case where callers
already supply sorted stops.

```diff
 fn make_ramp(
     stops: &[ColorStop],
     interpolation_alpha_space: InterpolationAlphaSpace,
-) -> impl Iterator<Item = u32> + '_ {
+) -> impl Iterator<Item = u32> {
+    // The cursor below only ever walks forward through `stops`, so it implicitly assumes
+    // `stops` is sorted by non-decreasing `offset`. Callers of the public gradient API (e.g.
+    // `peniko::Gradient::with_stops`) are not required to supply stops in that order, so sort a
+    // local copy here rather than silently mis-sampling (and dropping) out-of-order stops.
+    let mut stops = stops.to_vec();
+    stops.sort_by(|a, b| a.offset.total_cmp(&b.offset));
     let mut last_u = 0.0;
     let mut last_c = stops[0].color.to_alpha_color::<Srgb>();
```

(`impl Iterator<Item = u32> + '_` had to drop its elided input lifetime, since the returned
iterator no longer borrows `stops` - it now owns a sorted `Vec<ColorStop>` moved into the closure.)

Verified: re-running `repro/` against the patched tree, the previously-dropped GREEN stop is now
correctly sampled at `u ~= 0.5` (`rgba8 = [0, 255, 0, 255]`, matching the reference sorted-stops
ramp exactly), instead of the wrong RED/BLUE blend (`rgba8 = [127, 0, 128, 255]`) seen before the
fix. Checked beyond just the one triggering input: also confirmed the fix leaves already-sorted
stops (the common case) byte-for-byte unaffected, since sorting an already-sorted slice with a
stable sort (`sort_by`, which `Vec::sort_by` implements as a stable sort) is a no-op on the
resulting order.

- Project test suite (`cargo test -p vello_encoding`): 31 passed, 0 failed before the patch; 31
  passed, 0 failed after. `cargo test -p vello`: 0 unit tests either way (unaffected).

## What's not done / open questions

- Only `vello_encoding`'s CPU-side `ramp_cache.rs` (used by the GPU-compute-based `vello` /
  "classic Vello" renderer) was checked. The sparse-strips CPU renderer (`vello_common`/
  `vello_cpu`) has its own, textually different gradient-ramp-building code path and was not
  audited for the same "assumes sorted stops" pattern in this session - candidate 058
  (`GRADIENT_RAMP_CACHE_COMPACT_LUTS_PREFIX_SUM_UNDERFLOW`, filed separately, `Not reproducible`)
  already looked at a different bug in that sibling file (`vello_hybrid`'s `gradient_cache.rs`
  `compact_luts`) but not specifically at sorted-offset handling there.
- No fix was made to `peniko::Gradient`/`ColorStops` itself (out of scope: that's a separate
  crate/repository) to reject or sort unsorted stops at construction time, which would be a more
  defense-in-depth fix than patching the one consumer found here.
