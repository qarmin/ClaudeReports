# pixeldata-frame-size-overflow-remaining-spots

`pixeldata/src/lib.rs` has two more spots with the same unchecked-multiply-then-late-bounds-check
shape already fixed at two other spots in this file by candidate `086` (`DecodedPixelData::
frame_data` and the `ModalityLutOption::None` branch) and at a third spot by candidate `038`
(`PixelDecoder::decode_pixel_data_frame`'s default trait impl): the `bits_allocated == 1`
bit-expansion arithmetic inside `decode_pixel_data`'s native/non-encapsulated branch
(`frame_pixels`/`frame_samples`/`frame_size_all`, ~line 2367) and the `DicomValue::Primitive`
"non-encoded, single frame" branch inside `decode_pixel_data_frame` (`frame_pixels`/
`frame_samples`/`frame_size`/`frame_offset`, ~line 2519). Both multiply attacker-controlled
`Rows`/`Columns`/`SamplesPerPixel`/`BitsAllocated` (`US`, 16-bit unsigned DICOM attributes) by
either a legal-but-large `NumberOfFrames` (spot 1) or an unchecked caller-supplied `frame: u32`
with no bound against `NumberOfFrames` (spot 2), entirely in plain `usize` arithmetic, before any
length check runs. Flagged by an unmerged reference fix branch (`origin/fix/pixeldata-frame-size-
overflow`, `5cff515e`) that candidate `086` deliberately scoped down to its own two spots, leaving
these two (plus the already-separately-confirmed `038` spot) explicitly noted as open in `086`'s
own "What's not done" section and in `IDEAS.md`'s backlog.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value analysis of both multiply chains against `u64::MAX` on a 64-bit target, using each attribute's own legal maximum (`Rows`/`Columns`/`SamplesPerPixel` up to `65535` as `US`) | Static reasoning, no build needed | Spot 1: `frame_pixels*frame_samples/8` alone reaches ~3.5x10^13 at max legal `Rows`/`Columns`/`SamplesPerPixel`, so an ordinary-looking `NumberOfFrames` above ~525,000 (still a single legal `IS` integer, well under this attribute's own `i32` ceiling already established by candidate `086`) pushes `frame_size_all` past `u64::MAX`. Spot 2: `frame_size` alone (`Rows=Cols=65535`, `SamplesPerPixel=1`, `BitsAllocated=16`) reaches ~8.6x10^9, so a `frame` value above ~2.15 billion (well under `u32::MAX`) pushes `frame_offset` past `u64::MAX` |
| Hand-built boundary input via the public `PixelDecoder` API: a real `FileDicomObject<InMemDicomObject>` built with `put_element` (`Rows=Columns=SamplesPerPixel=65535`, `BitsAllocated=1`, `NumberOfFrames=600000`, a tiny 4-byte native `PixelData`) decoded via `.decode_pixel_data()` for spot 1; a second object (`Rows=Columns=65535`, `SamplesPerPixel=1`, `BitsAllocated=16`, `NumberOfFrames=1`, a tiny 2-byte native `PixelData`) via `.decode_pixel_data_frame(3_000_000_000)` for spot 2 - debug build, temporary `pixeldata/examples/repro_check.rs` added to the main project (reverted before this record was written) | `cargo run -p dicom-pixeldata --example repro_check --features image`, 3 separate runs | Both panic: spot 1 at `pixeldata/src/lib.rs:2370:42` (`let frame_size_all = frame_size * (number_of_frames as usize);`), spot 2 at `pixeldata/src/lib.rs:2526:36` (`let frame_offset = frame_size * (frame as usize);`), both `attempt to multiply with overflow`. Reproduced 3/3 runs each, identical message/location every time |
| Same two inputs, `--release` | `cargo run -p dicom-pixeldata --example repro_check --features image --release` | Neither spot panics: the wrapped (post-overflow) values happen to still exceed the tiny real `data.len()` (4 and 2 bytes respectively), so the existing length check still (accidentally) rejects both as `FrameOutOfRange` - same "coincidence of the exact values chosen" caveat already documented for candidates `037`/`086`, not a guarantee for every overflowing input |
| Standalone `repro/` (own `Cargo.toml`, bare `[workspace]`) via `just test`, built against a fresh clone of pristine upstream `df2b9e42` | Debug build (`cargo run`) | Both panics reproduced identically (same messages/locations as above) against the fresh clone; against the same clone with `patch.diff` applied, both return a graceful `Err(FrameOutOfRange)` instead, confirmed via a second `cargo run` in the same `just test` invocation |

## Status

**Confirmed** - deterministic, pure integer-overflow panic with no layout/seed/timing dependency;
reproduces 3/3 in-tree runs plus a further clean reproduction via the standalone `repro/`'s
fresh-clone `just test` run, for both spots, identical panic message and location every time.
**Debug build only** (`cargo run`/`cargo test` dev profile, debug-assertions on by default) - a
`--release` build with the specific inputs constructed this session does not panic, for the same
reason already documented by candidates `037`/`086` (not a guarantee for every overflowing input,
since the wrap could in principle land on a small value that silently defeats the length check
rather than merely surviving it by chance; this was not separately constructed or confirmed this
session).

**Fully reproducible**: Yes. `repro/` drives the real, public `PixelDecoder::decode_pixel_data()`/
`decode_pixel_data_frame()` API end to end on a real `FileDicomObject<InMemDicomObject>` built
through the crate's own public element-insertion API (`put_element`) - the same public entry point
any real caller (a DICOM viewer, a PACS client parsing an incoming file) would use. There is no
second "peer" process to run here: the untrusted input this bug is about is the DICOM object's own
attribute values, which this repro supplies through the exact same public path a real file parse
would populate them through, not a hand-constructed stand-in for another process's behavior.

## Class

`crash` (an `attempt to multiply with overflow` panic in a debug build, reachable from ordinary use
of the public `PixelDecoder::decode_pixel_data()`/`decode_pixel_data_frame()` API on a
crafted-but-individually-legal DICOM object - no `unsafe`, no memory-safety implication beyond the
panic itself in the manifestation observed this session).

## Root cause

Same root cause already documented in `projects/dicom-rs.md` for candidates `010`/`021`/`037`/
`038`/`086`: whenever two or more separately-sourced attacker-influenced integers get multiplied in
a type narrower than what their combined range could produce, every individual factor being
"legal" for its own attribute doesn't bound their product. Spot 1 multiplies three `US` attributes
together and then by `NumberOfFrames` (an `IS`-encoded `i32`, no upper bound enforced beyond
`> 0`); spot 2 multiplies the same shape of attributes together and then by an entirely unchecked
caller-supplied `frame: u32` that is never compared against `NumberOfFrames` before the arithmetic
runs. Both are the same general function/branch family as `086` and `038` but in genuinely
different branches (native/non-encapsulated pixel data with 1-bit samples for spot 1; the
non-encapsulated single-frame path for spot 2), matching exactly what the reference fix branch and
`086`'s own "What's not done" section anticipated.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Replaced the plain `*` arithmetic with `checked_mul` (for the attribute-value product) and
`saturating_mul`/`saturating_add` (for the `NumberOfFrames`/`frame`-index multiply), saturating to
`usize::MAX` on overflow - identical in shape to the fix already applied for candidates `037`/`038`/
`086`:

```
-                    let frame_pixels = (rows as usize) * (cols as usize);
-                    let frame_samples = frame_pixels * (samples_per_pixel as usize);
+                    let frame_pixels = (rows as usize)
+                        .checked_mul(cols as usize)
+                        .unwrap_or(usize::MAX);
+                    let frame_samples = frame_pixels.saturating_mul(samples_per_pixel as usize);
                     let frame_size = frame_samples / 8;
-                    let frame_size_all = frame_size * (number_of_frames as usize);
+                    let frame_size_all = frame_size.saturating_mul(number_of_frames as usize);
+                    let take_count = frame_pixels.saturating_mul(number_of_frames as usize);
```

(and the same shape for spot 2's `frame_pixels`/`frame_samples`/`frame_size`/`frame_offset`, plus
introducing a `frame_end = frame_offset.saturating_add(frame_size)` so the existing `data.get(a..b)`
call is bounds-checked against a saturated end rather than a plain `frame_offset + frame_size`
addition that could itself overflow). Saturating to `usize::MAX` means the existing
`.context(FrameOutOfRangeSnafu { .. })?` / `data.get(..)` calls continue to work unmodified and
unconditionally catch every overflow case as "frame out of range" or a `None` from `.get()`,
regardless of how large the true (unrepresentable) product would have been.

Verified against the same boundary inputs used to confirm the bug (`CONFIRMATION.md` section 7):
both overflow inputs above now return `Err(FrameOutOfRange { .. })` instead of panicking, in both
debug and release builds. Spot 1's own pre-existing regression tests for `bits_allocated == 1`
(`test_1bit_image_decoding_data`, `test_1bit_image_decoding_data_frame`) and spot 2's ordinary
single-frame decode path continue to pass unchanged, confirming the fix doesn't alter behavior for
any non-overflowing input.

- Project test suite (`cargo test -p dicom-pixeldata --lib --features image`): **58 passed, 0
  failed, 2 ignored** before the fix, and **58 passed, 0 failed, 2 ignored** after the fix
  (identical - the existing suite never exercised either overflowing spot). Confirmed both ways by
  stashing/unstashing the fix hunk against the same tree, not just running it once after applying.

## What's not done / open questions

- Candidate `086`'s own scope (`frame_data`, `ModalityLutOption::None`) and candidate `038`'s scope
  (the default `PixelDecoder::decode_pixel_data_frame` trait impl) already cover the other three
  spots the reference fix branch (`origin/fix/pixeldata-frame-size-overflow`, `5cff515e`) touches
  in this same file - between `038`, `086`, and this candidate, every hunk in that reference branch
  is now accounted for (confirmed by re-diffing `5cff515e` against this candidate's own
  `patch.diff` plus `038`'s and `086`'s).
- Release-mode behavior for an overflowing input that lands on a *small* wrapped value (which would
  silently defeat the length check rather than merely surviving it by luck, as candidates `037`/
  `086` already flagged as a theoretical concern for the same bug shape) was not constructed or
  confirmed this session for either spot here either - the fix closes this regardless via
  saturation, but the pre-fix release-mode risk was not independently demonstrated.
