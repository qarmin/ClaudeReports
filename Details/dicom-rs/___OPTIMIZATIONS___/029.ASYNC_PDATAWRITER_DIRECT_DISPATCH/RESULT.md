# AsyncPDataWriter: direct dispatch for large single writes (async counterpart to candidate 020)

`IDEAS.md` carried an open lead: candidate 020 fixed `PDataWriter` (the *synchronous* P-Data
sender) to skip copying a large single write's payload into its internal buffer before sending -
writing the PDU header and the payload as two separate writes directly to the stream instead of
one combined buffer. The async counterpart, `AsyncPDataWriter::poll_write`, was deliberately left
unfixed in that session: its `Poll::Pending`-resumable state machine (`WriteState::Ready`/
`Writing(pos, consumed)`) would need a new state tracking "mid-header, then mid-payload"
separately instead of one contiguous `self.buffer` position - real design work, not a copy-paste
of the sync fix. This candidate does that design work.

## What was tried

Added a new `WriteState::WritingDirect { header_pos, payload_pos, payload_len }` variant and a
`poll_write_direct` helper function that writes `self.buffer`'s header bytes first (resuming from
`header_pos` if continuing after a `Poll::Pending`), then writes `payload_len` bytes taken
**directly from the caller's own `buf`** (never copied into `self.buffer`) starting at
`payload_pos`. This relies on `AsyncWrite`'s own contract: if `poll_write` returns
`Poll::Pending`, the caller must pass the *same* `buf` again on the next call - so tracking a byte
offset into the caller's buffer across suspension points is sound. `poll_write`'s `Ready` branch
now takes this path whenever `self.buffer.len() == PDU_PDV_HEADER_SIZE` (a fresh PDU, no partial
payload already accumulated) and the incoming write alone overflows one PDU's capacity - the exact
same condition candidate 020 uses on the sync side. A new `WriteState::WritingDirect` match arm
resumes an in-flight direct dispatch the same way `WriteState::Writing` already resumes a
buffered one. Uses a small local `setup_pdata_header_direct` (data length given explicitly,
instead of inferred from `buffer.len()`) kept private to the async module rather than sharing
candidate 020's own modified `setup_pdata_header` signature - keeps this candidate independently
applicable without assuming 020's patch is present, per this workspace's "one fix per candidate"
discipline.

**Correctness validation, given this touches a `Poll::Pending`-resumable state machine in
network-facing code** (a materially higher-stakes area than this workspace's usual pure-CPU
candidates):
- The crate already had a purpose-built `ControlledStream` + `test_partial_write` test
  (parameterized via `rstest`, `#[case]`) that simulates `Poll::Pending` and short writes at
  controlled byte offsets, then verifies the reassembled PDU stream matches the original data
  byte-for-byte via `read_pdu`. The existing 4 cases already exercised `WritingDirect`'s
  payload-resume path (via a 1MB `write_all` from a fresh writer). **Added a 5th case**
  (`vec![None, Some(5), Some(7), Some(1000), None, None, Some(1000)]`) that specifically forces
  `Poll::Pending` *before any header byte is written*, then splits the 12-byte header itself
  across two separate `poll_write` calls (5 + 7 bytes) - none of the 4 existing cases happened to
  split the header, so this closes a real gap in coverage for the new `header_pos` resume logic.
  All 5 cases pass.
- `cargo test --release -p dicom-ul --features async`: **34 passed / 0 failed** (up from the
  baseline's 33 - the new test case), including `test_async_pdata_writer` (a real client+server
  association over a loopback TCP socket) and `test_async_write_large_pdata_and_finish`/
  `test_async_read_large_pdata_and_finish`.
- `cargo build --release --workspace`: clean, no errors, no new warnings.

Bench: added `async-pdatawriter-inprocess <payload-mb> <iters>` to `___OPTIMIZATIONS___/harness/
bench_pipeline` (async counterpart to candidate 020's own `pdatawriter-inprocess` mode) - writes
directly into an in-memory `Vec<u8>` sink via a benchmark-only `AsyncPDataWriter::new_for_bench`
shim (`bench-shim.diff`, same pattern as candidate 020's own `PDataWriter::new_for_bench` - **not
part of the real fix**, applied only for this measurement and reverted immediately after, per
this workspace's own convention for reaching a normally-`pub(crate)` constructor from an external
harness). Isolates the same userspace-copy mechanism from real socket/kernel I/O noise the sync
candidate's own follow-up found necessary (see candidate 020's RESULT.md: a real loopback TCP
measurement was too noisy to show a consistent delta for one avoided memcpy at realistic payload
sizes).

| Payload size | baseline p50 (2 rounds) | patched p50 (2 rounds) |
|---|---|---|
| 100MB | 35.16/35.15 ms | 35.40/35.37 ms |
| 300MB | 111.17/110.94 ms | 108.21/108.38 ms |

## Result

**Scale-dependent, same shape as candidate 020's own sync-side finding.** At 100MB: **no
consistent change** (+0.7%/+0.6% across two rounds, 3/9 and 2/9 - neither clears this workspace's
consistency bar). At 300MB: **-2.7%/-2.3% across two independent rounds, 9/9 paired wins both
times, fully consistent.** `uptime` load average was 0.8-1.6 throughout, no unusual system load.
The mechanism (skip one memcpy of the whole payload) only pays for itself once the copy avoided is
large enough to matter relative to everything else `poll_write`/`write_all` does per call - at
100MB the win is too small to clear this box's noise floor; at 300MB it's real and consistent.

## Does it change behavior/output?

- **Bit-exact?** Yes - `test_partial_write`'s own byte-for-byte comparison (`assert_eq!(res,
  test_buffer)`, reconstructed via re-parsing every emitted PDU with `read_pdu`) passed for all 5
  cases, including the new header-split case. The harness's own `out_len` (total bytes written to
  the in-memory sink, including all PDU headers) was also cross-checked equal between baseline and
  patched builds for both payload sizes tested.
- **Is this a breaking change?** No public API/type changes - `WriteState::WritingDirect` and
  `poll_write_direct`/`setup_pdata_header_direct` are all private to the `non_blocking` module.
- Project test suite: **34 passed / 0 failed** (dicom-ul, `async` feature), up from 33 at baseline
  due to the added test case; all other workspace crates unaffected (this candidate touches only
  `ul/src/association/pdata.rs`), full workspace build clean.

## Why the result looks the way it does

Same reasoning as candidate 020: the win is proportional to the size of the memcpy avoided, and at
100MB the fixed overhead of everything else in the write path (the tokio runtime, the in-memory
`Vec<u8>` sink's own growth/reallocation as it accumulates hundreds of PDU headers, `poll_write`'s
own per-call bookkeeping) is still a large enough fraction of total time that removing one memcpy
doesn't clear the noise floor; at 300MB the same fixed overhead is a smaller relative share, and
the avoided copy (nearly the whole 300MB payload, since it's written directly rather than being
copied into `self.buffer` first) becomes the dominant, measurable factor.

## What's not done / open questions

- **Not measured over a real socket** (deliberately, matching candidate 020's own precedent) -
  that candidate's real loopback-TCP measurement found socket I/O noise dominates one avoided
  memcpy at realistic payload sizes; the in-memory-sink isolation used here is the same
  methodology, with the same caveat that a real-network measurement might not show a consistent
  effect even where this isolated one does.
- The exact crossover point between "no effect" (100MB) and "real effect" (300MB) was not
  narrowed down further (e.g. 150MB, 200MB) - not necessary to establish the qualitative finding,
  but would sharpen the practical guidance for when this fix actually helps.
- Peak RSS not measured - the mechanism avoids one large temporary copy per big write, which is
  exactly the class of change `projects/dicom-rs.md` flags for a peak-RSS check; not measured this
  session given the wall-clock win was already the primary question this candidate answered.
- `bench-shim.diff`'s `AsyncPDataWriter::new_for_bench` is benchmark-only instrumentation, not
  part of `patch.diff` - do not confuse the two when re-reading this candidate's files.
