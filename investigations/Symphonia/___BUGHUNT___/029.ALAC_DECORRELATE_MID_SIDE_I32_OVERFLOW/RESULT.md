# ALAC_DECORRELATE_MID_SIDE_I32_OVERFLOW

`symphonia-codec-alac/src/lib.rs:668-669` (`decorrelate_mid_side`) does the mid-side inverse with
plain, unguarded `i32` arithmetic - `*s0 = *s0 + *s1 - ((*s1 * weight) >> shift); *s1 = *s0 - *s1;`
- while both operands come straight from the bitstream: `weight` is the element's 8-bit
`mid_side_weight` field (`lib.rs:518`, so -128..=127) and `*s1` is a decoded sample. The caller
validates `mid_side_shift > 31` (`lib.rs:555`) but never bounds either magnitude, and unlike the
neighbouring residual/predictor code (which uses `wrapping_add`/`wrapping_mul`/`wrapping_sub`
throughout `read_residuals` and most of `predict`) these two lines have no overflow handling at
all.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual code read of the whole ALAC decode path to work out whether an in-range sample can actually reach `decorrelate_mid_side` large enough to overflow | Source only, no build | Yes, and with entirely ordinary header values. For a 24-bit stereo CPE with element shift 0, `bps = bit_depth - shift + is_cpe = 25` (`lib.rs:511`), so an escaped rice code (`prefix > 8`, `lib.rs:616`) reads a raw 25-bit value, and `rice_code_to_signed(0x1ff_ffff)` = -16777216 = -2^24. With `lpc_order == 0`, `predict` returns before `clip_msbs` runs (`lib.rs:173`), so that value reaches `decorrelate_mid_side` unmodified. -16777216 * -128 = 2147483648 = i32::MAX + 1 |
| Debug/overflow-checks build driven by a real, complete `.caf` file through symphonia's public probe -> format reader -> audio decoder path (`repro/main.rs`, no arguments) | 185-byte `.caf` on disk (24-bit stereo ALAC, `frame_length` 4, `mid_side_weight` = -128, both channels' samples = -2^24), `cargo run` and `cargo run --release` with `overflow-checks = true` | Reproduces every time: `thread 'main' panicked at symphonia-codec-alac/src/lib.rs:668:28: attempt to multiply with overflow`. Backtrace goes `repro::main` -> `AudioDecoder::decode` -> `AlacDecoder::decode_inner` -> `decode_sce_or_cpe` -> `decorrelate_mid_side` (see output.txt). 5/5 debug runs, 2/2 release-with-overflow-checks runs |
| Same repro built with overflow checks explicitly disabled | `RUSTFLAGS="-C overflow-checks=off" cargo run --release` | No panic - the identical overflow wraps silently and the decoder returns 4 frames of arithmetically wrong samples, confirming the same defect is a silent wrong-output bug in a stock release build rather than being absent there |
| Same-bug-class variation sweep through the same public path (`repro/main.rs --sweep`) | 17920 synthesized CAF/ALAC streams: `bit_depth` in {16,20,24,28,31,32} x element shift {0,1,2} bytes x every `mid_side_shift` 0..=31 x `mid_side_weight` in {-128,-127,-1,1,127} x 7 extreme/interior coded-sample values paired across channels, `cargo run --release` with overflow checks on | Against the pristine tree this is not runnable to completion (the first case already panics); against the patched tree all 17920 either decode or return a graceful `decode error`, 0 panics - this is the fix-completeness check, not an additional discovery |

Two independent approaches (code read, live end-to-end repro) agreed immediately, so the soft cap
of 3 escalating tools was not needed - no fuzzer or sanitizer was run for this candidate.

## Status

**Confirmed.** Reproduces deterministically, 5/5 runs in a debug build and 2/2 runs in a release
build with `overflow-checks = true`, plus 1/1 through the candidate's own `just test` against a
fresh clone of the upstream repo at the base commit. The panic message, file, line and column
(`symphonia-codec-alac/src/lib.rs:668:28`) are identical on every run. With overflow checks off
there is no panic and the wrap is silent (verified, see the table above) - the finding is
build-configuration-dependent in *symptom*, not in existence.

**Fully reproducible**: **Yes**. `repro/` writes a real `.caf` file to disk and then decodes it
using only symphonia's public API - `File` -> `MediaSourceStream` ->
`symphonia::default::get_probe().probe(..)` -> the real `CafReader` ->
`symphonia::default::get_codecs().make_audio_decoder(..)` -> `decoder.decode(packet)` - exactly the
sequence `GETTING_STARTED.md` documents for applications. No decoder state, packet or
`AudioCodecParameters` is hand-constructed and nothing private is touched; the only hand-authored
thing is the media file itself, which *is* the genuine external artifact an attacker/user supplies
(CONFIRMATION.md section 11's single-participant case - a malformed file handed to a parser's real
public entry point). The backtrace in output.txt shows the full public call chain.

## Class

crash

## Root cause

`decorrelate_mid_side` implements the inverse of ALAC's mid-side transform with plain `i32`
operators. The reference decoder does the same arithmetic in C `int32_t`, where overflow wraps in
practice; Rust's `+`/`-`/`*` instead panic when overflow checks are enabled (and wrap - producing
wrong samples - when they are not). Both inputs are attacker-controlled and neither is bounded:

- `weight` is `mid_side_weight`, read as a signed 8-bit field, so it can be -128 (the value whose
  negation is not representable in 8 bits, and the one that makes the product largest).
- `*s1` is a decoded sample. For a CPE, `bps` is `bit_depth - shift + 1`, i.e. 25 for the very
  common 24-bit stereo case, so a legal escaped rice code can decode to exactly -2^24. `predict`
  would clip a sample to `bps` bits via `clip_msbs`, but with `lpc_order == 0` it returns before
  doing anything (`lib.rs:173`), so nothing narrows the value.

-2^24 * -128 = 2^31, one past `i32::MAX`, so `*s1 * weight` overflows. `*s0 + *s1` and
`*s0 - *s1` on line 669 are equally unguarded and can overflow for the same reason (large
same-sign / opposite-sign sample pairs), so all three operators need fixing, not only the
multiply. Nothing about the triggering stream is exotic: version-0 magic cookie, 24-bit stereo,
`lpc_order` 0, shift 0, `mid_side_shift` 0 - only the field *values* are adversarial. (A wider
variant exists too: `bit_depth` 31 in a CPE gives `bps` 32, so `clip_msbs`'s `num_clip_bits` is 0
and even the predictor path clips nothing.)

## Introduced by

6e4653c "alac: Implement decoder." (2021-11-11) - `git log -S decorrelate_mid_side` returns this
single commit and `git blame -L 663,671` attributes both lines to it, so the code has had this
shape since the ALAC decoder was first added; no later commit touched these lines.

## The fix

`patch.diff` (+5/-2 lines) switches the three operators to their wrapping equivalents, matching
what `read_residuals` and `predict` in the same file already do:

```rust
*s0 = s0.wrapping_add(*s1).wrapping_sub(s1.wrapping_mul(weight) >> shift);
*s1 = s0.wrapping_sub(*s1);
```

Why this is the right shape rather than a clamp or a validation error: whenever no overflow occurs
the wrapping operators are bit-for-bit identical to the plain ones, so no valid stream's output
changes at all, and in a release build without overflow checks the generated code is unchanged
(plain `i32` arithmetic already wraps there). The patch therefore only removes the debug/overflow-
checks panic while keeping the decoder's output identical to both its own release behaviour and
the reference decoder's `int32_t` arithmetic. Rejecting the stream instead would risk refusing
files the reference decoder accepts, which is a maintainer-facing policy decision, not a bounded
fix.

- `repro/` before the fix: panics at `symphonia-codec-alac/src/lib.rs:668:28`, "attempt to
  multiply with overflow". After the fix: `decoded 4 frames` / `finished: 1 packet(s) decoded, no
  panic`. Verified in both the working tree and via `just test` against a fresh upstream clone.
- Checked against more than the one triggering input (CONFIRMATION.md section 7): the 17920-stream
  sweep described in the table above (all `mid_side_shift` values, six bit depths, three element
  shifts, five weights including both -128 and +127, and the extreme/interior coded-sample values
  paired across channels, which also exercises the line-669 subtraction) - 0 panics after the fix.
- Project test suite (`cargo test --workspace`): 106 passed / 0 failed before the patch, 106
  passed / 0 failed after. No new failures. `cargo clippy -p symphonia-codec-alac` is clean on the
  patched file.

## What's not done / open questions

- `ElementChannel::predict` (`lib.rs:212-259`) has its own plain, non-wrapping `i32` arithmetic on
  bitstream-controlled values that this candidate did not investigate or fix: `s - past0`
  (line 216, inside the `wrapping_mul`), `past0 - sample` (lines 233, 248),
  `(1 + j as i32) * ((sign * val) >> self.shift)` and the `res -=` that consumes it (lines 238,
  253), and `*coeff -= sign` / `*coeff += sign` (lines 236, 251). These are a separate candidate,
  not covered by `patch.diff`.
- Only the CAF container was used to reach the decoder. The MP4/M4A path (`symphonia-format-isomp4`,
  the far more common ALAC container) feeds the identical magic cookie and packet bytes to the same
  `AlacDecoder`, so it is expected to be affected identically, but that was not built or run here.
- The mono/SCE path cannot reach this code: `decode_sce_or_cpe` rejects a non-zero
  `mid_side_weight` for non-CPE elements (`lib.rs:521`) and `decorrelate_mid_side` is only called
  when `mid_side_weight != 0`. Only channel pair elements are affected.
- Not investigated: whether a real encoder could ever emit a stream that trips this. The
  reproduction needs deliberately adversarial field values, so in practice this is a
  malformed/hostile-input crash rather than something a legitimate encoder would produce.
- A longer fuzzing campaign (via `fuzz-bug-repro`) pointed at `symphonia-codec-alac` would likely
  be worthwhile - the crate's fuzz entry point already exists, and this bug is exactly the shape a
  fuzzer finds quickly once it learns the CPE header layout.
