# OGG_SEEK_SEEKABLE_BISECTION_CROSS_STREAM

Follow-up to candidate 005 (OGG_SEEK_CROSS_STREAM_INFINITE_LOOP), which left open whether the same
missing-`discard_logical_packet()` bug in `OggReader::do_seek()`'s packet-skip loop
(`symphonia-format-ogg/src/demuxer.rs:262-293`) is reachable while `self.reader.is_seekable() ==
true` - i.e. through the byte-range bisection fast path (`demuxer.rs:177-260`), not just the
non-seekable packet-by-packet scan candidate 005 demonstrated via `ReadOnlySource`. The two paths
share the exact same packet-skip loop after the bisection block, so the question was whether a
seekable-source caller could ever land in the loop with a cross-stream packet already queued up.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading | Re-read `do_seek()`'s bisection block (lines 177-260) and the shared packet-skip loop (262-293) it falls into afterward | After bisection converges, only the target logical stream's buffered-packet queue is populated (`stream.read_page(&self.pages.page())` is called only for `s == serial` at line 256-258); if that page's buffered packets run out before `target_ts`, the loop's `_ => match self.read_page()` arm fetches the *next physical page* unconditionally - which, in an interleaved multi-logical-stream file, can belong to a different serial. This looked reachable in principle, not just in theory. |
| Manual construction: sweep of seek targets against the real two-Vorbis-stream `.ogg` file from candidate 005, opened via a plain `File` (real seekable `MediaSource`, no `ReadOnlySource`) | Release build, reused `assets/out_long.ogg` (ffmpeg/libvorbis, 2 interleaved logical streams), swept `SeekTo::Time` targets in 0.05s steps from 0.02s to 20s on both tracks, 3s per-attempt watchdog | Found a hang at `track_index=0` (serial `0x344085ff`), `t=1.07s`: bisection lands on a page whose own buffered packets are exhausted right at that timestamp, the very next physical page belongs to the other logical stream (`0x769f9c6f`), and the loop spins forever peeking that never-discarded packet - reached entirely through the seekable/bisection fast path. |
| Repeat-run check (single target, `t=1.07s`, track 0) | Same release binary, same input file, 3 consecutive single-shot runs, 10s watchdog | 3/3 runs hang (`TIMEOUT: seek() did not return within 10s`), deterministic |
| Mechanism confirmation via temporary instrumentation | Added a scratch `eprintln!` (never committed) at the `packet.track_id != serial` arm, printing the peeked page's serial, the packet's `track_id`, and the seek target's serial, capped to abort after 200 identical iterations | Printed `page_serial(peeked-from)=0x769f9c6f packet.track_id=0x769f9c6f seek_target_serial=0x344085ff` repeatedly, then aborted at iteration 200 - confirms the exact mechanism described above, on the seekable path |

## Status

**Confirmed**. Reproduces 3/3 runs, release build, deterministic (same interleaved pages read in
the same order every run; no `ReadOnlySource`/non-seekable wrapper involved anywhere in this
repro).

**Fully reproducible**: **Yes**. The repro opens the same real, ordinary `ffmpeg`-produced Ogg
file used by candidate 005 (not hand-crafted bytes) through `OggReader::try_new`, wrapping it in a
plain `std::fs::File` (asserted seekable via `MediaSource::is_seekable`), and calls the plain
public `FormatReader::seek(SeekMode::Accurate, SeekTo::Time { .. })` API exactly as a real caller
would. Per `CONFIRMATION.md` section 11, this is a single external artifact (a file) handed to a
parser's real public entry point with no second participant being faked, so the single-process
repro shape is correct here.

## Class

logic (unconditional infinite loop / hang, not a crash) - same class as candidate 005, confirmed
reachable via a second, independent entry path (the seekable bisection fast path) into the same
underlying missing-discard bug.

## Root cause

Identical underlying defect to candidate 005, now confirmed reachable from the seekable path too.
`OggReader::do_seek()`'s bisection block (`demuxer.rs:177-260`), when `self.reader.is_seekable()`
is true, converges on a physical page belonging to the target logical stream (serial `s ==
serial`) and buffers only that stream's packets from that one page (`demuxer.rs:250-259`):

```rust
for (&s, stream) in self.streams.iter_mut() {
    stream.reset();
    if s == serial {
        stream.read_page(&self.pages.page())?;
    }
}
```

Execution then falls through unconditionally into the same "consume packets until reaching the
desired timestamp" loop candidate 005 root-caused (`demuxer.rs:262-293`):

```rust
let actual_ts = loop {
    match self.peek_logical_packet() {
        Some(packet) => {
            if packet.track_id != serial {
                continue;                       // <- missing discard_logical_packet()
            }
            ...
            self.discard_logical_packet();
        }
        _ => match self.read_page() { ... },    // <- fetches next physical page, any serial
    }
};
```

If the bisected-to page's own buffered packets are exhausted (`peek_logical_packet()` returns
`None`) before `packet.pts + packet.dur >= target_ts` is reached, the `_ =>` arm calls
`self.read_page()`, which reads whatever physical page comes next in byte order - with no
guarantee it belongs to the target serial. In an interleaved multi-logical-stream Ogg file (pages
from different logical streams alternating in the physical stream, the normal case for a
multi-track container, e.g. two audio tracks or an audio + Skeleton/metadata track), this next
page frequently belongs to a different serial. `peek_logical_packet()` then returns `Some(packet)`
with `packet.track_id != serial`, taking the `continue;` branch without consuming anything - the
identical packet is peeked again on the next iteration, forever. This makes the bug reachable
whenever a seek doesn't land exactly on a page boundary aligned with `target_ts` for the target
track, which is a normal, unremarkable outcome of interleaved multiplexing, not an edge case
requiring adversarial input.

This closes the open question left by candidate 005's RESULT.md: the seekable/bisection path is
not immune to this bug - it shares the exact same faulty loop and can enter it with a cross-stream
packet already queued up, via ordinary `File`-backed seeking with no non-seekable wrapper involved.

## Introduced by

2cafae8 "ogg: Support basic seeking." (2021-08-12) - same commit identified for candidate 005 via
`git log -S"fn do_seek"` on `symphonia-format-ogg/src/demuxer.rs`; this candidate demonstrates a
second, independent trigger path into the exact same missing-discard code introduced by that
commit, not a separate defect.

## The fix

Identical fix to candidate 005 - discard the mismatched packet before looping back, so the logical
stream's packet queue always advances instead of leaving the same non-matching packet at its front
forever:

```rust
if packet.track_id != serial {
    self.discard_logical_packet();
    continue;
}
```

Because both the seekable and non-seekable paths share this exact loop, one fix (already recorded
as candidate 005's `patch.diff`) covers both trigger paths - this candidate's `patch.diff` is
therefore identical to candidate 005's, applied to confirm it also resolves this independently
constructed seekable-path trigger.

Verified: re-running this candidate's `repro/` against the patched tree, 3/3 runs now complete
immediately (`seek returned: ok=true`), no hang. A full sweep of seek targets (0.02s-20s, 0.05s
steps, both tracks) against the patched tree found no remaining hangs anywhere in the swept range.

- Project test suite (`cargo test --workspace --release`, full worktree at base_commit): **106
  passed, 0 failed** before the fix, **106 passed, 0 failed** after the fix (identical pass count,
  no regressions).

## What's not done / open questions

- Only `SeekTo::Time` was exercised, on the specific `t=1.07s` target found by sweeping; the same
  underlying loop is shared by `SeekTo::Timestamp`, so it is affected identically but was not
  separately re-run.
- The sweep only covered the one asset file (`out_long.ogg`, 2 interleaved Vorbis streams,
  ~20s duration) reused from candidate 005; a file with more logical streams, different page
  sizes, or different packet-duration distributions was not constructed, though none of that is
  expected to change the mechanism.
- No other Symphonia demuxer's seek implementation was checked for this pattern this session (same
  scope note as candidate 005 - only `symphonia-format-ogg` was examined).
