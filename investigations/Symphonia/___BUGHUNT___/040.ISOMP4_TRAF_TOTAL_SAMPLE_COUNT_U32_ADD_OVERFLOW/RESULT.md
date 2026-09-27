# ISOMP4_TRAF_TOTAL_SAMPLE_COUNT_U32_ADD_OVERFLOW

`TrafAtom::read` (`symphonia-format-isomp4/src/atoms/traf.rs:40`) accumulates every
`TrunAtom::sample_count` found inside a track fragment (`traf`) into a plain `u32` with an
unchecked `total_sample_count += trun.sample_count`. `sample_count` (`trun.rs:246`) is a raw,
unvalidated `u32` read straight from the file, and `TrunAtom::read`'s own per-sample loop
(`trun.rs:274-297`) reads zero bytes per sample when none of the `SAMPLE_DURATION_PRESENT` /
`SAMPLE_SIZE_PRESENT` / `SAMPLE_FLAGS_PRESENT` / `SAMPLE_COMPOSITION_TIME_OFFSETS_PRESENT` flags
are set - so a `trun` claiming `sample_count = 0x80000000` costs only 16 bytes on disk. Two such
`trun`s inside one `traf` sum to `0x100000000`, one past `u32::MAX`. Flagged as a candidate because
the neighbouring `StscAtom::read` (`stsc.rs:93-99`) already uses `checked_mul`/`checked_add` for
its own analogous per-entry accumulator, making this one's plain `+=` look like an inconsistency
rather than an intentional choice.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction | Real fragmented AAC-in-MP4 produced by `ffmpeg` (`frag_keyframe+empty_moov+default_base_moof`), with its single real `trun` atom replaced by two crafted, flags=0 `trun`s (`sample_count = 0x80000000` each); opened via the real public `IsoMp4Reader::try_new`, then drained via the real public `FormatReader::next_packet()` | Panics with "attempt to add with overflow" at `traf.rs:40` in a debug-assertions (dev-profile) build, 3/3 runs, deterministic |
| Debug-assertions/dev profile vs. release-with-overflow-checks | Same crafted file, `cargo run` vs. `cargo run --release` (repro's own `Cargo.toml` sets `overflow-checks = true` in the `[profile.release]` too, since a plain release build would otherwise silently wrap and mask the bug) | Panics identically under both; a genuinely default release profile (no explicit `overflow-checks`) would instead silently wrap `total_sample_count` to `0`, a latent logic bug rather than a crash - not separately exercised this session, since the crate's own `Cargo.toml` does not turn overflow-checks on for release and this session's finding is specifically the debug-build panic |
| Second crafted input in the same class | A different `sample_count` split (`0xFFFFFFFE` + `3`, overflowing by exactly `2`) instead of the first asset's exact `0x80000000` + `0x80000000` split | Also panics before the fix, does not panic (returns a clean `decode_error`) after the fix - confirms the fix isn't narrowly tied to the first input's exact values |

Two escalating approaches were enough to get a clean, repeatable signal immediately; the soft cap
of 3 wasn't needed (plain unchecked integer arithmetic on attacker-controlled input, exactly the
class `HEURISTICS.md`/`SKILL.md` say a debug-assertions build plus boundary values is the right
tool for - no need for miri/ASAN/a fuzz harness).

## Status

**Confirmed.** Reproduces 3/3 runs in a debug-assertions (dev-profile) build, deterministically
(pure integer arithmetic on fixed input bytes - no seed/layout dependency). Also verified with a
second crafted input in the same overflow class (see table above).

**Fully reproducible**: **Yes.** The repro is a single external artifact (a malformed, fragmented
MP4 file) fed to the real, public `IsoMp4Reader::try_new` + `FormatReader::next_packet` entry
points - exactly what any real player/caller does to demux a fragmented MP4. `try_new`'s own
initial atom scan does not parse the `moof`/`traf` (it only records where the first fragment
starts); the crafted `moof`/`traf`/`trun` bytes are only actually parsed the first time
`next_packet()` needs to look past the initial `moov`-derived segment, via
`try_read_more_segments` - the real, ordinary lazy-fragment-parsing path, not a hand-invoked
internal method. There is no second participant to fake here (CONFIRMATION.md section 11's
"single external artifact" exception applies).

## Class

crash (panic on attacker-controlled input in a debug-assertions build; a release build without
explicit `overflow-checks = true` would instead silently wrap `total_sample_count`, corrupting the
fragment's own recorded sample-count total rather than crashing - not independently exercised this
session, see table above).

## Root cause

`TrafAtom::read` (`traf.rs:24-56`) loops over every child atom of a `traf` box; for each
`TrackFragmentRun` (`trun`) child it parses the `TrunAtom` and immediately folds its `sample_count`
field into a running `u32` total with a bare `+=` (`traf.rs:40`). `TrunAtom::read` (`trun.rs:243-311`)
reads `sample_count` (line 246) directly off the wire with no upper-bound validation, and its
per-sample loop (lines 274-297) is a pure no-op when none of the four `SAMPLE_*_PRESENT` flags are
set in the `trun`'s own extended header - so the box's on-disk cost is independent of the claimed
`sample_count` (16 bytes: 8-byte atom header + 4-byte extended header + 4-byte `sample_count`,
regardless of whether it claims 1 sample or `0x80000000`). A `traf` containing two or more such
minimal `trun`s whose `sample_count` values sum past `u32::MAX` overflows the unchecked
accumulator. This is reachable on the ordinary demuxing path: `IsoMp4Reader::try_read_more_segments`
(`demuxer.rs:435-498`) calls `read_atom::<MoofAtom>()` (line 467) the first time `next_packet()`
needs to advance past the current segment, which recurses into `MoofAtom::read` -> `TrafAtom::read`
for every `traf` in the fragment - so any caller that simply drains packets from a fragmented MP4
via the crate's normal public API can trigger this on a crafted file.

## Introduced by

78778a320fcc1610286174141df03d1fe60e8866 "mp4: Improve abstraction of fragmented and
non-fragmented streams." (2021-01-17) - this commit introduced `TrafAtom::total_sample_count` and
the unchecked `total_sample_count += trun.sample_count` accumulation from day one (verified via
`git log -p --follow` on `traf.rs`); the code is functionally unchanged since (only cosmetic
`if let`/`let else` refactors to the surrounding match arms in later commits).

## The fix

Changed the accumulator from a plain `+=` to `total_sample_count.checked_add(trun.sample_count)`,
returning `decode_error("isomp4 (traf): total sample count overflow")` on overflow - the same
`decode_error`-on-overflow pattern `StscAtom::read` already uses for its own analogous
per-entry accumulator (`stsc.rs:96-101`), and consistent with `TrafAtom::read`'s existing `Result`
return type (no signature change needed, unlike `SttsAtom`'s two lookup functions in candidate 015,
which return `Option` and had no `Result` to propagate through). Also gave `total_sample_count` an
explicit `u32` type annotation (needed once the initializer literal `0` is no longer directly
consumed by an untyped `+=`, for `.checked_add` to resolve).

Verified against a second, differently-crafted input in the same overflow class (see "What was
tried" table) - the fix isn't narrowly tied to the exact `0x80000000`/`0x80000000` split of the
first triggering input.

- Project test suite (`cargo test --workspace`): 106 passed, 0 failed before the fix; 106 passed,
  0 failed after (identical - no regressions). `symphonia-format-isomp4` itself has 0 unit/doc
  tests in either state (the 106 come from other crates in the workspace).

## What's not done / open questions

- The release-profile silent-wraparound case (a plain `cargo build --release` with the crate's own
  default `Cargo.toml`, which does not set `overflow-checks = true`) was not independently
  exercised this session - only the repro's own release profile, which explicitly turns
  `overflow-checks` on to keep demonstrating the panic rather than mask it. A real downstream
  release build would instead have `total_sample_count` silently wrap (to `0` for the exact
  `0x80000000`+`0x80000000` input, or to `1` for the `0xFFFFFFFE`+`3` input). This value is not
  dead: `MoofSegment::new` (`stream.rs:118`) copies it into `SequenceInfo::total_sample_count`,
  which `stream.rs:291` then uses to compute `track.first_sample..track.first_sample +
  track.total_sample_count` - the sample-number range the demuxer believes this fragment covers
  for the track. A wrapped value would make the demuxer believe the fragment has far fewer samples
  than it actually claims (or, depending on the exact wrap, a wildly wrong range), a genuine
  logic-bug consequence in release builds rather than just an inert stored field - not
  independently reproduced this session (would need its own repro isolating the release-mode
  wraparound's effect on seeking/packet iteration, separate from this candidate's debug-build
  panic).
- Not checked whether any other ISO/MP4 atom parser in this crate has the same
  "cheap-to-claim, expensive-to-sum" shape (a `sample_count`/entry-count field whose per-entry cost
  can be made zero by omitting optional per-entry data, combined with an unchecked running-sum
  accumulator) beyond the ones already covered by candidates 015 (`stts`), 034 (`sidx`), and 036
  (`stsc`) in this same workspace.
