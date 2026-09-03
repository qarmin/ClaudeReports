# ISOMP4_STTS_SAMPLE_COUNT_ACCUMULATOR_OVERFLOW

`SttsAtom::find_timing_for_sample`/`find_sample_for_timestamp`
(`symphonia-format-isomp4/src/atoms/stts.rs:38,68`) accumulate `entry.sample_count` - a raw `u32`
read straight from an untrusted MP4 `stts` box - into a plain `u32` with unchecked `+=`.
`SttsAtom::read()` only overflow-checks the *duration* accumulator
(`total_duration.checked_add(sample_count * sample_delta)`), never the running sum of
`sample_count` itself, so a crafted `stts` box with `sample_delta == 0` sails through read-time
validation (the duration product is always 0) while still being able to overflow the separate
sample-count accumulator in the two lookup functions. Flagged by an earlier session's
temporary/reverted `#[cfg(test)]` unit test (per this candidate's own description) - this session
built a permanent standalone repro and a verified fix.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction | Crafted a real MP4 (via `ffmpeg`, AAC-in-MP4) with its `stts` atom's 2 entries binary-patched to `{sample_count: u32::MAX, sample_delta: 0}, {sample_count: 1, sample_delta: 0}`; opened via the real public `IsoMp4Reader::try_new`, then called the real public `FormatReader::seek(SeekMode::Accurate, SeekTo::Time{time: 0.0, ..})` | Panics with "attempt to add with overflow" at `stts.rs:68` in a debug-assertions build (3/3 runs, deterministic) |
| Debug-assertions build vs. release build | Same crafted file, `cargo run` (debug, `overflow-checks = true`) vs. `cargo run --release` | Debug: panics every time. Release: silently wraps, `seek()` returns `Err(SeekError(OutOfRange))` instead of crashing - a real logic bug under release too, just not a crash |
| Second crafted input in the same class | Entries `{sample_count: 0xFFFFFFFE, sample_delta: 0}, {sample_count: 5, sample_delta: 0}` (overflow by a different, smaller margin) | Also panics before the fix, does not panic after the fix - confirms the fix isn't narrowly tied to the exact `u32::MAX`/`1` values of the first input |

Two escalating approaches were enough to get a clean, repeatable signal on the first try; the soft
cap of 3 wasn't needed since a real, deterministic panic appeared immediately (no need for
miri/ASAN/a fuzz harness - this is plain unchecked integer arithmetic, exactly the class
`HEURISTICS.md`/`SKILL.md` say a debug-assertions build plus boundary values is the right tool
for).

## Status

**Confirmed.** Reproduces 3/3 runs in a debug build (`overflow-checks = true`), deterministically
(pure integer arithmetic on fixed input bytes - no seed/layout dependency). Also verified with a
second crafted input in the same class (see table above).

**Fully reproducible**: **Yes.** The repro is a single external artifact (a malformed MP4 file) fed
to the real, public `IsoMp4Reader::try_new`/`FormatReader::seek` entry points - the same file a real
attacker/untrusted-source-supplied MP4 would present to any caller of this crate. There is no
second participant to fake here (see `CONFIRMATION.md` section 11's "single external artifact"
exception) - the crafted bytes are handled exclusively via the crate's real public API, not by
calling `SttsAtom`'s internal methods directly.

## Class

crash (panic on attacker-controlled input; also a silent-wraparound logic bug in a release build,
since the overflow is real either way, only debug-assertions makes it visible as a panic).

## Root cause

`SttsAtom::read()` (`stts.rs:76-102`) validates that `total_duration` (`sample_count *
sample_delta`, summed across entries) never overflows `u64`, using `checked_add` and a
`decode_error` on failure - but it never separately validates the running sum of `sample_count`
alone. `find_timing_for_sample` (line 38) and `find_sample_for_timestamp` (line 68) each maintain
their own independent `u32` accumulator of `sample_count` (`next_entry_first_sample` and
`sample_num` respectively) to track which entry a given sample index/timestamp falls into. Because
an entry with `sample_delta == 0` contributes `0` to `total_duration` regardless of how large its
`sample_count` is, an attacker can set `sample_delta = 0` on every entry and put arbitrarily large
`sample_count` values in the box - `read()`'s only real check is neutralized, and the file parses
successfully. When either lookup function is later called (reachable from the real per-packet
timing path, `next_sample_info` -> `MoovSegment::sample_timing`, and from the real seek path,
`seek_track_by_ts` -> `MoovSegment::ts_sample`), the unchecked `+=` on the `sample_count` running
sum panics once the sum of processed entries' `sample_count` values exceeds `u32::MAX`.

The seek path (`find_sample_for_timestamp`, line 68) is the easier one to trigger externally: with
`sample_delta == 0` on every entry, the "found it" branch (`ts_accum + delta > ts`) is never taken
(`delta` is always `0`), so the loop runs through every entry unconditionally regardless of the
requested timestamp, hitting the unchecked add on the second entry already. `find_timing_for_sample`
(line 38) has the identical accumulator-overflow shape, but triggering it externally needs the
*sample index* being looked up to actually reach past the overflow point, which (via the
per-packet path) would require driving `next_packet()` sequentially through billions of samples -
not attempted this session (see "What's not done" below); the fix is applied to both functions
since they share the exact same unchecked-`+=`-on-untrusted-input pattern.

## Introduced by

1f3b1f55ffb5801e531aa7438a495a72392ed295 "mp4: Add experimental seeking support" (2021-06-18) -
this commit added `find_sample_for_timestamp` (the function this candidate's repro actually
triggers) with the unchecked `sample_num += entry.sample_count` from day one; the code is
unchanged since. `find_timing_for_sample`'s own unchecked `next_entry_first_sample +=
entry.sample_count` predates this (introduced in 78778a320fcc1610286174141df03d1fe60e8866 "mp4:
Improve abstraction of fragmented and non-fragmented streams", 2021-01-17) with the same shape.

## The fix

Changed both accumulators from a plain `+=` to `.checked_add(entry.sample_count)?` - on overflow,
both functions now return `None` (the pattern the loop already used to mean "not found") instead of
wrapping/panicking, semantically equivalent to running off the end of the entry list. Also gave
`next_entry_first_sample` an explicit `u32` type annotation (its initializer literal `0` was
otherwise inferred by nearby untyped arithmetic; needed for `.checked_add` to resolve). This does
not match `StscAtom::read()`'s pattern exactly (that one returns a `decode_error` at read time,
because it validates during atom parsing where a `Result` is already the return type) - these two
functions return `Option`, are called from arbitrary points after a file is already open (not just
once at parse time), and already use `None` to mean "no such sample/timestamp", so mapping overflow
to `None` is the natural, non-signature-breaking equivalent for this call shape.

Verified against a second, differently-crafted input in the same overflow class (see "What was
tried" table) - the fix isn't narrowly tied to the exact `u32::MAX`/`1` split of the first
triggering input.

- Project test suite (`cargo test --workspace`): 106 passed, 0 failed before the fix; 106 passed, 0
  failed after (identical - no regressions). `symphonia-format-isomp4` itself has 0 unit/doc tests
  in either state.

## What's not done / open questions

- `find_timing_for_sample`'s own copy of this bug (line 38, now also fixed) was not independently
  triggered via its own real call path this session (would need billions of sequential
  `next_packet()` calls to drive `state.next_sample` past the overflow point, or a `stsz`/`stco`
  crafted to make the demuxer believe a colossal sample count exists without needing to actually
  decode that many packets - not attempted). The fix was applied to it anyway since it shares the
  exact unchecked-accumulator pattern with the function that *was* triggered, and the patched
  behavior (return `None` on overflow) was manually re-checked to compile and preserve the
  function's existing "not found" semantics.
- Not checked whether any other ISO/MP4 atom parser in this crate (`stsz`, `stco`/`co64`, `ctts`,
  `stss`, etc.) has the same "per-read overflow check exists, but a separately-maintained runtime
  lookup accumulator doesn't" gap - `stsc.rs`'s own post-processing loop already uses
  `checked_mul`/`checked_add` throughout (the pattern this fix now matches), but the others weren't
  audited this session.
