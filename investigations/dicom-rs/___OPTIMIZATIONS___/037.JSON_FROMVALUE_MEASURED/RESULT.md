# Candidate 026's fix, measured for `dicom_json::from_value` (never had raw JSON text to begin with)

Candidate 026's `RawValue`+`from_str` fix was only ever benchmarked via the `from_str`/
`from_reader`-style entry points (real JSON text in, typed object out). `dicom_json::from_value`
(`json/src/de/mod.rs:43`) is a third public entry point that deserializes directly from an
already-built `serde_json::Value` (e.g. constructed via the `json!` macro) - it never has raw JSON
text at all, so whether/how much it benefits from a fix whose whole mechanism is "capture raw text
instead of a generic tree" was unclear and unmeasured. `IDEAS.md` flagged this as an open gap
(confirmed still *correct* post-fix via the existing test suite, but never benchmarked).

## What was tried

Added a new harness mode, `json-decode-fromvalue <file> <iters>` (harness-only, not part of
`patch.diff`; needed adding `serde_json` as a direct harness dependency, matching `dicom-json`'s
own resolved version `1.0.96`, since the harness previously had no direct access to
`serde_json::Value`). Builds a `serde_json::Value` once (via `serde_json::from_str` on the
already-encoded JSON, outside the timed loop, same isolation discipline as the existing
`json-decode` mode), then times `dicom_json::from_value` per iteration on a fresh `.clone()` of
that `Value` (the clone itself sits outside the timer - `from_value` consumes its argument).

Re-applied candidate 026's `patch.diff` unchanged; two binaries (baseline, patched) compared via
`tools/paired.sh` (`RUNS=9`) on `seq_10k.dcm` (the same fixture candidate 026's own headline
number came from).

| Variant | round 1 | round 2 (re-check) | round 3 (fresh `just test` clone) |
|---|---|---|---|
| baseline | 14.30 ms p50 | 17.05 ms p50 | 14.94 ms p50 |
| patched | 10.98 ms p50 | 10.69 ms p50 | 10.75 ms p50 |

## Result

**A real, consistent win, but smaller and noisier than the `from_str` entry point's own -59 to
-66%-class win on the same fixture (candidate 036)**:

- Round 1: -23.2%, 9/9 - consistent.
- Round 2 (re-check): -37.3%, 9/9 - consistent, same direction, but a notably different
  magnitude (baseline itself varied more between rounds too - 14.30ms vs. 17.05ms p50 - this
  fixture's `from_value` path is noisier on this box than `from_str`'s own measurement was).
- Round 3 (fresh-clone `just test`): -28.1%, 9/9 - consistent, sitting between rounds 1 and 2,
  confirming the noisier-but-real-win shape rather than either round being an outlier.

All three rounds agree `from_value` benefits from the fix, just less dramatically and less
precisely than `from_str`/`from_reader`. Time only was measured.

## Does it change behavior/output?

- **Bit-exact?** Yes - `checksum=0xd583b32455a46900` identical baseline vs. patched, and matches
  candidate 036's own `from_str`-path checksum on the same fixture exactly (confirms `from_value`
  and `from_str` produce the identical decoded object, as expected).
- **Is this a breaking change?** No - internal `json` crate deserializer change, unaffected by
  this candidate; `from_value`'s own public signature is untouched.
- Project test suite: `cargo test --release -p dicom-json`: **12 + 5 = 17 passed / 0 failed**
  (patch applied), matching candidates 026/036's own recorded count. Reverted-tree (baseline)
  count is identical.

## Why the result looks the way it does

`serde_json`'s `Value` type implements `Deserializer` well enough to support the `raw_value`
feature's special token, but doing so for a value that's already a parsed `Value` tree (not raw
text) requires internally re-serializing the relevant sub-`Value` back into a string to hand to
`Box<RawValue>` - a real cost `from_str` never pays, since it already has the text. This plausibly
explains both the smaller relative win (some of what `from_str` avoids gets partially paid back
here) and the noisier measurement (an extra serialize-then-reparse round trip per raw-captured
field, on top of the timed work, is a less uniform cost than a straight buffer read). Not confirmed
via profiling - this is the most likely explanation given `serde_json`'s documented `raw_value`
mechanics, not something traced in code for this candidate.

## What's not done / open questions

- The exact re-serialization mechanism inside `serde_json`'s `Value`-as-`Deserializer` raw-value
  support was not traced in `serde_json`'s own source to confirm the "Why" section's explanation -
  treat it as a plausible hypothesis, not a verified fact.
- Only tested on `seq_10k.dcm` - not cross-checked against `rtplan.dcm` or a flat-only fixture
  (candidate 036's own `gen-flat` mode) to see whether `from_value`'s SQ-vs-flat win breakdown
  matches `from_str`'s (candidate 036 found `from_str`'s win is dominated by the SQ-recursion
  path - untested whether the same split holds for `from_value`).
- This closes the `IDEAS.md` backlog item asking for this measurement - the result (real but
  smaller/noisier win) is the answer, no further action strictly required, though the two open
  items above would sharpen it.
