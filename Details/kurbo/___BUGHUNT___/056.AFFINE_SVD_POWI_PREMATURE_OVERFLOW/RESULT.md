# AFFINE_SVD_POWI_PREMATURE_OVERFLOW

`Affine::svd` (`kurbo/src/affine.rs`, `pub(crate)`, backing the public `Affine::spectral_norm()`
and `Ellipse::radii()`/`x_radius()`/`y_radius()`/`rotation()`/`with_radii()`/`with_rotation()`)
squares intermediate sums via `.powi(2)` before taking a final `.sqrt()`. A large-but-perfectly-
representable uniform scale (e.g. `a = d = 1e200`, well under `f64::MAX ~= 1.7977e308`) makes the
intermediate squaring step (`(a+d).powi(2)` = `(2e200)^2` = `4e400`) overflow to `inf`, even though
the true mathematical result (`1e200`) is trivially representable. This looked suspicious as an
instance of the classic "square-then-sqrt" premature-overflow pattern in floating-point code
(HEURISTICS.md's arithmetic-on-untrusted-magnitudes class), reported as candidate 056.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction (debug build, `cargo run --example`) | `Affine::new([v,0,0,v,0,0]).spectral_norm()` swept over `v` in `{1e150, 1e153, 1e154, 1e155, 1e200, 1e300}`, plus the exact `1e200` input from the candidate report, plus `Ellipse::new((0,0),(1e200,1e200),0.0).radii()` | Confirmed on the first try: `spectral_norm()` and `radii()` both return `inf`/`Vec2{inf,inf}` for `v >= 1e154`; `v = 1e153` still returns the correct finite value. No second tool was needed - this is a deterministic, pure floating-point arithmetic bug with no adversarial/randomized input space, so escalating to miri/ASAN/fuzzing would add nothing (there is no `unsafe`, no untrusted memory, no parser here - just an arithmetic formula). |
| Extra boundary sweep after the fix (non-uniform scale, rotated large scale, value near `f64::MAX`, tiny subnormal-adjacent value) | debug build, `cargo run --example` against the patched tree | All finite and numerically correct (see "The fix" below) - confirms the fix isn't narrowly tailored to the exact `1e200` input |

## Status

**Confirmed**. Reproduces deterministically 2/2 runs (and, being pure floating-point arithmetic
with a fixed input and no randomness/allocator/thread dependency, will reproduce 100% of the time
on any IEEE-754-compliant `f64` implementation) in a debug build (`cargo run --example`) and again
in a release build via the standalone `repro/` (`cargo run --release`), both directly on this
crate's real public API (`Affine::spectral_norm()`, `Ellipse::radii()`).

**Fully reproducible**: **Yes**. This is a single-participant numeric bug: the large-magnitude
coefficients passed to `Affine::new`/`Ellipse::new` *are* the genuine external input a real caller
would supply (e.g. any application building a transform/ellipse from measured or computed
coordinates that happen to be very large), not a stand-in for a second participant's behavior.
`repro/` calls the real public `Affine::spectral_norm()` and `Ellipse::radii()` directly - no
internal/private API, no hand-constructed internal state standing in for another component.

## Class

logic (silent wrong-result / precision bug: a representable computation incorrectly reports `inf`)

## Root cause

In `Affine::svd` (`kurbo/src/affine.rs`, function body around what is currently lines 507-559):

```rust
let s1 = ((a + d).powi(2) + (b - c).powi(2)).sqrt();
let s2 = ((a - d).powi(2) + (b + c).powi(2)).sqrt();
```

This is mathematically `sqrt(x^2 + y^2)`, i.e. a Euclidean norm/hypotenuse computation. Squaring
`x` and `y` before summing and taking the square root is numerically unsafe for magnitudes anywhere
above `sqrt(f64::MAX) ~= 1.34e154`: `x.powi(2)` alone overflows to `inf` once `|x| > ~1.34e154`,
even though the final `sqrt(x^2+y^2)` result (which is on the order of `|x|` itself) is nowhere near
`f64::MAX` and would be perfectly representable if computed without the premature intermediate
overflow. `Affine::spectral_norm()` (`self.svd().0.x`) and every `Ellipse` method built on
`Affine::svd` (`radii()`, `x_radius()`, `y_radius()`, `rotation()`, `with_radii()`,
`with_rotation()`) inherit this because they all route through the same `svd()` call.

The standard fix for exactly this shape of computation is `f64::hypot`, which is specified/
implemented to avoid intermediate overflow/underflow for any finite, representable inputs whose
true hypotenuse is itself representable.

## Introduced by

This exact `powi(2)`/`sqrt()` pair (the code as it stands today) was introduced by
9386bd9e53ec91f32fe9a855bb13ae421143950d "Rework `Affine::svd` to only require two parallel square
roots (#499)" (2025-09-18), found via `git log -S"fn svd" -- '*affine.rs'` plus reading that
commit's diff.

However, the underlying bug class (squaring matrix coefficients before a final `sqrt`, causing the
same premature overflow at essentially the same boundary, roughly `1e154`-`1e155`) was already
present in the very first version of `Affine::svd`, introduced by
531daa7ce9bfb085a6f8fb352b27dd0e82e91a0f "Use an `Affine` map to represent `Ellipse`s" (2020-05-16):
that version computed `a2 = a*a` (etc.) unconditionally and used `s1 = a2+b2+c2+d2` directly, which
overflows for the same class of large-but-representable inputs (verified by reading that commit's
diff: `git show 531daa7:src/affine.rs`, though this was not re-run against a live build of that
2020-vintage commit, since the pre-#499 code is no longer present in current `main` and rebuilding
that old revision was judged unnecessary - the current code (post-9386bd9) is what's live-verified
above and what the fix targets).

The public `Affine::spectral_norm()` entry point itself (one of the two ways this bug is reachable)
was added later, by 77d6deb "Add nuclear/Frobenius/spectral norms to `Affine`, add interpretation
for `determinant` (#569)" - it simply calls the pre-existing, already-buggy `svd()`.

## The fix

Replace the manual `powi(2) + powi(2)).sqrt()` pattern with `f64::hypot`, which computes the same
mathematical quantity (`sqrt(x^2+y^2)`) without the premature intermediate overflow:

```rust
let s1 = (a + d).hypot(b - c);
let s2 = (a - d).hypot(b + c);
```

Verified against `repro/` (see `output.txt` for the actual captured before/after run via
`just test`, a real clone of the project at `base_commit` with `patch.diff` applied in between):
before the fix, both `spectral_norm()` and `Ellipse::radii()` return `inf` for the `1e200` input;
after the fix, both return the correct finite `1e200`. Also re-checked (per CONFIRMATION.md
section 7, not just the one triggering input) against: a non-uniform large scale
(`a=1e200, d=1e150`), a rotated large-scale transform, a value near `f64::MAX` (`f64::MAX/3`, still
finite and correct after the fix), and a tiny subnormal-adjacent value (`1e-200`, unaffected by
either version) - all produced finite, numerically correct results after the fix.

- Project test suite (`cargo test -p kurbo`): 187 unit tests + 55 doctests passing before the fix,
  187 unit tests + 55 doctests passing after the fix (0 failures either way; no new tests were
  added by this fix). Ran both directly in a git worktree of the project (not just inside
  `repro/`, which only depends on the library's public API).

## What's not done / open questions

- The `angle` computation two lines above (`let angle = 0.5 * (2.0 * (ab + cd)).atan2(a2 - b2 + c2
  - d2);`, using `a2 = a*a` etc. computed unconditionally at the top of `svd()`) has the exact same
  premature-squaring shape and the same overflow boundary (`a2` alone overflows once `|a| >
  ~1.34e154`). This wasn't part of this candidate's specific claim (which named only `s1`/`s2`) and
  wasn't independently confirmed/fixed this session - `atan2` with `inf`/`inf`-derived `NaN`
  arguments could plausibly return a wrong angle (or `NaN`) for the same large-magnitude inputs
  that trigger the `s1`/`s2` bug, without necessarily returning `inf` itself (since `atan2` is
  bounded), which is why it wasn't caught by the same finite-value check used above. This is a
  plausible sibling bug in the same function and is worth a follow-up candidate.
- Did not check whether any other code path in the crate independently re-implements a similar
  "square then sqrt" hypotenuse/norm pattern outside of `Affine::svd` (a broader sweep for this bug
  class across the crate was out of scope for this single candidate).
- No fuzzing/miri/ASAN was used - this is a pure, deterministic floating-point arithmetic bug with
  no `unsafe`, no untrusted memory, and no adversarial input space beyond magnitude, so manual
  boundary-value construction alone was sufficient and additional tools were judged not to add
  signal (per the soft-cap-of-3 guidance, one clean, deterministic, immediately-confirmed approach
  is enough here - there was no partial/flaky signal that would justify escalating further).
