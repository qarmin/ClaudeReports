# has_non_constant_alpha / has_non_constant_alpha_f32_impl width*CHANNELS overflow

`src/alpha_check.rs:126-164` - the generic integer path (`has_non_constant_alpha`, backing
`has_non_constant_alpha_rgba8/la8/rgba16/la16`) and the f32 path
(`has_non_constant_alpha_f32_impl`, backing `has_non_constant_alpha_rgba_f32/luma_alpha_f32`)
compute the per-row chunk size as `width * CHANNELS` with plain (non-checked) `usize`
multiplication before passing it to `store.chunks_exact(...)`. This was flagged in `IDEAS.md` as
the opposite extreme of candidate 002 (zero-width panic, already fixed and reverted per this
skill's workflow): an unusually large `width` could overflow the multiplication instead of
underflowing it to zero.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction: in-tree `tests/bughunt_alpha_check_width_overflow.rs` calling `has_non_constant_alpha_rgba8`/`has_non_constant_alpha_rgba_f32` with a tiny 4-element store and `width = usize::MAX / 2` | `cargo test`, debug build | Both calls panicked immediately: `attempt to multiply with overflow` at `src/alpha_check.rs:131:35` and `:157:35` |
| Same test, repeated | debug, 2 runs | Identical panic location and message every time |
| Same shape, release build (`cargo test --release`), `width` chosen so `width * 4` wraps to exactly `0` mod `2^64` | release build | Multiplication silently wrapped (no overflow panic, overflow-checks off by default in release); the wrapped chunk size of `0` then hit `chunks_exact`'s own precondition panic, `chunk size must be non-zero`, at the same call site - the same externally-visible panic as candidate 002, reached via wraparound instead of a literal `width == 0` |
| Standalone `repro/` against a fresh clone at the base commit, `cargo run` (debug) and `cargo run --release`, covering: `width = usize::MAX/2` (outright overflow, rgba8 and rgba_f32), `width` chosen to wrap `row_len` to exactly `0` (rgba8), plus a sanity check with a genuinely-varying-alpha input | debug 2x, release 2x | All 3 overflow/wrap cases panicked in debug (2/2 runs); the wrap-to-zero case also panicked in release (2/2 runs) with the "chunk size must be non-zero" message instead; the other two overflow cases did not panic in release (the wrapped `row_len` happened to land above `store.len()`, so `chunks_exact` simply produced zero chunks and returned `false`) - consistent with this being an arithmetic-overflow bug, not a memory-safety one: `chunks_exact` never reads out of the slice's bounds regardless of the wrapped chunk size. The sanity check (`has_non_constant_alpha_la8` on a real varying-alpha input) returned the correct `true` in every run |

No sanitizer/miri needed - `width` is a plain caller-supplied `usize` parameter independent of
`store.len()` (it does not need to match the slice's real size for the multiplication to run), so
a boundary value alone reaches the bug directly; there is no unsafe code or memory corruption
involved (`#![forbid(unsafe_code)]` for this crate, see `___BUGHUNT___/README.md`).

## Status

**Confirmed** - reproduces deterministically. Debug build: 2/2 runs panic with "attempt to
multiply with overflow" for both an outright-overflowing `width` and a `width` that wraps the
chunk size to `0`. Release build: 2/2 runs panic with "chunk size must be non-zero" for the
wrap-to-zero case specifically (the two outright-overflow cases tried do not panic in release,
since their wrapped chunk size happens to exceed `store.len()`, so `chunks_exact` degrades to zero
iterations rather than a `0`-size panic - not layout/seed-dependent, this is a deterministic
function of the exact `width` value chosen, not randomness).

## Class

crash

## Root cause

Both `has_non_constant_alpha` and `has_non_constant_alpha_f32_impl` take `width: usize` as a
parameter entirely independent of `store.len()` - nothing requires the caller to pass a `width`
consistent with the slice's actual size before this arithmetic runs. Each function computes
`width * CHANNELS` with plain multiplication and hands the result straight to
`store.chunks_exact(...)`:

- In a **debug build** (`overflow-checks = true`, Rust's dev-profile default), any `width` large
  enough that `width * CHANNELS` exceeds `usize::MAX` panics immediately at the multiplication
  itself ("attempt to multiply with overflow"), before any pixel scanning happens.
- In a **release build** (`overflow-checks = false` by default, and this crate's `Cargo.toml` has
  no profile override), the same multiplication silently wraps modulo `2^64`. Three sub-cases
  follow, all driven purely by which value the wrap happens to produce:
  - Wraps to exactly `0` -> `chunks_exact` panics on its own zero-chunk-size precondition (the
    same panic message and root cause class as already-fixed candidate 002, just reached through
    arithmetic wraparound here instead of a literal `width == 0` argument).
  - Wraps to a small nonzero value larger than `store.len()` -> `chunks_exact` simply yields no
    chunks; the function returns `false` regardless of the store's actual alpha data. Not
    exercised as a *crash* here, but this is the "silently wrong answer" sibling of the same
    overflow - a real per-pixel alpha-variance signal would be silently discarded.
  - Wraps to a small nonzero value smaller than `store.len()` -> `chunks_exact` would scan the
    store using an unrelated row length, producing a result keyed on the wrong pixel grouping.
    Also not memory-unsafe (`chunks_exact` still never reads past the slice's bounds), but wrong.

This is the identical class of bug already fixed and reverted in candidates 002 and 009
(unchecked `usize` multiplication on an input-derived size, ahead of any bounds/overflow guard) -
here it hits the multiplication itself instead of a downstream zero-chunk-size or length-mismatch
check.

## Introduced by

5ee1e8c "Refactors" (2024-11-20) - same origin commit as sibling candidate 002 (this is the arithmetic-overflow manifestation of the same missing-validation shape in `has_non_constant_alpha`/`has_non_constant_alpha_f32_impl`, introduced together).

Still present in current HEAD (checked while filling in this section: `width * CHANNELS` is still a plain, unchecked multiplication) - not yet fixed upstream.

## The fix

Replace the plain `width * CHANNELS` multiplication with `width.checked_mul(CHANNELS)`, and treat
both overflow (`None`) and a `width == 0` result identically to the function's own existing
"nothing to scan" convention (matching candidate 002's already-established `false`-on-degenerate-
input precedent, so this fix does not reintroduce that already-documented bug as a side effect of
touching the same line):

```rust
let row_len = match width.checked_mul(CHANNELS) {
    Some(row_len) if row_len != 0 => row_len,
    _ => return false,
};
```

applied identically in `has_non_constant_alpha` (`src/alpha_check.rs:126`, right after the
existing `store.is_empty()` guard) and `has_non_constant_alpha_f32_impl`
(`src/alpha_check.rs:152`), with `store.chunks_exact(width * CHANNELS)` in each function changed
to `store.chunks_exact(row_len)`.

Verified against the same three overflow/wrap boundary cases used to confirm the bug, plus the
sanity check for a real varying-alpha input, in both build profiles:

- Before fix (pristine clone, `cargo run`): 3 panics in debug (2/2 repeat runs), 1 panic in
  release (the wrap-to-zero case, 2/2 repeat runs) - see `output.txt`.
- After fix (`git apply patch.diff`, `cargo run`): all cases return `false` gracefully (no panic)
  in both debug and release, and the sanity check still returns the correct `true` for a
  genuinely-varying-alpha input - see `output.txt`'s "after applying patch.diff" sections.

- Project test suite: `cargo test --workspace` before fix - `app` binary crate's 1 test
  (`image_wrapper::tests::test_rescaling`) passed (1 passed, 0 failed, ~71s); `pic-scale-safe` lib
  itself has 0 tests (N/A, no unit/integration tests in this package). After fix: identical counts
  (1 passed, 0 failed for `app`; 0 tests for the lib) - no regression, and the existing suite
  doesn't exercise this code path either way.

## What's not done / open questions

- No test was added to the main project's own test suite (per this skill's workflow, the working
  tree is always reverted; adding a permanent regression test is a decision for whoever applies
  `patch.diff` for real).
- The fix returns `false` for an overflowing/degenerate `width` rather than surfacing an error -
  this mirrors the function's own existing behavior for `store.is_empty()` (also silently `false`)
  and candidate 002's established precedent for `width == 0`, so it's consistent with the existing
  API contract rather than a new error-reporting convention; a maintainer may prefer a `Result`
  return type instead (matching `resize_fixed_point`/`resize_floating_point`'s `Err(...)`
  convention), which would be a breaking API change out of scope for a bounded fix here.
- Not checked: whether `ALPHA_CHANNEL_INDEX`/`CHANNELS` combinations other than the ones exercised
  here (1..=4, all currently used by the 6 public callers) change this analysis - they don't, since
  `CHANNELS` is a compile-time constant in every real call site and the overflow is driven entirely
  by the runtime `width` value, but this wasn't independently re-verified for every const-generic
  instantiation.
- Same underlying arithmetic-overflow bug class as candidates 002/009/013 in this crate; not
  independently re-swept across the rest of `src/` for other unchecked `size * CHANNELS`-shaped
  multiplications beyond the ones already turned into their own candidates.
