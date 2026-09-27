# ul-pdatareader-unbounded-buffer-growth-oom

`ul/src/association/pdata.rs`'s `PDataReader::read` (and its async `poll_read` counterpart)
receive P-Data-TF PDUs in a loop: try to parse a complete PDU out of `self.read_buffer`; if
parsing returns `None` (not enough bytes yet), fetch more bytes from the peer and append them,
then try again:

```rust
let msg = loop {
    let mut buf = Cursor::new(&self.read_buffer[..]);
    match read_pdu(&mut buf, self.max_pdu_length, false).map_err(std::io::Error::other)? {
        Some(pdu) => { self.read_buffer.advance(buf.position() as usize); break pdu; }
        None => { buf.set_position(0) }
    }
    let recv = reader.fill_buf()?.to_vec();
    reader.consume(recv.len());
    self.read_buffer.extend_from_slice(&recv);
    if recv.is_empty() {
        return Err(std::io::Error::other("Connection closed by peer"));
    }
};
```

There is no upper bound anywhere in this loop on how large `self.read_buffer` can grow. A P-Data-
TF PDU's declared `pdu_length` is a 4-byte field read directly off the wire - fully
peer-controlled. A peer that declares a `pdu_length` near `u32::MAX` (~4 GiB) and then either
keeps trickling data indefinitely, or genuinely sends that much data over time, causes this loop
to keep buffering forever (or until the process runs out of memory) with `read_pdu` returning
`None` on every attempt, since the declared length is never satisfied.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Git archaeology (same technique as candidates 056-072): checked ~35 unmerged `origin/fix/*` branches against `df2b9e42` via `git merge-base --is-ancestor` | Found `origin/fix/pdata-reader-strict-pdu-length` (commit `e71bc0f3`) adding a buffer-size cap to both the sync and async read loops | Confirmed unmerged |
| Cross-checked against already-completed candidate 034 (`ul-pdata-strict-mode-not-honored`, same file) | Read 034's `RESULT.md` | Confirmed genuinely distinct: 034 is about `strict` mode never being consulted for P-Data-TF PDUs at all (accepting PDV payloads larger than the negotiated `max_pdu_length` with no limit); this candidate is about *unbounded memory growth* while *waiting* for a PDU that never completes - a different failure mode in the same file, worth its own candidate |
| Temporary in-tree `#[test]` in `ul/src/association/pdata.rs` (reverted before write-up): a custom `Read` impl (`DribblingStream`) that delivers a P-Data-TF header declaring `pdu_length = 10,000,000`, followed by 2,000,000 real bytes, then closes the connection; construct a real `PDataReader` and call `.read()` on it | `cargo test -p dicom-ul pdata_reader_unbounded_buffer_growth_repro` | The reader buffers the peer's *entire* 2,000,000-byte stream (no error, no cap) before finally failing with the generic "Connection closed by peer" once the fake peer disconnects - confirming there is no growth limit at all, only ever bounded by whatever the peer is willing to send |
| **First fix attempt** (a direct, verbatim port of the unmerged branch's own cap: `max_pdu_length + 2*PDU_PDV_HEADER_SIZE`) | `cargo test -p dicom-ul --all-features --lib` | **Broke this crate's own pre-existing `test_async_read_large_pdata_and_finish` test** with `"PDU too large: 2168 bytes buffered without a complete PDU (max 1042)"` - that test legitimately sends 3000-byte PDV payloads (well over the negotiated `max_pdu_length = 1018`), which this module's reader deliberately accepts (candidate 034 already established that `strict` is hardcoded `false` for P-Data-TF PDUs specifically because larger-than-negotiated frames are an accepted, exercised part of this reader's design) |
| **Corrected fix**: cap based on `max(max_pdu_length, LARGE_PDU_SIZE) * 2 + overhead` instead of `max_pdu_length` alone, decoupling the cap from the (deliberately unenforced) negotiated value while still bounding growth well short of actually buffering megabytes-to-gigabytes for a bogus declared length | `cargo test -p dicom-ul --all-features --lib` | **34 passed / 0 failed** - the same large-legitimate-frame test now passes, and the OOM repro is still rejected (at ~528KB instead of buffering the full 2MB) |
| **[REBUILT per CONFIRMATION.md section 11]** Real, two-real-participant, real-loopback-TCP reproduction (`repro/`): a real `PDataReader` wraps a real `TcpStream`; a genuinely separate thread ("attacker") connects over a real socket, writes the same malicious header + 2,000,000 real trickled bytes, then disconnects | `cargo run` | Reproduces the exact same unbounded-growth failure over a real socket boundary; see `backtrace.txt` |
| (superseded, kept for context) Earlier session's temporary in-tree `#[test]` and standalone `repro/` using a custom in-process `Read` impl, never a real socket | `cargo test`/`cargo run` | Demonstrated the same conclusion but never exercised a real socket - retired in favor of the real two-participant repro above once `CONFIRMATION.md` section 11 (added mid-session, after this candidate's original write-up) required it |

## Status

**Confirmed** - deterministic, reproduces every run (a fixed declared `pdu_length` and a fixed
amount of real follow-up data before disconnect, no timing dependency). Verified via a real
end-to-end (real `PDataReader` + real socket + genuinely separate attacker thread)
reproduction.

**Fully reproducible: Yes** - `repro/` wraps a real `TcpStream` in the real `PDataReader` and
feeds it from a genuinely separate thread over a real loopback TCP socket (see
`CONFIRMATION.md` section 11).

## Class

`crash`/`security` (unbounded memory growth / OOM DoS - a malicious or misbehaving peer that
declares an oversized `pdu_length` and either trickles data slowly or genuinely floods the
connection can force this process to buffer an unbounded amount of memory before ever detecting
anything is wrong).

## Root cause

The receive loop's only termination conditions were "a complete PDU was parsed" or "the peer
closed the connection" - nothing bounded how much could accumulate in `self.read_buffer` while
waiting for either of those, so a peer that never satisfies either condition (keeps the
connection open, keeps trickling bytes, but never enough to complete the absurdly large declared
`pdu_length`) can drive unbounded growth indefinitely.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Add a buffer-size cap checked every time parsing returns `None`: if the buffer has already grown
past a generous-but-bounded threshold and still can't produce a complete PDU, bail out with an
explicit "PDU too large" error instead of continuing to wait. The threshold is intentionally
**not** `max_pdu_length` itself (see the first fix attempt above) - it's based on
`max(max_pdu_length, LARGE_PDU_SIZE)` doubled, since this reader deliberately accepts individual
PDU-TF frames larger than the negotiated `max_pdu_length` (candidate 034), and the cap must not
regress that already-relied-upon behavior:

```rust
let max_buffer_size =
    (self.max_pdu_length.max(LARGE_PDU_SIZE) as usize) * 2 + (PDU_PDV_HEADER_SIZE * 2);
// ... inside the None branch:
if self.read_buffer.len() >= max_buffer_size {
    return Err(std::io::Error::other(format!(
        "PDU too large: {} bytes buffered without a complete PDU (max {})",
        self.read_buffer.len(), max_buffer_size,
    )));
}
```

(and the analogous change in the async `poll_read`). See `patch.diff` for the exact diff (scoped
to `ul/src/association/pdata.rs` only).

- `cargo build -p dicom-ul --all-features`: builds cleanly with the fix applied.
- `cargo test -p dicom-ul --all-features --lib` (against the patched pristine clone): **34
  passed / 0 failed**, confirming zero regressions - including the large-legitimate-frame tests
  that the first (naive, verbatim-ported) fix attempt broke.
- `cargo test --workspace --lib` (against the live tree, fix applied then reverted): all tests
  passed, no failures introduced.
- `repro/` (`just test`): buffers the peer's entire stream before erroring, before the patch;
  rejects the oversized PDU early after.
