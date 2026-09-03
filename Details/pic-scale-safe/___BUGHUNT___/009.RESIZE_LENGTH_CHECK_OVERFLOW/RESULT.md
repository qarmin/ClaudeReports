# resize_fixed_point / resize_floating_point input-length check overflow

`src/resize_fixed_point.rs:89` and `src/resize_floating_point.rs:109` validate the caller-supplied
`src.len()` against `source_size.width * CHANNELS * source_size.height` using plain
(non-overflow-checked) `usize` multiplication, immediately before code a few lines below that goes
to the trouble of using `overflowing_mul` for the exact same shape of computation (stride/size
overflow guards). Flagged by a prior `code-audit-structured` pass as backlog item
"`resize_fixed_point`/`resize_floating_point` input-length check overflow"
(`___BUGHUNT___/IDEAS.md`, Added 2026-08-25).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction + debug-assertions build | `cargo run` (dev profile, overflow-checks on), public API (`resize_rgba8`, `resize_rgb_f32`), `width = 1<<62`/`(1<<61)+1`/`usize::MAX` paired with small `height` | Panics with "attempt to multiply with overflow" at `resize_fixed_point.rs:89:21` and `resize_floating_point.rs:109:21`, reached before either overflow guard below it runs - confirmed on first try |
| Same inputs under `--release` | `cargo run --release` | Does not panic (plain multiplication silently wraps per release semantics); for the specific inputs tried, the wrapped value still differs from `src.len()`, so the (wrong-for-the-wrong-reason) length-mismatch branch itself returns `Err` - the bug is masked for these inputs in release, not fixed, since the wrapped value could coincidentally equal `src.len()` for other inputs (see "What's not done" below) |

Two tools were enough - the first (debug build + boundary values) reproduced deterministically, so
no escalation to fuzzing/miri was needed (arithmetic-overflow class, per `HEURISTICS.md`'s tool
selection: "a debug-assertions build plus deliberately constructed boundary values... no sanitizer
needed").

## Status

**Confirmed** - reproduces 3/3 runs against the standalone `repro/` (built and run three times in
a row against a fresh clone of `base_commit`), debug build (`cargo run`, dev profile with Rust's
default `overflow-checks = true`). Does not reproduce as a panic under `--release` for the
specific boundary inputs tried (see above) - this is stated explicitly per `CONFIRMATION.md`
section 2, not silently omitted.

## Class

crash

## Root cause

Both `resize_fixed_point` and `resize_floating_point` start their length-validation block with:

```rust
if src.len() != source_size.width * CHANNELS * source_size.height {
    return Err(format!(...));
}
```

evaluated with plain `usize` multiplication, which panics on overflow in any build with
`overflow-checks = true` (Rust's dev-profile default). Immediately below, the function computes
`source_size.width.overflowing_mul(CHANNELS)` and `src_stride.overflowing_mul(source_size.height)`
specifically to detect this same class of overflow and return a graceful `Err` instead. Because the
plain-multiplication check runs first, a caller-supplied `width`/`height` large enough to overflow
`usize` panics before those overflow guards are ever reached, defeating their purpose entirely for
any debug build (which includes, in this ecosystem, `cargo test`/`cargo run` during development and
any downstream consumer that ships with debug assertions on, e.g. via `overflow-checks = true` in
their own release profile).

In a release build (`overflow-checks = false`) the same multiplication silently wraps instead of
panicking. For the three boundary inputs tried here, the wrapped product still doesn't equal the
real `src.len()`, so the mismatch is (accidentally) caught by the same `if` that computed it, and
the later `overflowing_mul` guards never even get a chance to run. This happens to be safe for
these specific inputs but is incidental, not by design - see "What's not done" below.

## Introduced by

5e72885 "Initial commit" (2024-10-16) - the plain-multiplication length check running before the `overflowing_mul`-based guards has been present in `resize_fixed_point.rs`/`resize_floating_point.rs` since this crate's first commit.

**Already fixed upstream**: commit b26f840 "Fixes" (2026-08-25, merged into current HEAD via PR #22) reordered the check to run after the overflow-safe `overflowing_mul` calls, reusing their result (`src_slice_len`) instead of recomputing with plain multiplication - the current working tree no longer reproduces this bug.

## The fix

Reordered the checks so the overflow-checked stride/size computation (`overflowing_mul`) runs
*before* the length-equality check, and reused its already-overflow-checked result (`src_size`)
for the comparison against `src.len()`, instead of recomputing the product with plain
multiplication:

```rust
let (src_stride, is_stride_overflowed) = source_size.width.overflowing_mul(CHANNELS);
if is_stride_overflowed {
    return Err("Stride must never exceed usize::MAX".parse().unwrap());
}
let (dst_stride, is_stride_overflowed) = destination_size.width.overflowing_mul(CHANNELS);
if is_stride_overflowed {
    return Err("Stride must never exceed usize::MAX".parse().unwrap());
}
let (src_size, is_size_overflowing) = src_stride.overflowing_mul(source_size.height);
if is_size_overflowing {
    return Err("Image size must never exceed usize::MAX".parse().unwrap());
}
let (_, is_size_overflowing) = dst_stride.overflowing_mul(destination_size.height);
if is_size_overflowing {
    return Err("Image size must never exceed usize::MAX".parse().unwrap());
}
if src.len() != src_size {
    return Err(format!(
        "Source slice size must be width * channels * height ({src_size}) but got {}",
        src.len(),
    ));
}
```

Applied identically to both `src/resize_fixed_point.rs` and `src/resize_floating_point.rs` (see
`patch.diff`). This removes the plain multiplication entirely - the only remaining arithmetic on
`width`/`height`/`CHANNELS` in this validation block is already overflow-checked, so there is no
path left that can panic on this input shape.

Verified against more than the one input that first triggered the bug (per `CONFIRMATION.md`
section 7): the standalone `repro/` exercises three different boundary shapes -
(a) `width = 1<<62, height = 4` (overflow already in the first `width*CHANNELS` step),
(b) `width = (1<<61)+1, height = 3` (`width*CHANNELS` fits, only the final `*height` step
overflows), and (c) `width = usize::MAX, height = 1` - plus a well-formed `16x16 -> 8x8` resize as
a sanity check that the fix doesn't change behavior for valid input.

Before -> after, re-running `repro/` against the same pristine clone:

- Before (pristine `base_commit`, debug build): panics on all 3 boundary cases (`attempt to
  multiply with overflow`), `repro` exits non-zero. Reproduced 3/3 runs.
- After (patch applied, debug build, rebuilt): all 3 boundary cases return a graceful `Err`
  (`"Stride must never exceed usize::MAX"` / `"Image size must never exceed usize::MAX"`), the
  sanity-check resize still succeeds and returns the correct length, `repro` exits 0.

- Project test suite: 0 passed; 0 failed before -> 0 passed; 0 failed after (`cargo test` in both
  the main working tree and the standalone clone). The library crate has zero `#[test]`s (see
  `___BUGHUNT___/README.md`); this is the honest "N/A-shaped" result, not an omission.

## What's not done / open questions

- The release-mode "masking" observed above (wrapped product happens to still mismatch
  `src.len()` for the three inputs tried) was not proven safe for *all* inputs before the fix -
  it's plausible some other `width`/`height`/`CHANNELS`/`src.len()` combination makes the wrapped
  product equal the actual (smaller) buffer length, in which case the pre-fix code would have
  proceeded past the length check with a `source_size` that doesn't actually describe the real
  buffer, only to be caught (or not) by the very same `overflowing_mul` guards a few lines later -
  those guards check `width*CHANNELS` and `stride*height` overflow independently of `src.len()`,
  so they would still catch a `width`/`height` that itself overflows, just not a case where the
  *wrapped* length happens to coincidentally match a smaller real buffer while `width`/`height`
  individually don't overflow their own stride/size checks. This was not constructed as a separate
  repro; the fix (removing the plain multiplication entirely) closes this class regardless of
  whether such an input exists, so it wasn't pursued further as its own candidate.
- `alpha_check.rs`'s `has_non_constant_alpha`/`has_non_constant_alpha_f32_impl` has the same
  "plain `width * CHANNELS` multiplication, no overflow guard" pattern for the opposite extreme
  (large `width`, not zero `width` as in candidate 002) - already separately tracked in
  `___BUGHUNT___/IDEAS.md`'s backlog, not folded into this candidate.
- A crate-wide grep for the same `width * CHANNELS * height`-shaped plain multiplication (per
  `SKILL.md`'s "Critical pitfalls" #7) turned up more instances in `fixed_point_dispatch.rs`,
  `floating_point_dispatch.rs`, and `resize_nearest.rs` - all of these are internal-only functions
  reachable exclusively from inside `resize_fixed_point`/`resize_floating_point` *after* this
  candidate's overflow guards already ran (the `Nearest` branch and the dispatch calls both sit
  below the guards in both functions, before and after this fix), so they were not treated as a
  separate candidate. `alpha_check.rs`'s `width * CHANNELS` (candidate 002's zero-width sibling
  bug, opposite extreme) remains separately tracked in `___BUGHUNT___/IDEAS.md`.
