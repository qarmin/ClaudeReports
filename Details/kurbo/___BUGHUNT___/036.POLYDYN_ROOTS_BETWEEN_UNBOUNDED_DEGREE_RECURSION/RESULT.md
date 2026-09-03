# POLYDYN_ROOTS_BETWEEN_UNBOUNDED_DEGREE_RECURSION

`polycool/src/poly_dyn.rs`'s `PolyDyn::roots_between_with_buffer` (public API, also reachable via
`PolyDyn::roots_between`, both re-exported from `polycool/src/lib.rs:42`) recurses once per degree
of derivative taken, bottoming out only once `self.degree() <= 3`. `PolyDyn` wraps an uncapped
`Vec<f64>` of coefficients (unlike `mindist.rs`'s `u8` degree cap in already-investigated
candidates 006/019/030), so nothing stops a caller from constructing a polynomial whose degree
alone drives the native call stack past its limit - structurally the same class of bug as
already-confirmed candidate 022 (`mindist::min_dist_param`'s unbounded recursion), but a distinct
location whose recursion depth is tied directly to the polynomial's degree rather than to an
accuracy parameter.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Direct execution against the real public API (`PolyDyn::new(vec![0.0; N + 1]).roots_between(-1.0, 1.0, 1e-6)`), debug build | `N = 200_000` | Reproduced immediately - process aborts with "has overflowed its stack" (SIGABRT) |
| Boundary-value probing to find a build/stack-size-independent, fast-and-deterministic degree | Varied `N` from 50 up to 2,000,000 across default (8 MiB), and artificially shrunk (down to 128 KiB via `ulimit -s`) stack sizes, both debug and release profiles | At the default 8 MiB stack, `N = 50,000` overflows reliably and fast in both debug and release builds (large `N`, e.g. 200,000+, sometimes instead ran for a very long time before crashing/being killed, because each recursion level also does O(current degree) work building its own derivative coefficients on the way down - `N = 50,000` avoids that confound and gives a clean, fast, deterministic crash) |
| Repeat runs, same build/input | `N = 50,000`, 3/3 debug runs and 3/3 release runs (separately) | 6/6 total runs aborted with "has overflowed its stack", deterministic |

Direct execution reproduced deterministically on the first real attempt, so no further escalation
(miri/ASAN/fuzzing) was needed - this is a plain, unbounded-recursion stack exhaustion, not a
memory-safety/UB question miri or a sanitizer would add signal to.

## Status

**Confirmed**. Reproduces deterministically: 3/3 debug-build runs and 3/3 release-build runs at
`N = 50,000`, default 8 MiB stack, default environment, same input each time (see `output.txt`).

**Fully reproducible**: **Yes**. The repro calls `polycool`'s real, unmodified public
`PolyDyn::new`/`PolyDyn::roots_between` with an ordinary `Vec<f64>` of a chosen length - the
complete external input is a single integer (the coefficient count) supplied directly by the
immediate caller of a public API, not something produced by an upstream peer/protocol, so there is
no second participant to fake here (`CONFIRMATION.md` section 11's "single external artifact, no
other side" case applies directly, same as candidate 022's).

## Class

crash (stack overflow / process abort, both debug and release builds)

## Root cause

`PolyDyn::roots_between_with_buffer` (polycool/src/poly_dyn.rs:156-212, pre-fix) first checks
whether `self.degree() <= 2` (`to_quadratic`) or `<= 3` (`to_cubic`) and, if so, resolves the roots
directly with no further recursion. Otherwise it computes `self.deriv()` and calls
`deriv.roots_between_with_buffer(...)` recursively on that lower-degree polynomial, using the
lower-degree polynomial's own critical points (found by *its* recursive call) to bracket root
intervals for `self`. Each recursive call reduces the degree by exactly 1, and there is no
recursion-depth counter or any other backstop - the only way the recursion ever stops (short of the
degree-<=3 base case) is the already-existing `if !deriv.is_finite() { return; }` early-out,
which only fires if a derivative's own coefficients overflow to non-finite values, not merely
because there have been many levels of recursion. Since `PolyDyn`'s coefficient vector has no
upper-length cap (unlike `mindist.rs`'s deliberately-capped `u8` degree, candidates 006/019/030),
a caller can construct a `PolyDyn` with an arbitrarily large number of (e.g. all-zero, perfectly
finite) coefficients and call `roots_between`/`roots_between_with_buffer` on it, driving recursion
`degree() - 3` levels deep with nothing to stop it before the native call stack is exhausted.

## Introduced by

0786080 "Bring poly-cool into the kurbo repo (#502)" (2025-10-18) - this is the commit that first
brought the `poly-cool` crate (and its `poly_dyn.rs`, unchanged in this respect since) into the
`kurbo` repo's own tracked history; `git show 0786080:polycool/src/poly_dyn.rs` already shows the
identical unbounded `deriv.roots_between_with_buffer(...)` recursion with no depth counter. The bug
may predate this within the original standalone `poly-cool` crate's own history, which isn't part
of this repository's tracked history and wasn't checked.

## The fix

Applied directly to `polycool/src/poly_dyn.rs` in a scratch worktree at
ca273499e3e48bd2de6f02aa8e99a148984e45f3 (see `patch.diff`):

- Split the public `roots_between_with_buffer` (unchanged signature, so `PolyDyn::roots_between`
  and every other call site need no changes) into a thin wrapper that calls a new private
  `roots_between_with_buffer_rec`, adding one extra parameter: `depth: u32`, starting at `0` and
  incremented by 1 on every recursive call.
- Just before computing the next derivative, the function now checks `depth >=
  MAX_RECURSION_DEPTH` and, if so, returns immediately with an empty result - exactly the same
  early-out shape already used for the pre-existing `!deriv.is_finite()` case, so callers one
  level up already handle "the recursive call below produced nothing" gracefully (they fall back
  to treating only the interval's own endpoints as a single bracketing interval).
- `MAX_RECURSION_DEPTH = 200` was chosen empirically from the boundary-value probing above: even
  with an artificially shrunk 128 KiB stack (far smaller than any realistic default, which is
  1-8 MiB), recursion up to depth ~200-500 completed without overflowing in that probing; 200
  leaves comfortable headroom below that, while remaining far larger than any degree this
  quadratic-time algorithm is practical for in the first place (the type's own doc comment already
  notes run time is quadratic in the degree).

Verified against `repro/`, before and after, release build (see `output.txt`):

- Before: `PolyDyn::new(vec![0.0; 50_001]).roots_between(-1.0, 1.0, 1e-6)` aborts with "has
  overflowed its stack" (3/3 debug runs, 3/3 release runs).
- After: the same call, and one at degree 2,000,000 (40x higher), both return normally with an
  empty root list instead of crashing.

- Project test suite (`cargo test --workspace`): 252 passed / 0 failed before the fix, 252 passed
  / 0 failed after the fix (`polycool`'s own suite: 10/10 both before and after). No new failures.

## What's not done / open questions

- The fix caps recursion depth by simply giving up (returning an empty/partial root list) once
  `MAX_RECURSION_DEPTH` is exceeded, mirroring the pre-existing `!deriv.is_finite()` behavior -
  for a legitimately huge-degree polynomial (however impractical given the algorithm's quadratic
  time complexity) this silently drops roots rather than erroring or reporting truncation. A
  maintainer may prefer a different policy (returning a `Result`, panicking with a clear message,
  or documenting the cap) - this wasn't decided here, only that the process must not crash.
- The standalone `poly.rs` (`Poly<N>`/`Quartic`/`Quintic`) and `cubic.rs` implementations of the
  same `roots_between_with_buffer` pattern (see `polycool/src/poly.rs:140-159`,
  `polycool/src/cubic.rs:156-163`) use `const N`/fixed-size arrays rather than a `Vec`, so their
  degree is fixed at compile time and this specific unbounded-recursion class does not apply to
  them - not re-checked in depth this session beyond confirming they're generic-const-bounded, not
  `Vec`-backed.
- Whether the same bug is present in `poly-cool`'s own pre-kurbo history (outside this repo's
  tracked commits) was not investigated.
