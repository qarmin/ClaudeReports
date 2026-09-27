# PDataWriter (DIMSE P-Data-TF sender): skip the copy-into-buffer step for large single writes

Follow-up to candidate 019, prompted by a direct request to look specifically at CIOD
*transmission* for the same class of problem: is data cloned/copied more than once where a
single copy (or none) would do, even if it doesn't show up in peak RSS because the extra buffer
is bounded and reused, not something that grows with the object?

`ul/src/association/pdata.rs`'s `PDataWriter` (the writer behind `Association::send_pdata`,
used by every DIMSE C-STORE-style transmission of a data set, e.g. `storescu`) implements
`std::io::Write`. Its `write()` accumulates incoming bytes into a reusable `self.buffer` sized to
one PDU (`Vec<u8>`, capacity bounded by `max_pdu_length`), and once full, does one
`self.stream.write_all(&self.buffer)` to dispatch the PDU and reuse the buffer for the next one.
This buffer never grows with the object being sent - exactly the "not visible in RSS because the
chunk gets freed/reused afterward" shape.

But for a **single large `write_all()` call** (e.g. writing a large CIOD's native pixel data,
which typically arrives at the writer as one big contiguous slice from the encoder), every chunk
that fills a PDU is `memcpy`'d into `self.buffer` first, then `memcpy`'d *again* by
`self.stream.write_all(&self.buffer)` (a syscall copy into the kernel socket buffer, unavoidable
for any `Write`-based socket send). The first copy is redundant: when `self.buffer` is empty
(only the header skeleton present, no partial payload waiting from a previous small write), the
next chunk-sized slice could be handed to the stream directly - only the 12-byte header needs
its own tiny buffer.

## What was tried

Added `PDataWriter::dispatch_pdu_borrowed(&mut self, payload: &[u8])`: builds the PDU header in
the existing 12-byte skeleton (unchanged), then does **two** separate `stream.write_all()` calls
- one for the header, one for the payload slice **borrowed directly from the caller**, never
copied into `self.buffer`. `Write::write()` takes this path only when `self.buffer` holds
nothing but the header skeleton (no partial data from a smaller previous write already sitting
there to combine with) - the exact situation a single large `write_all()` call produces for
every PDU after the first. All other cases (small writes, or a write that only partially fills
what's left in the buffer) are untouched, byte-for-byte identical to before.
`setup_pdata_header` was refactored to take `data_len` as an explicit parameter instead of
inferring it from `buffer.len()`, so it can compute the header for a payload that was never
copied into that buffer.

## Correctness verification

- **`cargo test -p dicom-ul --release --all-features`: 83 passed / 0 failed** (33+7+2+4+2+2+10+3+20
  across lib/unit/integration/doctests), including `test_write_large_pdata_and_finish` - which
  already asserts exact PDU byte content and boundaries at a 1024-byte `MINIMUM_PDU_SIZE`, and
  which incidentally exercises `dispatch_pdu_borrowed` directly (its 2500-byte single `write_all`
  call produces two full 1012-byte PDUs that hit the new borrowed path, plus one partial-fill PDU
  that doesn't) - all assertions passed unmodified.
- **`cargo test --workspace` (default features): 537 passed / 0 failed, both before and after.**
- **New harness mode `pdatawriter-throughput`**: a real client+server association over a loopback
  TCP socket (not synthetic - `ClientAssociationOptions`/`ServerAssociationOptions`, matching the
  existing test suite's own pattern). Client sends an N MB payload via `send_pdata`/`write_all`/
  `finish`; server reads it back and reports a checksum. **Payload reconstructed byte-for-byte
  identical (`received_match=true`) at 5 MB and 100 MB**, both baseline and patched.
- **New harness mode `pdatawriter-inprocess`** (gated behind `--features pdatawriter-inprocess`,
  see "Instrumentation" below): writes directly into an in-memory `Vec<u8>` sink via a
  benchmark-only constructor. Output length matched exactly between baseline and patched at every
  size tried (5/100/300 MB) - not a full content checksum (the mode doesn't compute one), but
  combined with the loopback test's full content check and the existing byte-exact unit tests,
  this is sufficient corroborating evidence, not the sole correctness claim.

## What was tried (performance) and Result

Two measurements, because the first one turned out to be inconclusive - reported honestly rather
than discarded, per this workspace's standing instruction to report memory/allocation-shape
findings even when a wall-clock number doesn't cooperate.

**1. Realistic end-to-end (`pdatawriter-throughput`, real loopback TCP, 100 MB, 1 MiB max PDU
length, 9 paired runs):** median 155.7 ms -> 152.2 ms, **only 4 of 9 pairs favored the patched
build - not consistent, within noise.** Real socket I/O (syscall + kernel copy + loopback stack
overhead) dominates total time at this scale enough that removing one ~1 MB userspace `memcpy`
per PDU doesn't produce a measurable, reliable end-to-end difference. This is reported as a real
result, not hidden: **the fix's effect on real network transmission time, on this box, over
loopback, was not measurable.**

**2. Isolated mechanism (`pdatawriter-inprocess`, in-memory `Vec<u8>` sink, no socket, removes
kernel-copy/syscall noise from the comparison):**

| Payload | multi-core p50 | single-core p50 (`taskset -c 0`) |
|---|---|---|
| 100 MB | 37.3 ms -> 35.2 ms | (not run at this size) |
| 300 MB | 110.2 ms -> 107.0 ms | 110.0 ms -> 106.8 ms |

**-5.6% at 100 MB (9/9 paired wins), -3.0% at 300 MB both multi-core and single-core (9/9 each,
2 independent configurations) - real and fully consistent, but modest, not large.** Smaller than
the naive "removing one of two full-buffer copies should roughly halve copy time" prediction -
most of the measured wall time in this synthetic benchmark is the *destination* `Vec<u8>` sink's
own `extend_from_slice` cost (unavoidable, identical in both variants) plus a fresh ~100-300 MB
allocation and first-touch page faults every iteration (also identical in both variants) - the
eliminated `self.buffer` copy is a real but comparatively small fraction of that total.

## Does it change behavior/output?

- **Bit-exact?** Yes - see "Correctness verification" above.
- **Is this a breaking change?** No. `dispatch_pdu_borrowed` is a new private method;
  `setup_pdata_header`'s signature changed but it's a private (non-`pub`) function, invisible
  outside this module. No public API changed.
- Project test suite: **537 passed / 0 failed, both before and after** (`cargo test --workspace`);
  **83 passed / 0 failed, both before and after**, specifically for `dicom-ul --all-features`.

## Why the result looks the way it does

The mechanism is real and provable by reading the code (one fewer full-payload `memcpy` per PDU,
confirmed structurally, not inferred from timing alone) - but its *visibility* depends entirely on
what it's being compared against. Against a real socket (syscall + kernel copy dominate, itself
already close to memory-bandwidth-bound for loopback), the removed userspace copy is a small
fraction of total cost and gets lost in system noise. Against a bare in-memory sink (isolating just
the userspace copy work), the effect is real and consistent but still modest, because the
*destination* copy (writing into the sink) was never eliminated - only the *redundant extra* copy
was, and that's one of two, not the only one.

## What's not done / open questions

- **The async path (`AsyncPDataWriter::poll_write`) was deliberately left unfixed.** Its
  `Poll::Pending`-resumable state machine would need a new state tracking "mid-header, then
  mid-payload" separately (instead of the current single contiguous `self.buffer` position) to
  apply the same optimization safely - a real, nontrivial change, out of scope for this pass. Its
  `setup_pdata_header` call site was updated only to match the new required parameter, with
  *zero* behavior change (still copies into `self.buffer` exactly as before). Flagged in
  `IDEAS.md` as a follow-up.
- **Real-world significance is genuinely uncertain**, and this RESULT.md says so rather than
  picking a side: over an actual network (rather than loopback), TCP throughput is very likely to
  be the bottleneck long before this copy matters, making the fix's real-world value plausibly
  close to zero for typical WAN/LAN C-STORE traffic - but for very fast local/loopback transport
  (e.g. inter-process DICOM tooling on the same machine, or a future zero-copy transport), the
  isolated measurement suggests a small, consistent, real saving. Both statements are true
  simultaneously; this document doesn't resolve which one matters to a given deployment.
- Not measured: allocation *count* (only copy *volume*/timing were examined). The `self.buffer`
  itself is never reallocated after the first `Vec::with_capacity` (`extend`/`truncate` reuse it),
  so allocation-count is unaffected by this fix either way - the win is purely in bytes-touched
  (memcpy volume), not allocation counts. Worth stating explicitly since the user's request singled
  out allocation count as a metric to watch, not just RSS/timing.

## Instrumentation added for this candidate (benchmark-only, not part of the fix)

`PDataWriter::new_for_bench` (a thin `pub` wrapper around the normally `pub(crate)` constructor)
and the `bench-internals` feature gating it in `ul/Cargo.toml` were added **temporarily** during
this session to build the `pdatawriter-inprocess` harness mode, then **fully reverted** - they are
**not** part of `patch.diff`. `patch.diff` contains only the real fix
(`dispatch_pdu_borrowed`/`setup_pdata_header` refactor). Reproducing the `pdatawriter-inprocess`
measurement requires re-adding that same small shim first (see `justfile`, which does this itself
in a clearly separate step from applying `patch.diff`).
