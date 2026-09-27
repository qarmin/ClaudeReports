# ALAC_PREDICT_FIR_I32_SUB_OVERFLOW

`ElementChannel::predict` in `symphonia-codec-alac/src/lib.rs` deliberately uses
`wrapping_add`/`wrapping_mul` for almost all of its `i32` arithmetic (with comments explaining
why), but five operations inside the FIR filter and the coefficient-adaptation loop are still
plain `-`/`*`. Suspected because the surrounding code documents that the values there are
attacker-influenced and can overflow, and because the clipping that would keep them in range
(`clip_msbs`) becomes a no-op for the largest legal ALAC bit depth.

The five sites (line numbers at base commit ee35874):

| Line | Expression |
|---|---|
| 216 | `.map(\|(&coeff, &s)\| coeff.wrapping_mul(s - past0))` - the `s - past0` subtraction |
| 233 | `let val = past0 - sample;` (positive-residual branch) |
| 238 | `res -= (1 + j as i32) * ((sign * val) >> self.shift);` |
| 248 | `let val = past0 - sample;` (negative-residual branch) |
| 253 | `res -= (1 + j as i32) * ((-sign * val) >> self.shift);` |

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual code read of `predict()`, `read_residuals()`, `clip_msbs()`, `decode_sce_or_cpe()`, and `MagicCookie::read()` | base commit ee35874 | Established the preconditions: `bit_depth = 32` is accepted (`symphonia-common/src/apple/audio/alac.rs` only rejects `> 32`), so `bps == 32`, `num_clip_bits == 32 - 32 == 0`, and `clip_msbs(v, 0)` is the identity - the rice-decoded residuals keep the full `i32` range instead of being clipped |
| Boundary-value construction against the decoder's public API (`AlacDecoder::try_new` + `decode_ref` with a hand-built magic cookie and packet) | dev profile (debug-assertions + overflow-checks on) | Panic: `attempt to subtract with overflow` at `symphonia-codec-alac/src/lib.rs:216:56` from a 30-byte packet |
| End-to-end repro through the public `symphonia` API only: generated `.caf` files on disk -> `symphonia::default::get_probe()` -> real `symphonia-format-caf` demuxer -> `symphonia::default::get_codecs()` -> real ALAC decoder (`repro/`) | dev profile (debug-assertions + overflow-checks on), 403 generated files, 3 repeat runs | Identical output all 3/3 runs: 329 of 403 files panic; all five suspected sites are hit (308x line 216, 8x line 233, 1x line 238, 9x line 248, 3x line 253) |
| Same repro, release profile with overflow checks forced on, driven by this folder's `just test` from a fresh `git clone` of the upstream repo | release + `overflow-checks = true` | Same 329 panics, same five sites - the bug is not a dev-profile-only codegen artifact |
| Same repro, release profile with overflow checks explicitly **off** (`RUSTFLAGS="-C overflow-checks=off -C debug-assertions=off"`) | release, no overflow checks | 0 panics, all 403 files decode - the overflow then wraps silently instead of aborting (see Status for what this means for the finding) |
| Deterministic 400-file sweep over the same bug class (random LPC order 1-8, random 16-bit coefficients, random quantisation shift 0-15, random `pb` bits, 4-31 samples per frame, mono SCE at `bit_depth=32` and stereo CPE at `bit_depth=31`, escaped residuals mixing `i32::MIN`/`i32::MAX`/random/small values) - used both to reproduce and, after the fix, to check the fix is not narrow | dev and release, fixed RNG seed | Before: 326/400 panic. After: 0/400 panic, 400/400 decode |

## Status

**Confirmed**

Deterministic and bit-for-bit repeatable: 3/3 identical runs in a dev-profile build (debug
assertions + overflow checks, cargo's defaults), plus one more identical run in a release build
with overflow checks on, driven from a fresh upstream clone via this folder's `just test`. The
panic is an arithmetic-overflow panic, so it only aborts in builds with overflow checks enabled
(the cargo `dev`/`test` default, and any release build that turns them on). In a release build
with overflow checks off, the same input instead makes the arithmetic wrap silently, which is the
behaviour the fix makes explicit and unconditional.

**Fully reproducible**: **Yes**. `repro/` writes real CAF files to disk and decodes them through
the public `symphonia` API exactly as `symphonia/examples/getting-started.rs` does - probe,
demux, decode - with nothing crate-internal reached into and no hand-built decoder state. The
input is a single external artifact (a media file), so there is no second participant to stand in
for.

## Class

crash

## Root cause

`predict()` computes `num_clip_bits = 32 - self.bps` and then relies on
`clip_msbs(val, num_clip_bits)` (`(val << num) >> num`) to keep every intermediate value inside
the coded sample width, which is what makes the remaining plain `i32` arithmetic safe for the
common bit depths. For a 32-bit ALAC stream `bps == 32`, so `num_clip_bits == 0` and `clip_msbs`
is the identity function: the samples in `out` keep whatever the rice decoder produced, i.e. the
full `i32` range including `i32::MIN` (`rice_code_to_signed(0xffff_ffff) == i32::MIN`, reachable
via the 9-bit escape prefix followed by 32 raw bits).

`bps == 32` is reachable from two legal configurations:

- an SCE (mono/LFE element) with `bit_depth == 32` and `shift == 0`: `bps = 32 - 0 + 0`;
- a CPE (channel pair) with `bit_depth == 31` and `shift == 0`: `bps = 31 - 0 + 1` (a CPE gets one
  extra bit for mid-side coding).

Both pass every existing validation: `MagicCookie::read` only rejects `bit_depth > 32`, and
`decode_sce_or_cpe` only rejects `bps > 32`.

With unclipped samples in `out`, the five plain operations overflow:

- the warm-up pass computes `out[1] = out[1].wrapping_add(out[0])`, so with two `i32::MIN`
  residuals `out[1]` becomes `0` while `out[0]` stays `i32::MIN`. The FIR filter then evaluates
  `s - past0` = `0 - i32::MIN`, which panics (line 216). The `wrapping_mul` right next to it shows
  the overflow potential of this expression was already understood, just not for the subtraction
  feeding it.
- the coefficient-adaptation loop recomputes the same difference with the operands swapped
  (`past0 - sample`, lines 233/248), so fixing only the FIR filter just moves the panic a few
  lines down.
- `sign * val` overflows for `val == i32::MIN` (`sign == -1`), and `(1 + j as i32) * (...)`
  overflows whenever the shifted product exceeds `i32::MAX / (1 + j)` - both inside the same
  statement (lines 238/253), plus the `res -=` accumulation itself.

The reference decoders (Apple's ALACDecoder, ffmpeg's `alac.c`) perform this arithmetic in plain C
`int`s and depend on two's-complement wraparound, so wrapping is the semantically correct
behaviour here, not merely a way to silence the panic.

## Introduced by

6e4653c "alac: Implement decoder." (2021-11-11) - the initial ALAC decoder implementation.
`git log -S "s - past0"` and `git log -S "past0 - sample"` on
`symphonia-codec-alac/src/lib.rs` both return only that commit, and `git log -S
"wrapping_mul(s - past0)"` returns it too, i.e. the FIR line has had a wrapping multiply next to a
non-wrapping subtraction since the file was first added. Later commits touching this file
(8ef55b0 rustfmt, e9a4f63 copyright year, 209ad99 warning fixes) do not change these expressions.

## The fix

`patch.diff` converts the five remaining sites to wrapping arithmetic, matching what the rest of
`predict()` already does and what the reference decoders' C `int` arithmetic does:

- `s - past0` -> `s.wrapping_sub(past0)`
- `past0 - sample` -> `past0.wrapping_sub(sample)` (both branches)
- `res -= (1 + j as i32) * ((sign * val) >> self.shift)` ->
  `res = res.wrapping_sub((1 + j as i32).wrapping_mul(sign.wrapping_mul(val) >> self.shift))`
  (and the `-sign` mirror)

Plus one comment recording why wrapping is required here at all (a 32-bit coded depth makes
`clip_msbs` a no-op).

Why this is sufficient rather than input-specific: `wrapping_*` is bit-identical to the plain
operator on every input that does not overflow, so no currently-working stream changes its decoded
output, and there is no remaining non-wrapping `i32` operation on bitstream-derived values in
`predict()`. `*coeff -= sign` / `*coeff += sign` were left alone deliberately: a coefficient
starts in `[-32768, 32767]` (read as a 16-bit signed value), a fresh `ElementChannel` is parsed
per element per packet, and each coefficient moves by at most 1 per sample, so with the frame
length capped at `4096 * 16` a coefficient cannot get anywhere near `i32::MIN`/`i32::MAX`.

Before/after with `repro/` (release build, overflow checks on, from a fresh upstream clone - see
`output.txt`):

- before: `329 panics, 0 decode errors, 74 decoded without panicking`, exit status 1
- after: `0 panics, 0 decode errors, 403 decoded without panicking`, exit status 0

The 400-file sweep in case 4 is the "more than one input in the same class" check: 326 different
generated files panicked before the patch across all five sites, none after.

- Project test suite (`cargo test --workspace`, dev profile): 106 passed / 0 failed before the
  patch, 106 passed / 0 failed after (identical set). `cargo +nightly fmt -p symphonia-codec-alac
  -- --check` and `cargo clippy -p symphonia-codec-alac` are both clean on the patched file.

## What's not done / open questions

- Only the CAF container was used to deliver the packets. ALAC in MP4/M4A
  (`symphonia-format-isomp4`, the far more common carrier) attaches the same magic cookie as
  `extra_data`, so the same file content should reach the same code path there, but that was not
  built and run.
- The mid-side path was deliberately kept out of this repro (`mid_side_weight == 0` in every
  generated file) so that only `predict()` is exercised. `decorrelate_mid_side` has its own
  separate non-wrapping arithmetic (`*s1 * weight`, `*s0 + *s1 - ...`); that is a different root
  cause and code path and is not covered here.
- Not investigated: whether a 32-bit ALAC stream's *decoded output* is correct after the fix
  (wrapping matches the reference decoders, but no reference-decoder differential test was run -
  there is no ALAC test asset in the repo and no oracle available in-tree). This candidate only
  establishes that the panic is gone and that non-overflowing inputs are unaffected.
- The same "arithmetic is only safe because `clip_msbs` normally narrows it, but `num_clip_bits`
  can be 0" reasoning applies to `read_residuals` and to the tail-bit reassembly
  (`*s0 = (*s0 << shift) | val as i32`) in `decode_sce_or_cpe`; those were not audited for the
  32-bit case.
- The repo's existing `symphonia/fuzz/fuzz_targets/decode_alac.rs` target *can* reach this: it
  takes the first 24/48 bytes of the fuzz input as the magic cookie, so `bit_depth = 32` is in its
  reachable input space, and ALAC's `decode_inner` does not depend on `Packet::block_dur()` (which
  the shared `fuzz_audio_decoder` helper always sets to zero). This candidate did not run that
  target - a longer campaign via `fuzz-bug-repro` pointed at `decode_alac`, ideally with a valid
  32-bit cookie seeded into its corpus, is the obvious follow-up for the rest of this crate.
