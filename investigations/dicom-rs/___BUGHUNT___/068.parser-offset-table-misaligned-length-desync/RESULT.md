# parser-offset-table-misaligned-length-desync

`parser/src/stateful/decode.rs`'s `read_u32_to_vec` (used to read the Basic Offset Table item
of an encapsulated PixelData sequence into a `Vec<u32>`) computed the number of `u32` elements
via a bit shift:

```rust
fn read_u32_to_vec(&mut self, length: u32, vec: &mut Vec<u32>) -> Result<()> {
    self.read_u32((length >> 2) as usize, vec)
}
```

`length` here is the offset table item's own declared byte length - taken directly from the
file/stream (`parser/src/dataset/read.rs:457`, itself from the item header's length field, fully
file-controlled). `length >> 2` silently floors, so if `length` isn't a multiple of 4 (e.g. 5),
only `length / 4` whole `u32`s are actually read from the underlying stream - the remaining
`length % 4` bytes are never consumed. Since `StatefulDecoder`'s internal `position` (and the
actual stream cursor) only advance by what was really read, this desynchronizes the parser from
the file's own byte layout by `length % 4` bytes for the rest of the parse - misinterpreting
whatever comes next (typically the following item's header), an unrecoverable parsing
corruption from a single malformed length field.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Git archaeology (same technique as candidates 056-067): checked ~35 unmerged `origin/fix/*` branches against `df2b9e42` via `git merge-base --is-ancestor` | Found `origin/fix/bot-length-not-multiple-of-4` (commit `d2d2f931`) fixing this exact computation | Confirmed unmerged |
| Traced the one real call site (`parser/src/dataset/read.rs:457`, the Basic Offset Table item of a `PixelSequence`) to confirm `length` is attacker/file-controlled, not an internally-computed value | Static code reading | Confirmed: `len` comes straight from `SequenceItemHeader`'s own length field on the wire |
| Read `read_u32`'s implementation to confirm the decoder's internal `position` tracking (and the real I/O cursor, since the read goes straight to `self.from`) only advances by the number of bytes *actually* read, not the declared `length` | Static code reading (`self.position += n as u64 * 4`) | Confirmed: a misaligned `length` leaves a genuine, unrecovered gap in the stream, not just an internal accounting mismatch |
| Temporary in-tree `#[test]` in `parser/src/stateful/decode.rs` (kept as a permanent regression test in `patch.diff`): construct a 5-byte raw buffer (`[0x01, 0x00, 0x00, 0x00, 0xFF]`), call `read_u32_to_vec(5, ...)` directly on a `StatefulDecoder`, check the resulting `position()` | `cargo test -p dicom-parser read_u32_to_vec_misaligned_repro` | Reads only 4 of the 5 declared bytes; `position()` reports `4`, not `5` - the 5th byte (`0xFF`) is left unconsumed in the stream |
| Standalone `repro/` (path-dependency on a pristine clone of `df2b9e42`, via `just test`), using the same minimal reproduction directly against the public `dicom_parser::{StatefulDecode, StatefulDecoder}` API | `cargo run` | Reproduces the same desynchronization deterministically; see `backtrace.txt` |

## Status

**Confirmed** - deterministic, reproduces every run, no fixed seed needed (a fixed 5-byte input
demonstrating the exact desync mechanism).

## Class

`logic` (silent stream desynchronization/parsing corruption - not a crash by itself, but
corrupts all subsequent parsing of the data set - triggered by a single malformed length field
in file-controlled content, no other adversarial conditions needed).

## Root cause

`length >> 2` (equivalent to `length / 4` rounding toward zero) silently discards the remainder
when `length` isn't a multiple of 4, and nothing downstream detects or compensates for the
unconsumed trailing bytes - the position/stream-cursor bookkeeping only reflects what was
actually read, not what was declared, so the mismatch propagates invisibly into every subsequent
read.

## Introduced by

N/A - not attempted this session (deferred; dicom-rs's history is large enough that a proper git log -S/blame/bisect pass was left for a dedicated later session rather than done superficially here).

## The fix

Reject a misaligned length outright with a descriptive I/O error, matching the "fail loudly on
malformed structural framing" pattern already used elsewhere in this same file, rather than
silently reading a truncated amount and leaving the stream corrupted:

```rust
fn read_u32_to_vec(&mut self, length: u32, vec: &mut Vec<u32>) -> Result<()> {
    if length % 4 != 0 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("offset table length {length} is not a multiple of 4"),
        ))
        .context(ReadValueDataSnafu {
            position: self.position,
        });
    }
    self.read_u32((length / 4) as usize, vec)
}
```

See `patch.diff` for the exact diff (scoped to `parser/src/stateful/decode.rs`; includes the
regression test kept in the `tests` module).

- `cargo build -p dicom-parser`: builds cleanly with the fix applied.
- `cargo test -p dicom-parser --lib` (against the patched pristine clone): **50 passed / 0
  failed** (49 pre-existing + this candidate's own new regression test), confirming zero
  regressions.
- `cargo test --workspace --lib` (against the live tree, fix applied then reverted): all tests
  passed, no failures introduced.
- `repro/` (`just test`): panics on its own desync-detection assertion before the patch; the
  misaligned length is cleanly rejected with an error after.
