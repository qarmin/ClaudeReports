# H264_CODECPROFILE_CONSTRAINED_INTRA_CONSTANTS_UNREACHABLE

`symphonia-core/src/codecs/video.rs` defines six H.264 `CodecProfile` constants
(`CODEC_PROFILE_H264_CONSTRAINED_BASELINE`, `_PROGRESSIVE_HIGH`, `_CONSTRAINED_HIGH`,
`_HIGH_10_INTRA`, `_HIGH_422_INTRA`, `_HIGH_444_INTRA`) that OR one or more `constraint_set{n}_flag`
bits into bits 8+ on top of the plain `profile_idc` (e.g. `CodecProfile(66 | 1 << (8 + 1))`). The
only real H.264 profile parser, `AVCDecoderConfigurationRecord::read`
(`symphonia-common/src/mpeg/video/mod.rs`), reads `avc_profile_indication` and explicitly discards
the next byte (`_profile_compatibility`, which carries exactly those constraint_set flags) without
folding any bits into the constructed `CodecProfile`. `CodecProfile` derives plain
`PartialEq`/`Eq`/`Ord`/`Hash` on the raw `u32` (`symphonia-core/src/codecs/mod.rs`), so this looked
like a structural, code-reading-detectable mismatch worth verifying end-to-end.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: compare `CodecProfile` constant definitions in `video.rs` against every real call site of `AVCDecoderConfigurationRecord::read`/`HEVCDecoderConfigurationRecord::read` | Grepped whole repo (excluding `___OPTIMIZATIONS___`/`___BUGHUNT___`) for the 6 constant names and for `AVCDecoderConfigurationRecord::read` call sites | Confirmed only 2 real callers exist (`symphonia-format-isomp4/src/atoms/avcc.rs`, `symphonia-format-mkv/src/codecs.rs`), both via the shared `symphonia-common` parser; neither constant is currently matched against anywhere in-tree, but the parser's discard of `_profile_compatibility` is unconditional and structural, not input-dependent - HEVC's own profile constants (`CODEC_PROFILE_HEVC_MAIN`/`_MAIN_10`/`_MAIN_STILL_PICTURE`) are plain small values with no equivalent constraint-bit encoding, so this specific bug class doesn't recur there |
| Standalone repro (`repro/`) against a debug-assertions build, using real ISO/IEC 14496-15 `AVCDecoderConfigurationRecord` bytes (`[0x01, 0x42, 0x40, 0x1E]` - `profile_idc=66`, `constraint_set1_flag=1`, `level=30`, the exact byte layout a real H.264 encoder writes for a Constrained Baseline Profile stream) fed to the crate's real public `AVCDecoderConfigurationRecord::read` entry point | `cargo run --release`, 3 repeat runs, deterministic (no fuzzing needed - the bug is a pure, input-independent code omission) | Reproduced 3/3: parsed profile is always `CodecProfile(66)`, never equal to `CODEC_PROFILE_H264_CONSTRAINED_BASELINE` (`CodecProfile(578)`) |

Manual boundary-value/code-reading was sufficient on the first approach; a debug-assertions repro
run against the real public parser was used to confirm the finding empirically rather than from
static reading alone (2 approaches total, well under the soft cap of 3 - no further escalation was
needed since the mismatch is deterministic and structural, not something a sanitizer/miri/fuzzer
would add signal for).

## Status

**Confirmed**, deterministic, reproduces 3/3 identical runs (release build, no fixed seed needed -
the bug is not flaky).

**Fully reproducible**: **Yes**. The repro calls the crate's real public API,
`symphonia_common::mpeg::video::AVCDecoderConfigurationRecord::read` - the exact function
`symphonia-format-isomp4`'s real `avcc.rs` atom reader calls when demuxing a real mp4 file's `avcC`
box - with a byte sequence that is the genuine ISO/IEC 14496-15 `AVCDecoderConfigurationRecord`
layout a real H.264 encoder (e.g. x264's "Constrained Baseline" preset) writes to disk. This is a
single external artifact (an `avcC` payload) handed to a real public parsing entry point, not a
hand-faked second participant - per `CONFIRMATION.md` section 11, no second participant needs to be
simulated here since there is no peer/caller relationship to fake.

## Class

logic

## Root cause

`AVCDecoderConfigurationRecord::read` reads `avc_profile_indication` (the bare `profile_idc`,
1 byte) and then reads and discards the next byte into `_profile_compatibility` (the
`constraint_set0_flag` .. `constraint_set5_flag` bits, per ISO/IEC 14496-10 Annex A), constructing
`CodecProfile::new(avc_profile_indication)` with no reference to the discarded byte. Meanwhile,
`symphonia-core/src/codecs/video.rs`'s well-known H.264 profile constants encode
"profile_idc + specific constraint_set flags" pairs by OR-ing `1 << (8 + n)` for each relevant
constraint set `n` into the base `profile_idc` (e.g.
`CODEC_PROFILE_H264_CONSTRAINED_BASELINE = CodecProfile(66 | 1 << (8 + 1))` for `profile_idc=66`
with `constraint_set1_flag=1`). Since `CodecProfile` derives plain `PartialEq`/`Eq` on the raw
`u32`, and the real parser never sets any bit at position 8 or above, `record.profile` for a real
Constrained Baseline/Progressive High/Constrained High/`*_Intra` stream is always the bare
`profile_idc` (66/100/110/122/244) and can never equal the corresponding "constrained"/"intra"
constant - a permanent false negative for any downstream code (in this crate or a consumer of the
public API) that matches on these 6 constants.

## Introduced by

0c9f1f7 "isomp4: Basic video and subtitle track detection." (2024-09-07). This commit added the
original `AvcCAtom::read` (later moved verbatim into `symphonia-common` by 704c08c "common: Move
HEVC/AVCDecoderConfigurationRecord to common. (#319)"), which already read
`_profile_compatibility` into a discarded, unused binding. It lands (same day, ~5 hours later) on
top of 7ea26b7 "core (codecs): Rewrite and refactor codecs module." (2024-09-07 14:58:13), which is
the commit that introduced the 6 constraint-bit-encoded `CODEC_PROFILE_H264_*` constants in the
first place (verified: `git merge-base --is-ancestor 7ea26b7 0c9f1f7` succeeds, i.e. 7ea26b7 is an
ancestor of 0c9f1f7). So the disconnect existed from the moment the real parser was first written -
the constants already expected constraint-set bits that the parser being added in the same session
never wired up.

## The fix

Changed `AVCDecoderConfigurationRecord::read` (`symphonia-common/src/mpeg/video/mod.rs`) to keep
`profile_compatibility` instead of discarding it, and fold each `constraint_set{n}_flag` bit
(byte bit position `7-n`, per ISO/IEC 14496-10 Annex A, for `n` in `0..=5`) into bit `8+n` of the
constructed profile code - exactly the encoding the existing `CODEC_PROFILE_H264_*` constants
already use. This is a general mapping (all 6 constraint sets, not just the one exercised by the
repro's Constrained Baseline input), so it also covers Progressive High (constraint_set4),
Constrained High (constraint_set4+5), and the three `*_INTRA` profiles (constraint_set3) without
special-casing any one of them - checked by re-deriving each constant's expected bit pattern by
hand and confirming the loop produces it for the corresponding constraint_set flag combination.
Streams with no constraint_set flags set (the common case: Baseline/Main/Extended/High/etc. with
`profile_compatibility=0`) are unaffected - `profile_code` reduces to the bare `profile_idc`,
identical to the old behavior.

Re-running `repro/` against the patched tree: `record.profile` is now `CodecProfile(578)`, equal to
`CODEC_PROFILE_H264_CONSTRAINED_BASELINE` (`CodecProfile(578)`). See `output.txt` for the full
before/after capture.

- Project test suite (`cargo test --workspace`, pristine checkout at commit
  ee35874b571a35a9a6e15d3bc9a3aaf8f11fbeee): 106 passed; 0 failed; 0 ignored, before and after the
  fix (identical per-crate results both times - see `output.txt`).

## What's not done / open questions

- No downstream code in this repository currently matches on any of the 6 affected constants (only
  their definitions exist in-tree, per the grep in "What was tried"), so this bug has no currently
  observable effect on any shipped Symphonia demuxer/decoder's own behavior - the impact is on any
  external consumer of the public `symphonia_core`/`symphonia_common` API that compares a parsed
  H.264 `CodecProfile` against these constants (a documented, intended use per the constants'
  existence and naming).
- `HEVCDecoderConfigurationRecord::read` in the same file also discards
  `_general_profile_compatibility_flags`/`_general_constraint_indicator_flags`, but this is not the
  same bug class: `CODEC_PROFILE_HEVC_MAIN`/`_MAIN_10`/`_MAIN_STILL_PICTURE` are plain small values
  (1/2/3) with no equivalent higher-bit encoding for HEVC's own constraint flags, so there is no
  corresponding constant these discarded HEVC bits would ever need to satisfy - not investigated
  further as its own candidate since there's nothing to reproduce a mismatch against.
- Not checked: whether any out-of-tree consumer of `symphonia_core`/`symphonia_common` already
  depends on the old (unfixed) behavior in a way this fix could regress - the fix only changes
  `CodecProfile`'s value for streams that actually set a `constraint_set{n}_flag` bit in their
  `avcC`/`avc1` box, which the crate never correctly reported the identity of before this fix.
