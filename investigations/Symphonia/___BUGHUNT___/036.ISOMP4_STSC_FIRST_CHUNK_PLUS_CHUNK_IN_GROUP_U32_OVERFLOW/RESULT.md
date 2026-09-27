# ISOMP4_STSC_FIRST_CHUNK_PLUS_CHUNK_IN_GROUP_U32_OVERFLOW

`MoovSegment::sample_data` (`symphonia-format-isomp4/src/stream.rs:416`) computes
`let chunk_in_stream = group.first_chunk + chunk_in_group;` as unchecked `u32` arithmetic, where
`group.first_chunk` comes straight out of the untrusted `stsc` table and `chunk_in_group` grows
with the requested sample index. Flagged by a manual code read of the "isomp4 atoms other than
stts/trun" area (the `stsc` consumer in `stream.rs`), matching `HEURISTICS.md`'s
"arithmetic on untrusted sizes/indices" pattern.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code read of the `stsc` producer/consumer pair | `atoms/stsc.rs::read` vs. `stream.rs::sample_data` | Confirmed `first_chunk` is only validated to be `>= 1` and monotonic across entries; it is never bounded against the `stco`/`co64` chunk count, so `first_chunk` can be `0xFFFFFFFE` internally while `chunk_in_group` is attacker-scalable via `samples_per_chunk = 1`. Also established that `next_packet()` alone cannot reach the overflow (sample 0 already errors out cleanly in `get_chunk_offset`), so the trigger has to come through `seek()` |
| Manual boundary-value construction + debug-assertions build | Real ffmpeg-produced AAC-in-MP4 (`assets/base.m4a`, 23 samples), single `stsc` entry binary-patched in place to `{first_chunk: 0xFFFFFFFF, samples_per_chunk: 1}`; opened with the real public `IsoMp4Reader::try_new` and seeked with the real public `FormatReader::seek(SeekMode::Accurate, SeekTo::Timestamp{ts: 2048})`; `cargo run` with `overflow-checks = true` | Panics `attempt to add with overflow` at `symphonia-format-isomp4/src/stream.rs:416:31`, 3/3 runs, deterministic |
| Same input, release build | `cargo run --release` (`overflow-checks = false`) | No panic; the addition wraps and `seek()` returns `Ok(SeekedTo { actual_ts: 2048 })` while `next_sample_pos` was derived from a wrapped, unrelated chunk offset - a silent wrong-result bug rather than a crash (see `output.txt`, "BEFORE the fix - RELEASE profile") |
| Second crafted input in the same class, exact boundary probe | `{first_chunk: 0xFFFFFFFE, samples_per_chunk: 1}` (`assets/evil_stsc_v2.mp4`): seek to sample 2 (`0xFFFFFFFD + 2 == u32::MAX`) and to sample 3 (`0xFFFFFFFD + 3`) | Sample 2 returns a clean `DecodeError("isomp4: missing stco entry")`; sample 3 panics at the same line. The boundary is exactly where the arithmetic says it should be, and the finding is not tied to the specific `0xFFFFFFFF` value of the first input |

Two escalating approaches (targeted code read, then boundary-value construction under a
debug-assertions build) produced a deterministic panic immediately, so the soft cap of 3 was not
needed - no miri/ASAN/fuzz run was required for what is plain unchecked integer arithmetic on an
untrusted field.

## Status

**Confirmed.** Reproduces 3/3 consecutive runs in a debug build (`debug-assertions = true`,
`overflow-checks = true`), deterministically - pure integer arithmetic over fixed file bytes, no
seed or memory-layout dependence. Two independently-crafted inputs in the same class both panic
(and the two neighbouring non-overflowing sample indices both return a clean decode error), see
`output.txt`.

**Fully reproducible**: **Yes.** The repro's input is a single external artifact - a malformed MP4
file, the same thing any untrusted-source MP4 would be - handed to this crate's real public entry
points (`IsoMp4Reader::try_new`, `FormatReader::seek`). No internal function is called directly and
no second participant's behaviour is faked, so `CONFIRMATION.md` section 11's single-external-artifact
case applies.

## Class

crash (panic on attacker-controlled input in a build with overflow checks on; the same overflow is
a silent wrong-byte-position logic bug in a release build).

## Root cause

`StscAtom::read` (`symphonia-format-isomp4/src/atoms/stsc.rs:63-108`) reads each `stsc` entry's
1-based `first_chunk` field and stores `first_chunk_raw - 1`. Its only validation is that
`first_chunk_raw != 0` and that `first_chunk` is monotonically non-decreasing across entries; the
value is never compared against the number of chunk offsets actually present in `stco`/`co64`. A
crafted file can therefore hold `first_chunk_raw == 0xFFFFFFFF`, i.e. `group.first_chunk ==
0xFFFFFFFE`, in a table that parses without error.

`MoovSegment::sample_data` then computes, for a requested `sample_num`:

- `sample_in_group = sample_num - group.first_sample`
- `chunk_in_group = sample_in_group / group.samples_per_chunk`
- `chunk_in_stream = group.first_chunk + chunk_in_group`  <- unchecked `u32` add, line 416

With `samples_per_chunk == 1`, `chunk_in_group` equals the sample index, so the sum overflows `u32`
as soon as `sample_num >= u32::MAX - group.first_chunk + 1` (sample 2, for
`group.first_chunk == 0xFFFFFFFE`). The out-of-range *result* is handled safely - `get_chunk_offset`
looks the index up with `.get()` and returns `DecodeError("isomp4: missing stco entry")` - but the
addition that produces it happens first, so the bounds check is never reached.

Reachability: `next_packet()` cannot drive `sample_num` high enough on its own, because sample 0
already maps to chunk `0xFFFFFFFE` and fails with the clean "missing stco entry" decode error,
ending packet iteration. The real `seek()` path does reach it: `IsoMp4Reader::seek_track_by_ts`
resolves an arbitrary target timestamp to a sample number via `stts`
(`SttsAtom::find_sample_for_timestamp`) and passes that sample number straight to
`MoovSegment::sample_data`, so a single `seek()` to a timestamp corresponding to sample 2 hits the
overflow with no prior successful packet reads.

In a release build (`overflow-checks = false`) the add wraps instead: `0xFFFFFFFE + 2` becomes
chunk `0`, whose offset exists, so `seek()` reports success and leaves the track positioned at a
byte offset computed from an entirely unrelated chunk.

The neighbouring `sample_in_chunk = sample_in_group - (chunk_in_group * group.samples_per_chunk)`
on line 428 is safe by construction (it is the remainder of the division two lines above), so line
416 is the only unchecked site in this function.

## Introduced by

1f3b1f55ffb5801e531aa7438a495a72392ed295 "mp4: Add experimental seeking support." (2021-06-18) -
`git log -S "group.first_chunk + chunk_in_group"` returns this single commit, which added
`MoovSegment::sample_data` with the unchecked addition from day one; the line is unchanged since.
(The producer side's `first_chunk_raw - 1` and its `first_chunk >= 1` check were later touched by
79de36b "isomp4, adpcm: Fix some crashes. (#497)", which added the `first_chunk_raw == 0` guard but
not an upper bound.)

## The fix

`patch.diff` replaces the unchecked add with `checked_add` and maps overflow to a decode error, in
the same shape the surrounding code already uses for an out-of-range chunk index:

```rust
let Some(chunk_in_stream) = group.first_chunk.checked_add(chunk_in_group)
else {
    return decode_error("isomp4: chunk index overflow");
};
```

This is the minimal, behaviour-preserving fix: any `first_chunk + chunk_in_group` that would
overflow is necessarily far outside the `stco`/`co64` chunk table (both are bounded by the file
size), so the only reachable difference for a well-formed file is none at all, and for a malformed
one the caller now gets the same class of `Error::DecodeError` it would have got from
`get_chunk_offset`. It also removes the release-build silent wrap, not just the debug panic.

Formatting follows the project's own nightly-rustfmt `control_brace_style` (`else` on its own line),
matching the existing `let Some(...) else` blocks in this crate.

- `repro/` before the fix: `BUG REPRODUCED: 2 of 5 cases panicked` (exit 1), panic at
  `stream.rs:416:31`. After the fix: `BUG NOT REPRODUCED: no case panicked` (exit 0), with the two
  previously-panicking cases now returning `Err(DecodeError("isomp4: chunk index overflow"))` and
  the pristine control file still seeking successfully. Verified in both debug and release profiles
  (`output.txt`).
- Project test suite (`cargo test --workspace`): 106 passed / 0 failed before the fix; 106 passed /
  0 failed after (identical, no new failures). `symphonia-format-isomp4` itself contributes 0
  unit/doc tests in either state.

## What's not done / open questions

- The complementary read-time fix was not attempted: `StscAtom::read` could additionally reject
  `first_chunk` values beyond the `stco`/`co64` chunk count, which would catch the whole class of
  nonsense `stsc` tables at parse time rather than at first use. That needs a decision by the
  maintainers about atom ordering inside `stbl` (`stsc` is currently parsed independently of
  `stco`/`co64`, and `StblAtom::read` only warns when both offset atoms are missing), so it is out
  of scope for a bounded one-line fix here.
- Only the `MoovSegment` (non-fragmented) `sample_data` was audited for this arithmetic.
  `MoofSegment::sample_data` (`stream.rs:218`) uses a different index scheme (`trun`-relative
  offsets) and was not checked for an equivalent unchecked add this session.
- Not checked whether other consumers of `stsc`-derived values elsewhere in the crate assume
  `first_chunk` is in range.
- A longer fuzzing campaign pointed at the `stbl` table family (`stsc`/`stsz`/`stco`/`co64`
  cross-consistency) would likely surface further arithmetic of this shape; not run here.
