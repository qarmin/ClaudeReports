# MP3_ANTIALIAS_CLAMPS_RZERO_MIXED_BLOCK

`symphonia-bundle-mp3/src/layer3/hybrid_synthesis.rs:240` writes
`channel.rzero = 18 * sb_limit.min(sb_rzero + 2).min(32);`. The surrounding comment says `rzero`
must be *extended* because anti-aliasing smears energy into the first zeroed sub-band - but for
`BlockType::Short { is_mixed: true }` the local `sb_limit` is `2` (line 225: a mixed block only has
2 long sub-bands to anti-alias), so the expression collapses to a constant `36` and *reduces*
`rzero`, which for a mixed block can legitimately be up to 576. Flagged from code reading, matching
`HEURISTICS.md`'s "a state-machine invariant that isn't checked, only assumed" pattern: `rzero` is a
"how much of this granule is non-zero" invariant that three consecutive stages
(`reorder` -> `antialias` -> `hybrid_synthesis`) each read and rewrite without any of them
validating it.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading of the `reorder` -> `antialias` -> `hybrid_synthesis` chain plus `requantize`/`bitstream` | commit ee35874 | Confirmed by hand that for `is_mixed: true` the expression is a constant 36 regardless of the real `rzero`, that `hybrid_synthesis` then derives `sb_limit = 2` (line 287), leaving its short-block IMDCT loop `2..2` empty (line 335), and that the tail loop `for sb in sb_limit..32` (line 351) overwrites sub-bands 2..32 with zeroed overlap. Gave the exact bitstream fields needed to trigger it. |
| Hand-built MPEG-1 Layer III bitstream + self-contained differential, decoded through the public API | release build, `symphonia` crate with `mp3` feature, 8-frame mono 44.1 kHz 320 kbps streams | Reproduced. Two .mp3 files that differ *only* in whether spectral coefficients 36..576 are coded decode to bit-identical PCM (`max |A - B| = 0.000000000`). No encoder in common use emits `mixed_block_flag = 1` (LAME never does), so the bitstream had to be written bit by bit - see `repro/src/main.rs`. |
| Differential against ffmpeg (the reference decoder `symphonia-check` itself uses for MP3) | ffmpeg 8.0.1-3ubuntu2, `-f f32le`, same generated .mp3 files | Reproduced, decisively. Symphonia's decoding of the *full-spectrum* mixed file is off from ffmpeg's by `max delta 0.524741888`; Symphonia's decoding of that file matches ffmpeg's decoding of the *low-bands-only* file instead (rms 0.020005477 vs 0.020005486). A 36-case sweep over 3 sample rates x 3 block types x 4 `rzero` positions mismatches in exactly the 9 mixed-block cases with `rzero > 36`, and agrees to <= 8.1e-7 in all 27 non-mixed cases. |
| `cargo test --workspace --all-features` | release+debug, before and after the patch | 106 passed / 0 failed both times - the existing suite does not cover this path at all. |

Not tried, and why: `miri`/ASAN/UBSAN (this is a pure-safe-Rust arithmetic/logic bug in a bounded
`[f32; 576]`, there is no UB to catch - see the workspace `README.md`); the existing
`symphonia/fuzz` `decode_mpa` target (a fuzzer has no oracle for "the output is quietly
low-passed", and reaching `mixed_block_flag = 1` from an empty corpus is far less direct than
constructing the frame).

## Status

**Confirmed** - 5/5 runs of `repro/` against the pristine tree, release build, no randomness
anywhere in the path (all three streams and the comparison are fully deterministic; the printed
`max |A - B| = 0.000000000` was bit-identical on every run). `just test` in this folder runs the
whole before/apply/after cycle and was verified to exit 0.

**Fully reproducible**: **Yes**. `repro/` writes real .mp3 files and decodes them through
Symphonia's real public path - `symphonia::default::get_probe()` -> `MpaReader` -> `MpaDecoder` -
which is exactly what any caller decoding an .mp3 does. Per `CONFIRMATION.md` section 11 there is
no second participant here to stand in for: an .mp3 file handed to the real public probe/decode
entry point *is* the genuine external input. The oracle is a second, independent decoder (ffmpeg)
run on the very same files, not an assumption about what the output should be.

## Class

logic

## Root cause

`rzero` is the index of the first sample of the all-zero ("rzero") partition of a granule, i.e. the
non-zero extent of the 576-sample spectral buffer. Three stages in a row read and rewrite it:

1. `reorder()` de-interleaves the short sub-bands and extends `rzero` to the end of the last
   reordered band (line 213).
2. `antialias()` applies the alias-reduction butterflies and is supposed to extend `rzero` by one
   sub-band, because a butterfly at the boundary of the last non-zero sub-band smears energy into
   the first zeroed one.
3. `hybrid_synthesis()` derives `sb_limit = channel.rzero.div_ceil(18)` and only runs the IMDCT
   over sub-bands below it; sub-bands `sb_limit..32` are overwritten with the (zeroed) overlap
   buffer.

Step 2 does the extension with the wrong bound. `sb_limit` in `antialias()` is the *number of
sub-bands this block type may anti-alias* - 2 for a mixed block, since only the block's 2 long
sub-bands get alias reduction, and 32 otherwise. Reusing that same bound as the ceiling for the
`rzero` extension turns the statement into an unconditional assignment for a mixed block:
`18 * 2.min(sb_rzero + 2).min(32)` is `36` for every `sb_rzero >= 0`. So a mixed granule whose real
`rzero` was, say, 576 leaves `antialias()` claiming 36.

Step 3 then believes it: `sb_limit = 36.div_ceil(18) = 2`, `sb_split = 2` for a mixed block, so the
long-block loop runs `0..2` (correct), the short-block loop runs `sb_split.min(sb_limit)..sb_limit`
= `2..2` (empty - the 30 short sub-bands never get their 12-point IMDCT), and the tail loop
`for sb in sb_limit..32` copies the always-zero overlap over sub-bands 2..32 and zeroes it again.
Every mixed block therefore loses all 30 short sub-bands: a hard low-pass at 2/32 of the Nyquist
rate (1378 Hz at 44.1 kHz, 1500 Hz at 48 kHz, 1000 Hz at 32 kHz).

For non-mixed block types nothing is wrong: with `sb_limit == 32`,
`18 * min(32, sb_rzero + 2)` is always `>= channel.rzero`, so the assignment only ever extends and
the old and new code produce identical values (empirically confirmed - the 27 non-mixed sweep cases
have byte-identical deltas before and after the patch).

The trigger is three plain side_info bitstream fields with no validation anywhere:
`window_switching_flag = 1`, `block_type = 0b10`, `mixed_block_flag = 1`
(`layer3/bitstream.rs:76-90`), plus enough coded coefficients that `rzero > 36`. Real-world impact
is limited by encoder behaviour rather than by anything in the decoder: LAME and most other
encoders never set `mixed_block_flag`, so this mostly affects streams from encoders that do, and
any conformance/ISO test vector that exercises mixed blocks.

## Introduced by

f89f2b8 "mpa: Aggressively avoid processing zeroed sub-bands." (2023-01-31), first released in
v0.5.4. `git log -L 218,245:symphonia-bundle-mp3/src/layer3/hybrid_synthesis.rs` gives three
commits for this range; the other two are a bulk `rustfmt` run (8ef55b0) and the original file
split (a8fe0d0). Before f89f2b8, `antialias()` took `&GranuleChannel` (immutable), never touched
`rzero` at all, and used a local `sb_end` of `2 * 18` for mixed blocks; `hybrid_synthesis()` had no
`sb_limit` and always ran all 32 sub-bands. That commit introduced both the `rzero` write and the
`sb_limit`-driven loops that consume it.

## The fix

`patch.diff` (+9/-5, one function) splits the two things the single statement was conflating:

- `aa_end` - how far the anti-aliasing butterfly loop may run. Same expression as before
  (`18 * sb_limit.min(sb_rzero + 2).min(32)`), so the filter itself still touches exactly the
  sub-bands it did before, and still only the 2 long sub-bands of a mixed block.
- `channel.rzero` - now `channel.rzero.max(aa_end)`, so it can only ever be *extended* (which is
  what the comment always said the intent was), never truncated.

The butterfly loop's bound was changed from `channel.rzero` to `aa_end` so that raising `rzero`
does not widen the filter.

Verification, per `CONFIRMATION.md` sections 6 and 7 - the fix was checked against more than the
one input that first triggered it:

- `repro/`'s primary differential: `max |A - B|` goes from `0.000000000` (before) to `0.524741888`
  (after), i.e. the coefficients above sub-band 2 now reach the output.
- ffmpeg cross-check on the same three files: `max |symphonia(A) - ffmpeg(A)|` goes from
  `0.524741888` to `0.000000261`, inside `symphonia-check`'s own compliant-decoder tolerance of
  `0.00001`.
- 36-case sweep (44100/48000/32000 Hz x long/short/short-mixed x 36/80/180/576 coded
  coefficients, i.e. four different `rzero` positions including one below 36): 9 of 36 cases
  exceeded the tolerance before, 0 of 36 after. The 27 non-mixed cases printed byte-identical
  deltas before and after, confirming the patch is a no-op for every other block type. The
  `rzero = 36` mixed case is also unchanged (`0.000000060` both times), which is the boundary case
  where the truncation happened to be harmless.
- Project test suite: 106 passed / 0 failed before -> 106 passed / 0 failed after
  (`cargo test --workspace --all-features`), no new failures, identical `test result:` lines. The
  suite has no coverage of layer 3 hybrid synthesis, so this is a "did not break anything" result
  rather than positive evidence.

## What's not done / open questions

- **A second, independent bug in the same feature was found while building this repro and is NOT
  fixed here** (one bug per candidate folder). `layer3/requantize.rs:374-375` requantizes a mixed
  block as `requantize_long(channel, &bands[..switch], buf)` +
  `requantize_short(channel, &bands[switch..], switch, buf)` where
  `switch = SFB_MIXED_SWITCH_POINT[sr] == 8` for MPEG-1. `SFB_MIXED_BANDS[0][..8]` is
  `[0, 4, 8, 12, 16, 20, 24, 30]`, i.e. 7 intervals ending at coefficient 30 - so the long part's
  last scale-factor band `[30, 36)` is requantized by neither call, and coefficients 30..35 of
  every mixed block keep their raw `s(i)^(4/3)` value with `2^(0.25*A) * 2^(-B)` never applied.
  Observed directly: with `global_gain` lowered by 40 (a factor of 2^-10) the decoded rms of a
  mixed granule did not move at all, because those 6 unscaled coefficients dominated everything
  else. The long slice most likely wants to be `&bands[..=switch]` (9 boundaries, 8 bands, ending
  at 36 where the short part starts). `repro/src/main.rs` deliberately codes coefficients 28..36 as
  zero to keep this candidate clean of it. Worth its own candidate.
- Only MPEG-1 was exercised (44100/48000/32000 Hz). MPEG-2 and MPEG-2.5 take the same
  `antialias()`/`hybrid_synthesis()` code path with the same `sb_limit = 2` for mixed blocks, so
  the bug should be identical there, but that was not measured. MPEG-2.5 at 8 kHz additionally has
  a `TODO` at `bitstream.rs:134` about `region1_start` being wrong, which would confound such a
  test.
- Only mono was exercised. The joint-stereo path (`layer3/stereo.rs`) also rewrites both channels'
  `rzero` (lines 521, 551-552) before `antialias()` runs; whether mid/side or intensity stereo
  interacts with this in some further way was not checked.
- `reorder()`'s own `channel.rzero = channel.rzero.max(i)` (line 213) rounds `rzero` up to a short
  scale-factor-band boundary. That is deliberate and unrelated, but it means `rzero` is written by
  three different stages with three different notions of what it bounds - a maintainer may prefer
  to give the two uses separate fields rather than keep overloading one.
- A longer fuzzing campaign would not have found this (a fuzzer has no oracle for "quietly
  low-passed output"); a differential harness that runs `symphonia-check --ref ffmpeg` over
  generated mixed-block streams would, and would be the right regression test to add.
