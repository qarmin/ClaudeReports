# StatefulDecoder::read_value_strs: single-value fast path avoiding split-iterator + collect

`read_value_strs` (`parser/src/stateful/decode.rs`, the decode-side counterpart to candidate 024's
`encode_texts_element`) always decodes via `self.buffer.split(b'\\').map(codec.decode).collect()`,
even when there is exactly one value (no delimiter) - the overwhelming majority case for most
multi-valued VRs (CS, UI, PN, LO, SH, ...) in real DICOM data. `IDEAS.md` flagged this as a
lower-confidence lead ("splitting is arguably required regardless of value count... confirm with a
profiler before committing time") since, unlike the encode-side fix, there's no obvious redundant
*buffer copy* here - only iterator/collect machinery overhead.

## What was tried

Added a fast path: when `self.buffer` contains no `b'\\'`, decode the whole buffer directly (one
`codec.decode()` call) and wrap it in a single-element `smallvec![decoded]`, skipping the
`split()` iterator and the generic `Result<C<String>>` collect entirely for the common case.
Multi-valued elements (2+, i.e. the buffer contains at least one backslash) fall through to the
existing `split`+`collect` path unchanged.

Bench: `parse` mode (isolates the whole decode path, the natural place this function's cost
shows up) against three fixtures of increasing single-value-string weight: `seq_10k.dcm` (10,000
sequence items, short ~20-40 byte single-valued UI strings), `seq_10k_long1000.dcm` (same
structure, but each item's extra `InstitutionName` value is 1000 bytes - candidate 032's own
longer-string fixture), and `pydicom/liver.dcm` (a real, unmodified clinical fixture, for a
real-world sanity check).

| Variant | `seq_10k.dcm` | `seq_10k_long1000.dcm`, round 1 | `seq_10k_long1000.dcm`, round 2 (re-check) | `seq_10k_long1000.dcm`, single-core (`taskset -c 2`) | `liver.dcm` (real-world) |
|---|---|---|---|---|---|
| baseline | 8.876 us p50 | 33.211 us p50 | 33.937 us p50 | 33.549 us p50 | 27.0 us p50 |
| patched | 8.093 us p50 | 30.127 us p50 | 30.664 us p50 | 30.517 us p50 | 23.0 us p50 |

## Result

**A real, consistent win at every scale and fixture tried - `IDEAS.md`'s low-confidence hedge did
not hold up:**

- `seq_10k.dcm` (short strings): -8.8%, 9/9 - consistent; reproduced independently via
  fresh-clone `just test` at -9.4%, 9/9.
- `seq_10k_long1000.dcm` (1000-byte single-value strings), round 1: -9.3%, 9/9 - consistent.
- `seq_10k_long1000.dcm`, round 2 (re-check): -9.6%, 9/9 - consistent, reproduces round 1;
  reproduced a third time via fresh-clone `just test` at -9.7%, 9/9.
- `seq_10k_long1000.dcm`, single-core (`taskset -c 2`): -9.0%, 9/9 - consistent, no rayon
  involvement in this code path so single/multi-core parity is expected and confirmed.
- `pydicom/liver.dcm` (real, unmodified clinical fixture, mostly short single-valued elements):
  **-15.5%**, 8/9 - consistent, the largest relative win of the fixtures tried, on real data.

Time only was measured - this fix's mechanism (skip an iterator/collect machinery call, not an
allocation) has no plausible memory-volume effect distinct from the already-minimal output size;
no peak-RSS follow-up taken.

## Does it change behavior/output?

- **Bit-exact?** Yes - full parse-then-write round trip (`dump` mode) `sha256sum` identical
  baseline vs. patched on both `seq_10k_long1000.dcm`
  (`e1b19b45a08a9f5801a995483c875f1ab7c4cc537deab336e03fd6d4c3812b17`) and `liver.dcm`
  (`4f8fb316b6df067bdf2ef7bc2385fd571ad5be67e171aed3ed902a71293d9d5c`).
- **Is this a breaking change?** No - `read_value_strs` is a private method of `parser`'s internal
  `StatefulDecoder`; the empty-buffer edge case (a zero-length value) is preserved identically
  (`[].contains(&b'\\')` is `false`, so it takes the fast path and decodes an empty buffer into a
  single empty-string element - the same result the old `split`-based path produces for an empty
  buffer, since `[].split(...)` yields exactly one empty slice).
- Project test suite: `cargo test --release -p dicom-parser -p dicom-object`: **75 + 9 + 49 + 22 =
  155 passed / 0 failed**, both before and after - identical counts (matches this workspace's
  usual baseline for these two crates, e.g. candidates 014/024/032).

## Why the result looks the way it does

Unlike `encode_texts_element`'s fix (candidate 024, a genuine null result - that fix only removed
one `memcpy` of an already-allocated buffer, work too small to clear this box's noise floor even
at large scale), this fix removes real per-call overhead: `split()`'s iterator state machine plus
`Result<C<String>>::collect()`'s generic fold/size-hint logic, for a target type (`SmallVec`-backed
`C<String>`) whose `FromIterator` impl isn't a trivial `Vec::extend`. For a single short-lived
element, this iterator/collect machinery is a larger fraction of the total per-element cost than
`encode_texts_element`'s single skipped memcpy was - explaining why this decode-side fix shows a
real, measurable win where its encode-side sibling did not, despite looking superficially similar
in shape ("add a single-value fast path"). This is a useful corrective to `IDEAS.md`'s own
hedge: "looks similar to an already-null-result fix" is not a reliable predictor when the
*mechanism* being skipped differs (memcpy avoidance vs. iterator/collect avoidance).

## What's not done / open questions

- Only VR `Strs`'s own `read_value_strs` was fixed; the same iterator/collect pattern likely
  exists in `read_value_da`/`ds`/`dt`/`is`/`tm` (mentioned in the same original `IDEAS.md` entry) -
  not attempted here, scoped to `read_value_strs` only for this candidate. Worth a follow-up sweep
  given this result reverses the original low-confidence assessment for the whole family.
- No peak-RSS/allocation-count follow-up (see "Result" above for why this wasn't expected to
  matter for a fix this local).
