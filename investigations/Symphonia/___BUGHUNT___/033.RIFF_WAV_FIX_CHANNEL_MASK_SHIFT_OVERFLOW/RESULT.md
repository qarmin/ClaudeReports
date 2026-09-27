# RIFF_WAV_FIX_CHANNEL_MASK_SHIFT_OVERFLOW

`fix_wave_channel_mask` (`symphonia-format-riff/src/wave/chunks.rs:689-690`) computes a `u32` shift
amount and a `u32` shift operand from two raw, still-unvalidated `fmt ` chunk fields
(`nChannels` and `dwChannelMask`):

```rust
let shift = 32 - (!channel_mask).leading_ones();
channel_mask |= ((1 << channel_diff) - 1) << shift;
```

Flagged by the `HEURISTICS.md` pattern "arithmetic on sizes/lengths/indices derived from input",
specialised to the shift-amount case: both `shift` and `channel_diff` are attacker-controlled and
neither is clamped to `< 32`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual code read of `fix_wave_channel_mask` and its one caller | `read_ext_fmt` (chunks.rs:342) -> `fix_wave_channel_mask` (chunks.rs:679), checking where `num_channels`/`channel_mask` are validated relative to where they are used | Both operands reach the shift unvalidated: `num_channels` is the raw `u16` read at chunks.rs:420, and the only channel-count/mask sanity check is `Position::from_wave_channel_mask` at chunks.rs:345, i.e. *after* `fix_wave_channel_mask` has returned. Two independent overflow conditions derived: `shift == 32` when bit 31 of the mask is set, and `channel_diff >= 32` when `nChannels - popcount(mask) >= 32`. The `else` branch (`31 - (!channel_mask).leading_ones()`) was checked too and cannot underflow: it only runs while `count_ones() > num_channels >= 0`, so the mask is never 0 there |
| Boundary-value unit probe in a debug build | two temporary `#[test]`s added to the project's own `wave/chunks.rs` in a git worktree, calling `fix_wave_channel_mask(0x8000_0000, 2)` and `fix_wave_channel_mask(0, 40)`; `cargo test -p symphonia-format-riff` (debug, overflow checks on) | Both panicked `attempt to shift left with overflow` - at chunks.rs:690:25 (the outer `<< shift`) for the bit-31 case and chunks.rs:690:26 (the inner `1 << channel_diff`) for the 40-channel case, confirming the two sites are independent. Temporary tests reverted afterwards |
| Manual boundary-value construction + overflow-checks build, end-to-end | standalone `repro/`, `cargo run --release` with `overflow-checks = true`, 8 hand-built ~60-byte WAVE_FORMAT_EXTENSIBLE files fed through `symphonia::default::get_probe().probe()` on real files on disk | Panics `attempt to shift left with overflow` at wave/chunks.rs:690 for exactly the 4 predicted cases (nChannels=32/mask=0, nChannels=40/mask=0, nChannels=2/mask=0x8000_0000, nChannels=3/mask=0xc000_0000). The 4 predicted-safe cases (nChannels=31/mask=0 - the largest `channel_diff` that still fits, nChannels=2/mask=1<<18, and two well-formed stereo files) do not panic |
| Same repro without overflow checks | `CARGO_PROFILE_RELEASE_OVERFLOW_CHECKS=false cargo run --release`, same 8 files | No panic: the shift amount is masked to `amount & 31`, so the parser continues with a silently wrong channel layout. nChannels=40/mask=0 yields mask `0xff` and the file **probes successfully as an 8-channel track**; nChannels=32/mask=0 yields mask `0` (zero positioned channels) and is only rejected further downstream, by `"riff: block size is 0"` |

## Status

**Confirmed**

Reproduces 4/4 runs (`cargo run --release` with `overflow-checks = true`), deterministically, at
the same two source locations (`wave/chunks.rs:690:25` and `690:26`) for the same 4 of 8 boundary
inputs each run - see `output.txt` (run 1 with full backtraces, runs 2-4 as summary line + exit
status). Only manifests as a panic in a build with overflow checks enabled (any debug build, a
release build with `overflow-checks = true`, or a `cargo-fuzz` target, which enables them by
default); in a plain release build the shift amount is masked and the visible effect is a wrong
channel layout instead - stated here as a fact, not as a reason to discard it
(`CONFIRMATION.md` section 2).

**Fully reproducible**: **Yes**. `repro/` writes real `.wav` files to disk and opens each one
through the actual public entry point an application uses -
`symphonia::default::get_probe().probe(&hint, MediaSourceStream::new(File...), ...)` - not by
calling `fix_wave_channel_mask`/`read_ext_fmt`/`WavReader::try_new` directly. There is no second
participant to fake here: the input is a single external artifact (a malformed media file), exactly
what a real user/attacker would hand the library (`CONFIRMATION.md` section 11's
explicitly-fine single-participant case).

## Class

crash

## Root cause

`WaveFormatChunk::parse` reads `nChannels` verbatim out of the `fmt ` chunk (wave/chunks.rs:420) and
`read_ext_fmt` reads `dwChannelMask` verbatim out of the WAVEFORMATEXTENSIBLE extension
(wave/chunks.rs:220). For every non-Ambisonic sub-format GUID it passes both straight to
`fix_wave_channel_mask` (wave/chunks.rs:342), whose job is to add missing bits to a mask whose
population count is lower than the declared channel count:

```rust
let channel_diff = num_channels as i32 - channel_mask.count_ones() as i32;
...
if channel_diff > 0 {
    let shift = 32 - (!channel_mask).leading_ones();
    channel_mask |= ((1 << channel_diff) - 1) << shift;
}
```

`(!channel_mask).leading_ones()` is `channel_mask.leading_zeros()`, so `shift` is the bit position
just above the mask's most significant set bit, and `((1 << channel_diff) - 1)` is a run of
`channel_diff` ones to place there. Both quantities are `u32` shift amounts derived from file
content, and neither is bounded below 32:

| condition | value | expression that overflows |
|---|---|---|
| bit 31 of `dwChannelMask` set, and mask still short of `nChannels` | `shift == 32` (`leading_zeros() == 0`) | `x << 32` (chunks.rs:690:25) |
| `nChannels - popcount(dwChannelMask) >= 32` | `channel_diff >= 32` | `1u32 << channel_diff` (chunks.rs:690:26) |

Neither condition is filtered out beforehand: the only thing that would reject such a file is
`Position::from_wave_channel_mask` (which returns `None` for any mask with a bit above 18) on the
line *after* the call, or `map_amb_channel_count`, which is only reached for the two Ambisonic
B-format GUIDs. So every input that triggers this is a file the parser is about to reject anyway; it
just panics on the way to rejecting it. The trigger needs nothing but a ~60-byte WAVE file whose
`fmt ` chunk is `wFormatTag = 0xfffe` (WAVE_FORMAT_EXTENSIBLE), `cbSize = 22`, the PCM sub-format
GUID, and a hostile `nChannels`/`dwChannelMask` pair - reached from any
`probe()`/`WavReader::try_new` call on an untrusted `.wav`.

In a build without overflow checks the same two shifts are masked to `amount & 31` instead, which
turns the panic into a silently wrong result: `nChannels = 40, dwChannelMask = 0` computes
`(1u32 << (40 & 31)) - 1 == 0xff`, so the mask becomes `0xff` and the file is accepted as an 8-channel
track, and `nChannels = 32, dwChannelMask = 0` computes `(1u32 << 0) - 1 == 0`, leaving an empty
positioned-channel set for a file that declares 32 channels.

## Introduced by

ad4ba8f "wav: Handle incorrect channel mask. (#180)" (2023-02-02) - that commit added
`fix_wave_channel_mask` (then in the now-deleted `symphonia-format-wav` crate,
`symphonia-format-wav/src/chunks.rs`) with the overflowing expression written exactly as it still
reads today. Found with `git log -S "leading_ones"` (and `git log -S "fix_wave_channel_mask"`, which
only shows f1722b6 "core (audio): New audio module.", the commit that moved the function into
`symphonia-format-riff` without changing the arithmetic).

## The fix

`patch.diff` (1 file, +14/-1, of which 5 lines are comment and 7 are new test assertions) builds the
run of ones in `u64` and truncates it back to the mask's width:

```rust
let shift = 32 - (!channel_mask).leading_ones();
channel_mask |= (((1u64 << channel_diff.min(32)) - 1) << shift) as u32;
```

Why this is sufficient for every input, not just the ones found:

- `channel_diff.min(32)` bounds the inner shift amount at 32, which is in range for `u64`, and is
  exactly equivalent to no cap at all for the result, because any one beyond bit 31 is truncated
  away by the `as u32` regardless.
- `shift` is `32 - leading_zeros(...)`, so it is always in `0..=32`, also always in range for `u64`.
  The widest possible intermediate is `0xffff_ffff << 32`, which fits `u64` with 0 bits to spare
  but no overflow.
- For every input that did not overflow before, the `u64` computation produces bit-for-bit the same
  low 32 bits, so the behaviour is unchanged (the existing `test_fix_channel_mask` assertions still
  pass untouched).
- For the inputs that did overflow, ones that don't fit in the 32-bit mask are dropped, which leaves
  a mask whose population count still doesn't match `nChannels`. That is the honest outcome - such a
  file genuinely cannot be represented - and the caller then rejects it with the error it already
  has for an unrepresentable mask (`"wav: too many channels in mask for fmt_ext"`), because the
  resulting mask has bits set above bit 18.

The patch also extends the module's existing `test_fix_channel_mask` unit test with 6 boundary
assertions covering both overflow conditions and the largest non-overflowing `channel_diff`
(`(0, 31)`, `(0, 32)`, `(0, 40)`, `(0x8000_0000, 2)`, `(0x8000_0000, 33)`, `(0xc000_0000, 3)`).

Before: 4 of 8 files panic the probe, exit 1. After: all 8 files are handled without panicking (the
6 unrepresentable ones return `Err(Unsupported("wav: too many channels in mask for fmt_ext"))`, the
2 well-formed stereo ones probe successfully), exit 0. See `output.txt` and `just test`.

- Project test suite (`cargo test --workspace`, in the candidate's own git worktree): 106 passed /
  0 failed before the patch -> 106 passed / 0 failed after. No new failures, no tests newly skipped.
  (The 6 new assertions live inside the already-counted `test_fix_channel_mask` test, so the total
  is unchanged by design.) `cargo +nightly fmt --check -p symphonia-format-riff` reports no diff in
  `wave/chunks.rs` after the patch.

## What's not done / open questions

- No test was added at the `read_ext_fmt`/file-bytes level, only at the `fix_wave_channel_mask`
  level. The module has no harness for driving a chunk parser from a byte buffer, so a file-level
  regression test would need one built first; the maintainers may prefer to add it alongside this
  fix.
- A maintainer might reasonably prefer a different policy for an unfixable mask: returning an
  explicit `decode_error("wav: channel mask cannot be corrected for the stated channel count")`
  from `fix_wave_channel_mask` (making the function fallible) instead of returning a
  still-mismatched mask and relying on the caller's existing rejection. This patch deliberately
  keeps the function infallible and its signature unchanged; the choice is a design call for the
  project.
- The non-panicking release-build consequence documented above is worth a maintainer's attention on
  its own: `nChannels = 40, dwChannelMask = 0` currently *succeeds* as an 8-channel track in a plain
  release build. The patch turns that into a rejection, which is a behaviour change for such files -
  intentional, but a change.
- Related, not covered here: `nChannels = 32, dwChannelMask = 0` produces `Channels::Positioned`
  with an *empty* position set in a release build - a zero-channel track from a WAV file. Candidate
  031 (`CORE_AUDIOBUFFER_NEW_ZERO_CHANNEL_DIV_BY_ZERO`) covers what zero-channel specs do further
  down the pipeline; whether any other RIFF path can produce an empty positioned set was not swept
  as part of this candidate.
- Only `symphonia-format-riff` was grepped for this specific class (a shift amount derived from a
  file field). A repo-wide sweep for `<<`/`>>` with a non-constant right-hand side derived from
  input was not done and would be a reasonable separate candidate.
- A `cargo-fuzz` run against the existing `demux_wav` target would very likely find this within
  seconds (the trigger is inside the first 60 bytes and `cargo-fuzz` enables overflow checks by
  default), which suggests `demux_wav` has not been run for long. A longer campaign there (via
  `fuzz-bug-repro`) is cheap and worthwhile.
