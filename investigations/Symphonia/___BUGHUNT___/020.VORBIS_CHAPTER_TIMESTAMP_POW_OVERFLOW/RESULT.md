# VORBIS_CHAPTER_TIMESTAMP_POW_OVERFLOW

`symphonia-metadata/src/embedded/vorbis.rs:249`, `parse_chapter_timestamp`, computes
`10u64.pow(u32::from(frac_s_digits))` where `frac_s_digits` is the full length (including leading
zeros; only trailing zeros are trimmed) of the fractional-seconds substring of an
attacker-controlled `CHAPTERnnn=HH:MM:SS.FFFF...` Vorbis-comment value. This looked suspicious
because `parse_unsigned_num`'s own `checked_mul`/`checked_add` overflow guard only protects the
numerator, not the digit-count-derived exponent, and existing unit tests only ever exercise up to
9 fractional digits.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction + `rustc -C debug-assertions=on` on an isolated `10u64.pow(n)` snippet | `n=20` vs `n=19` | `n=20` panics ("attempt to exponentiate with overflow"); `n=19` doesn't - confirmed the arithmetic itself overflows exactly where predicted |
| Real end-to-end repro: a minimal, otherwise-valid FLAC file with a crafted `CHAPTER001=00:00:00.00000000000000000001` (20-digit fractional seconds) `VORBIS_COMMENT` block, opened via the real public `symphonia_bundle_flac::FlacReader::try_new` | Default (dev) cargo profile, `overflow-checks = true`, `debug-assertions = true` (Rust's own dev-profile default) | Reproduced: panics inside `parse_chapter_timestamp` via the real FLAC demuxer call path, 3/3 runs, deterministic |

Only 2 of the allotted 3 approaches were needed since the second one produced a full, deterministic, real end-to-end reproduction with a clean root cause - no further escalation (miri/ASAN/fuzzing) added anything a pure-arithmetic overflow like this would benefit from.

## Status

**Confirmed.** Reproduces 3/3 runs, standard `cargo run` (dev profile, which has
`overflow-checks`/`debug-assertions` on by default in this workspace's own `Cargo.toml`), fully
deterministic (pure integer arithmetic, no randomness or timing dependency, no flakiness expected
on any run).

**Fully reproducible: Yes.** The repro constructs a real FLAC file (byte-for-byte: `fLaC` magic +
a spec-valid `STREAMINFO` block + a `VORBIS_COMMENT` block holding the malicious comment) and
opens it with `symphonia_bundle_flac::FlacReader::try_new` - the exact public entry point any real
downstream application (e.g. `symphonia-play`) uses to open a `.flac` file. This is a
single-participant (file-format-parser) finding, not a network-peer/two-process protocol; there is
no second live participant to fake here - the "attacker" is simply the bytes of the file, which
this repro constructs and hands to the real, unmodified demuxer, exactly as a real malicious file
would reach it.

## Class

crash (panic on malformed/crafted input; a release build without `overflow-checks` instead
silently wraps to a bogus, attacker-influenced `ns` value rather than panicking)

## Root cause

`parse_chapter_timestamp` parses fractional seconds from the free-form `CHAPTERnnn` comment value.
`parse_unsigned_num(frac_s_str, false)` trims only *trailing* zeros, so a crafted fractional-second
string with many *leading* zeros (e.g. 19 zeros followed by a single `1`) yields
`frac_s_digits = 20` while the numeric value (`frac_s_numer`) itself stays tiny (`1`) - every
partial product in `parse_unsigned_num`'s `checked_mul`/`checked_add` fold stays `0` or `1`, so its
own overflow guard never triggers. The following line then computes
`10u64.pow(u32::from(frac_s_digits))` unconditionally: for `frac_s_digits >= 20`, `10^20` exceeds
`u64::MAX` (~1.8446744e19), so `u64::pow` panics with "attempt to exponentiate with overflow" in
any build with overflow checks enabled (Rust's own dev-profile default), or silently wraps to an
arbitrary, attacker-influenced value in a release build without them, which then feeds directly
into `ns as u32` and `Time::from_hhmmss`.

## Introduced by

90893e9 "meta (vorbis): Support the chapter extension." (2024-12-06) - this commit added
`parse_chapter_timestamp` (and the vulnerable `10u64.pow(...)` line) in its entirety; `git blame`
confirms the line has not been touched since.

## The fix

Changed `10u64.pow(...)` to `10u64.checked_pow(...)`, using the fact that `frac_s_numer` is
bounded to `u32::MAX` by its own return type: `1_000_000_000 * u64::from(frac_s_numer)` is always
< `1e19`, so once the true denominator (`10^frac_s_digits`) would exceed what `u64` can hold
(`frac_s_digits >= 20`), the mathematically-correct fractional-second contribution is already
negligible/rounds down to `0` nanoseconds - `None` from `checked_pow` is therefore mapped directly
to `ns = 0` instead of panicking or wrapping. This is not merely "stop the one crafted input from
crashing": it was checked with both the original 20-digit trigger and a much larger boundary value
(1000 digits) - see `output.txt` - both now behave identically to any other non-overflowing digit
count.

- Verified by re-running `repro/` against the patched tree: no panic for `frac_s_digits=20` or
  `frac_s_digits=1000` (see `output.txt`, "AFTER" section) - the demuxer proceeds past comment
  parsing and only fails later with the same benign, expected "unexpected end of file" that every
  other non-overflowing digit count also produces (from the synthetic test file intentionally
  carrying no audio frame data - unrelated to this bug).
- Project test suite (`symphonia-metadata`, `--features vorbis`): 12 passed; 0 failed before -> 12
  passed; 0 failed after (identical, including the pre-existing `verify_parse_chapter_timestamp`
  test, which does not itself exercise more than 9 fractional digits).
- Full workspace test suite (`cargo test --workspace --all-features`): 106 passed; 0 failed before
  -> 106 passed; 0 failed after (identical).

## What's not done / open questions

- The same `parse_unsigned_num`/digit-count pattern is local to `parse_chapter_timestamp` only;
  no other call site in this codebase uses `parse_unsigned_num`'s digit count to drive a `pow()`,
  so this class of bug was not found to be duplicated elsewhere in this session.
- The fix caps the *practical* effect of arbitrarily long fractional-second strings to 0ns rather
  than rejecting the input outright with `FMT_ERR`/`OOB_ERR`; a maintainer might prefer to reject
  fractional-second strings above some sane digit limit (e.g. > 9 or > 19) as malformed instead of
  silently treating them as 0 - both are reasonable, this session picked the minimal, behavior-
  preserving option matching what the pre-overflow arithmetic would have produced.
- Not investigated: the same file/module's other `parse_unsigned_num` call sites (`h_str`,
  `m_str`, `s_str`) are unaffected because none of them feed a digit count into `pow()`.
