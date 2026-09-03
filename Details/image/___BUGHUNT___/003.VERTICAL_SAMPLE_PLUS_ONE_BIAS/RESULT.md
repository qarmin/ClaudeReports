# `vertical_sample` resampling accumulator seeded with 1.0 instead of 0.0

`src/imageops/sample.rs:544` (`vertical_sample`, the vertical resampling pass used by the public
`image::imageops::resize`/`resize_exact` free functions) seeds its per-pixel accumulator with
`crate::Rgba([1.0; 4])` right before summing filter-tap weights that are normalized to sum to
`1.0` (`*w /= sum` just above). Carried over from `___OTHER___/001.RESIZE_VERTICAL_SAMPLE_OFF_BY_ONE/REPORT.md`
(and `IDEAS.md`'s backlog) into this workspace's structure, re-verified from scratch this session.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Direct source read + manual reasoning from the documented invariant (weights normalized to sum to 1.0, accumulator should start at 0.0) | Reading `src/imageops/sample.rs` lines 494-560 on current `main` (44ce9226) | Confirmed the accumulator is seeded `[1.0; 4]`, not `[0.0; 4]` - the only place in the function that diverges from the "weighted average of already-normalized weights" invariant. `horizontal_sample`'s equivalent accumulator is correctly seeded at `[0.0; MAX_CHANNEL]` for comparison. |
| Manual boundary-value construction, single-process, real public API (`image::imageops::resize`), release + debug builds, directly in the main project's working tree (scratch `examples/` binary, deleted before this candidate folder was written) | Uniform 4x4 100-valued `Rgb<u8>` resized to 2x2 with `FilterType::Triangle`; `cargo run --release --example` x3, `cargo run --example` (debug, debug-assertions on) x1 | Every output pixel is `Rgb([101,101,101])`, not `[100,100,100]` - reproduces 4/4 (3 release + 1 debug), byte-identical each time. |
| Boundary sweep across sizes/filters/dimensions (same scratch binary), still real public API, release build | `(4,4)->(2,2)` Triangle, `(10,10)->(3,3)` Lanczos3, `(8,6)->(8,3)` Lanczos3 (width unchanged), `(1,100)->(1,10)` Lanczos3 (width == 1), uniform 50-valued `Rgb<u8>` | Every case biased by exactly `+1` on pixel(0,0) (checked across all output pixels in-process) - not specific to the original triangle-filter/4x4 case. |
| Standalone `repro/` (release profile) via `just test` end-to-end (real local clone of the working checkout pinned at `base_commit`, real `cargo build`, real `image::imageops::resize` public API call, no hand-constructed internal state) | `cargo run --release` in `repro/`, run twice via two full `just test` invocations | Both full `just test` runs: `BUG REPRODUCED` before `patch.diff`, `Not reproduced` after - 2/2, byte-identical. |

Two tool classes used (manual boundary-value construction is the correct fit per `HEURISTICS.md`'s
"Logic bugs" guidance - no sanitizer/miri/fuzzing needed for a pure floating-point accumulator
seed error; a debug-assertions build was tried alongside it as a free extra check, without turning
up anything different since this isn't an overflow/UB class of bug). Soft cap of 3 was not needed -
the first approach already gave a clear, deterministic signal.

## Status

**Confirmed.** Deterministic, no layout/seed/timing dependence: reproduces 4/4 direct in-tree runs
(3 release + 1 debug) plus 2/2 full `just test` runs of the standalone `repro/` against a pristine
clone - 6/6 total, byte-identical every time.

**Fully reproducible: Yes.** `repro/`'s `main.rs` calls only `image::imageops::resize`, the crate's
real public API, on a real `ImageBuffer` built through its own public constructor. There is no
second participant/peer/wire input involved in this finding (a pure single-process logic bug in
how a caller-supplied image is resampled, not a reaction to another participant's data), so
`CONFIRMATION.md` section 11's "real second participant" requirement does not apply.

## Class

logic

## Root cause

`vertical_sample`'s inner loop (`src/imageops/sample.rs`, around line 544):

```rust
for x in 0..width {
    let mut pix = crate::Rgba([1.0; 4]);   // should be [0.0; 4]

    for (i, w) in ws.iter().enumerate() {
        let p = image.get_pixel(x, left + i as u32);
        for (tc, &c) in pix.channels_mut().iter_mut().zip(p.channels()) {
            *tc += <f32 as NumCast>::from(c).unwrap() * w;
        }
    }

    out.put_pixel(x, outy, pix);
}
```

`ws` is normalized just above this loop (`for w in ws.iter_mut() { *w /= sum; }`), so
`ws.iter().sum::<f32>() == 1.0`. The loop computes a weighted average of the source pixels in that
column using those already-normalized weights, then writes the result into `pix`. Since `pix`
starts at `1.0` per channel instead of `0.0`, every computed weighted average gets a flat `+1.0`
added to it before the accumulation even begins - equivalent to `weighted_average + 1.0`. For
`u8`/`u16` output types this becomes a `+1` after the later `nearest_from`/clamp step (pixels
already at the type's max value are unaffected, since clamping saturates); for `f32` output
(`Rgb32FImage` etc., no clamping step in this path) the `+1.0` bias is unconditional and
unclamped. This affects every pixel of every resize where vertical resampling actually runs, which
is the normal case (`resize` always runs `vertical_sample` before `horizontal_sample`, for every
resize that isn't a same-size no-op copy) - not an edge case.

Per `git log -L`, this was introduced by commit `20a2308218d110925bd3c3ff3b9ebee61354c4b0`
("Remove Pixel channels4/from_channels"), which replaced a correctly-zero-initialized tuple
accumulator (`let mut t = (0.0, 0.0, 0.0, 0.0);`) with the `Rgba([1.0; 4])` form shown above. This
commit is not yet in any published crates.io release (`image` 0.25.10 on crates.io predates it and
does not reproduce this bug) - it is only live on `image-rs/image`'s `main` branch, confirmed
still present as of `base_commit` `44ce9226c541dd2b11897b9ef07156a28871cdb7`.

Only one call site of `vertical_sample` exists in the crate (`git grep 'vertical_sample('` -
inside `resize` itself), and this bug does **not** affect `DynamicImage::resize`/`resize_exact`/
`resize_to_fill`, which delegate to a separate `pic-scale-safe`-backed implementation in
`src/imageops/resize.rs`, nor `imageops::thumbnail` (a different, unrelated integer-averaging
algorithm), nor `horizontal_sample` alone (its own accumulator is correctly seeded at
`[0.0; MAX_CHANNEL]`).

## Introduced by

20a2308 "Remove Pixel channels4/from_channels" (2025-12-30)

## The fix

One-line change: seed the accumulator at `[0.0; 4]` instead of `[1.0; 4]`, matching the invariant
that the weights it's about to sum against already add up to `1.0`:

```diff
-            let mut pix = crate::Rgba([1.0; 4]);
+            let mut pix = crate::Rgba([0.0; 4]);
```

Verified against more than just the original triggering input, per `CONFIRMATION.md` section 7:
all four boundary-sweep cases (`(4,4)->(2,2)` Triangle, `(10,10)->(3,3)` Lanczos3,
`(8,6)->(8,3)` Lanczos3, `(1,100)->(1,10)` Lanczos3) produced the exact expected uniform value
after the fix, not just the original 4x4-Triangle case - see `output.txt` and `repro/src/main.rs`.

- Project test suite (`cargo test --lib`, run in the main project's own working tree):
  **331 passed, 0 failed** before the patch -> **331 passed, 0 failed** after the patch (identical
  pass count, no new failures, no regressions). Notably, the pristine (buggy) tree already passes
  all 331 tests unchanged - **no existing test in this crate currently catches this bug** (see
  "What's not done" below).

`patch.diff` contains this fix as a diff against `base_commit`
(`44ce9226c541dd2b11897b9ef07156a28871cdb7`).

## What's not done / open questions

- No existing test in the crate's own suite exercises `image::imageops::resize`'s output values
  closely enough to catch a flat `+1` bias (331/331 pass identically before and after the fix) -
  worth adding a regression test asserting `resize` on a uniform-color image returns that same
  uniform color, so this class of accumulator-seed bug can't silently reappear.
- Not checked this session: whether any other accumulator in `src/imageops/sample.rs` or nearby
  filter code follows the same "seed to the identity element of the operation" pattern and could
  have an analogous seed-value bug (`horizontal_sample`'s own accumulator was spot-checked and is
  correctly seeded at `[0.0; MAX_CHANNEL]`, but the wider file wasn't swept for this specific
  pattern).
- This is a git-`main`-only regression (not yet in a published crates.io release as of this
  session) - the fix is small and safe to apply directly on `main` before any release ships with
  the bug.
