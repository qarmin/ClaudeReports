# CAF_SEEK_SEARCH_RANGE_OFFSET

`symphonia-format-caf/src/demuxer.rs`'s `CafReader::seek()`, `PacketInfo::VariableAudioPacket` arm
(~lines 257-276), computes `packets[search_range].partition_point(...)` to find the seek target
within a `current_packet_index..packets.len()` or `0..current_packet_index` sub-slice, then uses
that result directly as an index into the *full* `packets` array without adding back
`search_range.start`. This looked suspicious because `partition_point` on a sub-slice always
returns an index relative to that sub-slice, not the original array - flagged via `IDEAS.md`
(sourced from upstream PR #549, open/unmerged, which fixes the same spot the same way, with added
unit tests comparing against a linear reference implementation).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading | Read `CafReader::seek()`'s `VariableAudioPacket` arm (`symphonia-format-caf/src/demuxer.rs:257-305`) and the `PacketTable`/`CafPacket` types it operates on (`symphonia-format-caf/src/chunks.rs`) | Confirmed by inspection: `search_range.start` is discarded after slicing; `seek_packet_index` is used both to index `packets` directly and to update `current_packet_index`, so a wrong index also corrupts subsequent `next_packet()` reads |
| Manual construction of a real CAF file with a variable-packet-size (`pakt` chunk) codec + standalone repro against the public `CafReader::try_new`/`FormatReader::seek` API | Release build, hand-built but format-valid 140-byte CAF file (`.mp3` format id with `bytes_per_packet == 0 && frames_per_packet == 0`, selecting `PacketInfo::VariableAudioPacket`; 6 packets of 10 frames each via a real `pakt` chunk), read `next_packet()` a few times to advance `current_packet_index > 0`, then `seek(SeekMode::Accurate, SeekTo::Timestamp { .. })` forward past the current position | Reproduced: seeking forward from `current_packet_index=3` (start_frame 30) to `required_ts=45` lands on `actual_ts=10` instead of the correct `actual_ts=40` - the seek goes *backwards* past the reader's already-consumed position instead of forward |
| Repeat-run + boundary-value check | Same release binary, same pristine tree, 5 consecutive runs of the same case, plus a second distinct case (`current_packet_index=1`, `required_ts=25`, correct=20, buggy=10) and a third case in the *other* (backward-seek) branch as a sanity check that it stays unaffected | 5/5 runs of case A reproduce identically; case B (a different `current_packet_index`/`required_ts` pair) reproduces the same class of bug; case C (backward seek, `search_range.start == 0`) is correct even before the fix, consistent with the root cause only affecting the forward-seek branch where `search_range.start > 0` |

## Status

**Confirmed**. Reproduces 5/5 runs (plus a second distinct boundary case), deterministic, release
build, no sanitizer or fuzzing needed - this is a plain index-arithmetic logic bug, not
crash/UB, so a debug-assertions build and manual boundary-value construction (per
`HEURISTICS.md`'s guidance for this bug class) were sufficient; a fuzz harness/miri/ASAN would add
nothing here since there is no crash or memory-safety violation to catch, just a silently wrong
result.

**Fully reproducible**: **Yes**. The repro constructs a real, format-valid CAF file (a genuine
`desc`/`pakt`/`data` chunk sequence, not an in-process stand-in for the demuxer's internal state)
and drives it entirely through `CafReader::try_new` + the plain public
`FormatReader::next_packet()`/`seek()` API, exactly as a real caller (e.g. a player doing forward
seeking during playback) would. Per `CONFIRMATION.md` section 11, this is a single external
artifact (a file) handed to a parser's real public entry point with no second participant being
faked, so the single-process repro shape is the correct one here.

## Class

logic (wrong seek target - no crash, no panic, just landing on the wrong packet)

## Root cause

`CafReader::seek()`'s `PacketInfo::VariableAudioPacket` arm:

```rust
let search_range = if current_ts < required_ts {
    *current_packet_index..packets.len()
}
else {
    0..*current_packet_index
};

let packet_after_ts = packets[search_range]
    .partition_point(|packet| packet.start_frame < required_ts);
let seek_packet_index = packet_after_ts.saturating_sub(1);
let seek_packet = &packets[seek_packet_index];
```

`packets[search_range]` produces a sub-slice of the full `packets: Vec<CafPacket>`.
`Iterator`/`[T]::partition_point` operates on that sub-slice and returns an index counted from
*its own* start (index 0 == the sub-slice's first element), not from the start of `packets`. When
`search_range.start == 0` (the backward-seek branch, `0..current_packet_index`), the sub-slice and
full-array indices coincide, so the bug is silent there. But when `search_range.start > 0` (the
forward-seek branch, `current_packet_index..packets.len()`, which is the common case once
`current_packet_index` has advanced past the first packet), `packet_after_ts` is an index into the
sub-slice that gets used unmodified as `seek_packet_index` into the full `packets` array - short by
exactly `search_range.start`. The result (`packets[seek_packet_index]`) is systematically an
earlier packet than intended, and since `seek_packet_index` is also written back into
`current_packet_index`, every subsequent `next_packet()` call continues from the wrong,
earlier position too.

Concretely, with packets at `start_frame` `[0, 10, 20, 30, 40, 50]` and
`current_packet_index == 3` (positioned at `start_frame 30`), seeking forward to `required_ts =
45`: the sub-slice is `packets[3..6] = [30, 40, 50]`; `partition_point(|p| p.start_frame < 45)` on
that sub-slice returns `2` (both 30 and 40 are `< 45`); `seek_packet_index = 2 - 1 = 1`; the code
then reads `packets[1]` (`start_frame == 10`) instead of the correct `packets[3 + 2 - 1] =
packets[4]` (`start_frame == 40`).

## Introduced by

abb497b "caf: Add initial support for Core Audio Format / CAF files (2024-01-16)" (PR #232) - `git
log -S"partition_point" -- symphonia-format-caf/src/demuxer.rs` finds only this one commit, which
introduced `search_range`/`partition_point`/`seek_packet_index` verbatim as part of the CAF
demuxer's original `seek()` implementation; the missing `search_range.start` offset has been
present unchanged since this commit, not introduced by any later refactor.

## The fix

Capture `search_range.start` before it is consumed by indexing (`Range<usize>` is not `Copy`), and
add it back to `partition_point`'s result before using it as an absolute index:

```rust
let search_range_start = search_range.start;
let packet_after_ts = search_range_start
    + packets[search_range]
        .partition_point(|packet| packet.start_frame < required_ts);
let seek_packet_index = packet_after_ts.saturating_sub(1);
let seek_packet = &packets[seek_packet_index];
```

This is sufficient for both branches: in the backward-seek branch (`search_range.start == 0`) the
added offset is a no-op, so already-correct behavior is unchanged; in the forward-seek branch it
correctly re-bases the sub-slice-relative index back into the full `packets` array. Verified
against more than the one triggering input (`CONFIRMATION.md` section 7): re-ran `repro/` against
the patched tree for the original case (`current_packet_index=3`, `required_ts=45`, now correctly
`actual_ts=40`, 5/5 runs), a second distinct forward-seek case (`current_packet_index=1`,
`required_ts=25`, now correctly `actual_ts=20`), and the backward-seek branch as a regression check
(`current_packet_index=5`, `required_ts=5`, still correctly `actual_ts=0`, unaffected as expected).

- Project test suite (`cargo test --workspace`): **106 passed, 0 failed** before the fix, **106
  passed, 0 failed** after the fix (identical pass count, no regressions; `symphonia-format-caf`'s
  own `cargo test -p symphonia-format-caf` run is 2 passed, 0 failed both before and after - no
  existing test in this package exercised `VariableAudioPacket` seeking at all).

## What's not done / open questions

- Only `SeekTo::Timestamp` was exercised directly; `SeekTo::Time` reaches the same
  `VariableAudioPacket` arm after converting through `TimeBase::calc_timestamp`, so it is affected
  identically but was not separately re-run (same downstream code, same fix).
- `PacketInfo::FixedAudioPacket`'s seek arm (the sibling match arm just above this one) does not
  use `partition_point`/sub-slicing at all - it computes the seek position arithmetically - so it
  does not share this bug class; this was confirmed by reading that arm, not by a separate repro.
- No other Symphonia demuxer was checked for the same sub-slice-index-used-as-absolute-index
  pattern; this was checked only for `symphonia-format-caf`.
- Matches upstream PR #549 (open, unmerged at the time of this session) - same root cause, same
  fix shape, and that PR additionally adds a `find_seek_packet_index` unit test comparing against
  a linear-scan reference implementation, which was not duplicated here (this session's `repro/`
  demonstrates the bug end-to-end through the public API instead, per `CONFIRMATION.md` section
  11, rather than as an internal function-level unit test).
