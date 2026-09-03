# RIFF_WAV_FMT_BLOCK_ALIGN_U16_MUL_OVERFLOW

`WaveFormatChunk::read_pcm_fmt` (`symphonia-format-riff/src/wave/chunks.rs:100`) multiplies two
raw, still-unvalidated `fmt ` chunk fields in `u16` arithmetic
(`num_channels * (bits_per_sample / 8)`) purely to log whether the file's declared `nBlockAlign`
looks right. Flagged by the `HEURISTICS.md` pattern "arithmetic on untrusted sizes in a parser,
performed before the value is range-checked": `num_channels` is read verbatim from the file and the
only channel-count sanity check (`map_wave_channel_count`, which rejects anything above 26) runs on
the line *after* the multiplication.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual code read of `symphonia-format-riff` | wave/chunks.rs `parse` -> `read_pcm_fmt`, checking where each `fmt` field is validated relative to where it's used | Found the multiplication at line 100 happens before `map_wave_channel_count` at line 106; multiplier from `bits_per_sample` is 1..4, so a `u16` product overflows at `nChannels >= 65536 / (bits/8)` |
| Manual boundary-value construction + overflow-checks build | standalone `repro/`, `cargo run --release` with `overflow-checks = true`, 7 hand-built WAV files fed through `symphonia::default::get_probe().probe()` on real files on disk | Panics `attempt to multiply with overflow` at wave/chunks.rs:100:36 for exactly the 3 predicted boundary cases (16384ch/32-bit, 21846ch/24-bit, 32768ch/16-bit); the 3 just-below-threshold cases (16383ch/32-bit, 21845ch/24-bit) and 2 normal files do not panic |
| Same repro without overflow checks | `CARGO_PROFILE_RELEASE_OVERFLOW_CHECKS=false cargo run --release`, same 7 files | No panic: product wraps silently, `expected_block_align` becomes a wrong value that is only fed to an `info!` log, and the file is then rejected by `map_wave_channel_count` anyway ("riff: invalid channel count") |
| Same-class sweep of neighbouring code | grep for other `num_channels`/`n_channels` arithmetic in `symphonia-format-riff` (wave + aiff) | No other pre-validation `u16` products found; the AIFF `CommonChunk` path computes no expected block align, and the other WAVE format arms (`read_adpcm_fmt`, `read_ieee_fmt`, `read_ext_fmt`, a-law/mu-law) call `map_wave_channel_count` without multiplying first |

## Status

**Confirmed**

Reproduces 4/4 runs (`cargo run --release` with `overflow-checks = true`), deterministically, at
the same source location (`wave/chunks.rs:100:36`) for all three boundary inputs each run - see
`output.txt` (run 1 with a full backtrace, runs 2 and 3 as exit-status checks). Only manifests in a
build with overflow checks enabled (any debug build, or a release build with
`overflow-checks = true`); in a plain release build the product wraps silently and the only visible
effect is a wrong number in an `info!` log line - stated here as a fact, not as a reason to discard
it (`CONFIRMATION.md` section 2).

**Fully reproducible**: **Yes**. `repro/` writes real `.wav` files to disk and opens each one
through the actual public entry point an application uses -
`symphonia::default::get_probe().probe(&hint, MediaSourceStream::new(File...), ...)` - not by
calling `read_pcm_fmt`/`WavReader::try_new` directly. There is no second participant to fake here:
the input is a single external artifact (a malformed media file), exactly what a real user/attacker
would hand the library (`CONFIRMATION.md` section 11's explicitly-fine single-participant case).

## Class

crash

## Root cause

`ParseChunk::parse` for `WaveFormatChunk` reads `nChannels` and `wBitsPerSample` verbatim out of the
`fmt ` chunk (wave/chunks.rs:421 and :425) and passes them straight into `read_pcm_fmt`.
`read_pcm_fmt` validates `bits_per_sample` (must be 8/16/24/32) but not `num_channels`, then
computes

```rust
let expected_block_align = num_channels * (bits_per_sample / 8);
```

in `u16`, solely so the next statement can `info!` a warning when the file's declared `nBlockAlign`
disagrees. With the multiplier bounded to 1..4 by the preceding `bits_per_sample` match, the product
leaves `u16` range as soon as `num_channels >= 65536 / (bits_per_sample / 8)`:

| `wBitsPerSample` | multiplier | smallest overflowing `nChannels` |
|---|---|---|
| 16 | 2 | 32768 |
| 24 | 3 | 21846 |
| 32 | 4 | 16384 |

(8-bit cannot overflow: the multiplier is 1.)

The only thing that would bound `num_channels` is `map_wave_channel_count(num_channels)?` - which
rejects everything above 26 - but it is called on the *next* line but one, after the overflowing
multiplication has already happened. So every one of these inputs is a file the parser is about to
reject anyway; it just panics on the way to rejecting it. Reached from `WavReader::try_new`, i.e.
from any `probe()`/`format_reader` call on an untrusted `.wav`.

## Introduced by

7c01671 "riff (wave): Don't use block align if is not required." (2026-04-15) - that commit split
the monolithic `fmt` handling into per-format `read_*_fmt` helpers and added both the
`expected_block_align` computation and its `info!` log in `read_pcm_fmt` (found with
`git log -S "num_channels * (bits_per_sample / 8)"`, single hit). Before it, `block_align` was kept
on the chunk struct and consumed later, with no such product computed at parse time.

## The fix

`patch.diff` (1 file, +4/-3, including 2 lines of comment) widens the computation to `u32`:

```rust
let expected_block_align = u32::from(num_channels) * u32::from(bits_per_sample / 8);

if u32::from(block_align) != expected_block_align {
```

The value is never used for anything but the `info!` message, so widening it is behaviour-preserving
for every input that did not overflow, and for the ones that did it now logs the true expected block
align instead of a wrapped one. It cannot overflow for any input: the largest possible product is
`u16::MAX * 4 = 262140`, well inside `u32`. It is deliberately independent of statement ordering -
moving `map_wave_channel_count` above the log line would fix the same three inputs, but would leave
the next person free to reintroduce a `u16` product; widening removes the trap itself.

Checked against more than the one triggering input (`SKILL.md` pitfall #3): the repro covers all
three bit depths that can overflow, each at the exact threshold and (for 32- and 24-bit) one channel
below it, plus two ordinary files. Before: 3 of 7 files panic, exit 1. After: all 7 files are handled
without panicking (the five large-channel-count ones return
`Err(DecodeError("riff: invalid channel count"))`, the two normal ones probe successfully), exit 0.
See `output.txt` and `just test`.

- Project test suite (`cargo test --workspace`, in the candidate's own git worktree): 106 passed /
  0 failed before the patch -> 106 passed / 0 failed after. No new failures, no tests newly skipped.

## What's not done / open questions

- No regression test was added to the project's own test suite. `wave/chunks.rs` already has unit
  tests for `map_wave_channel_count`; a `read_pcm_fmt`-level test would need the chunk reader to be
  driven from a byte buffer, which the module currently has no test harness for. The maintainers may
  prefer to add one alongside this fix.
- The AIFF side of the same crate and the other WAVE format arms were grepped for the same class of
  pre-validation `u16` arithmetic and none was found (see the "What was tried" table), but that was
  a targeted grep of `symphonia-format-riff` only - other format crates were not swept for this
  pattern as part of this candidate.
- A maintainer might reasonably prefer to drop the `expected_block_align`/`info!` diagnostic
  entirely (it is purely informational for a field the PCM path ignores) rather than widen it; that
  is a design call, not something this patch decides.
- A longer fuzzing campaign pointed at `WavReader::try_new` (via `fuzz-bug-repro`) would likely be
  cheap and worthwhile - this bug is reachable within the first 24 bytes of a file, which suggests
  the `fmt`-chunk parsers have not been fuzzed hard.
