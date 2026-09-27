# flatten()/CubicBez::to_quads capacity overflow (and a sibling unbounded loop) for zero/degenerate tolerance

`CubicBez::to_quads(&self, accuracy)` (`kurbo/src/cubicbez.rs:94`) computes a segment count
`n = ((err / max_hypot2).powf(1./6.0).ceil() as usize).max(1)` with `max_hypot2 = 432.0 * accuracy
* accuracy` and no validation that `accuracy` is a normal, positive, finite `f64`. This looked
suspicious as a direct sibling of already-confirmed candidate 005
(`ARC_APPEND_ITER_UNBOUNDED_TOLERANCE`, `kurbo/src/arc.rs`) - candidate 005's own "What's not
done" section explicitly flagged this exact line as an unchecked suspect with "a superficially
similar shape". Unlike `Arc::append_iter`, this `n` doesn't just become an iterator's own loop
bound - it also feeds `ToQuads::size_hint()` (`kurbo/src/cubicbez.rs:799-802`), which is read
directly by the crate's own public `kurbo::flatten()` entry point (`kurbo/src/bezpath.rs:664-666`)
to call `quad_buf.reserve(iter.size_hint().0)`, turning an `f64`-saturated `usize::MAX` into an
immediate, deterministic `Vec::reserve` panic instead of an unbounded loop. Two structurally
identical, unguarded `n` computations at the `QuadTo` and cubic-arm call sites in `flatten()`
itself (`kurbo/src/bezpath.rs:644` and `:674`) were bundled into this same candidate as an
equivalent mechanism, since they're reached through the exact same public entry point with the
exact same `tolerance` parameter.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction (pure arithmetic, no crate involved) | Standalone `rustc -O` snippet computing `max_hypot2`/`err/max_hypot2`/`.powf(1./6.0)`/`.ceil() as usize` by hand for `accuracy=0.0`, `err=1.0` (an arbitrary nonzero value standing in for any non-degenerate cubic) | `max_hypot2=0`, `ratio=inf`, `pw=inf`, `ceil=inf`, `ceil as usize = 18446744073709551615` (`usize::MAX`) - confirms the saturating-cast mechanism in isolation. |
| Real end-to-end run of the public API (`kurbo::flatten`), debug build (`debug-assertions = true`, `overflow-checks = true`, the project's own default dev profile) | Standalone crate depending on `kurbo` via a path dependency to this session's worktree, `flatten(path_with_one_curve_to, 0.0, |_| {})` | Panics deterministically: `thread 'main' panicked at .../alloc/src/raw_vec/mod.rs:28:5: capacity overflow`, backtrace pinpoints `kurbo::bezpath::flatten` at `kurbo/src/bezpath.rs:666:30` (the `quad_buf.reserve(...)` call). Reproduced 4/4 runs (3 explicit repeats plus the original), and confirmed in *both* debug and release builds - `Vec::reserve`'s capacity-overflow check is unconditional, not gated by `debug-assertions`. |
| Real end-to-end run of the public API, targeted boundary-value sweep across the two `bezpath.rs` `n` computations (via temporary `eprintln!` instrumentation of `estimate_subdiv`/the `QuadTo` arm, reverted before finishing the candidate) | Same path dependency, varying quad control-point shapes at `tolerance=0.0` | The `QuadTo` arm (`bezpath.rs:644`) is *not* uniformly unsafe: for symmetric "hump" quads, `estimate_subdiv`'s internal cusp-branch division by `sqrt_tol` produces `0.0/approx_parabola_integral(0.0)` = `NaN` (not `+inf`), and `NaN as usize` is documented to saturate to `0`, so `n.max(1) == 1` - a safe no-op, not a crash or a hang. A specific asymmetric shape (`(0,0)-(0,1)-(0.5,1.5)`) was found where `x0.signum() == x2.signum()` (both `+1.0`, since `f64::signum(0.0) == 1.0`), taking the *other* branch whose `val` doesn't depend on `sqrt_tol` at all - for that shape, `0.5 * val / sqrt_tol` is a genuine `+inf` for `tolerance=0.0`, and the `for i in 1..n` loop (no `Vec::reserve` at this call site) spins without terminating in any practical time instead of panicking. The cubic arm's own second `n` (`bezpath.rs:674`) was confirmed unreachable via `tolerance=0.0` specifically, because the earlier `to_quads`/`reserve` panic at line 666 always fires first for any `CurveTo` before that line is ever reached. |
| Standalone `repro/` (own `Cargo.toml`, bare `[workspace]`) against a real, fresh `git clone` of the project at the exact base commit, release build, run via the candidate's own `justfile` (`just clone && just reproduce-before`) | `cargo run --release`, no cutoff needed for case A (deterministic panic); case B used a 5,000,000-element / 5s wall-clock cutoff, same technique candidate 005 used | Case A: panics with `capacity overflow` exactly as above. Case B: cutoff fired at 5,000,000 elements in ~90ms without the loop finishing on its own - see `output.txt`. |

No further tool escalation (miri/ASAN/fuzzing) was needed: this is a pure floating-point
arithmetic/saturating-cast boundary-value bug in safe Rust with no `unsafe` involved anywhere on
the path, so the manual + real-API approaches above already gave a complete, repeatable picture
(see `HEURISTICS.md`'s guidance for "arithmetic on untrusted sizes/casts between integer widths").

## Status

**Confirmed**. Case A (the primary, originally reported mechanism: `CurveTo` + `tolerance=0.0` ->
`Vec::reserve` capacity-overflow panic) reproduces deterministically, 4/4 runs, in both debug and
release builds, both via an ad hoc path-dependency crate and via the candidate's own standalone
`repro/` run against a genuinely fresh clone (`just clone && just reproduce-before`). Case B (the
bundled `QuadTo`-arm sibling: `tolerance=0.0` on a specific non-symmetric quad shape -> unbounded
loop) reproduces deterministically as "cutoff fires, iterator never finishes on its own" every run
performed (3/3 explicit runs plus the `repro/` run) - inherently a "doesn't terminate in the tested
window" result rather than a fixed count, same caveat candidate 005's own unbounded-loop finding
stated.

**Fully reproducible**: **Yes**. The repro calls `kurbo::flatten` directly - the real, public
entry point any renderer or path-processing caller built on this crate would use, with `tolerance`
passed straight through as an ordinary `f64` parameter. There is no second participant/peer/wire
format involved to fake here - this is a single external artifact (a degenerate parameter value)
handed to a parser-like public API, the same carve-out `CONFIRMATION.md` section 11 documents for
"a malformed file handed to a parser's real public `open_file`/`from_reader` entry point".

## Class

crash (case A: deterministic `Vec::reserve` panic) / logic-hang (case B: unbounded loop, no crash,
no progress toward termination - same resource-exhaustion class as candidate 005's finding)

## Root cause

```rust
// kurbo/src/cubicbez.rs:90-94
let max_hypot2 = 432.0 * accuracy * accuracy;
let p1x2 = 3.0 * self.p1.to_vec2() - self.p0.to_vec2();
let p2x2 = 3.0 * self.p2.to_vec2() - self.p3.to_vec2();
let err = (p2x2 - p1x2).hypot2();
let n = ((err / max_hypot2).powf(1. / 6.0).ceil() as usize).max(1);
```

`accuracy` is never validated (not checked for `<= 0.0`, not checked for non-finiteness). For
`accuracy == 0.0`, `max_hypot2` is exactly `0.0`; for any non-degenerate cubic (`err > 0.0`),
`err / max_hypot2` is `+inf` by IEEE-754 division, which propagates unchanged through
`.powf(1./6.0)` and `.ceil()`. Rust's `f64 as usize` cast is a *saturating* cast (not a panic, not
a wrap), so `inf as usize` becomes `usize::MAX` silently. This `n` becomes `ToQuads`'s own field,
and `ToQuads::size_hint()` (`kurbo/src/cubicbez.rs:799-802`) reports `(n, Some(n))` unchanged - so
any caller that reads `size_hint()` to pre-size a buffer inherits the saturated value directly.
`kurbo::flatten()`'s cubic arm (`kurbo/src/bezpath.rs:664-666`) does exactly that:
`quad_buf.reserve(iter.size_hint().0)` - and `Vec::reserve`'s capacity-overflow check
(unconditional, not gated by `debug-assertions`, see `alloc::raw_vec::capacity_overflow`) panics
immediately, before a single quadratic is ever produced by the iterator.

The two `bezpath.rs` call sites bundled into this same candidate (`:644`, the `QuadTo` arm's own
`n`, and `:674`, the cubic arm's second `n` used for the point-emission loop) share the identical
`(something / sqrt_tol_derived_value).ceil() as usize` shape for the same reason (`sqrt_tol =
tolerance.sqrt()` is `0.0` for `tolerance == 0.0`), but manifest differently: neither is read via
`size_hint()`/`reserve` - both instead directly bound a `for` loop that calls the user's callback,
so a saturated `n` there means "never observably terminates", not "panics". Whether that division
actually reaches `+inf` (rather than the safely-clamped `NaN` case demonstrated for symmetric
quads) depends on which branch of `QuadBez::estimate_subdiv`'s cusp-detection (`x0.signum() ==
x2.signum()`) the specific quad shape takes - the `val` computed in the non-cusp branch doesn't
depend on `sqrt_tol` at all, so dividing that finite, tolerance-independent `val` by `sqrt_tol ==
0.0` is unconditionally `+inf` for that branch, regardless of quad shape specifics beyond which
branch is taken.

## Introduced by

d536e6d6ca0f23c77c3b07d3aba5bbdf1fbc85b5 "Slight simplification of cubic to_quads" (2020-01-29).
Verified via `git log -S"err / max_hypot2).powf" -- kurbo/src/cubicbez.rs`: this commit replaced
`CubicBez`'s earlier state-machine-style `ToQuads` iterator (from the crate's initial commit,
92feb79 "Start curves crate", which recomputed an `err < self.max_hypot2` check incrementally per
subdivision step, never dividing by `max_hypot2` directly) with the current precomputed-`n`
approach that introduced the unguarded `err / max_hypot2` division. The subsequent commit
af1d76be5a0a5f0f9cc60dc1a5ce3c72cd9d47f5 "CubicBez::to_quads always returns at least one quad"
(2020-06-04) only added the trailing `.max(1)` floor and is not implicated in the `+inf`/saturating
-cast mechanism itself. `flatten()`'s own two call sites (`bezpath.rs:644`, `:674`) were introduced
together with `flatten()` itself by f6b90b1 "Add BezPath flattening" (already identified as the
introducing commit for a different mechanism in candidate 015,
`FLATTEN_CUSP_ISSUE426`) - not independently re-verified via `git log -S` in this session since
`flatten()`'s presence as a single commit is already established by that earlier candidate.

## The fix

Clamp each of the three affected `n` computations to a fixed, generous upper bound (`1 << 20` =
1,048,576) right after the `.ceil()` that currently produces an unbounded value, before the cast to
`usize` - the same technique candidate 005 applied to `Arc::append_iter`:

```rust
// kurbo/src/cubicbez.rs
const MAX_TO_QUADS_SUBDIVISIONS: usize = 1 << 20;
...
let n = ((err / max_hypot2).powf(1. / 6.0).ceil())
    .min(MAX_TO_QUADS_SUBDIVISIONS as f64) as usize;
let n = n.max(1);
```

```rust
// kurbo/src/bezpath.rs (both the QuadTo arm and the cubic arm's second n)
const MAX_FLATTEN_SEGMENTS: usize = 1 << 20;
...
let n = ((0.5 * params.val / sqrt_tol).ceil())
    .min(MAX_FLATTEN_SEGMENTS as f64) as usize;
let n = n.max(1);
```

`f64::min` returns the non-`NaN` operand when either side is `NaN` (documented semantics), so this
also neutralizes a `NaN`-producing `tolerance` (e.g. from a negative `tolerance`, whose `.sqrt()`
is itself `NaN`) as a side effect, without a separate check - consistent with candidate 005's own
reasoning for its identical `.min()`-based fix.

Verified by re-running `repro/` against the patched worktree (`just apply && just
reproduce-after`): case A now completes without panicking (2 elements produced instead of
panicking); case B now finishes on its own at exactly 1,048,577 elements (`MoveTo` + 1,048,576
`LineTo`s) in ~19ms instead of hitting the 5,000,000-element cutoff - see `output.txt`. Also
checked, per `CONFIRMATION.md` section 7, against inputs varying within the same bug class rather
than just the one triggering value: `tolerance = -0.0`, `-1.0`, and `NaN` on the same `CurveTo` all
now produce the same safe 2-element result as `tolerance = 0.0`; `tolerance = 1e-322` (tiny but
nonzero) hits the same `1,048,577`-element bound as the exact-zero case; normal, non-degenerate
tolerances (`0.1`) still produce the same small counts as before the patch (11 elements for the
same `CurveTo`), i.e. the fix does not change behavior for any real-world tolerance value.

- Project test suite: **before** (pristine worktree, `cargo test --workspace --all-features`): 187
  + 10 + 55 tests passed, 0 failed, 1 ignored doctest (pre-existing, unrelated). **After** (patched
  worktree, same command): identical - 187 + 10 + 55 passed, 0 failed, 1 ignored. No new failures
  either direction.

## What's not done / open questions

- Candidate 005's own "What's not done" section already flagged `Circle::path_elements`
  (`kurbo/src/circle.rs`, around line 110-122) as sharing the exact same
  `scaled_err`/`powf(1./6.0)`/`as usize` shape with no clamp - still not investigated as part of
  this candidate either (one bug per candidate folder); still an open suspect for a future
  session.
- Whether any other `Shape`/`ParamCurve`-adjacent method elsewhere in the crate shares this
  `powf(1./N)`-then-saturating-cast pattern wasn't swept exhaustively; only the three call sites
  bundled into this candidate (`cubicbez.rs:94`, `bezpath.rs:644`, `bezpath.rs:674`) were confirmed
  and fixed.
- The chosen bound (`1 << 20`, matching candidate 005's own constant) is a judgment call, not
  derived from a documented project invariant - a maintainer may prefer a smaller/larger constant,
  a single shared constant across all three call sites (and `arc.rs`'s own `MAX_ARC_SEGMENTS`)
  instead of three independently-named ones, or may prefer a documented minimum valid `tolerance`
  / a `Result`-returning API instead of silently clamping. Silently clamping was chosen here to
  keep `to_quads`/`flatten`'s existing infallible signatures, matching this workspace's established
  precedent from candidate 005.
- `flatten()`'s cubic arm's second `n` (`bezpath.rs:674`) was fixed defensively (same pattern,
  same file, same public entry point) even though this session could not construct an input that
  reaches it in a still-`+inf` state via `tolerance == 0.0` specifically (the earlier `to_quads`
  panic always fires first for that exact value) - whether some other combination of `tolerance`
  and cubic shape could reach line 674 with `sqrt_remain_tol` still effectively zero while
  `to_quads`'s own `n` stays small was not exhaustively proven impossible, just not found; the fix
  is applied regardless since it costs nothing for the common case and closes the entire class at
  that call site either way.
