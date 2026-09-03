# resize_nearest fixed-point scale factor overflow

`src/resize_nearest.rs:44-45` computes `k_x`/`k_y` as `((src_width as u64) << 32) / dst_width as u64`
(and the height equivalent). Flagged in `IDEAS.md` from the prior `code-audit-structured` pass as
the cheapest-to-confirm-or-discard backlog item, with a note that it was "almost certainly not
practically reproducible" since it looked like it needed a multi-gigapixel image, and that the
next session should start with a feasibility check before investing in a full repro attempt.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Feasibility check (manual reasoning + `free -h`) | Computed the exact memory needed: a single-channel `u8` image with one dimension `== 2^32` needs only `2^32` bytes (~4 GiB) via `resize_plane8` (`CHANNELS=1`), not a "multi-gigapixel" 2D image as the backlog note assumed | Feasible: 19 GiB free on the test machine; no earlier bounds check elsewhere in the crate rejects `width`/`height` individually below `2^32` (only the total `width*height*CHANNELS` is checked against `usize::MAX`, and stride/size overflow are checked with `overflowing_mul`) |
| Manual boundary-value construction, standalone probe crate (`{ path = "..." }` dependency on this checkout), `cargo run --release` | `resize_plane8` via public API, `ResamplingFunction::Nearest`, `src_width = 2^32, dst_width = 2` (and the height-axis mirror) | Reproduced immediately, 1st attempt: `dst == [0x11, 0x11]` (both destination pixels sampled source index 0) instead of the two distinct, well-separated marker bytes placed at the two source indices a correct computation would sample |
| Same probe, repeated run (no rebuild) | identical build/input | Reproduced again, byte-for-byte identical output - 2/2 |
| Same probe, height-axis mirror (`src_height = 2^32`, `dst_height = 2`, width=1) | identical logic, transposed | Reproduced identically - confirms `k_y` shares the exact same defect as `k_x`, not just the one axis |
| Same probe, non-power-of-two boundary (`src_width = 2^32 + 777`, `dst_width = 3`) against an independent u128-computed oracle | Sanity/spread check only (source all-zero, can't check sampled *values* for this one) | Consistent with the bug class (all-zero source means this case alone can't discriminate values, but the discriminating `case_kx`/`case_ky` cases above already cover the class) |

No `miri`/ASAN/fuzzing was needed - this is deterministic `usize`/`u64` arithmetic with no
`unsafe` involved, so a debug-assertions build plus direct boundary-value construction (per
`HEURISTICS.md`'s guidance for this bug class) was the right first tool and it produced a
reproduction on the first attempt.

## Status

**Confirmed.** Bit-for-bit repeatable: 2/2 runs of `case_kx`, 2/2 of `case_ky`, identical build
(`cargo run --release`), identical input, identical output each time; also identical in a debug
build (verified in the initial probe, not carried into the final `repro/` for run time reasons -
the bug is a silent `u64` left-shift truncation, not an overflow panic, so debug-assertions do not
change its manifestation).

## Class

logic

## Root cause

`src_width`/`src_height` are `usize`, 64-bit on the platform tested. `SHIFT` is a compile-time
constant `32`. `(src_width as u64) << SHIFT` is a **value** overflow (the shifted-out high bits
are silently dropped), not a **shift-amount** overflow (32 is well under u64's 64-bit width, so
Rust never panics on it, in debug or release). For `src_width == 2^32`, the pre-shift value
`0x1_0000_0000` shifted left by 32 needs 65 bits to represent (`0x1_0000_0000_0000_0000`); only the
low 64 bits survive, i.e. exactly `0`. `k_x = 0 / dst_width` is then `0` regardless of `dst_width`,
so every iteration of the inner loop leaves `src_x_fixed` at `k_x_half == 0`, and every destination
pixel in the row reads `src_x == 0` - the entire row collapses onto the first source pixel instead
of sampling across the full source width. The same mechanism applies to `k_y`/`src_height`
independently.

This is reachable through the crate's public API: `resize_fixed_point`'s guards before calling
`resize_nearest` (`src.len()` length check, `overflowing_mul`-based stride/size checks) only
reject inputs where `width*CHANNELS*height` doesn't fit in `usize`, or where `width*CHANNELS`
individually overflows `usize` - they never bound `width`/`height` to below `2^32` on their own.
A single-channel (`CHANNELS=1`) `u8` image with `width == 2^32` and `height == 1` passes every
existing guard and needs only a ~4 GiB `Vec<u8>`, which the test machine allocated without
difficulty. The original backlog note's assumption (that this would need an impractical
"multi-gigapixel-wide image") undercounted by conflating "reaching 2^32 in one dimension" with
"needing a 2D image of comparable total pixel count" - a 1-pixel-tall row is enough.

## The fix

Widened the shift+division to `u128` before narrowing back to `u64`:

```rust
let k_x: u64 = (((src_width as u128) << SHIFT) / dst_width as u128) as u64;
let k_y: u64 = (((src_height as u128) << SHIFT) / dst_height as u128) as u64;
```

`u128` can represent `(usize::MAX as u128) << 32` (~2^96) without loss, so the shift itself can no
longer truncate for any `usize`-representable `src_width`/`src_height`. The final `as u64` narrows
the *quotient* back down; this is sound for every case checked (`case_kx`, `case_ky`, and the
non-power-of-two boundary case) because a resize ratio large enough to make the quotient itself
exceed `u64::MAX` (roughly `src_dim / dst_dim > 2^32`) would require a source dimension over
`2^64` bytes for `dst_dim == 1`, which cannot be constructed as a real `usize`-addressable slice
in the first place - see "What's not done" below for the theoretical edge this doesn't close.

Verified: re-ran `repro/` against the same clone before and after `git apply patch.diff` -
`case_kx`/`case_ky` go from `[17, 17]`/`[17, 17]` (wrong, bug present) to `[170, 187]`/`[170, 187]`
(correct, matching the independently-derived expected sample indices), 2/2 runs each side. Also
re-verified with `git stash`/`git stash pop` in the main checkout (not just the clone) to isolate
that this exact hunk, and nothing else, is what flips the outcome.

- Project test suite: **N/A, no tests in this package** (`cargo test --release` reports `0 passed;
  0 failed` both before and after the fix - there is no existing test coverage for this or any
  other function in the crate to regress).

## What's not done / open questions

- The final `as u64` narrowing of the corrected `u128` quotient is not itself checked/saturated -
  if `src_dim / dst_dim` ever exceeds `2^32` (e.g. an extreme downscale ratio), the quotient could
  in principle exceed `u64::MAX` and get silently truncated by the `as u64` cast, reproducing a
  variant of the same failure mode one level removed. This was reasoned about but not constructed
  as a repro: it requires a source dimension whose byte size exceeds what a real `usize`-indexed
  `Vec`/slice could ever hold (`> 2^64` bytes for `dst_dim == 1`), so it's out of scope for a
  bounded fix in this session - noted for the backlog in case a future session finds a smaller,
  practically-constructible ratio that still crosses this threshold.
- `src/resize_fixed_point.rs` and `src/resize_floating_point.rs` are the only two call sites of
  `resize_nearest` in the crate (both already checked as part of confirming reachability) - no
  other code path independently reimplements this fixed-point scale computation.
- Not checked whether the sibling (non-`-safe`) `pic-scale` repo at `/home/rafal/test/pic-scale`
  (used as a differential-test oracle by earlier candidates in this session) has the same or a
  different implementation of this scale-factor computation - added to `IDEAS.md`.
