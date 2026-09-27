# VEC2_HYPOT_NORMALIZE_PREMATURE_OVERFLOW

`Vec2::hypot` (`kurbo/src/vec2.rs:91-94`) computes `self.hypot2().sqrt()`, i.e. `(x*x + y*y).sqrt()`,
deliberately avoiding `f64::hypot` per an inline comment ("Avoid f64::hypot as it calls a slow
library function"). A large-but-perfectly-representable component (e.g. `x = y = 1e200`, well under
`f64::MAX ~= 1.7977e308`) makes the intermediate squaring step (`1e200 * 1e200 = 1e400`) overflow to
`inf`, even though the true mathematical result (`~1.41421e200`) is trivially representable.
`Vec2::length` (an alias) and `Point::distance` (`(self - other).hypot()`) inherit the same bug.
`Vec2::normalize()` (`self / self.hypot()`) then divides by that `inf`, silently returning the zero
vector instead of a correct unit vector - a wrong-geometric-result footgun, not just an `inf`. This
is the same "square-then-sqrt" premature-overflow class already confirmed in candidate 056
(`Affine::svd`), but at a more foundational, more widely-reachable location (any curve/vector
operation built on `hypot`/`length`/`normalize`/`distance`). Reported as candidate 059.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction (debug build, `cargo run` against a small standalone probe crate depending on the project's checkout via a path dependency) | `Vec2::new(v, v).hypot()`/`.length()`/`.normalize()` and `Point::distance()` swept over `v` in `{1e100, 1e150, 1e153, 1e154, 1e155, 1e200, 1e300}`, plus the exact `1e200` input from the candidate report, plus edge cases (`0,0`; one component `0`; `f64::MAX,f64::MAX`; subnormal-range `MIN_POSITIVE`; `inf`/`NaN` combinations) | Confirmed on the first try: `hypot()`/`length()`/`distance()` all return `inf` for `v >= 1e154`, and `normalize()` silently returns the zero vector for the same inputs; `v = 1e153` still returns the correct finite value. No second tool was needed - this is a deterministic, pure floating-point arithmetic bug with no adversarial/randomized input space, no `unsafe`, and no untrusted memory - escalating to miri/ASAN/fuzzing would add nothing beyond what boundary-value construction already showed. |
| Extra boundary sweep after the fix (values from `1e100` to `1e300`, zero, subnormal-range, `f64::MAX`, `inf`, `NaN`, and mixed `inf`/`NaN` pairs), diffed directly against `f64::hypot`'s own output | debug build, standalone probe crate against the patched tree | All finite/correct where `f64::hypot` itself is finite/correct, and bit-for-bit-equal to `f64::hypot`'s output in every case tried, including the IEEE-754 special cases (`hypot(NaN, inf) == inf`, `hypot(NaN, finite) == NaN`) that a naive `x.max(y)`-based scaled implementation gets wrong (`f64::max`/`min` silently ignore `NaN`) - confirms the fix isn't narrowly tailored to the exact `1e200` input and doesn't introduce a new NaN-swallowing regression. |

## Status

**Confirmed**. Reproduces deterministically on every run (pure floating-point arithmetic with a
fixed input, no randomness/allocator/thread dependency - will reproduce 100% of the time on any
IEEE-754-compliant `f64` implementation); explicitly re-run 3/3 times via the standalone probe crate
during investigation, and again via `repro/`'s `just test` (fresh clone, release build) which is
captured verbatim in `output.txt`.

**Fully reproducible**: **Yes**. This is a single-participant numeric bug: the large-magnitude
coordinates passed to `Vec2::new`/`Point::new` *are* the genuine external input a real caller would
supply (e.g. any application building vectors/points from measured or computed coordinates that
happen to be very large), not a stand-in for a second participant's behavior. `repro/` calls the
real public `Vec2::hypot()`, `Vec2::length()`, `Vec2::normalize()`, and `Point::distance()` directly
- no internal/private API, no hand-constructed internal state standing in for another component.

## Class

logic (silent wrong-result / precision bug: a representable computation incorrectly reports `inf`,
which then propagates into a silently-wrong zero vector via `normalize()`)

## Root cause

In `Vec2::hypot` (`kurbo/src/vec2.rs`, currently lines 90-94):

```rust
pub fn hypot(self) -> f64 {
    // Avoid f64::hypot as it calls a slow library function.
    self.hypot2().sqrt()
}
```

where `hypot2()` is `self.dot(self)` = `x*x + y*y`. This is mathematically `sqrt(x^2 + y^2)`, i.e. a
Euclidean norm computation. Squaring `x` and `y` before summing and taking the square root is
numerically unsafe for magnitudes anywhere above `sqrt(f64::MAX) ~= 1.34e154`: `x*x` alone overflows
to `inf` once `|x| > ~1.34e154`, even though the final `sqrt(x^2+y^2)` result (on the order of `|x|`
itself) is nowhere near `f64::MAX` and would be perfectly representable if computed without the
premature intermediate overflow.

This is reachable from three public entry points that all route through the same buggy `hypot()`:

- `Vec2::hypot()` / `Vec2::length()` (alias) directly.
- `Point::distance()` (`kurbo/src/point.rs:70-72`, `(self - other).hypot()`).
- `Vec2::normalize()` (`kurbo/src/vec2.rs:181-183`, `self / self.hypot()`) - here the bug is worse
  than a bare `inf`: dividing a finite `x`/`y` by `inf` silently produces `0.0` for both components,
  so `normalize()` returns the *zero vector* instead of a unit vector, with no `inf`/`NaN` anywhere
  in the result to signal that anything went wrong.

The standard fix for exactly this shape of computation is a scaled/overflow-safe hypotenuse
(equivalent to what `f64::hypot` computes, but without the library-call overhead the introducing
commit specifically wanted to avoid).

## Introduced by

62287b55007821ecd1c9f9dca11676d7358482b4 "Avoid calling f64::hypot, it is slow (#448)" (2025-06-13),
found via `git log --follow -p -- src/vec2.rs` / reading that commit's diff. That commit replaced
the previously-correct `self.x.hypot(self.y)` (which delegated to the overflow-safe platform
`f64::hypot`) with `self.hypot2().sqrt()`, explicitly trading away `f64::hypot`'s overflow-avoidance
guarantee for speed (the commit message: "It turns out that f64::hypot calls a slow C standard
library function... The extra complexity most likely comes from how the C hypot function is
designed to avoid 'undue overflow or underflow at intermediate stages of the computation'" - i.e.
the tradeoff being made, and its consequence, was described accurately in the commit message itself,
just not accounted for as a correctness regression). A later commit, c7d8eaf "Remove stale comment
about platform hypot (#451)", only removed doc comments that had become inaccurate after 62287b5;
it did not touch the arithmetic.

## The fix

Replace the naive `hypot2().sqrt()` with a scaled hypotenuse that avoids the premature intermediate
overflow while still avoiding the `f64::hypot` library call the introducing commit specifically
wanted to avoid (i.e. it keeps the performance property that motivated 62287b5, without the
correctness regression it introduced):

```rust
pub fn hypot(self) -> f64 {
    // Avoid f64::hypot as it calls a slow library function. Instead, scale
    // by the larger component first so that squaring never overflows for
    // any finite input that the true magnitude would still fit in an f64.
    let x = self.x.abs();
    let y = self.y.abs();
    // `f64::max`/`min` ignore NaN, so infinities and NaNs need to be special-cased
    // to match `f64::hypot`'s IEEE-754-derived behavior (infinite if either input is
    // infinite, even if the other is NaN; NaN otherwise if either input is NaN).
    if x.is_infinite() || y.is_infinite() {
        return f64::INFINITY;
    }
    if x.is_nan() || y.is_nan() {
        return f64::NAN;
    }
    let max = x.max(y);
    let min = x.min(y);
    if max == 0.0 {
        0.0
    } else {
        let r = min / max;
        max * r.mul_add(r, 1.0).sqrt()
    }
}
```

The `is_infinite`/`is_nan` special cases were added after an initial version (bare `x.max(y)`/
`x.min(y)` with no special-casing) was checked against `f64::hypot(NaN, 1.0)`: `f64::max`/`f64::min`
silently ignore `NaN` (returning the other, non-NaN operand), which would have made the naive scaled
version silently return `1.4142...` instead of propagating `NaN` like `f64::hypot` does - a new,
different silent-wrong-result bug that would have been introduced by the fix itself had it not been
caught by widening the boundary-value sweep (per CONFIRMATION.md section 7) to include `NaN`/`inf`
inputs, not just the large-finite-magnitude class the original report named.

Verified against `repro/` (see `output.txt` for the actual captured before/after run via
`just test`, a real clone of the project at `base_commit` with `patch.diff` applied in between):
before the fix, `hypot()`/`length()`/`distance()` all return `inf` and `normalize()` returns the zero
vector for the `1e200` input; after the fix, all four return the correct finite/unit-vector result.
Also re-checked directly against `f64::hypot`'s own output (not just "did it stay finite") across a
wide sweep: `1e100` through `1e300`, `0.0`, one-zero-component, `f64::MIN_POSITIVE`, `f64::MAX`,
`inf`, `NaN`, and `inf`/`NaN` mixed pairs - bit-for-bit equal to `f64::hypot` in every case tried.

- Project test suite (`cargo test --workspace`, run directly in the git worktree used for this
  candidate, not just inside `repro/` which only depends on the library's public API): 187 unit
  tests + 55 doctests passing before the fix, 187 unit tests + 55 doctests passing after the fix (0
  failures either way; no new tests were added by this fix). Also confirmed the crate still builds
  under `--no-default-features --features libm` (the `no_std`/`libm` configuration) both before and
  after, since `x.max(y)`/`x.min(y)`/`.mul_add()`/`.abs()`/`.is_infinite()`/`.is_nan()` all remain
  available in that configuration. `cargo clippy -p kurbo --all-targets` shows no new warnings from
  this change (the one pre-existing warning it reports is an unrelated `needless_late_init` in
  `solve.rs`, present identically before and after this patch).

## What's not done / open questions

- Did not check whether any other code path in the crate independently re-implements a similar
  "square then sqrt" hypotenuse/norm pattern outside of `Vec2::hypot` - candidate 056
  (`Affine::svd`) already confirmed one sibling instance; a broader sweep for every occurrence of
  this bug class across the whole crate was out of scope for this single candidate.
- No fuzzing/miri/ASAN was used - this is a pure, deterministic floating-point arithmetic bug with
  no `unsafe`, no untrusted memory, and no adversarial input space beyond magnitude, so manual
  boundary-value construction alone was sufficient (per the soft-cap-of-3 guidance, one clean,
  deterministic, immediately-confirmed approach plus one confirmatory re-check is enough here -
  there was no partial/flaky signal that would justify escalating further).
- The fix's IEEE-754 special-casing (`inf`/`NaN` handling) was checked against `f64::hypot`'s
  documented/observed behavior on this platform (`x86_64-unknown-linux-gnu`, glibc's `hypot`) but
  not cross-checked against every platform kurbo supports (e.g. `no_std`/`libm`'s own `hypot`
  implementation, which this fix intentionally avoids calling at all) - the fix does not call
  `f64::hypot`/`libm::hypot` anywhere, so this is a completeness note about the *reference* used for
  comparison, not a gap in the fix's own portability.
