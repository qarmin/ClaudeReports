# pixeldata-embedded-overlay-needed-size-overflow

`pixeldata/src/overlay.rs`'s `extract_embedded_overlay_bits` (decodes a legacy DICOM overlay plane
embedded in unused bits of the pixel data samples, PS3.3 C.9.3 - reached via
`OverlayPlane`/`PixelDecoder::decode_overlay`/`decode_overlays`) computes
`let needed = offset + num_pixels * bytes_per_sample;` in plain `usize` arithmetic, where `offset`,
`num_pixels`, and `bytes_per_sample` are all derived from untrusted DICOM attribute values (Rows,
Columns, Number of Frames in Overlay, Image Frame Origin, Bits Allocated). `pixeldata/src/
overlay.rs` was added/last touched in commit `a4bf81f6` (2026-08-13, "[pixeldata] add overlay
error type"), only days before this session - genuinely new code, untouched by the prior
2026-08-24 bug-hunt session, and a strong match for HEURISTICS.md's "arithmetic on sizes/lengths
derived from input" pattern, plus the "two independently attacker-influenced integers
multiplied/added in a type whose range their product can exceed" class already confirmed twice in
this codebase (`010`, `021` from the 2026-08-24 session).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value analysis (per HEURISTICS.md: construct 0, 1, max, max+1 for each attacker-controlled input) using Python to compute the exact worst-case product against `usize::MAX`/`u64::MAX` on a 64-bit target, for every multiplication/addition in the file | Static reasoning, no build needed | `num_pixels = frames * rows * columns` alone (frames capped at `i32::MAX` since `number_of_frames_in_overlay` parses via `to_int::<i32>()`, rows/columns capped at `u16::MAX`) does **not** overflow `u64` (`9,223,090,559,730,712,575 < u64::MAX`) - but `needed = offset + num_pixels * bytes_per_sample`, where `offset` itself involves `image_frame_origin` (read via `to_int::<i32>()`, so representable up to `i32::MAX` regardless of its dictionary VR, `US`, being nominally narrower - this crate does not cross-validate a wire element's declared VR against the dictionary's expected VR for a tag), **does** overflow `u64`/`usize` at worst case (~3.69×10^19, roughly double `u64::MAX`) |
| Hand-built boundary input (Rows=Columns=65535, Number of Frames in Overlay=`i32::MAX`, Image Frame Origin encoded as `VR::UL`/`U32` with value `i32::MAX` - wider than its dictionary VR `US` would carry, Bits Allocated=16, a real but tiny 8-byte PixelData buffer), debug build, `cargo test` (temporary unit test added to `pixeldata/src/overlay.rs`, reverted before this record was written) | `obj.decode_overlay(0)` via the public `PixelDecoder` trait | Panics: `attempt to add with overflow` at `pixeldata/src/overlay.rs:414` (the `needed = offset + ...` line). Reproduced 3/3 in-tree unit-test runs, then 2/2 more via the standalone `repro/` built against a fresh clone of pristine upstream `df2b9e42` (5/5 total) |
| Same input, `--release` build (per CONFIRMATION.md section 2, "state which build shows it") | `cargo test -p dicom-pixeldata --lib --release` | Does **not** panic in release: the wrapped `needed` value (`18,445,618,156,623,626,234` - the true ~3.69×10^19 result minus one `u64::MAX+1` wrap) happens to still be astronomically larger than the tiny actual `samples.len()` (8 bytes), so the existing `if samples.len() < needed` check still (accidentally) rejects it with a graceful `DataLength` error. This specific input does not defeat the check in release mode - see "Root cause" for why this is a coincidence of the exact values chosen, not a guarantee |
| Second boundary input in the same class (Number of Frames in Overlay at `i32::MAX`, but Image Frame Origin left within its realistic `US`-range maximum, `65535`, instead of widened) | Same technique, temporary unit test | Confirmed this narrower combination does **not** overflow (`needed_max ≈ 1.84467440350×10^19`, just under `u64::MAX`) - consistent with the Python boundary analysis; used to verify the fix doesn't just special-case the exact overflowing input (CONFIRMATION.md section 7) |

## Status

**Confirmed** - reproduces 5/5 runs (3 in-tree unit-test runs + 2 standalone `repro/` runs against
a fresh clone of pristine upstream `df2b9e42`), same panic location and message every time.
**Debug build only** for the specific input constructed this session (`cargo test`/`cargo run` dev
profile, debug-assertions on by default) - a `--release` build with the same input does not panic
(see above), though this is not a guarantee for all overflowing inputs, since the wrap could in
principle land on a small `needed` value for a different combination, defeating the length check
silently rather than merely surviving it by chance; this was not separately constructed or
confirmed this session (see "What's not done" below).

## Class

`crash` (a `panic!`-shaped integer-overflow abort in a debug build, reachable from ordinary use of
the public `PixelDecoder::decode_overlay`/`decode_overlays` API on a crafted DICOM object - no
`unsafe`, no memory-safety implication in the observed manifestation).

## Root cause

`extract_embedded_overlay_bits` computes the number of pixel-data bytes needed to contain an
embedded overlay plane by multiplying/adding several attacker-controlled values (Number of Frames
in Overlay, Rows, Columns, Image Frame Origin, and Bits Allocated's derived byte width) with no
overflow protection. Every individual value is bounded by its own DICOM type's *nominal* range
(`i32` for the `IS`-typed Number of Frames in Overlay, `u16` for the `US`-typed Rows/Columns/Image
Frame Origin *per the dictionary* - though Image Frame Origin's actual value is read via a generic
`to_int::<i32>()` conversion that does not enforce the dictionary's `US` width against whatever VR
the wire element actually declares), but their **combined product** can exceed what a 64-bit
`usize` can represent. This is architecturally the same root cause already documented in
`projects/dicom-rs.md` for candidates `010`/`021` from the prior session: *"whenever two
separately-sourced attacker-influenced integers get subtracted/added/multiplied in a type narrower
than what their combined range could produce, check the extremes of both, not just one."* Here it
is three-to-four values multiplying/adding together rather than two, but the shape is identical.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Replaced the plain `*`/`+` arithmetic with `checked_mul`/`checked_add`, saturating to `usize::MAX`
on any overflow:

```
-    let offset = (image_frame_origin as usize - 1) * frame_pixels * bytes_per_sample;
-    let needed = offset + num_pixels * bytes_per_sample;
+    let offset = (image_frame_origin as usize - 1)
+        .checked_mul(frame_pixels)
+        .and_then(|v| v.checked_mul(bytes_per_sample))
+        .unwrap_or(usize::MAX);
+    let needed = num_pixels
+        .checked_mul(bytes_per_sample)
+        .and_then(|v| offset.checked_add(v))
+        .unwrap_or(usize::MAX);
```

Saturating to `usize::MAX` (rather than, say, wrapping or erroring separately) means the very next
line's existing `if samples.len() < needed { return DataLengthSnafu { .. } }` check continues to
work unmodified and unconditionally catches every overflow case as "not enough data", regardless
of how large the true (unrepresentable) product would have been - closing both the debug-panic
path and the theoretical release-mode small-wrap path in one change, without needing a wider
integer type or a separate overflow-specific error variant.

Verified against both boundary inputs (CONFIRMATION.md section 7): the original overflowing
combination (Rows=Columns=65535, Frames=`i32::MAX`, Image Frame Origin=`i32::MAX`) now returns
`Err(DataLength { needed: 18446744073709551615 /* usize::MAX */, .. })` instead of panicking; the
second, non-overflowing boundary combination (Image Frame Origin capped at realistic `US`-range
`65535`) continues to behave exactly as before (still resolves to a graceful outcome, unaffected by
the fix, since its `needed` value was never large enough to need saturation).

- Project test suite (`cargo test -p dicom-pixeldata --lib`): **40 passed, 1 failed (this
  candidate's own temporary repro test, reverted before writing this record) -> 41 passed, 0
  failed** after the fix, with both temporary repro tests included. No other test in the
  `dicom-pixeldata` crate's existing suite was affected.

## What's not done / open questions

- The "does release mode ever get defeated by a wrap landing on a small value" question (raised in
  "Status" above) was not separately constructed - it would require finding a combination whose
  true (unrepresentable) product happens to be a near-exact multiple of `2^64` plus a small
  remainder, which is a much narrower target to hit than the debug-panic case and was judged lower
  priority once the fix (which closes both paths identically via saturation) was already found and
  verified working for the cases actually tested.
- The "standard" (non-embedded) overlay path in `decode_overlay_group` (`num_pixels =
  frames as usize * rows as usize * columns as usize`, used for `unpack_overlay_bits` when
  Overlay Data *is* present) was checked and confirmed **not** to overflow on its own for any valid
  combination of its inputs (max ≈ 9.22×10^18, comfortably under `u64::MAX`) - only the *embedded*
  path's additional `offset`/`bytes_per_sample` terms push the total over the edge. Not itself a
  bug, but worth remembering if `frames`'s type or bound ever changes.
- Whether Image Frame Origin being read via `to_int::<i32>()` without cross-checking the wire
  element's VR against the dictionary's `US` expectation is itself a separate, broader concern
  (beyond just this overflow) was not investigated as its own candidate - noted here as a
  general observation about this crate's attribute-reading conventions, not a confirmed second bug.
- 32-bit target platforms were not considered in this analysis (this session's repro and fix were
  built/verified only for `x86_64-unknown-linux-gnu`) - on a 32-bit `usize`, several of these
  multiplications would already overflow well before the specific `i32::MAX`/`u16::MAX` extremes
  used here, meaning the *unpatched* code likely has an even lower bar to trigger this crash on
  32-bit; the fix (which uses `usize`-generic `checked_mul`) closes that too, but this wasn't
  independently verified by actually building for a 32-bit target this session.
