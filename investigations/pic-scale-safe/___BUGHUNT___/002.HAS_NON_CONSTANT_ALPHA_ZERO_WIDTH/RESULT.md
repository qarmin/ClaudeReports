# has_non_constant_alpha / has_non_constant_alpha_f32_impl zero-width panic

`src/alpha_check.rs:126-164` - the generic integer path (`has_non_constant_alpha`, backing
`has_non_constant_alpha_rgba8/la8/rgba16/la16`) and the f32 path
(`has_non_constant_alpha_f32_impl`, backing `has_non_constant_alpha_rgba_f32/luma_alpha_f32`) only
guard `store.is_empty()` before calling `store.chunks_exact(width * CHANNELS)`. This was flagged by
a prior `code-audit-structured` pass as looking like it would panic when `width == 0` on a
non-empty `store`, since `chunks_exact` requires a non-zero chunk size.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction: throwaway `tests/bughunt_alpha_check_zero_width.rs` calling `has_non_constant_alpha_la8`/`has_non_constant_alpha_rgba_f32` with a non-empty 4-element store and `width = 0` | `cargo test` in-tree, debug build | Both calls panicked immediately: `chunk size must be non-zero` at `src/alpha_check.rs:131:22` and `:157:22` |
| Same test, repeated (debug + `cargo test --release`) | debug 2x, release 1x | Identical panic location and message every time, both build profiles |
| Standalone `repro/` against a fresh clone at the base commit, `cargo run --release`, all 6 public entry points (`has_non_constant_alpha_rgba8/la8/rgba16/la16/rgba_f32/luma_alpha_f32`) with `width = 0` | release build | All 6 panicked, reproduced 2/2 runs (only the thread id differed between runs; panic site/message identical) |

No sanitizer/miri needed - this is a safe-Rust panic from `chunks_exact`'s own precondition check
(chunk size must be non-zero), not memory-unsafety; the first, cheapest approach already produced
a deterministic, unconditional repro, so no further escalation was needed.

## Status

**Confirmed** - reproduces deterministically, 2/2 runs in a standalone release-build repro against
a fresh clone, plus 2/2 debug + 1/1 release runs in-tree. Not layout- or seed-dependent: any call
with `width == 0` and a non-empty `store` hits the exact same panic every time, for all 6 public
functions built on the two shared implementations.

## Class

crash

## Root cause

Both `has_non_constant_alpha` and `has_non_constant_alpha_f32_impl` compute the per-row chunk size
as `width * CHANNELS` and pass it straight to `store.chunks_exact(...)`. `chunks_exact` panics
unconditionally if given a chunk size of `0` ("chunk size must be non-zero"). The existing guard
(`if store.is_empty() { return false; }`) only covers the case of an empty slice; it does nothing
for `width == 0` on a non-empty slice, since `width * CHANNELS == 0` regardless of `CHANNELS`
(1..=4 in every caller). All 6 public functions (`has_non_constant_alpha_rgba8`, `_la8`, `_rgba16`,
`_la16`, `_rgba_f32`, `_luma_alpha_f32`) funnel into one of these two shared implementations and
share the identical guard, so all 6 are affected identically - this is one root cause, not six
separate ones.

This matches an established convention elsewhere in the crate: `resize_fixed_point.rs:112-117` and
`resize_floating_point.rs:132-137` both explicitly check `source_size.width == 0 ||
source_size.height == 0` (and the same for `destination_size`) before doing any chunk-size-shaped
arithmetic, returning an `Err` in that case. `alpha_check.rs`'s guard was simply never extended to
cover the same `width == 0` case, even though it already special-cases the sibling
"nothing to scan" condition (`store.is_empty()`).

## Introduced by

5ee1e8c "Refactors" (2024-11-20) - `git log -S"fn has_non_constant_alpha"` finds this as the commit that introduced `alpha_check.rs`'s shared implementation, already with only the `store.is_empty()` guard and no `width == 0` check.

Still present in current HEAD (checked while filling in this section: `alpha_check.rs` still has only the `is_empty()` guard, no `width == 0` check) - not yet fixed upstream, unlike several sibling candidates from the same session.

## The fix

Extend the existing early-return guard in both functions to also cover `width == 0`, treating it
the same way as an empty store (no pixels to scan -> no non-constant alpha -> `false`):

```rust
if store.is_empty() || width == 0 {
    return false;
}
```

applied identically in `has_non_constant_alpha` (`src/alpha_check.rs:126`) and
`has_non_constant_alpha_f32_impl` (`src/alpha_check.rs:152`). This is a natural extension of the
function's own existing degenerate-input handling (an empty store already returns `false` for "no
scannable pixels"; a zero-width store has the same property) rather than a new invented behavior,
and it doesn't change the return value semantics for any width > 0 input.

Verified against all 6 public entry points, not just the one function first tried: re-ran the
standalone `repro/` against the patched tree - all 6 completed normally (`did not panic`) instead
of panicking. See `output.txt` (before) and the "before/after" note above; the after-run
transcript is reproduced in this candidate's own verification (not re-saved separately, since it's
just "no panic" for all 6 functions, already stated here).

- Project test suite: before fix - `cargo test --workspace`: `app` binary crate's 1 test
  (`image_wrapper::tests::test_rescaling`) passed (1 passed, 0 failed); `pic-scale-safe` lib itself
  has 0 tests (N/A, no unit/integration tests in this package). After fix: identical counts (1
  passed, 0 failed for `app`; 0 tests for the lib) - no regression, and the existing suite doesn't
  exercise this code path either way (consistent with why this had gone unnoticed).

## What's not done / open questions

- No test was added to the main project's own test suite (per this skill's workflow, the working
  tree is always reverted; adding a permanent regression test is a decision for whoever applies
  `patch.diff` for real).
- Not checked: whether an unusually large `width` (one that makes `width * CHANNELS` overflow
  `usize`) causes a related but distinct problem in the same functions - this is a separate suspect
  (arithmetic-overflow class, not the zero-chunk-size panic class fixed here) and has been added to
  `IDEAS.md`'s backlog rather than folded into this candidate.
- The fix returns `false` for `width == 0` rather than surfacing an error - this mirrors the
  function's own existing behavior for the `store.is_empty()` case (also silently `false`, no
  `Result`/panic), so it's consistent with the existing API contract rather than introducing a new
  error-reporting convention; a maintainer may prefer a different signature (e.g. `Result<bool,
  String>`) if `width == 0` should instead be surfaced as caller error, matching
  `resize_fixed_point`/`resize_floating_point`'s `Err("Image size must not be zero")` convention -
  that would be a breaking API change out of scope for a bounded fix.
