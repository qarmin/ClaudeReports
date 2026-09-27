# JPEG XL 24-bit float decode branch: unclamped cast, signal seen but not confirmed

From `IDEAS.md`'s backlog, noticed while working candidate 017
(`017.jpegxl-jpegls-dimension-mismatch-check`): `JpegXlAdapter::decode_frame`'s 24-bit
(float-per-sample) branch casts each decoded `f32` sample directly to a 24-bit integer with no
clamp, which looked worth a closer look given DICOM's float pixel data support can legitimately
carry HDR content (samples outside the usual `[0.0, 1.0]` normalized range).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading | `transfer-syntax-registry/src/adapters/jpegxl.rs`'s `24 =>` branch: `let bytes = &((sample * 16777215.) as u32).to_ne_bytes();`. Confirmed Rust's `as` float-to-int cast has been saturating (not wrapping, not UB) since Rust 1.45 - a `sample` outside `[0.0, 1.0]` (or NaN) cannot panic or produce memory-unsafety here, only a silently-clamped-to-`[0, 16777215]` (or `0` for NaN) integer value, i.e. a possible wrong-but-plausible-looking pixel value for genuinely out-of-range HDR float content. Confirmed the `24`-bit branch is a deliberately reachable, non-hypothetical case (the surrounding `match bits_allocated` has explicit arms for `1` (rejected), `8`, `16`, `24`, and `_ => unreachable!()` - the `unreachable!()` implies an earlier validation elsewhere already restricts `bits_allocated` to this exact set, so `24` is a real, supported configuration, not dead code). |
| Dependency documentation check | Read `jxl-oxide` 0.12.6's own source (`fb.rs`) for its `FrameBufferSample for f32` output contract, and compared it against the sibling `u16`/`u8` impls in the same file. | Confirmed the asymmetry directly from the dependency itself: the `f32` output impl's doc comment states "Output as 32-bit float samples, with **nominal** range of `[0, 1]`" (not "guaranteed"/"clamped to"), and its `copy_from_f32` conversion path has no `.clamp(...)` call anywhere - unlike the `u16`/`u8` sibling impls in the same file, which explicitly `.clamp(0.0, 65535.0)`/`.clamp(0.0, 255.0)` before their own integer conversion. This corroborates the mechanism directly (JPEG XL's real HDR support can legitimately produce float samples outside `[0, 1]`, and this specific dependency's own float output path does not clamp them), stronger evidence than speculation alone, but still not an *observed* wrong output. |
| Attempted reproduction | Considered constructing a JPEG XL image with float samples genuinely outside `[0.0, 1.0]` (HDR content) to demonstrate silently-wrong output end-to-end through this crate. | Not attempted - would require either a real-world HDR JPEG XL DICOM sample (none available in this project's existing test corpus, `dicom-test-files`, or `dcms/`) or manually encoding one with a JPEG XL encoder capable of emitting genuinely out-of-range linear-light float samples - a nontrivial construction not attempted given the low severity (a display-quality wrong-pixel-value issue, not a crash/security concern) relative to the effort required, especially compared to other candidates this session with a clearer, faster path to a real reproduction. |

## Status

**Inconclusive** - a real code-level signal (an unclamped float-to-int cast in a genuinely
reachable code path, on data whose value range this specific format can legitimately exceed the
assumed `[0.0, 1.0]` bound for) was found by reading the code, but no actual wrong-output
reproduction was constructed, per `CONFIRMATION.md`'s bar that a repeatable, *observed* bad
behavior - not just a plausible mechanism - is required before writing `Confirmed`.

## Class

N/A (not confirmed).

## Root cause

Strongly suspected but not conclusively established: `jxl-oxide` 0.12.6's own `f32` output path
does not clamp (confirmed by reading its source directly - see above), and its documentation only
claims a "nominal" `[0, 1]` range rather than a guaranteed one, consistent with JPEG XL's real
HDR capability. What's still missing is an *observed* out-of-range value flowing through this
exact code path from a real decode, which is what `CONFIRMATION.md` requires before writing
`Confirmed`.

## The fix

Not applied - per this skill's discipline, no unverified fix was committed for a signal that
didn't clear the confirmation bar. A defensive `sample.clamp(0.0, 1.0)` before the multiply-cast
would be a trivial, low-risk change if a future session does confirm this with real HDR content,
but adding it now without a demonstrated problem would be exactly the kind of "fix without a
proven bug" this skill's discipline exists to avoid.

## What's not done / open questions

- The core open question is empirical: does any realistic JPEG XL content `jxl-oxide` can decode
  (via this crate's usage of it) ever actually produce `f32` samples outside `[0.0, 1.0]` for the
  24-bit DICOM pixel format path? If yes, this is very likely a real (low-severity) bug worth
  fixing with a clamp; if `jxl-oxide` itself already normalizes/clamps before returning samples,
  this is a non-issue and the lead should be closed as ruled-out instead. Neither was established
  this session.
- If picked up again: the fastest path to a real answer is probably reading `jxl-oxide`'s own
  `write_to_buffer`/pixel-format documentation directly (does it document guaranteed output range
  for float formats?) rather than attempting to construct a specific triggering JPEG XL file from
  scratch.
