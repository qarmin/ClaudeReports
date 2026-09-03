# lossless_scan_marker_ok_swallow

`src/decoder/lossless.rs:179-182`, at the end of `decode_scan_lossless`:

```rust
let mut marker = huffman.take_marker(&mut self.reader)?;
while let Some(Marker::RST(_)) = marker {
    marker = self.read_marker().ok();
}
Ok((marker, results))
```

`self.read_marker().ok()` discards any real `Err` from `read_marker()` (e.g. a genuine `io::Error`
from a stream truncated right after one or more RST markers), collapsing it into `marker = None` -
the exact same value used for "no trailing marker, nothing unusual". The open question was whether
the caller's own subsequent `self.read_marker()?` (in `decoder.rs`'s outer marker loop, reached
whenever `pending_marker` is `None`) re-triggers and surfaces the same error one iteration later
(harmless deferral) or whether the truncation is a true silent swallow (`decode()` returns `Ok`
with wrong/truncated data instead of an `Err`).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction: hand-built a real, spec-shaped single-component lossless (SOF3) JPEG through the crate's own public `Decoder::new()`/`decoder.decode()` entry point (encoded with a from-scratch bit-writer implementing ITU-T T.81 H.1.2.1's default prediction + Ra predictor + byte-stuffing, cross-checked against a control case that decodes correctly), then appended a spurious trailing RST marker (something a corrupted/truncated real file can easily contain even though a conforming encoder wouldn't normally place one after the very last MCU) and truncated the file at 5 different points relative to that marker | Release build, `cargo run --release` against a standalone repro crate depending on the worktree's own `jpeg-decoder` via a path dependency; each of the 5 truncation-point cases run once, plus the primary case (RST0 then immediate EOF) repeated 5 times total (1 initial + 4 more) | Control case decodes correctly (`Ok`, `[10, 20, 30, 40]`). All 5 truncation variants - (1) truncated immediately after one spurious RST0, (2) same, repeated 4 more times, (3) two consecutive spurious RST markers (RST0, RST1) then truncated, (4) RST0 + one stray non-marker byte then truncated, (5) truncated mid-marker (bare trailing `0xFF`, no marker byte at all) - returned `Err(Io(Error { kind: UnexpectedEof, .. }))` from `decoder.decode()`, 9/9 runs total, 0 silent `Ok` results |

One approach was enough to reach a clean, thorough negative with no partial signal at any boundary
tried (see `HEURISTICS.md`'s "Logic bugs" guidance: differential/boundary-value testing against the
real public API is the right tool here, not a memory-safety instrument - there is no `unsafe` and
no UB-shaped mechanism in this code path). No second/third tool was needed per the soft cap.

## Status

**Not reproducible.**

The suspected "true silent swallow" does not occur. Tracing the mechanism: `decode_scan_lossless`
returning `Ok((None, results))` after the swallow sets `pending_marker = None` in `decoder.rs`'s
`Marker::SOS` handling. The outer decode loop's very next iteration unconditionally does
`match pending_marker.take() { Some(m) => m, None => self.read_marker()? }` - since nothing consumed
any bytes between the swallowed failed read and this retry (the underlying reader was already
genuinely exhausted, not merely at a bad offset), the retry hits the identical `UnexpectedEof`
condition and this time propagates it via `?`, all the way out through `decode_internal` to the
caller of `decode()`. The error is deferred by exactly one loop iteration, not lost. Every
truncation point tried (right after one RST, after two consecutive RSTs, after RST + one stray
byte, and mid-marker with only a bare `0xFF`) hit this same deferred-but-surfaced path, and none
returned `Ok`. Covered: 5 distinct truncation-point/marker-count variants, 9 total runs, release
build, deterministic (`Cursor<Vec<u8>>` input, no seed/layout dependency) - no case found where the
retry's own read could avoid re-hitting the same failure.

## Root cause (why the suspect turned out to be safe)

The `.ok()` swallow only ever discards an error at a point where the underlying reader has already
reached a state (genuine end-of-input, in every input actually constructible from a real truncated
file) that the very next read attempt - made unconditionally by the caller one iteration later -
will hit again in the same way. Nothing in between the swallow and the retry advances the reader
or changes its state, so the "swallowed" error is not lost, just relabeled and re-derived from a
different call site (`decoder.rs`'s outer `self.read_marker()?` instead of `lossless.rs`'s inner
`self.read_marker().ok()`). This matches candidate 011's own, independently-reached finding for the
general (non-lossless) `decode_scan` path: `decoder.rs`'s main marker loop has no special handling
for a scan returning `marker = None`, so it always retries `read_marker()` immediately, which is
exactly what turns an inner swallow into a harmless, one-iteration-later `Err` rather than a lost
one - see that candidate's `RESULT.md` for the twin mechanism in the non-lossless code path.

The only way this class of `.ok()` swallow could become a genuine silent swallow is if the retried
`read_marker()` call could plausibly *succeed* (or fail differently) despite the reader being
logically exhausted at the swallow point - e.g. a non-idempotent `Read` implementation where a
second read after a transient error returns different bytes than the first attempt would have (a
network stream with a genuinely transient I/O hiccup, as opposed to real end-of-file). That is
outside what a `Cursor`/`File`/`BufReader`-backed input can exhibit and outside the scope of what
was tested this session - noted below rather than chased further.

## Introduced by

Not determined - not applicable/not pursued, since this candidate is `Not reproducible` (no bug to
attribute to an introducing commit). For context only: `git log` on `src/decoder/lossless.rs` shows
the file has existed with this general shape since very early history (`bd7cbd3` "initial" through
`fb2c126` "lossless prediction decoding"); no attempt was made to pin the exact commit that added
this specific `.ok()` line, since it isn't a confirmed bug.

## The fix

Not applicable (Not reproducible, nothing to fix).

## What's not done / open questions

- Not checked: a hand-rolled `Read` implementation that behaves non-idempotently across repeated
  calls at the same logical position (returns different data or a different error kind on a retry
  after already failing once) - the theoretical scenario noted above where the swallow could stop
  being harmless. Realistic callers of this crate (`File`, `BufReader<File>`, `&[u8]`, `Cursor<_>`)
  do not behave this way, so this was not pursued further this session.
- This candidate only exercises the lossless (`SOF3`) scan path's own copy of this pattern
  (`src/decoder/lossless.rs:179-182`). Candidate 011 already independently investigated and fixed
  a related-but-distinct issue in the shared, non-lossless `decode_scan`/`decode_internal` path
  (the general "scan ends without a marker at all, not via a swallowed error" case) - see that
  candidate's own `RESULT.md`/`patch.diff` for the twin mechanism; that fix was verified against
  its own repro/test suite there and, per this skill's "the one rule that matters most", was
  reverted from the real source afterward like every other candidate's fix.
- No fuzzing/miri/ASAN was run for this candidate - the suspected bug class (a swallowed `io::Error`
  either lost or deferred) has no UB/memory-safety shape, so those tools would not add signal beyond
  the boundary-value testing already done (per `HEURISTICS.md`'s tool-selection guidance for logic
  bugs).
