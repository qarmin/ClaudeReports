# Peak-RSS follow-up for candidate 029 (AsyncPDataWriter direct dispatch)

Candidate 029's `AsyncPDataWriter::poll_write` direct-dispatch fix avoids copying a large payload
into `self.buffer` before writing it - exactly the class of change `projects/dicom-rs.md` flags
as worth a peak-RSS check (candidates 019/021/026 all showed this mechanism can move peak RSS
independently of wall-clock time, in either direction). Candidate 029 itself only measured
wall-clock time; `IDEAS.md` flagged the peak-RSS check as a separate, still-open item.

## What was tried

Added a new harness mode, `async-pdatawriter-peak-mem <payload-mb>` (harness-only, not part of
`patch.diff`): a single `AsyncPDataWriter::write_all`+`finish` call into an in-memory sink, then
reports this process's peak RSS (`VmHWM` from `/proc/self/status`) - same technique as
`write-dataset-peak-mem` (candidate 019). Re-applied candidate 029's `patch.diff` +
`bench-shim.diff` unchanged; two binaries (baseline = shim only, patched = shim + real fix),
3 runs each at 100MB and 300MB (the same two payload sizes candidate 029's own wall-clock
measurement used).

| Payload | baseline VmHWM (3 runs) | patched VmHWM (3 runs) |
|---|---|---|
| 100MB | 209,820 / 210,176 / 209,832 kB | 209,940 / 210,096 / 209,968 kB |
| 300MB | 619,736 / 619,392 / 619,564 kB | 619,676 / 619,852 / 619,696 kB |

## Result

**No measurable peak-RSS effect at either payload size** - the baseline/patched values overlap
within ~0.1-0.2% at both 100MB and 300MB, far smaller than run-to-run noise for either build.
This is a genuine, confirmed null for this dimension, not an unmeasured gap - unlike candidate
029's own wall-clock result (a real, consistent win at 300MB), the memory dimension shows nothing
to report. See "Why" below for the mechanism that explains this.

## Does it change behavior/output?

- **Bit-exact?** Yes - same differential verification as candidate 029 (byte-for-byte
  reconstructed PDU stream via `test_partial_write`), unchanged by this candidate.
- **Is this a breaking change?** No - same internal `ul` crate change as candidate 029.
- Project test suite: `cargo test --release -p dicom-ul --features async,bench-internals`:
  **34 passed / 0 failed** (patch applied), matching candidate 029's own recorded count exactly.
  Reverted-tree (baseline) count is identical.

## Why the result looks the way it does

`AsyncPDataWriter::buffer` (`ul/src/association/pdata.rs:428`) is allocated once at construction
as `Vec::with_capacity((max_pdu_length.min(LARGE_PDU_SIZE) + PDU_HEADER_SIZE) as usize)` - a
**fixed, PDU-size-capped buffer**, not a buffer sized to the whole payload. The baseline's "copy
into `self.buffer`" cost that candidate 029 avoids is therefore copying the payload in repeated
PDU-sized chunks (1MB in this harness's own `LARGE_PDU_LEN`) through one small, reused buffer -
never a single large allocation that scales with total payload size. Candidate 029's fix removes
the *repeated small memcpy* cost (real, measurable in wall-clock time, per-write CPU work) without
removing any *allocation this process was making anyway* (the buffer's fixed size means peak RSS
was never proportional to payload size in the baseline either) - explaining why time moved and
memory didn't. This matches `projects/dicom-rs.md`'s own documented distinction between
allocation-*count* fixes (little/no RSS effect) and allocation-*volume* fixes (large RSS effect,
e.g. candidates 019/026) - this fix is closer to the former in its RSS-relevant behavior, even
though its mechanism (avoiding a copy) looks superficially similar to 019/026's.

## What's not done / open questions

- Only measured via the isolated in-memory-sink harness mode, same scope limitation as candidate
  029's own wall-clock measurement (a real loopback-TCP peak-RSS check was not attempted, and
  remains open per `IDEAS.md`).
- This closes the `IDEAS.md` backlog item asking for this peak-RSS measurement - the confirmed
  null is the answer, no further action needed for this specific question.
