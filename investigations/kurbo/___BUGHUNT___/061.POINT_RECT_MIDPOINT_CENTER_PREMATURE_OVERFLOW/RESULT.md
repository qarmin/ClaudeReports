# POINT_RECT_MIDPOINT_CENTER_PREMATURE_OVERFLOW

`Point::midpoint` (`kurbo/src/point.rs:63`) and `Rect::center` (`kurbo/src/rect.rs:174`) both
compute `0.5 * (a + b)`, i.e. summing the two operands before scaling by `0.5`. Two same-sign
operands that are each individually well within finite `f64` range (e.g. `1.5e308` and `1.7e308`,
both far below `f64::MAX ~= 1.7977e308`) can sum past `f64::MAX` before the `0.5` scaling is ever
applied, overflowing the intermediate sum to `inf` even though the true mathematical midpoint
(`1.6e308`) is trivially representable. `cargo clippy`'s `manual_midpoint` lint (nursery-derived,
promoted to a normal warn-by-default lint in current clippy) flags both sites by name and suggests
the overflow-safe `f64::midpoint`. This is the same premature-overflow bug class already confirmed
in candidates 056 (`Affine::svd`, square-then-sqrt) and 059 (`Vec2::hypot`, square-then-sqrt), here
manifesting via `+` instead of `*`. Reported as candidate 061.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction (release build, standalone probe crate depending on the project's checkout via a path dependency) | `Point::midpoint()`/`Rect::center()` swept over same-sign pairs from `1e150` to `f64::MAX`, plus the exact `(1.5e308, 1.7e308)` input from the candidate report, plus mixed-sign, negative-same-sign, zero, `f64::MAX`, `NaN`, and `inf` inputs; also checked downstream propagation into `CubicBez::subdivide`/`QuadBez::subdivide` | Confirmed on the first try: both `midpoint()` and `center()` return `(inf, ...)` for same-sign pairs at/above `9e307`, while the mathematically-correct midpoint (`8.9e307`, one step below) still returns the correct finite value - a deterministic, pure floating-point arithmetic bug with no adversarial/randomized input space, no `unsafe`, and no untrusted memory. No second tool was needed, matching the precedent set by candidates 056/059 for this exact bug class. |
| `cargo clippy -p kurbo --all-features -- -W clippy::manual_midpoint` (static analysis, confirming the candidate's own claim) | debug build | Flags exactly these two sites by name (`kurbo/src/point.rs:63` and `kurbo/src/rect.rs:174`), plus 15 other unrelated `0.5 * (a + b)`-shaped sites elsewhere in the crate (see "What's not done" below) - confirms the report's premise and scopes this candidate to the two named sites only. |
| Re-run of the standalone probe crate (3 repeat runs, same build/input) | release build | Identical `(inf, 0.0)` output for `Point::midpoint`/`Rect::center` on the `(1.5e308, 1.7e308)` input on all 3 runs - fully deterministic, no flakiness. |

## Status

**Confirmed**. Reproduces deterministically on every run (pure floating-point arithmetic with a
fixed input, no randomness/allocator/thread dependency - will reproduce 100% of the time on any
IEEE-754-compliant `f64` implementation); explicitly re-run 3/3 times via the standalone probe
crate during investigation, and again via `repro/`'s `just test` (fresh clone, release build),
captured verbatim in `output.txt`.

**Fully reproducible**: **Yes**. This is a single-participant numeric bug: the large-magnitude
coordinates passed to `Point::new`/`Rect::new` *are* the genuine external input a real caller would
supply (e.g. any application building points/rects from measured or computed coordinates that
happen to be very large), not a stand-in for a second participant's behavior. `repro/` calls the
real public `Point::midpoint()` and `Rect::center()` directly - no internal/private API, no
hand-constructed internal state standing in for another component.

## Class

logic (silent wrong-result bug: a representable computation incorrectly reports `inf` instead of
the correct finite midpoint)

## Root cause

In `Point::midpoint` (`kurbo/src/point.rs`, line 63):

```rust
pub const fn midpoint(self, other: Point) -> Point {
    Point::new(0.5 * (self.x + other.x), 0.5 * (self.y + other.y))
}
```

and identically in `Rect::center` (`kurbo/src/rect.rs`, line 174):

```rust
pub const fn center(&self) -> Point {
    Point::new(0.5 * (self.x0 + self.x1), 0.5 * (self.y0 + self.y1))
}
```

Both compute the midpoint as "sum, then scale by half", which is mathematically equivalent to the
correct answer only in infinite/real-number arithmetic. In `f64` arithmetic, `self.x + other.x` is
evaluated as its own intermediate step *before* the `0.5 *` scaling is applied - so if the two
same-sign operands individually sit anywhere in roughly `(f64::MAX / 2, f64::MAX]` (concretely,
confirmed via the boundary sweep: any same-sign pair at or above `9e307`), their sum alone
overflows to `inf`, and `0.5 * inf` stays `inf`. The true mathematical result (on the order of the
inputs themselves) is nowhere near `f64::MAX` and would be perfectly representable if computed
without the premature intermediate overflow - e.g. the correct midpoint of `1.5e308` and `1.7e308`
is `1.6e308`, comfortably finite, but `1.5e308 + 1.7e308 = 3.2e308` overflows `f64::MAX`'s
`~1.7977e308` bound before the halving ever happens.

This is the identical bug class already confirmed in candidate 056 (`Affine::svd`,
`((a+d).powi(2)+...).sqrt()`, overflow via squaring) and candidate 059 (`Vec2::hypot`,
`(x*x+y*y).sqrt()`, overflow via squaring) - here the premature intermediate overflow happens via
`+` on two same-sign large operands instead of via `*`/`.powi(2)` on one large operand.

`cargo clippy`'s `manual_midpoint` lint (documented at
https://rust-lang.github.io/rust-clippy/rust-1.98.0/index.html#manual_midpoint) independently
flags both sites, confirming this is a recognized, named bug pattern with a standard mitigation
(`f64::midpoint`, stabilized in Rust 1.85 as a scaled/overflow-safe midpoint that never overflows
for any finite input pair whose true midpoint is itself finite).

## Introduced by

Present since the very earliest introduction of each function - not a later regression:

- `Point::midpoint`: e7bc38ca8122a2393a206240ecffabec3008b250 "Be systematic in vector types"
  (2019-06-19) - the commit that first added `Point::midpoint` to the crate, already with this
  exact `0.5 * (self.x + other.x)` formula.
- `Rect::center`: b6473de40217da2f75e10f4c3968d72ef0c92658 "Add rectangle datatype"
  (2019-01-02) - the commit that first added `Rect::center` (then returning `Vec2`, later changed
  to `Point`), already with this exact `0.5 * (self.x0 + self.x1)` formula.

Found via `git log --follow --diff-filter=A` / `git log -S` on each function's own text, tracing
back through every subsequent rename/signature change (`&self` -> `self`, `fn` -> `const fn`,
`Vec2` -> `Point` for `Rect::center`) - none of those later commits touched the arithmetic itself.

## The fix

Replace the naive "sum then scale" with `f64::midpoint` (stable since Rust 1.85, which is already
this crate's declared `rust-version`), which computes the same result via a scaled/overflow-safe
algorithm that never overflows for any finite operand pair whose true midpoint is itself finite:

```rust
// point.rs
pub const fn midpoint(self, other: Point) -> Point {
    Point::new(self.x.midpoint(other.x), self.y.midpoint(other.y))
}

// rect.rs
pub const fn center(&self) -> Point {
    Point::new(self.x0.midpoint(self.x1), self.y0.midpoint(self.y1))
}
```

Both functions stay `const fn` - `f64::midpoint` is itself a stable `const fn` on this toolchain
(verified directly: a `const fn` wrapper calling `f64::midpoint` compiles and evaluates correctly
under `rustc 1.98.0`), so no existing caller relying on `const`-context usage (e.g.
`RoundedRect::from_rect_scaled`-style code calling `Rect::center()` inside another `const fn`) is
broken.

Verified against `repro/` (see `output.txt` for the actual captured before/after run via
`just test`, a real fresh clone of the project at `base_commit` with `patch.diff` applied in
between): before the fix, both `Point::midpoint()` and `Rect::center()` return `(inf, 0.0)` for the
`(1.5e308, 1.7e308)` input (exit code 1, `REPRODUCED` lines present); after the fix, both return
the correct `(1.6e308, 0.0)` (exit code 0, no `REPRODUCED` lines).

Checked against more than the one triggering input (CONFIRMATION.md section 7): a full boundary
sweep from `1e150` up to `f64::MAX`, negative-same-sign pairs (`(-1.5e308, -1.7e308)`,
`(-1.7e308, -1.5e308)`), mixed-sign pairs (`(1.7e308, -1.7e308)`), zero, ordinary small values, and
`NaN`/`inf` inputs - all stayed correct/finite where mathematically expected, and `NaN`/`inf`
inputs still correctly propagate `NaN`/`inf` (no new NaN-swallowing or masking introduced by the
fix, since `f64::midpoint`'s own semantics for those cases match plain `+`/`*` arithmetic on
non-overflowing inputs).

- Project test suite (`cargo test --workspace`, run directly in the git worktree used for this
  candidate): 187 unit tests + 10 `polycool` tests + 55 doctests passing before the fix (0
  failures, 1 ignored doctest), identical counts and 0 failures after the fix. `cargo clippy -p
  kurbo --all-targets --all-features` shows no new warnings from this change (the one pre-existing
  warning it reports, `needless_late_init` in `common.rs`, is present identically before and after
  this patch, same as noted in candidate 059's `RESULT.md`). Also confirmed the crate still builds
  under `--no-default-features --features libm` (the `no_std`/`libm` configuration) both before and
  after, since `f64::midpoint` is a `core`-level arithmetic method (not a transcendental function
  requiring `libm`) and is available identically in both configurations.

## What's not done / open questions

- `cargo clippy -- -W clippy::manual_midpoint` also flags 15 other `0.5 * (a + b)`/`(a + b) / 2.0`
  sites across the crate (`affine.rs:554`, `common.rs:227,693,715`, `ellipse.rs:450`, `fit.rs:79,
  203,419`, `mindist.rs:42,43`, `offset.rs:460`, `stroke.rs:553`, `svg.rs:436,437`) that share the
  identical premature-overflow shape but were out of scope for this candidate (which targets only
  the two sites named in the candidate report, `Point::midpoint` and `Rect::center`) - each is a
  legitimate, separate candidate for the same bug class and should be triaged individually rather
  than bundled into one patch (per the skill's "one bug per candidate folder" rule).
- The propagation into `CubicBez::subdivide`/`QuadBez::subdivide` mentioned in the candidate report
  is only **partially** fixed by this patch: both functions call `self.p0.midpoint(self.p1)` (now
  fixed) but also compute a midpoint-shaped point via `self.eval(0.5)` and a separate weighted-sum
  blend formula (`(p0 + p1*2 + p2) * 0.25`-shaped) that do **not** go through `Point::midpoint` at
  all - these still overflow for the same extreme-magnitude inputs (see `output.txt`'s last
  section) and are a distinct, unaddressed instance of the same bug class in `eval`'s own
  Bernstein-basis arithmetic, not covered by this fix.
- No fuzzing/miri/ASAN was used - this is a pure, deterministic floating-point arithmetic bug with
  no `unsafe`, no untrusted memory, and no adversarial input space beyond magnitude, so manual
  boundary-value construction alone was sufficient (matching the precedent set by candidates
  056/059 for this exact bug class).
