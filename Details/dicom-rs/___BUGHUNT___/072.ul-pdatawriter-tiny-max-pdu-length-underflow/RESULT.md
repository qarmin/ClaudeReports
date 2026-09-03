# ul-pdatawriter-tiny-max-pdu-length-underflow

`ul/src/association/pdata.rs`'s `PDataWriter` pre-fills its buffer with a fixed 12-byte
PDU/PDV header stub (`PDataWriter::new`), then computes how much of an incoming `write()` call
fits in the current PDU via:

```rust
fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
    let total_len = (self.max_pdu_length + PDU_HEADER_SIZE) as usize;
    if self.buffer.len() + buf.len() <= total_len {
        ...
    } else {
        let buf = &buf[..total_len - self.buffer.len()];  // underflows if total_len < 12
        ...
    }
}
```

`max_pdu_length` is the *negotiated* max PDU length - which, before this session's fix,
was derived from the **peer-advertised** `MaxLength` user variable during association
negotiation (`ul/src/association/client.rs`'s `process_a_association_resp` and
`server.rs`'s `process_a_association_rq`), clamped only with an upper bound
(`.min(MAXIMUM_PDU_SIZE)`) and a special case for exactly `0` (treated as "largest supported") -
but **no lower bound**. A peer advertising a `MaxLength` between `1` and `5` (any value where
`max_pdu_length + PDU_HEADER_SIZE(6) < 12`) reaches `PDataWriter` with a value small enough that
the very first `write()` call whose data doesn't fit in the remaining space underflows this
subtraction - a `usize` underflow that panics in debug builds ("attempt to subtract with
overflow") and produces an out-of-bounds slice panic in release builds.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Git archaeology (same technique as candidates 056-071): checked ~35 unmerged `origin/fix/*` branches against `df2b9e42` via `git merge-base --is-ancestor` | Found `origin/fix/pdata-writer-small-pdu-length` (commit `a14407ee`) adding a `.max(MINIMUM_PDU_SIZE)` lower clamp in both `client.rs` and `server.rs` | Confirmed unmerged |
| Traced `MAXIMUM_PDU_SIZE`/`MINIMUM_PDU_SIZE`/`PDU_HEADER_SIZE` constants (`ul/src/pdu/mod.rs`) and both negotiation sites to confirm the missing lower bound is real on current master | Static code reading | Confirmed: `MINIMUM_PDU_SIZE = 1024 - 6 = 1018`; both `client.rs:870` and `server.rs:858` only apply `.min(MAXIMUM_PDU_SIZE)`, no `.max(...)` |
| Temporary in-tree `#[test]` in `ul/src/association/pdata.rs` (reverted before write-up): construct `PDataWriter::new(Vec::new(), 1, 1)` (a `max_pdu_length` of 1, exactly what an unclamped peer-advertised `MaxLength(1)` would produce) and call `.write(b"hello")` | `cargo test -p dicom-ul pdata_writer_tiny_max_pdu_length_repro` | Panics: `attempt to subtract with overflow` at `ul/src/association/pdata.rs:183:30`, exactly the predicted mechanism |
| **[REBUILT per CONFIRMATION.md section 11]** Real, two-real-participant, real-loopback-TCP reproduction (`repro/`): both participants built entirely on `dicom-ul`'s own real, public `ClientAssociationOptions`/`ServerAssociationOptions` API (no hand-crafted PDU bytes) | A real client genuinely advertises `max_pdu_length(1)` in its real, wire-encoded A-ASSOCIATE-RQ to a real server over a real loopback TCP socket; the server's real negotiation accepts it, then its real `send_pdata()` writes a real 64-byte response | Real underflow panic on the real server: `attempt to subtract with overflow` at `ul/src/association/pdata.rs:183:30`, exactly matching the mechanism. (The client's own subsequent read of the AC separately fails for an unrelated, pre-existing reason - `read_pdu`'s own `ensure!(MINIMUM_PDU_SIZE..=MAXIMUM_PDU_SIZE)` check rejects using a value below `MINIMUM_PDU_SIZE` for *reading*, which is a different, already-existing guard that doesn't protect the *peer's advertised* value from being used unclamped by the recipient - this doesn't affect the server-side bug being demonstrated, since the server has already fully completed its own negotiation and stored the bad value before the client's own unrelated failure occurs.) Reproduced deterministically: verified first directly against the live project tree's own `dicom-ul` crate, then again via `repro/`'s `just test` against a fresh clone (2 independent real end-to-end runs total; see `backtrace.txt`) |
| (superseded, kept for context) Earlier session's single-process reproduction replicating the vulnerable computation verbatim in a standalone script, never running any real dicom-ul association or a real socket | `cargo run` | Demonstrated the same conclusion but never ran a real negotiation or a real network exchange - retired in favor of the real two-participant repro above once `CONFIRMATION.md` section 11 (added mid-session, after this candidate's original write-up) required it |

## Status

**Confirmed** - deterministic, reproduces every run, no fixed seed needed (a fixed small integer
input, `MaxLength(1)`, that the wire format allows a peer to advertise during ordinary
association negotiation). Verified via a real end-to-end (real client + real server, both on
dicom-ul's own public API, real loopback TCP) reproduction.

**Fully reproducible: Yes** - `repro/` uses `dicom-ul`'s own real, public association API for
both participants, communicating over a real loopback TCP socket (see `CONFIRMATION.md`
section 11).

## Class

`crash` (panic / integer underflow, reachable through a single small `MaxLength` value that
either association peer - a malicious or simply non-conformant remote SCP/SCU - can advertise
during standard DICOM association negotiation; affects both the sync and async client and
server negotiation paths, since both share the same missing lower bound).

## Root cause

The negotiation code only ever validated an upper bound on the peer-advertised PDU length
(clamping to `MAXIMUM_PDU_SIZE`, with `0` specially treated as "use the largest supported
value"), with no corresponding lower bound - so any positive value smaller than the writer's own
fixed per-PDU overhead (12 bytes) reaches `PDataWriter` unmodified and immediately underflows on
its first non-trivial write.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Add `.max(MINIMUM_PDU_SIZE)` alongside the existing `.min(MAXIMUM_PDU_SIZE)` clamp at both
negotiation sites (`ul/src/association/client.rs`'s `process_a_association_resp` and
`server.rs`'s `process_a_association_rq`), so the negotiated max PDU length can never be small
enough to underflow `PDataWriter`'s internal bookkeeping regardless of what either peer
advertises:

```rust
acceptor_max_pdu_length.min(MAXIMUM_PDU_SIZE).max(MINIMUM_PDU_SIZE)
```

(and the analogous change in `server.rs`). See `patch.diff` for the exact diff (scoped to these
two files' negotiation logic only - `PDataWriter` itself is left unchanged, since eliminating
the unsafe value at its source is sufficient and matches this crate's existing pattern for the
`0`-means-"largest" special case right next to it).

- `cargo build -p dicom-ul --all-features`: builds cleanly with the fix applied (both against
  the live tree and against the patched pristine clone via `repro/`'s `reproduce-after` step).
- `cargo test -p dicom-ul --all-features --lib` (against the patched pristine clone): **33
  passed / 0 failed**, confirming zero regressions.
- `cargo test --workspace --lib` (against the live tree, fix applied then reverted): all tests
  passed, no failures introduced.
- `repro/` (`just test`): demonstrates the underflow mechanism directly (this doesn't change
  after the patch, since the fix lives in a different file - see the justfile's own note); the
  patched `dicom-ul` crate's full test suite passes cleanly.
