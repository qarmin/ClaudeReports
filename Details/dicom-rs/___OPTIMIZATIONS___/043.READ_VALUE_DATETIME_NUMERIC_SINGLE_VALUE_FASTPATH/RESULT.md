# Extending candidate 042's single-value fast path to read_value_da/ds/dt/is/tm

Candidate 042 added a single-value fast path to `read_value_strs` (`parser/src/stateful/decode.rs`)
and found a real, consistent -9 to -15% win, closing `IDEAS.md`'s original low-confidence hedge.
That same entry flagged five sibling functions sharing the identical
`buf.split(b'\\').map(...).collect()` shape - `read_value_da`, `read_value_ds`, `read_value_dt`,
`read_value_is`, `read_value_tm` (DA/DS/DT/IS/TM VRs) - as untested follow-up candidates.

## What was tried

Added the same single-value fast path (skip `split`+`collect`, execute the per-part transform
once directly on the whole buffer when it contains no `b'\\'`) to all 5 sibling functions. Each
function's own parse/validate logic (`parse_date_partial`, `f64`/`i32` parsing via
`DefaultCharacterSetCodec`, `parse_datetime_partial`, `parse_time_partial`) is reused unchanged -
only the split-iterator+collect machinery is skipped for the common single-value case.

Added a new harness mode, `gen-datetime-vrs <out> <n-items>` (harness-only, not part of
`patch.diff`): builds `n-items` groups of 5 single-valued elements (one DA, DS, DT, IS, TM each,
distinct private-group tags) - no real `dicom-test-files` fixture has enough of these VRs to time
meaningfully (a typical file has only a handful of DA/TM/DS/IS elements each). Built
`datetime_10k.dcm` (10,000 groups = 50,000 total elements).

| Variant | round 1 | round 2 (re-check) | round 3 (fresh `just test` clone) |
|---|---|---|---|
| baseline | 13.587 ms p50 | 13.534 ms p50 | 12.926 ms p50 |
| patched | 12.453 ms p50 | 12.668 ms p50 | 12.163 ms p50 |

## Result

**A real, consistent win, smaller in relative magnitude than candidate 042's own `read_value_strs`
result (-9 to -15%) but still clearly real**: -8.3% (round 1, 9/9), -6.4% (round 2, 9/9), and
-5.9% (round 3, fresh-clone `just test`, 9/9) - all three consistent, same direction every time.
Time only was measured (same reasoning as candidate 042 - this fix's mechanism has no plausible
memory-volume effect distinct from output size).

## Does it change behavior/output?

- **Bit-exact?** Yes - `sha256sum` of `dump` output (full parse-then-write round trip) identical
  baseline vs. patched: `76ed60c75cd50a6dfb3d694cf8485e65759e1132151ddced06236e8b888a1616`.
- **Is this a breaking change?** No - all 5 functions are private methods of `parser`'s internal
  `StatefulDecoder`. The empty-buffer edge case (each function already returns `PrimitiveValue::Empty`
  early for an empty trimmed buffer, before either code path) is unaffected; the single-value fast
  path only activates when `buf` is non-empty and contains no backslash, producing byte-identical
  results to the general `split`-based path for that same input (verified via the bit-exact check
  above, which exercises all 5 VRs at scale).
- Project test suite: `cargo test --release -p dicom-parser -p dicom-object`: **75 + 9 + 49 + 22 =
  155 passed / 0 failed**, both before and after - identical to candidate 042's own count (this
  workspace's usual baseline for these two crates). Also ran `cargo test --release --workspace`
  with the patch applied: every crate's test result line reported 0 failed, matching an unpatched
  baseline run.

## Why the result looks the way it does

Same mechanism as candidate 042 (skip `split()`'s iterator state machine + generic
`Result<C<T>>::collect()`), but the per-part transform these 5 functions do is itself more
expensive than `read_value_strs`'s plain text decode (date/time parsing via `chrono`-backed
`parse_date_partial`/etc., or a `str::parse::<f64>`/`<i32>` call) - so the *relative* share of
total per-call cost attributable to the skipped iterator/collect machinery is smaller here,
explaining the smaller (but still real) percentage win compared to candidate 042's own result.

## What's not done / open questions

- This closes the `IDEAS.md` backlog item asking for this extension - all 5 sibling functions
  candidate 042 identified as sharing the pattern are now covered, no further sites of this exact
  shape remain in this file (checked: `read_value_ob`/`_ss`/`_fl`/etc. are binary, not
  text-splitting, and were already the subject of candidates 009/025's separate zero-init work).
- No per-VR breakdown (which of the 5 contributes most to the win) was attempted - the fixture
  mixes all 5 evenly, matching `IDEAS.md`'s own original framing of the question at the
  `read_value_strs` family level, not per-VR.
