# musepack_sv7_gapless_bitrate_divzero

`lofty/src/musepack/sv7/properties.rs` (~line 374-384), `MpcSv7Properties::read`: flagged from
`IDEAS.md` as the sibling of `006.musepack_sv4to6_bitrate_divzero/` - SV7 and SV8 use different
header layouts than SV4-6 and weren't checked for an analogous divide-by-zero/degenerate-
denominator pattern in that candidate.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value/arithmetic proof: traced every path from the untrusted on-disk header bytes to the `total_samples`/`average_bitrate` division at lines 374-384, and read the sibling SV8 reader (`lofty/src/musepack/sv8/properties.rs`) for comparison | static read of `MpcSv7Properties::read` (lines 265-387) and `sv8::read` (lines 251-301) | SV7's `true_gapless` branch computes `total_samples = (frame_count as u64 * MPC_FRAME_LENGTH) - (MPC_FRAME_LENGTH - last_frame_length as u64)`, i.e. `(frame_count - 1) * 1152 + last_frame_length`. This is exactly `0` whenever `frame_count == 1` and `last_frame_length == 0` - both directly attacker-controlled (`frame_count` is a raw `u32` header field; `last_frame_length` is only constrained to `<= MPC_FRAME_LENGTH` (1152) by an earlier check at line 356, and `0` satisfies that). `average_bitrate` is then computed as `(stream_length * 8 * sample_freq) / (total_samples * 1000)` - **integer** `u64` division, not the saturating `f64` division the ruled-out SV4-6 candidate used, so this is an unconditional "attempt to divide by zero" panic (fires in both debug and release, unlike checked-arithmetic overflow). By contrast, SV8's `read()` (line 285-292) explicitly guards this exact case (`total_samples = sample_count - beginning_silence; if total_samples == 0 { ...; return Ok(properties); }`) before its own analogous division at line 297-298 - SV8 is not vulnerable to this pattern. |
| Real end-to-end confirmation via the crate's public API: a standalone `cargo run` binary (path dependency on the local `lofty` checkout) built a minimal 89-byte SV7 (`"MP+"` + version byte) Musepack stream (no ID3v2/ID3v1/APE/Lyrics3v2 tags) with `frame_count=1`, `sample_freq_index=0` (44100 Hz, nonzero), and section-5 chunk `0x8000_0000` (`true_gapless=1`, `last_frame_length=0`), then called `lofty::read_from_path` (the real public entry point) on it | `cargo run` (dev, `overflow-checks`/`debug-assertions` on) and `cargo run --release`, repeated 4 times total (3 dev + 1 release) | All 4/4 runs panicked identically: `thread 'main' panicked at .../sv7/properties.rs:383:32: attempt to divide by zero`. Deterministic, not flaky - reachable from `lofty::read_from_path`/`Probe::read`, the same entry point any caller opening an untrusted `.mpc` file uses. |

## Status

**Confirmed.** Reproduces 4/4 runs (3 debug-profile, 1 release-profile), 100% deterministic - this
is an unconditional Rust integer divide-by-zero, not a debug-only checked-arithmetic panic, so it
is not build-profile-dependent.

**Fully reproducible**: Yes. `repro/` calls `lofty::read_from_path` (the real public API entry
point, the same one `lofty::probe::Probe::read`/CLI-style callers use) on a crafted `.mpc` file
written to disk - this is the "malformed file handed to a parser's real public entry point" case
from `CONFIRMATION.md` section 11: the file bytes on disk are the actual external input a caller
would hand the parser, there is no second participant (peer/caller) to fake here.

## Class

crash

## Root cause

`MpcSv7Properties::read`'s `true_gapless` branch computes
`total_samples = (frame_count as u64 * MPC_FRAME_LENGTH) - (MPC_FRAME_LENGTH - last_frame_length as u64)`,
algebraically `(frame_count - 1) * 1152 + last_frame_length`. The code already guards
`frame_count == 0` (line 365-368) and `last_frame_length > MPC_FRAME_LENGTH` (line 356-358), but
nothing stops the specific combination `frame_count == 1 && last_frame_length == 0` (both valid,
independently-checked values) from making `total_samples` exactly `0`. The very next statement,
`average_bitrate = (stream_length * 8 * sample_freq) / (total_samples * 1000)`, is then a genuine
`u64` integer division, which panics unconditionally on a zero divisor in Rust - unlike the
`saturating_sub`-then-`f64`-division pattern in `006`'s SV4-6 candidate (which never panics, just
saturates to `u32::MAX`), or SV8's `read()` in the same file family, which explicitly checks
`total_samples == 0` before its own division and early-returns instead. SV7 is missing that same
guard for its own (differently-shaped) `total_samples` computation.

## The fix

Added the same defensive guard SV8 already has, immediately after `total_samples` is computed and
before it's used as a divisor:

```rust
if total_samples == 0 {
    log::warn!("Total samples is 0, unable to calculate bitrate");
    return Ok(properties);
}
```

This mirrors `sv8::read()`'s existing `if total_samples == 0 { ...; return Ok(properties); }`
early return (same crate, same file family, already an accepted pattern there). Verified against
more than the single triggering input's exact byte sequence per `CONFIRMATION.md` section 7: the
non-`true_gapless` branch's own `total_samples` computation (`frame_count * 1152 - 481`) was
independently checked and can't reach 0 for any `frame_count >= 1` (the only values that survive
the earlier `frame_count == 0` guard) - its minimum is `671` at `frame_count == 1` - so the new
guard is defense-in-depth for that branch and only ever actually fires for the `true_gapless`
`frame_count == 1, last_frame_length == 0` case (and any other input that happens to make
`(frame_count - 1) * 1152 + last_frame_length == 0`, which is the same condition).

Before/after with `repro/` (see `output.txt` for full captured output):
- Before (pristine): `thread 'main' panicked at .../sv7/properties.rs:383:32: attempt to divide by zero`
- After (patched): `RESULT: no panic. properties = duration=26ms audio_bitrate=Some(0)` (a
  degenerate-but-safe `0` bitrate, the same graceful-degrade style already used for the
  `sample_freq == 0`/`frame_count == 0` guards two lines above)

- Project test suite (`cargo test -p lofty --lib --tests`, main checkout, summed across all 7
  test binaries): before `646 passed; 0 failed; 73 ignored` -> after `646 passed; 0 failed; 73
  ignored`. No new failures, no count change.

## What's not done / open questions

- The non-`true_gapless` branch's `total_samples = frame_count * 1152 - MPC_DECODER_SYNTH_DELAY(481)`
  was proven safe by arithmetic (minimum `671` at `frame_count == 1`, the smallest value that
  survives the earlier `frame_count == 0` guard) rather than independently fuzzed - the new guard
  covers it anyway as defense-in-depth, but this session didn't try to construct an alternate
  crash in that specific branch since the proof already rules it out.
- SV8 (`lofty/src/musepack/sv8/properties.rs`) was read as a comparison reference in this same
  session and confirmed to already guard this exact pattern (`total_samples == 0` checked before
  its own division) - it does not need the same fix and was not re-investigated as its own
  candidate.
- This candidate's `IDEAS.md` entry named both SV7 and SV8 as unchecked; both are now checked (SV7
  confirmed vulnerable and fixed here, SV8 confirmed already safe) - no follow-up item added for
  SV8.
