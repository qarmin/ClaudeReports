# RIFF_AIFF_FORMATPCM_FRAME_LEN_U16_MUL_OVERFLOW

`symphonia-format-riff/src/common.rs:215` (`FormatPcm::make_packet_info`) computes the PCM frame
length as `(self.bits_per_sample / 8) * num_channels` with *both* operands `u16`, where
`num_channels` is `self.channels.count() as u16` on the line above. This matches
`HEURISTICS.md`'s "arithmetic on untrusted sizes/widths in a narrow integer type" pattern. The
suspicion was raised while reading `symphonia-format-riff` for a sibling candidate (027, the WAV
`expected_block_align` site) and looked distinct because AIFF's own channel-count validator,
`map_aiff_channel_count` (`aiff/chunks.rs:546-559`), *deliberately* accepts any count >= 4 by
falling through to `Channels::Discrete(count)` - so unlike the WAV path (which caps channels at 26
via `map_wave_channel_count`) the multiplication here is reachable with a channel count that has
already passed every validation the format applies.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading of `symphonia-format-riff`: all six `make_packet_info` implementations in `common.rs`, both channel-count validators (`map_aiff_channel_count`, `map_wave_channel_count`), and every `FormatData` construction site in `aiff/chunks.rs` and `wave/chunks.rs` | Traced the actual bound on each operand back to the byte in the file it comes from | `FormatPcm::make_packet_info` (common.rs:214-215) is the only reachable narrow-width site. `bits_per_sample` is clamped to 8/16/24/32 by `CommonChunk::read_pcm_fmt`, so `bits_per_sample / 8` is at most 4, but `Channels::Discrete(count).count()` is the raw 16-bit COMM channel-count field, up to 65535. 4 * 16384 == 65536 does not fit in u16. `FormatIeeeFloat`/`FormatALaw`/`FormatMuLaw` (the other AIFF-reachable variants) already do their arithmetic in `u32`, so they are safe. `FormatExtensible` (common.rs:295-297) and `FormatAdpcm` (common.rs:233/238/247) use the identical `u16` pattern but are built **only** by `wave/chunks.rs`: `read_adpcm_fmt` goes through `map_wave_channel_count` (max 26 channels, `bits_per_sample` forced to 4), and `read_extensible_fmt` derives its `Channels` either from `Position::from_wave_channel_mask` on a `u32` mask (at most 32 positions) or from `map_amb_channel_count` (bounded table) - both provably too small to overflow. AIFF's own `packet_info` returns `unsupported_error` for those two variants outright |
| Debug-assertions build + manual boundary-value construction, driven through the real public API | `cargo run --release` with `overflow-checks = true` (and separately a plain `cargo run`, cargo `dev` profile, where overflow-checks are on by default), five mechanically generated `.aiff` files opened from disk through `symphonia::default::get_probe().probe(...)` | Reproduced. `aiff_chan16384_size32.aiff` (16384 channels, 32-bit samples), `aiff_chan21846_size24.aiff` (21846 channels, 24-bit) and `aiff_chan65535_size32.aiff` (65535 channels, 32-bit) all panic `attempt to multiply with overflow` at common.rs:215:25, with the backtrace running `Probe::probe` -> `AiffReader::try_probe_new` -> `AiffReader::try_new` (aiff/mod.rs:211) -> `CommonChunk::packet_info` (aiff/chunks.rs:185) -> `FormatPcm::make_packet_info`. Both control files (`aiff_chan2_size16.aiff`, and `aiff_chan30000_size8.aiff` - 30000 channels but 1 byte/sample, so 30000 still fits in u16) open successfully, confirming the channel count alone is not what is being rejected |
| Repeat-run + build-matrix check | Same pristine clone: 5 consecutive `cargo run --release` runs of `aiff_chan16384_size32.aiff`; plus one run with `CARGO_PROFILE_RELEASE_OVERFLOW_CHECKS=false`; plus one plain `cargo run` (dev profile) | 5/5 runs panic identically at common.rs:215:25. Plain `cargo run` (dev) panics too - no special `RUSTFLAGS` needed. With overflow-checks disabled (cargo's stock `release` profile) there is no panic: the product wraps to 0, `PacketInfo::without_blocks(0)` hits its own `NonZero::new` guard and returns `malformed stream: riff: block size is 0`, so release builds degrade to a graceful decode error |

No fuzzing, miri or sanitizer run was needed - per `HEURISTICS.md` this bug class (narrow-width
arithmetic on an untrusted size field) is fully covered by checked arithmetic plus hand-built
boundary values, and the boundary is exactly computable (`num_channels >= 65536 / (bits/8)`).

## Status

**Confirmed**. Reproduces 5/5 consecutive runs, deterministic, on `cargo run --release` with
`overflow-checks = true` in the repro's own `[profile.release]` and equally on a plain `cargo run`
(cargo's `dev` profile enables overflow-checks by default). Three separate input files trigger it
(16384/32-bit, 21846/24-bit, 65535/32-bit); two control files with the same shape but a
non-overflowing product are handled normally.

**Fully reproducible**: **Yes**. `repro/` writes nothing internal by hand: it opens a real `.aiff`
file from disk as a `MediaSourceStream` and hands it to `symphonia::default::get_probe().probe(...)`
- the same public entry point `symphonia-play` and any other consumer uses - which selects
`AiffReader` itself and calls `AiffReader::try_new`. The only crafted thing is the file's own bytes
(a structurally valid FORM/AIFF container with a valid 18-byte COMM chunk and a real SSND chunk;
see `repro/gen_aiff.py`, which regenerates every asset). Per `CONFIRMATION.md` section 11, a
malformed file handed to a parser's real public entry point *is* the genuine external input - there
is no second participant being impersonated here.

## Class

crash (checked-arithmetic overflow panic, `overflow-checks = true` builds only)

## Root cause

AIFF stores its channel count as an unsigned 16-bit field in the mandatory COMM chunk.
`map_aiff_channel_count` (`symphonia-format-riff/src/aiff/chunks.rs:546-559`) intentionally does
**not** bound it: only 0 is rejected, 1/2/3 map to named layouts, and everything from 4 to 65535
falls through to `Channels::Discrete(count)` with the comment that layouts above 3 channels are
poorly defined and should be treated as discrete channels. That is a reasonable decision for the
channel model, but it means every downstream consumer of that `Channels` value must cope with a
count up to 65535.

`FormatPcm::make_packet_info` (`symphonia-format-riff/src/common.rs:213-217`) does not:

    let num_channels = self.channels.count() as u16;
    let frame_len = (self.bits_per_sample / 8) * num_channels;
    PacketInfo::without_blocks(u32::from(frame_len))

`bits_per_sample` is already normalised to 8/16/24/32 by `CommonChunk::read_pcm_fmt`, so
`bits_per_sample / 8` is 1..=4 - but the multiply is still evaluated in `u16` and only widened to
`u32` afterwards, one operation too late. Any `bits_per_sample / 8 * num_channels >= 65536`
overflows: 16384 channels at 32-bit, 21846 at 24-bit, 32768 at 16-bit, or 65536 at 8-bit (the last
being unreachable, since the field itself caps at 65535).

`AiffReader::try_new` calls `comm.packet_info()?` unconditionally (`aiff/mod.rs:210-211`), before
any decoding and before any track is produced, so simply opening/probing the file is enough. Nothing
between the COMM parse and this line validates the channel count any further.

The `u32::from(frame_len)` widening on the following line shows the intent was to compute in a wide
enough type; the bug is purely that the widening happens after the multiplication instead of before
it.

## Introduced by

7c01671 "riff (wave): Don't use block align if is not required." (2026-04-15) - it consolidated the
per-format packetization math into the shared `Format*::make_packet_info` methods in
`common.rs`. Before that commit, AIFF computed the same value itself in `CommonChunk::packet_info`
entirely in `u32` (`u32::from(self.n_channels) * (u32::from(self.sample_size + 7) / 8)`), which
cannot overflow, so the refactor narrowed a previously-safe computation to `u16`. Established with
`git log -L 213,216:symphonia-format-riff/src/common.rs` and `git blame` (both point at 7c01671 for
every line of the current function) plus `git show 7c01671^:symphonia-format-riff/src/aiff/chunks.rs`
to read the pre-refactor arithmetic; no bisect needed.

## The fix

`patch.diff` widens before multiplying, mirroring what the already-safe sibling implementations
(`FormatIeeeFloat`, `FormatALaw`, `FormatMuLaw`) in the same file already do:

    let num_channels = self.channels.count() as u32;
    let frame_len = u32::from(self.bits_per_sample / 8) * num_channels;
    PacketInfo::without_blocks(frame_len)

`u32` is provably sufficient rather than just larger: `bits_per_sample / 8` is at most 8 (`u64`
float samples in the extensible WAV path) and every channel count reaching a `Format*` in this
crate originates in a 16-bit field, so the product is at most 8 * 65535 == 524280. The result is a
*correct* frame length, not a clamp or an early error - the three previously-panicking files now
open normally with `block_size` = 65536 / 65538 / 262140 bytes respectively, which is what the
COMM chunk actually describes.

Per `CONFIRMATION.md` section 7 the fix was checked against more than the one input that first
triggered it: all three overflowing files (three different bits-per-sample values, including the
65535-channel extreme) plus the two non-overflowing controls, and a new in-crate regression test
`aiff::chunks::test_high_channel_count_packet_info` that asserts the resulting `block_size` equals
`num_channels * bytes_per_sample` for (16384, 32), (21846, 24) and (65535, 32). That test was
verified to actually fail on the unpatched tree (`panicked at common.rs:215:25`) and pass with the
patch, so it is a real regression guard rather than a tautology.

Before/after of `repro/` against the same clone (full transcript in `output.txt`):

- before: `attempt to multiply with overflow` at common.rs:215:25, exit 101, for each of the three
  high-channel-count files
- after: `OK: probe returned a reader with 1 track(s)` for all five files, exit 0

- Project test suite (`cargo test --workspace`, aggregated over all test binaries + doctests):
  106 passed / 0 failed before the patch -> 107 passed / 0 failed after (the one extra is the new
  regression test). No pre-existing test changed result. `cargo clippy -p symphonia-format-riff
  --all-targets` reports zero diagnostics attributed to this crate both before and after (the
  `clippy::unwrap_used` warnings in the build come from `symphonia-core`'s FFT code and are
  untouched); `cargo fmt --check` shows only the pre-existing stable-vs-nightly `else`-placement
  diffs in `aiff/mod.rs` that exist on the unmodified tree too.

## What's not done / open questions

- `FormatExtensible::make_packet_info` (common.rs:295-297) and `FormatAdpcm::make_packet_info`
  (common.rs:233, 238, 247) contain the *identical* `u16` multiply pattern
  (`(bits_per_sample / 8) * num_channels`, `bits_per_sample * num_channels`, `7 * num_channels`,
  `4 * num_channels`). They were checked and are **not** reachable with an overflowing operand
  today, for the reasons recorded in the "What was tried" table (only `wave/chunks.rs` constructs
  those variants, and every path there bounds the channel count to at most 32). They were therefore
  deliberately left out of `patch.diff` to keep it scoped to the one confirmed bug - but they are
  latent: if AIFF ever grows an Extensible/ADPCM path (`aiff/chunks.rs:187-193` already has arms
  for both, currently returning `unsupported_error`), or if a WAV channel-count validator is ever
  relaxed the way AIFF's was, they become the same bug. A maintainer taking this patch may prefer to
  widen all four sites in one go.
- The related WAV-side site `wave/chunks.rs:100` (`let expected_block_align = num_channels *
  (bits_per_sample / 8);`, which runs *before* `map_wave_channel_count` and so is not protected by
  the 26-channel cap) is a separate candidate (027) and is untouched here. Fixing 027 does not fix
  this one and vice versa - they are different files, different formats, and the AIFF panic here
  fires with WAV entirely uninvolved.
- Whether Symphonia wants to accept a 65535-channel AIFF at all is a design question this patch
  does not answer: with the patch the file opens and reports 65535 discrete channels, and the
  overflow is gone, but a decoder asked to allocate that many channel planes may run into its own
  limits. Rejecting implausible channel counts in `map_aiff_channel_count` would be a policy change
  and belongs to the maintainers, not to an overflow fix.
- No fuzzing campaign was pointed at `symphonia-format-riff`'s AIFF path in this session. A
  `fuzz-bug-repro` run over `AiffReader::try_new` would be a cheap way to look for further COMM/SSND
  field-combination issues (this one is trivially reachable - two bytes of the COMM chunk - so a
  fuzzer would likely have found it in seconds, which suggests the path has never been fuzzed).
