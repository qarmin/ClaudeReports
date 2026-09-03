# musepack_sv4to6_bitrate_divzero

`lofty/src/musepack/sv4to6/properties.rs` (~line 150), `MpcSv4to6Properties::read`: flagged from
`IDEAS.md` as a static-review suspect - `pcm_frames = (MPC_FRAME_LENGTH * frame_count).saturating_sub(576)`
looked like it could hit 0 for a small `frame_count` read from an untrusted stream header, and
`average_bitrate = (... / pcm_frames as f64 / ...) as u32` would then divide by `0.0`, producing
`+inf`, which Rust's saturating float-to-int cast turns into `u32::MAX` with no panic/UB.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value/arithmetic proof: traced every path that reaches the `pcm_frames` computation at line 153 | static read of `read()` (lines 90-161) | The function returns early at line 140-142 (`if properties.frame_count == 0 { return Ok(properties); }`) *before* line 153 is ever reached, and this early return happens strictly after the `stream_version < 6` adjustment at line 131-134 that can itself drive `frame_count` to 0 via `saturating_sub(1)`. So at line 153, `frame_count` (a `u32`) is always `>= 1`. `MPC_FRAME_LENGTH` is the constant `36 * 32 = 1152` (`lofty/src/musepack/constants.rs:17`). `pcm_frames = 1152 * frame_count - 576`, monotonically increasing in `frame_count`, with its minimum at `frame_count == 1` equal to `1152 - 576 = 576`. `u64` multiplication of `1152 * frame_count` for any `u32` `frame_count` (max ~4.9e12) can't overflow `u64` either, so `saturating_sub` never actually saturates on the low side. `pcm_frames == 0` is therefore mathematically unreachable through this code path; a divide-by-zero can't occur here regardless of what bytes an attacker puts in the header. |
| Real end-to-end confirmation via the crate's public API: a standalone `cargo run` binary (dependency on the local `lofty` checkout) built five minimal 32-byte SV4-6 Musepack streams (no ID3v2/ID3v1/APE/Lyrics3v2 tags) with hand-set header bits, and called `MpcFile::read_from` (the real `AudioFile` entry point used by `lofty::read_from_path`/`read_from`) with default `ParseOptions` (`ParsingMode::BestAttempt`), reading `MpcProperties::Sv4to6(_).average_bitrate()` back out | `cargo run`, dev profile (`overflow-checks`/`debug-assertions` on by default), covering: `stream_version=6, frame_count_field=1` (minimal nonzero directly), `stream_version=4, frame_count_field=2` (minimal nonzero after the `-1` version-6 adjustment), `stream_version=6, frame_count_field=0` and `stream_version=4, frame_count_field=1` (both drive `frame_count` to 0 and hit the early return), `stream_version=6, frame_count_field=2` | No panic in any case (dev profile would have panicked on any real overflow/underflow given `saturating_sub`'s use rules that out anyway, but this also rules out any other unexpected arithmetic panic in the surrounding code). The two frame_count==0 cases returned the early-return path's `average_bitrate` (`0`, from the header-decoded VBR/CBR field, not `u32::MAX`). The two minimal-nonzero cases (`frame_count == 1`) reached the `pcm_frames` computation and produced `average_bitrate == 17`, not `u32::MAX` or any other absurd value; `frame_count == 2` produced `average_bitrate == 5`. Confirms the mathematical proof above: `pcm_frames` never hits 0 through the real parsing entry point. |

## Status

**Not reproducible.** `frame_count == 0` is always intercepted by the early return at line 140-142
before the `pcm_frames`/`average_bitrate` computation at lines 153-159 runs, and for every
`frame_count >= 1` that does reach that computation, `pcm_frames = MPC_FRAME_LENGTH * frame_count -
576 >= 1152 - 576 = 576`, strictly greater than 0. Both a static arithmetic proof and a real
`MpcFile::read_from` run over hand-crafted boundary-value SV4-6 streams (the smallest possible
nonzero `frame_count` in both the `stream_version >= 6` and `< 6` branches) confirm this: no
`average_bitrate` value anywhere near `u32::MAX` was observed, and no panic occurred.

**Fully reproducible**: N/A (Not reproducible - no repro/ folder per the candidate loop, which only
requires one when Status is Confirmed).

## Class

N/A (not reproducible)

## Root cause

Not applicable in the "confirmed bug" sense. The suspect described in `IDEAS.md` assumed
`pcm_frames` could reach 0 for "small but nonzero `frame_count` values", but the code already
guards exactly that case: the `if properties.frame_count == 0 { return Ok(properties); }` check at
line 140-142 runs *after* the version-6 `-1` adjustment and *before* the division, so every
`frame_count` value that survives to the division site is `>= 1`, and `MPC_FRAME_LENGTH` (1152) is
comfortably larger than the `576` subtracted, so the minimum possible `pcm_frames` is `576`, never
`0`. The `saturating_sub(576)` in the suspect line is not masking an underflow in the reachable
input space - it's simply defensive/idiomatic and never actually triggers its saturating branch
for any input that reaches that line.

## The fix

No fix attempted - no reproducible incorrect behavior was found to fix.

- Project test suite: not run for this candidate (no source change was made).

## What's not done / open questions

- The nearby comment "Is this accurate? If not, it really doesn't matter." (line 155) suggests the
  `average_bitrate` estimate's general *precision* is an already-accepted tradeoff independent of
  this candidate; this session only checked the specific divide-by-zero/`u32::MAX`-saturation claim,
  not the broader accuracy of the bitrate estimate formula itself.
- The sibling SV7 (`lofty/src/musepack/sv7/properties.rs`) and SV8
  (`lofty/src/musepack/sv8/properties.rs`) property readers were not checked for an analogous
  divide-by-zero pattern in their own bitrate/duration math this session - they use different
  header layouts and weren't in scope for this specific `IDEAS.md` entry.
