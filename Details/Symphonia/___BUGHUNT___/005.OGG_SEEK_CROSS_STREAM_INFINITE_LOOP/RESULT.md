# OGG_SEEK_CROSS_STREAM_INFINITE_LOOP

`symphonia-format-ogg/src/demuxer.rs`'s `OggReader::do_seek()` has a "consume packets until
reaching the desired timestamp" loop that peeks the next logical packet and, if it belongs to a
different logical bitstream than the seek target, does `continue;` without discarding it. This
looked suspicious because peeking is documented (and implemented) as non-advancing, so skipping
without discarding should replay the same packet forever - flagged via `IDEAS.md` (sourced from
upstream PR #548, which fixes the same spot the same way this session independently confirmed).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading | Read `do_seek()`'s packet-skip loop (`symphonia-format-ogg/src/demuxer.rs:262-293`) and the `peek_logical_packet`/`discard_logical_packet` pair it calls | Confirmed by inspection: `peek_logical_packet` returns a reference into the still-buffered packet without consuming it; the `packet.track_id != serial` arm at line 267-269 does `continue;` with no call to `discard_logical_packet()`, so the next loop iteration peeks the identical packet again |
| Manual construction of a real multi-logical-stream Ogg file + standalone repro against the public `OggReader`/`FormatReader::seek` API | Release build, `ffmpeg`-encoded two-Vorbis-stream `.ogg` (20s, 44 interleaved pages), wrapped in `ReadOnlySource` (real public non-seekable-source wrapper) to force the packet-by-packet scan path, seek to `Time=1.0s` on the second track, 10s watchdog thread | Reproduced: process pegs one core at ~99.6-99.8% CPU and never returns; watchdog fires every time |
| Repeat-run check | Same release binary, same input file, 3 consecutive runs | 3/3 runs hang (process killed by `timeout`, exit code 124 every time) |

The first attempt (seekable `File` source, default bisection path) did **not** reproduce the hang:
`do_seek`'s bisection fast path calls `next_page_for_serial`, which itself skips over pages of
other serials while searching, so it usually lands directly on a target-track page whose own
buffered packets already satisfy the requested timestamp before any cross-stream page is ever
read. The bug is real regardless (see Root cause), but demonstrating it end-to-end required a
scenario where the packet-skip loop actually has to cross from one logical stream's page to
another's - which happens whenever the source is not seekable (`MediaSource::is_seekable() ==
false`, the real code path used for any non-seekable input, e.g. `ReadOnlySource` wrapping a
network/pipe reader as done in `symphonia-play` and other real consumers), since `do_seek` then
skips the whole bisection block and scans packet-by-packet from the current read position across
both interleaved logical streams.

## Status

**Confirmed**. Reproduces 3/3 runs, release build, deterministic (not a timing-dependent flake -
the same interleaved pages are read in the same order every run).

**Fully reproducible**: **Yes**. The repro opens a real, ordinary Ogg file - produced by `ffmpeg`
(libvorbis), not hand-crafted bytes - through `OggReader::try_new` and calls the plain public
`FormatReader::seek(SeekMode::Accurate, SeekTo::Time { .. })` API, exactly as a real caller would.
Per `CONFIRMATION.md` section 11, this is a single external artifact (a file) handed to a parser's
real public entry point with no second participant being faked, so the single-process repro shape
is the correct one here (not the two-participant/loopback-socket shape, which applies only when a
peer's reaction to another peer's wire input is the thing under test).

## Class

logic (unconditional infinite loop / hang, not a crash)

## Root cause

`OggReader::do_seek()`'s packet-skip loop (`symphonia-format-ogg/src/demuxer.rs:262-293`):

```rust
let actual_ts = loop {
    match self.peek_logical_packet() {
        Some(packet) => {
            // Skip packets not belonging to the stream being seeked.
            if packet.track_id != serial {
                continue;
            }
            ...
            self.discard_logical_packet();
        }
        _ => match self.read_page() { ... },
    }
};
```

`peek_logical_packet()` returns a reference to the front of the current logical stream's buffered
packet queue without consuming it (`LogicalStream::peek_packet`, unlike `next_packet`/
`consume_packet`, never advances the queue). When the packet peeked belongs to a track other than
the seek target (`packet.track_id != serial`), the loop does `continue;` and re-enters `match
self.peek_logical_packet()` - which, because nothing was consumed, returns the exact same `Some`
packet again. There is no code path out of this state: `read_page()` is only reached through the
`_ =>` arm, which requires `peek_logical_packet()` to return `None` first, and it never will while
the mismatched packet sits unconsumed at the front of the queue. The loop spins at 100% CPU
forever on the very first cross-stream packet it meets.

This is reachable whenever an Ogg file contains more than one logical bitstream (any multi-track
Ogg, e.g. two audio tracks, or an audio track plus a Skeleton/metadata track) and the seek target's
required timestamp is reached by scanning packets sequentially across pages rather than landing
directly on a target-track page via the seekable bisection fast path - concretely, this session
observed it whenever the source reports `is_seekable() == false` (a real, supported configuration:
`ReadOnlySource`, used for any non-seekable input such as a network stream or pipe).

## Introduced by

2cafae8 "ogg: Support basic seeking. (2021-08-12)" - `git log -S"fn do_seek"` on `symphonia-format-ogg/src/demuxer.rs` finds this as the commit that introduced `do_seek` (and its packet-skip loop) from scratch; the missing `discard_logical_packet()` call in the mismatched-track arm has been present since this commit, not added later.

## The fix

Discard the mismatched packet before looping back, exactly like the same loop already does for a
matching packet - this is also the fix upstream PR #548 applies at the same spot:

```rust
if packet.track_id != serial {
    self.discard_logical_packet();
    continue;
}
```

This advances the logical stream's packet queue past the non-matching packet, so the next
`peek_logical_packet()` call either returns the following buffered packet or `None` (driving the
loop into the `read_page()` arm to fetch more data) - either way the loop makes forward progress
instead of re-observing the same packet.

Verified: re-running `repro/` against the patched tree, 3/3 runs now complete immediately (`seek
returned: ok=true`, with a correct `actual_ts` reported via the existing `debug!` log,
`packet_ts=42560` for the `target_ts=43076` requested), no hang.

- Project test suite (`cargo test --workspace`): **106 passed, 0 failed** before the fix, **106
  passed, 0 failed** after the fix (identical pass count, no regressions, no `symphonia-format-ogg`
  unit tests exist to directly exercise this - the package's own `cargo test -p
  symphonia-format-ogg` run is 0 tests both before and after).

## What's not done / open questions

- Only the `SeekTo::Time` entry point was exercised; `SeekTo::Timestamp` drives the exact same
  `do_seek()` loop with the same missing-discard bug, so it is affected identically but was not
  separately re-run (same function, same fix, no reason to expect a different outcome).
- The seekable/bisection path was not separately forced into the same cross-stream state (e.g. by
  constructing a seekable multi-stream file where the landed bisection page's own buffered packets
  are exhausted before reaching `target_ts`, forcing a physical-page crossover into another
  stream's page while still seekable). The non-seekable-source repro already demonstrates the
  underlying bug unconditionally reachable through the public API, so this was not pursued further
  this session, but a maintainer wanting a seekable-source-only repro would need to construct that
  narrower scenario separately.
- No other Symphonia demuxer was checked for the same peek-without-discard pattern in its own seek
  implementation; this was checked only for `symphonia-format-ogg`.
