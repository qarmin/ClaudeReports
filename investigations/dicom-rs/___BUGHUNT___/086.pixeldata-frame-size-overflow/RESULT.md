# pixeldata-frame-size-overflow

`pixeldata/src/lib.rs`'s `DecodedPixelData::frame_data` (~line 570) and the `ModalityLutOption::
None` branch of `to_dynamic_image_with_options`'s 16-bit monochrome path (~line 1184) both compute
`frame_length = rows * cols * samples_per_pixel * <bytes-per-sample-or-2>`, then `frame_start =
frame_length * frame`, `frame_end = frame_start + frame_length` in plain `usize` arithmetic, and
only check `frame_end > data.len()` *after* the multiply/add already happened. `rows`/`cols`/
`samples_per_pixel`/`bits_allocated` are attacker-controlled DICOM attribute values (Rows/Columns/
SamplesPerPixel/BitsAllocated are all `US`, 16-bit unsigned, in the dictionary) and `frame` is an
unchecked caller-supplied `u32`, matching this codebase's already-confirmed "two+ attacker-
influenced integers multiplied in a type narrower than their combined range" pattern from
candidates `010`/`021`/`037`/`038`. Flagged by an unmerged backlog entry pointing at an existing
reference fix branch (`origin/fix/pixeldata-frame-size-overflow`, `5cff515e`) that explicitly calls
out these two spots in `pixeldata/src/lib.rs` as not yet covered by the closed-out candidates `037`
(a different file, `overlay.rs`) and `038` (the same file, but a different function -
`PixelDecoder::decode_pixel_data_frame`'s default trait impl).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value analysis (Python) of the exact multiply chain against `u64::MAX` on a 64-bit target, for both spots, using each attribute's own legal maximum (`Rows`/`Columns`/`SamplesPerPixel` up to `65535` as `US`, `BitsAllocated` up to `65535` giving `bytes_per_sample` up to `8192`) | Static reasoning, no build needed | Neither spot's `rows*cols*samples_per_pixel*<bytes-or-2>` product alone overflows `u64` at max legal attribute values (`frame_data`'s worst case is ~2.3x10^18, `ModalityLutOption::None`'s is ~8.6x10^9, both under `u64::MAX`/1.8447x10^19) - but multiplying either by an ordinary-looking `frame` value pushes it over: for `frame_data`, `frame=9` on the worst-case attribute values already exceeds `u64::MAX`; for the `ModalityLutOption::None` spot (which hardcodes `bytes_per_sample=2` and requires `samples_per_pixel=1` to be reachable via `to_dynamic_image_with_options`'s own dispatch), a `frame` around 2.15 billion (still within a plausible `NumberOfFrames`, capped at `i32::MAX` by this crate's own `number_of_frames` attribute reader) is enough |
| Hand-built boundary input via the public `PixelDecoder`/`DecodedPixelData` API: a real `FileDicomObject<InMemDicomObject>` built with `Rows=Columns=SamplesPerPixel=BitsAllocated=65535`, `NumberOfFrames=10`, a tiny 4-byte native `PixelData`, decoded via `.decode_pixel_data()`, then `.frame_data(9)` called directly (an entirely ordinary "9th frame of a claimed-10-frame file" request) - debug build, `cargo test` (temporary unit test added to `pixeldata/src/lib.rs`, reverted before this record was written) | `cargo test -p dicom-pixeldata --lib` | Panics: `attempt to multiply with overflow` at `pixeldata/src/lib.rs:576` (the `frame_start = frame_length * frame as usize` line). Reproduced 3/3 in-tree unit-test runs |
| Same technique for the second spot: `Rows=Columns=65535`, `SamplesPerPixel=1`, `BitsAllocated=16` (so `to_dynamic_image_with_options` dispatches through `build_monochrome_image`'s `bits_allocated == 16` arm, taking the `ModalityLutOption::None` branch), `NumberOfFrames=1`, tiny 2-byte native `PixelData`, `.decode_pixel_data()` then `.to_dynamic_image_with_options(3_000_000_000, &ConvertOptions::new().with_modality_lut(ModalityLutOption::None))` - debug build, `--features image`, temporary unit test | `cargo test -p dicom-pixeldata --lib --features image` | Panics: `attempt to multiply with overflow` at `pixeldata/src/lib.rs:1188` (the `frame_start = frame_length * frame as usize` line in the `ModalityLutOption::None` branch). Reproduced 3/3 in-tree unit-test runs |
| Standalone `repro/` (own `Cargo.toml`, bare `[workspace]`) built against a fresh clone of pristine upstream `df2b9e42`, exercising both spots through the real public `PixelDecoder`/`DecodedPixelData` API end to end | Debug build (`cargo run`), 3 separate runs | Both panics reproduced 3/3 runs each, identical message/location every run; against the same clone with `patch.diff` applied, both return a graceful `Err(FrameOutOfRange)` instead, 1/1 confirmed (see "The fix") |
| `--release` build with the same two inputs (per `CONFIRMATION.md` section 2) | `cargo test -p dicom-pixeldata --lib --features image --release` | Neither spot panics in release: for the specific inputs constructed this session, the wrapped (post-overflow) `frame_end` values happen to still be far larger than the tiny actual `data.len()` (4 and 2 bytes respectively), so the existing `if frame_end > data.len()` check still (accidentally) rejects both as `FrameOutOfRange` - the same "coincidence of the exact values chosen" caveat candidate `037` already documented for the same bug class, not a guarantee for every overflowing input |

## Status

**Confirmed** - deterministic, pure integer-overflow panic with no layout/seed/timing dependency;
reproduces 3/3 in-tree unit-test runs plus 3/3 standalone `repro/` runs against a fresh clone of
pristine upstream `df2b9e42`, for both spots, identical panic message and location every time.
**Debug build only** (`cargo test`/`cargo run` dev profile, debug-assertions on by default) - a
`--release` build with the specific inputs constructed this session does not panic (see above),
though this is not a guarantee for all overflowing inputs, since the wrap could in principle land
on a small `frame_end` for a different combination, silently defeating the length check rather than
merely surviving it by chance; this was not separately constructed or confirmed this session.

**Fully reproducible**: Yes. `repro/` drives the real, public `PixelDecoder::decode_pixel_data()` +
`DecodedPixelData::frame_data`/`to_dynamic_image_with_options` API end to end on a real
`FileDicomObject<InMemDicomObject>` built through the crate's own public element-insertion API
(`put_element`) - the same public entry point any real caller (a DICOM viewer, a PACS client
parsing an incoming file) would use. There is no second "peer" process to run here: the untrusted
input this bug is about is the DICOM object's own attribute values, which this repro supplies
through the exact same public path a real file parse would populate them through, not a hand-
constructed stand-in for another process's behavior.

## Class

`crash` (an `attempt to multiply with overflow` panic in a debug build, reachable from ordinary use
of the public `DecodedPixelData::frame_data` and `to_dynamic_image_with_options` API on a crafted-
but-individually-legal DICOM object - no `unsafe`, no memory-safety implication beyond the panic
itself in the manifestation observed this session).

## Root cause

Both spots multiply several attacker-controlled `u16` attribute values together and then multiply
the result again by a caller-supplied `u32` frame index, entirely in plain (unchecked) `usize`
arithmetic, before ever comparing the result against the actual buffer length. Each individual
factor is bounded by its own DICOM attribute's nominal 16-bit range, but their *combined* product
(including the `frame` multiplier) can exceed what a 64-bit `usize` can represent - the same root
cause already documented in `projects/dicom-rs.md` for candidates `010`/`021`/`037` in this
codebase: *"whenever two or more separately-sourced attacker-influenced integers get multiplied/
added in a type narrower than what their combined range could produce, check the extremes of
both/all, not just one."* This is the same file and the same general bug class as candidate `038`
(`PixelDecoder::decode_pixel_data_frame`'s default trait impl), but a different function/branch
pair, exactly as the backlog entry that raised this candidate anticipated.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Replaced the plain `*`/`+` arithmetic with `checked_mul` (for the attribute-value product) and
`saturating_mul`/`saturating_add` (for the `frame`-index multiply/add), saturating to `usize::MAX`
on overflow - identical in shape to the fix already applied for candidates `037` and `038`:

```
-        let frame_length = self.rows as usize
-            * self.cols as usize
-            * self.samples_per_pixel as usize
-            * bytes_per_sample;
-        let frame_start = frame_length * frame as usize;
-        let frame_end = frame_start + frame_length;
+        let frame_length = (self.rows as usize)
+            .checked_mul(self.cols as usize)
+            .and_then(|n| n.checked_mul(self.samples_per_pixel as usize))
+            .and_then(|n| n.checked_mul(bytes_per_sample))
+            .unwrap_or(usize::MAX);
+        let frame_start = frame_length.saturating_mul(frame as usize);
+        let frame_end = frame_start.saturating_add(frame_length);
```

(and the same shape for the `ModalityLutOption::None` branch's hardcoded-`* 2` variant). Saturating
to `usize::MAX` means the very next line's existing `if frame_end > data.len() { ... FrameOutOfRange
... }` check continues to work unmodified and unconditionally catches every overflow case as "frame
out of range", regardless of how large the true (unrepresentable) product would have been - closing
both the debug-panic path and the theoretical release-mode small-wrap path in one change, without
needing a wider integer type or changing either function's signature/behavior on any non-overflowing
input.

Verified against boundary inputs (`CONFIRMATION.md` section 7): both overflow inputs above now
return `Err(FrameOutOfRange { .. })` instead of panicking, in both debug and release builds; a
plain, non-overflowing `frame_data(0)` / `to_dynamic_image_with_options(0, ...)` call on the same
crafted objects still succeeds identically before and after the fix (confirmed via the existing
`test_frame_out_of_range` and other pre-existing tests in the crate's suite, none of which changed
outcome).

- Project test suite (`cargo test -p dicom-pixeldata --lib --features image`): **58 passed, 0
  failed, 2 ignored** before the fix, and **58 passed, 0 failed, 2 ignored** after the fix
  (identical - the existing suite never exercised either overflowing spot, so the fix changes no
  pre-existing test outcome; the two temporary unit tests added to confirm the bug were removed
  before this record was written, matching the numbers reported here).

## What's not done / open questions

- The reference fix branch this candidate was raised from (`origin/fix/pixeldata-frame-size-
  overflow`, `5cff515e`) also touches several *other* spots in this same file with the same
  unchecked-multiply shape: the bit-expansion arithmetic inside `decode_pixel_data` itself (around
  what is now line ~2320, `frame_pixels`/`frame_samples`/`frame_size_all` for `bits_allocated == 1`)
  and the "Encapsulated Uncompressed" primitive-value frame-slicing arithmetic inside
  `to_vec_frame`/`convert_pixel_slice`'s underlying helper (around line ~2478, `frame_pixels`/
  `frame_samples`/`frame_size`/`frame_offset`). Neither of those was in this candidate's assigned
  scope (the backlog entry named only `frame_data` and the `ModalityLutOption::None` branch,
  distinguishing them from candidate `038`'s already-fixed `decode_pixel_data_frame` default trait
  impl) and neither was investigated or fixed this session - added to `IDEAS.md` as a new, separate
  suspect.
- Release-mode behavior for an overflowing input that lands on a *small* wrapped `frame_end` (which
  would silently defeat the length check rather than merely surviving it by luck, as candidate
  `037` already flagged as a theoretical concern for the same bug shape) was not constructed or
  confirmed this session for either spot - the fix closes this regardless via saturation, but the
  pre-fix release-mode risk was not independently demonstrated.
