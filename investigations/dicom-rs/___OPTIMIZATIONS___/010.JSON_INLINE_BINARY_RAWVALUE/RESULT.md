# JSON InlineBinary serialization: bypass serde_json's per-byte escape scan via RawValue

A fresh discovery this session (not from prior session notes). Profiling `dicom_json::to_string`
on `pydicom/liver.dcm` (a native/non-encapsulated file whose ~96KB PixelData gets base64-encoded
as `InlineBinary` per the DICOM JSON model) with callgrind showed **41.19% of total instructions
in `serde_json::ser::format_escaped_str`** - the generic per-byte escape-scanning loop
`serialize_str` always runs, checking every byte against a 256-entry escape table even though the
base64 alphabet (`A-Za-z0-9+/=`) can never contain a byte that needs JSON escaping.
`json/src/ser/value.rs`'s `InlineBinary::serialize` calls `serializer.serialize_str(&str)` after
base64-encoding the value - unconditionally paying that scan for content that is provably
escape-free by construction.

## What was tried

Changed `InlineBinary::serialize` to wrap the base64 string in JSON quotes itself (`"` + base64 +
`"` - always a single well-formed JSON string value, since base64 never contains a quote,
backslash, or control character) and hand it to `serde_json::value::RawValue::from_string_unchecked`
(an `unsafe fn`, added a Cargo.lock bump from serde_json 1.0.150 to 1.0.151 to gain access to it -
the safe `from_string` re-parses/validates the content, which costs about as much as the scan being
avoided and was tried first, giving only a small -3.3% win before switching to the `_unchecked`
variant). `RawValue`'s own `Serialize` impl writes its bytes directly into the output stream,
skipping `serialize_str`'s escape scan entirely. Added a `RAW_VALUE_MIN_LEN = 64` byte threshold
below which the original `serializer.serialize_str(&str)` path is kept, since a real, measured
small regression showed up for content too short for the scan to matter (see Result).

Bench: `___OPTIMIZATIONS___/harness/bench_pipeline`'s new `json-encode` mode
(`dicom_json::to_string(&obj)`) against `pydicom/liver.dcm` (98KB native PixelData -> ~131KB base64
InlineBinary, well over the threshold) and `color3d_jpeg_baseline.dcm` (JSON output only 38.9KB
despite the 6.14MB source file - its pixel data is JPEG-encapsulated, which this DICOM-JSON
serializer does not inline as base64, so this file has little/no `InlineBinary` content and serves
as the "below threshold / not applicable" case).

| Variant | liver.dcm (131KB InlineBinary) p50 | color3d.dcm (~no InlineBinary) p50 |
|---|---|---|
| baseline (`serialize_str` always) | 0.142 ms (2 rounds) | 0.024 ms |
| patched (RawValue above 64 bytes) | 0.125/0.126 ms (2 rounds) | 0.025 ms |

## Result

**liver.dcm: -11.9%/-11.3% across two independent rounds, 9/9 paired wins both times,
consistent** - a real, solid win, though considerably smaller than the raw 41%-of-instructions
profile figure suggested (the escape scan is only part of the total encode cost - base64 encoding
itself was a separate 22.11% of the same profile, untouched by this fix, plus the harness's own
`fnv1a` checksum and JSON's numeric/structural overhead). **color3d.dcm (below the 64-byte
threshold, takes the unchanged `serialize_str` path): a small, real, reproducible +5.2%/+6.0%
regression across two rounds, both fully consistent (0/9 either time), despite the code path being
logically identical to baseline** - see "Why" below; this was not resolved and is reported as-is
rather than chased further, per this candidate's own bounded time budget.

## Does it change behavior/output?

- **Bit-exact?** Yes. The JSON output string is byte-identical between baseline and patched builds
  on both fixtures (FNV-1a checksum of the full JSON text matches) - `RawValue` only changes how
  the bytes get into the output stream, never their content.
- **Is this a breaking change?** No public API/type changes - `InlineBinary::serialize`'s behavior
  (the JSON it produces) is unchanged; only its internal implementation differs. Adds one `unsafe`
  block to `json/src/ser/value.rs` (none existed there before) and bumps `serde_json` from 1.0.150
  to 1.0.151 in `Cargo.lock` (within the crate's already-declared `"1.0.96"` requirement, a
  same-major/minor patch bump).
- Project test suite: **537 passed / 0 failed / 6 ignored, both before and after** (full
  `cargo test --workspace`, including doctests) - no new failures. All 12 `dicom-json` unit tests
  and 5 doctests (including `write_full_file_to_json`, which exercises a real `InlineBinary` value)
  pass unchanged.

## Why the result looks the way it does

`format_escaped_str_contents` (serde_json 1.0.151, confirmed by reading its source) is a scalar,
byte-by-byte loop with no SIMD/bulk fast path in this version - for a ~131KB string that's a real,
non-trivial cost, exactly the "bitstream/entropy-style single-byte-at-a-time loop" pattern
HEURISTICS.md flags as a high-hit-rate target. `RawValue::from_string_unchecked` sidesteps it
entirely by asserting (via its documented `unsafe` contract) that the content is already
known-valid JSON, which is true here by construction (the base64 alphabet guarantee holds
unconditionally, not just for this file). The **below-threshold regression is not explained** -
the guarded code path is logically identical to baseline (same `serializer.serialize_str(&str)`
call), so the residual +5-6% at ~24-27 microseconds total runtime is either measurement noise at a
very small absolute timescale (plausible - this operation is fast enough that a few mispredicted
branches or slightly different codegen from the extra length-check branch could show up
proportionally) or an effect of the `serde_json` 1.0.150->1.0.151 point-release bump touching
something in a shared/unrelated code path (not confirmed either way - reverting just the Cargo.lock
version bump while keeping the length-guarded code was not tried, an available next step for
whoever picks this up).

## What's not done / open questions

- The below-threshold color3d regression's root cause was not confirmed (see above) - a next step
  would be reverting only the `serde_json` version bump (keeping code changes, since the threshold
  makes them a no-op below 64 bytes) to isolate whether it's the dependency bump or something else.
- `RAW_VALUE_MIN_LEN = 64` was not tuned precisely - it's a round number chosen to clearly exclude
  tiny/empty content, not a measured crossover point (unlike this workspace's earlier
  rayon-threshold candidates, which did measure their crossovers directly).
- The base64 encoding step itself (22.11% of the original profile, `base64` crate's
  `internal_encode`) was not investigated - a separate, real cost this candidate doesn't touch.
- Not investigated: whether the same escape-scan-avoidance technique applies anywhere else in
  `dicom-json`'s serializer (e.g. other string-valued fields that are provably escape-free by
  construction, such as certain tag/VR string representations) - `InlineBinary` was the only site
  checked this session, chosen because it was the top profile hit.

## Follow-up: fresh-clone re-run leans toward "noise", not a real regression

Running this candidate's own `just test` from a fresh clone (a different build, same box)
reproduced the liver.dcm win cleanly (-11.0%, 8/9, consistent) but the color3d comparison came back
**NOT consistent** (2/9) this time, unlike the two live-repo rounds above (which were both fully
consistent 0/9). Combined with the fact that the below-threshold code path is logically identical
to baseline, this makes "measurement noise at a ~24-27 microsecond timescale" the more likely
explanation of the two offered above, rather than a real, systematic effect from the `serde_json`
version bump - though still not conclusively ruled out either way.
