# `BitPumpJPEG::peek_bits` indexes one byte past the end of the buffer on a bare trailing 0xFF

`rawler::pumps::BitPumpJPEG` (the bitstream reader used by the LJPEG decompressor, shared by
CR2/CRW's lossless-JPEG raw data path) has a slow-refill byte loop that, on finding a `0xFF` byte,
unconditionally indexes the *next* byte to check whether it's the JPEG byte-stuffing `0x00`
marker - with no check that a next byte exists. GitHub issue #587 reports this exact panic
(`index out of bounds: the len is 4809 but the index is 4809` - index equal to length, a
one-past-the-end read) from a real user's CR2 file, deep inside
`HuffTable::huff_decode -> decode_ljpeg -> LjpegDecompressor::decode -> Cr2Decoder::raw_image`.
This candidate was picked from `___BUGHUNT___/IDEAS.md`'s backlog as a "shared bit-pump
infrastructure, broad impact" lead, in the spirit of candidate 004.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read the reported crash (`issues_raw/issues/587.json`) and pinned the exact line at the reporter's own commit (`git show 9643ec9:rawler/src/pumps.rs`) | - | Line 226 at that commit is `} else if self.buffer[self.pos + 1] == 0x00 {` inside `BitPumpJPEG::peek_bits`'s slow-refill loop - identical code still present at current HEAD, just shifted to line 288 by unrelated later additions earlier in the file |
| Manual boundary-value construction: a 3-byte `BitPumpJPEG` buffer `[0x00, 0x00, 0xFF]`, calling `peek_bits(24)` directly | debug build, 1 run (in a scratch `#[cfg(test)]` module, later removed) | Panicked immediately: `index out of bounds: the len is 3 but the index is 3`, exact same off-by-one shape as the reported issue. Confirmed the mechanism at the lowest level before building the real end-to-end repro |
| Real end-to-end repro via `rawler::decompressors::ljpeg::LjpegDecompressor::new`/`.decode` (the exact public API `Cr2Decoder::raw_image` calls) fed a minimal spec-shaped LJPEG bitstream (SOI/SOF3/DHT/SOS) with a 3-byte truncated entropy segment ending in a bare `0xFF` | debug build, 3 runs; release build (via candidate's own `just test` against a fresh clone), 1 run | Deterministic panic at `rawler/src/pumps.rs:288:25`, `index out of bounds: the len is 3 but the index is 3`, identical every run |
| `cargo test --lib` (rawler's own suite) before and after the fix | debug build, 66 tests (existing suite) + 1 new regression test added during investigation | 66 passed/0 failed before (existing suite alone); with the new regression test added, 0 passed/1 failed (panic) unpatched -> 67 passed/0 failed patched |

Two escalating approaches were enough here (manual boundary-value construction, then the real
end-to-end entry point) - both agreed on the exact same root cause and panic site, so a third tool
(fuzzing/miri) wasn't needed; this is a plain out-of-bounds index on a `[u8]` slice, not raw-pointer
or FFI-adjacent UB that `miri`/ASAN would add anything over what the language's own bounds check
already reports.

## Status

**Confirmed.** Reproduces deterministically, 3/3 debug runs + 1/1 release run via the standalone
`repro/`, with no randomness/layout-dependence involved (a pure length-vs-index comparison bug).

**Fully reproducible**: **Yes**. `repro/` calls `LjpegDecompressor::new(src)` then
`.decode(...)` - the exact public API `Cr2Decoder::raw_image` calls with real CR2 file bytes
(`cr2.rs`: `LjpegDecompressor::new(src)?` then `decompressor.decode(...)`), and the crash's own
stack trace names this same function (`LjpegDecompressor::decode` at `mod.rs:307`) one frame above
the panic site. `src` is a minimal but spec-shaped LJPEG bitstream - the same format a real
(corrupted/truncated) CR2 file's raw strip data would contain, which `Cr2Decoder::raw_image` reads
via `file.subview_until_eof(offset)` (to the physical end of the file, with no length
cross-check) and hands to `LjpegDecompressor` unvalidated. This is a single external artifact
(file bytes) handed to its real public parsing entry point - no second participant is being faked,
per `CONFIRMATION.md` section 11.

## Class

`crash`

## Root cause

`BitPumpJPEG::peek_bits` (`rawler/src/pumps.rs`), slow-refill path:

```rust
let nextbyte = self.buffer[self.pos];
if nextbyte != 0xff {
  nextbyte
} else if self.buffer[self.pos + 1] == 0x00 {   // <-- no bounds check on pos + 1
  self.pos += 1; // Skip the extra byte used to mark 255
  nextbyte
} else {
  self.finished = true;
  0
}
```

This loop reads up to 4 bytes per `peek_bits` call, one byte at a time, guarded only by
`self.pos >= self.buffer.len()` at the *top* of each iteration (so `self.pos == self.buffer.len()
- 1`, the last valid index, passes that guard). If that last byte happens to be `0xFF` - the JPEG
byte-stuffing marker byte - the code unconditionally reads `self.buffer[self.pos + 1]` to check
whether it's the stuffed `0x00` continuation byte, with no check that `pos + 1` is still inside
the buffer. When the entropy-coded segment legitimately ends in a bare (non-stuffed) `0xFF` as its
very last byte - exactly what a truncated/corrupted real-world CR2 file can contain, since
`Cr2Decoder::raw_image` reads the LJPEG stream to the physical end of the file with no declared
length - `pos + 1 == buffer.len()` and the index panics with `index == len`, matching the reported
issue's exact panic shape.

## Introduced by

416d897a "Code refactoring for more raw formats" (2022-04-10). `git log -S "self.buffer[self.pos +
1] == 0x00" --all` across the whole repository finds only this one commit, which is a large,
whole-project restructuring/vendoring commit (the point at which the `rawler` crate as currently
laid out first appears in this repo's tracked history) - the buggy line is present verbatim from
that commit's first version of `pumps.rs`, with no earlier occurrence anywhere in this repo's
history to bisect further back into. It plausibly predates this repo's own history (likely
originating in the `rawloader` crate this project descended from), but that predecessor isn't
part of this repository's git history, so `not determined` beyond this commit.

## The fix

Added a bounds check before the `self.buffer[self.pos + 1]` access: if `pos + 1` would be out of
range, fall through to the same `self.finished = true; 0` branch already used for a genuinely
exhausted buffer (mirrors the existing `self.pos >= self.buffer.len()` guard a few lines above in
the same function, which already treats "no more bytes" as end-of-stream rather than an error).

```rust
-              } else if self.buffer[self.pos + 1] == 0x00 {
+              } else if self.pos + 1 < self.buffer.len() && self.buffer[self.pos + 1] == 0x00 {
```

Verified: `repro/` no longer panics against the patched tree (`did not reproduce -
LjpegDecompressor::decode completed (or returned Err) without panicking`), release build, via the
candidate's own `just test` against a fresh clone. Also re-verified in the working scratch
checkout by adding a `#[test]` exercising the same real entry point (kept out of `patch.diff`;
mirrored in `repro/` instead) - `cargo test --lib`: 0 passed/1 failed (panic) unpatched -> 67
passed/0 failed patched.

- Project test suite (`cargo test --lib` in `rawler/`): 66 passed/0 failed before -> 66 passed/0
  failed after (pre-existing suite, unaffected either way); with this candidate's own added
  regression test included, 0 passed/1 failed (the panic) before -> 67 passed/0 failed after.

## What's not done / open questions

- Only the exact reported trigger shape (entropy data ending in a bare `0xFF` as the buffer's last
  byte) was checked. The sibling condition just above in the same function -
  `self.pos >= self.buffer.len()` inside the same `while` loop - was already safe before this fix
  and is unaffected.
- `BitPumpLSB`/`BitPumpMSB`/`BitPumpMSB32`'s own `refill()` methods (`pumps.rs`, same file) use
  `std::slice::Chunks` rather than raw indexing and `panic!("Can't refill bitpump, buffer
  exhausted")` outright when the buffer is exhausted - a different, explicit panic rather than an
  implicit index-out-of-bounds, and not the same bug shape as this candidate. Not investigated as
  part of this session; whether any of those panics are reachable with attacker-controlled input
  (as opposed to only ever being called with enough bytes guaranteed available upstream) was not
  checked and would be a reasonable separate candidate.
- `ByteStream::skip_to_marker` (`pumps.rs`, same file) was noticed in passing to have the same
  *shape* of bug - `self.buffer[self.pos + 1]` with only a `self.pos >= self.buffer.len()` check
  performed *after* incrementing `pos` at the bottom of its loop, not before this specific access -
  and was in fact triggered by an early, since-corrected mistake in this session's own
  hand-crafted repro bytes (an under-length DHT payload caused the marker scanner to run off the
  end of the buffer while searching for SOS). Whether a real malformed file's header bytes (as
  opposed to this session's own transient repro bug) can reach that exact state was not
  independently confirmed and is left as a follow-up lead.
- Only the LJPEG/CR2 path (`BitPumpJPEG`) was investigated, matching issue #587 exactly. This is
  one of several `BitPump` implementations in the same file sharing similar "read ahead without a
  full bounds check" structure in their own `refill`/`peek_bits` methods (see the point above) -
  the fix here is scoped narrowly to `BitPumpJPEG::peek_bits`'s specific slow-refill branch and
  does not address the others.
