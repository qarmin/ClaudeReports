# ISOMP4_TRUN_WRONG_BITMASK_FIRST_SAMPLE_DURATION

`TrunAtom::is_first_sample_duration_present()` (`symphonia-format-isomp4/src/atoms/trun.rs:51-56`)
tests `first_sample_flags & FIRST_SAMPLE_FLAGS_PRESENT` (bit `0x4`, the trun-level flag that only
indicates whether `first_sample_flags` itself is present at all) instead of
`SAMPLE_DURATION_PRESENT` (bit `0x100`), unlike the analogous `is_first_sample_size_present()`
which tests `SAMPLE_SIZE_PRESENT` (`0x200`). This was flagged as an open suspect in candidate 017's
`RESULT.md` ("deserves its own candidate") but not investigated there.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read `git log -S`/`git show` on the introducing commit to understand original intent and confirm the constant mismatch | n/a | Confirmed the wrong constant (`FIRST_SAMPLE_FLAGS_PRESENT` instead of `SAMPLE_DURATION_PRESENT`) has been present since the feature's introduction in commit d797bbe (see "Introduced by"). Also established, by reading `TrunAtom::read()`'s per-sample loop, that `sample_duration` is *only ever* populated when the trun's own `SAMPLE_DURATION_PRESENT` flag is set - the exact same condition that makes `is_sample_duration_present()` true and short-circuits `total_duration()`/`sample_timing()`/`ts_sample()` before this buggy branch is even reached. This means `sample_duration` is unconditionally empty in every context where `is_first_sample_duration_present()`'s result is actually consulted, regardless of which bit is tested - a bounded fix needs an explicit non-empty guard, not just the "right" bitmask, to be a real fix (see "The fix"). |
| Real end-to-end repro: `ffmpeg`-produced fragmented AAC-in-MP4 (`assets/frag.mp4`, reused from candidate 017), binary-patched to set `trun` flags `0x1 -> 0x5` and insert `first_sample_flags = 0x02000004` in the first fragment only (leaving the trun's own `SAMPLE_DURATION_PRESENT` unset), driven through the real public `IsoMp4Reader::try_new` + `FormatReader::next_packet` API | release build, 3 repeat runs against the crafted file plus 1 baseline run against the real unmodified file | `assets/evil_first_sample_flags.mp4`: **3/3** runs panic with "index out of bounds: the len is 0 but the index is 0" at `trun.rs:92`, inside `MoofSegment::new` -> `IsoMp4Reader::try_read_more_segments` -> `FormatReader::next_packet` (real call stack, captured via `RUST_BACKTRACE=1`). Baseline `assets/frag.mp4` (same file, unmodified): `next_packet()` succeeds and returns a real 571-byte packet. Deterministic, identical output across all 3 runs. |

No third approach was needed - the second attempt (the real end-to-end repro) already produced a
fully repeatable, root-caused reproduction through the real public API.

## Status

**Confirmed.** 3/3 runs of the real end-to-end repro panic identically at `trun.rs:92`, driven
entirely through `IsoMp4Reader::try_new` + `FormatReader::next_packet` on a real, otherwise-valid
fragmented MP4 file with only the first fragment's `trun` atom's `first_sample_flags`/flags fields
binary-patched.

**Fully reproducible**: Yes. The repro opens a real file with the crate's own public
`IsoMp4Reader`/`FormatReader` API and calls `next_packet()`, the same entry point any real
caller (a decoder, a media player) uses to read a fragmented/CMAF stream - no direct calls into
`TrunAtom`/`MoofSegment` internals, and no participant is faked (the "second participant" here is
the on-disk file's byte content itself, not a live peer/process, so there is nothing further to
run in real time beyond feeding those bytes through the real parser, which the repro does).

## Class

crash

## Root cause

`TrunAtom::read()`'s per-sample loop only ever pushes into `self.sample_duration` when the trun's
own `SAMPLE_DURATION_PRESENT` (`0x100`) tf-level flag is set - in that case `sample_duration.len()
== sample_count`; otherwise it is always empty. `total_duration()` (and `sample_timing()`/
`ts_sample()`, which share the same helper) first checks `is_sample_duration_present()` (which
reads that same tf-level flag) and only falls through to the buggy branch when it is false -
meaning `self.sample_duration` is guaranteed empty by the time
`is_first_sample_duration_present()` is even consulted.

`is_first_sample_duration_present()` itself tests a bit of `first_sample_flags` - a completely
separate, per-first-sample ISO/IEC 14496-12 *sample-flags* word (is-leading /
sample-depends-on / sample-is-depended-on / sample-has-redundancy / padding / non-sync /
degradation-priority), unrelated to the trun's own tf-level flags namespace. It tests bit `0x4`
(`FIRST_SAMPLE_FLAGS_PRESENT`, the trun-level flag for "is `first_sample_flags` present at all"),
which happens to fall within the sample-flags word's low-order `sample_degradation_priority`
sub-field (bits 0-15) - an ordinary, encoder-controlled 16-bit value with no defined relationship
to sample duration. Any first sample whose `sample_flags` word has that bit set (plausible for
real encoder output - many encoders set a non-zero degradation priority) makes this function
incorrectly return `true`, and `total_duration()`/`sample_timing()`/`ts_sample()` then
unconditionally index `self.sample_duration[0]` on the empty `Vec`, panicking.

Reading a moof/traf/trun into a `MoofSegment` happens on the ordinary decode path for any
fragmented MP4/CMAF file (`MoofSegment::new` is called from
`IsoMp4Reader::try_read_more_segments`, itself called from the public `FormatReader::next_packet`
and `seek` methods) - no adversarial intent or malformed container structure is required, only an
ordinary encoder-produced `first_sample_flags` value with that one bit set.

## Introduced by

d797bbed0433455894ab97cbf37a03f2b42be258 "isomp4: Improve fragmented stream support." (2022-05-28)

`git log -S "is_first_sample_duration_present"` on `trun.rs` shows this is the single commit that
introduced `is_first_sample_duration_present()` (and the sibling
`is_first_sample_size_present()`, `total_duration()`, `sample_timing()`, `ts_sample()`) from
scratch, already with the `FIRST_SAMPLE_FLAGS_PRESENT` bitmask - i.e. this is not a regression
from a later edit, the wrong constant has been present since the feature's introduction.

## The fix

Changed `is_first_sample_duration_present()` to test `SAMPLE_DURATION_PRESENT` instead of
`FIRST_SAMPLE_FLAGS_PRESENT` (aligning it with the naming/intent of the analogous
`is_first_sample_size_present()`/`SAMPLE_SIZE_PRESENT`), **and** added an explicit
`!self.sample_duration.is_empty()` guard. The second part is not cosmetic: as established above,
`self.sample_duration` is unconditionally empty in every context where this function's result is
consulted, so testing a *different* bit of the same unrelated `first_sample_flags` word (per the
root-cause analysis, `SAMPLE_DURATION_PRESENT`'s bit `0x100` also falls within
`sample_degradation_priority`, and is just as "ordinary" a bit as `0x4`) would only change which
specific encoder-emitted bit pattern re-triggers the identical out-of-bounds panic, not eliminate
it. The explicit length guard makes the function safe regardless of which bit of
`first_sample_flags` is set, closing the panic for good rather than just relocating its trigger
condition.

```rust
pub fn is_first_sample_duration_present(&self) -> bool {
    match self.first_sample_flags {
        Some(flags) => {
            flags & TrunAtom::SAMPLE_DURATION_PRESENT != 0 && !self.sample_duration.is_empty()
        }
        None => false,
    }
}
```

Verified by re-running `repro/` against the patched tree: all 3 runs against
`evil_first_sample_flags.mp4` now complete with `next_packet() => Ok(Packet { len: 571 })` and no
panic (see `output.txt`, "after" section), identical in outcome to the real unmodified baseline
file.

- Project test suite (`cargo test --workspace`, debug build): **106 passed, 0 failed** before the
  fix, **106 passed, 0 failed** after the fix (identical pass count and set - no regressions,
  `symphonia-format-isomp4` itself has 0 unit tests in this package; the 106 come from other
  crates in the workspace).

## What's not done / open questions

- **`is_first_sample_size_present()` (`trun.rs:64-69`) has the exact same latent defect, not just
  a superficially-similar one.** It tests `first_sample_flags & SAMPLE_SIZE_PRESENT` (`0x200`) -
  per this candidate's root-cause analysis, bit `0x200` also falls within the sample-flags word's
  `sample_degradation_priority` sub-field, exactly as arbitrary/encoder-controlled as bit `0x4`
  was for the duration case. `self.sample_size` is, by the identical argument, unconditionally
  empty in every context where `is_first_sample_size_present()`'s result is consulted (populated
  only when the trun's own `SAMPLE_SIZE_PRESENT` flag is set, which is mutually exclusive with
  reaching that branch). This candidate's description called it "the correctly-written parallel",
  but it is not correct in the sense of being safe - it merely requires a different, presumably
  rarer, bit of `first_sample_flags` to be set to trigger the identical
  `self.sample_size[0]`-on-empty-`Vec` panic in `total_size()`/`sample_size()`/`sample_offset()`.
  This was noticed but not separately confirmed with its own end-to-end repro this session -
  recorded as a new candidate-shaped suspect rather than folded into this patch (one bug per
  candidate).
- Only the fragmented-MP4 (`moof`/`traf`/`trun`) path was checked, consistent with candidate 017's
  own scope note; the non-fragmented `stts`-based timing path is unrelated to this code and not
  re-examined here.
