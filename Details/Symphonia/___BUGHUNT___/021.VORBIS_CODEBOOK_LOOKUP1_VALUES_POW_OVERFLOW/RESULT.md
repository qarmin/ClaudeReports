# VORBIS_CODEBOOK_LOOKUP1_VALUES_POW_OVERFLOW

`lookup1_values(entries, dimensions)` (`symphonia-codec-vorbis/src/codebook.rs:34-45`) computes
`value` via `(entries as f32).powf(1.0 / dimensions).floor() as u32`, then does
`assert!(value.pow(u32::from(dimensions)) <= entries)`. The suspicion (candidate 021): if
floating-point rounding ever pushes `value` one unit too high right at the `dimensions == 32`
boundary, `.pow()` itself (an unchecked-looking call that uses ordinary `*` internally, so it
panics under `overflow-checks`/debug and wraps in release) could panic *before* the assert's
comparison ever runs, rather than the assert cleanly rejecting a bad `value`. The one call site
(`codebook.rs:339`) is reached with `codebook_entries`/`codebook_dimensions` already bounds-checked
by the caller (`codebook.rs:239-244`: `dimensions` in `1..=32`, `entries` in `0..=131072`) - the
candidate flagged this as lower-confidence, not yet demonstrated with a concrete crafted codebook
header.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value reasoning: for `value.pow(dimensions)` to overflow `u32`, `value >= 2` and `dimensions >= 32` are both required (`2^32` already exceeds `u32::MAX`); but for `dimensions >= 18`, `floor(131072^(1/dimensions))` is always `<= 1` since `2^18 = 262144 > 131072`, so `value >= 2` and `dimensions >= 32` can never co-occur once `entries` is capped at 131072 | pen-and-paper / calculator, no build needed | No combination of `entries` in `[0, 131072]` and `dimensions` in `[1, 32]` can make `value.pow(dimensions)` overflow `u32`; the worst case is `dimensions = 17`, `value = 2`, `2^17 = 131072` (no overflow) |
| Exhaustive differential/property check: a standalone copy of `lookup1_values` plus the file's own `naive_lookup1_values` reference (integer-only, `checked_pow`-based), run across the ENTIRE real caller-enforced domain (`entries` 0..=131072, `dimensions` 1..=32 - all 4,194,336 combinations), compiled with `overflow-checks=on` | `rustc -O -C debug-assertions=on -C overflow-checks=on`, then re-run via `cargo run --release`/`cargo run` (dev) inside `repro/boundary_probe.rs` | **0 panics**, all 4,194,336 results match the naive reference exactly (`repro/boundary_probe.rs`, "Approach A") |
| Same standalone copy, targeted boundary probes across the FULL underlying-type domain (`dimensions` 1..=64, `entries` near `u32::MAX`) to see whether the private helper itself has a latent issue outside anything a real caller could ever pass it | same build, `repro/boundary_probe.rs` "Approach B" | **Confirmed the latent mechanism is real**, but only for `entries` within ~8 of `u32::MAX` (e.g. `entries=4294967295, dimensions=2` panics with "attempt to exponentiate with overflow" - `.pow()` overflowing *before* the assert's comparison runs) - `entries` values more than ~2^32/2^17 = 32768x larger than the real cap of 131072 |
| Real end-to-end repro: hand-crafted (but well-formed enough) Vorbis identification + setup header packets driven through the REAL public `symphonia_codec_vorbis::VorbisDecoder::try_new` entry point, with the sole codebook placed exactly at the real caller-enforced maximum (`codebook_dimensions=32`, `codebook_entries=131072`), plus one-past-cap variants and a few other points in the valid domain | debug build and release build (both with `overflow-checks=true` via `Cargo.toml` profile), 3/3 repeat runs each build | **No panic in any run.** The max-boundary case reaches `lookup1_values` successfully (proven by it failing *later*, at `synthesize_codewords`, with a clean `"vorbis: codebook overspecified"` decode error - i.e. `lookup1_values` itself completed without panicking). The one-past-cap cases are rejected by the explicit `codebook_dimensions > 32`/`codebook_entries > 128*1024` guards before `lookup1_values` is ever reached. Deterministic across all repeats (`repro/main.rs`, `output.txt`) |

## Status

**Not reproducible** via the real trigger path. The underlying mechanism the candidate suspected
is real in isolation (Approach B above: `lookup1_values`, called directly with values far outside
what any real caller passes, does panic on `.pow()` overflow rather than being caught by the
assert) - but it requires `entries` within single digits of `u32::MAX` (~4.29 billion), while the
function's one real call site (`codebook.rs:339`) can never pass `entries` above 131072
(`128 * 1024`), enforced by an explicit check five lines above the call
(`codebook.rs:242-244`) that runs before `lookup1_values` is ever invoked. Exhaustively checked:
all 4,194,336 `(entries, dimensions)` combinations in the real caller-enforced domain produce no
panic and match the naive integer reference exactly (Approach A). Verified additionally via a real
end-to-end run through the actual public decoder entry point at the exact maximum boundary
(`dimensions=32, entries=131072`): 3/3 runs in both debug and release builds, no panic, deterministic.

**Fully reproducible**: N/A (not applicable to a Not-reproducible result - no bug to reproduce via
the real trigger path).

## Class

N/A (Not reproducible).

## Root cause

Not applicable to the specific bug suspected here - the suspected pow-overflow-inside-assert *is* a
real, demonstrable behavior of `lookup1_values` in isolation, but it's structurally unreachable via
any real Vorbis bitstream: `VorbisCodebook::read` validates `codebook_entries <= 128 * 1024` and
`1 <= codebook_dimensions <= 32` (`codebook.rs:232-244`) before `lookup1_values` is ever called
(`codebook.rs:339`), and within that enforced domain the largest `value.pow(dimensions)` can ever
be is `2^17 = 131072` (at `dimensions=17`), nowhere close to `u32::MAX`. Overflow only becomes
possible once `entries` is within ~8 of `u32::MAX` - about 32,768x past the real 131072 cap - which
no bitstream, however crafted, can make `codebook_entries` reach, since it's a 24-bit wire field
(max 16,777,215) additionally clamped to 131,072 by the check above.

## Introduced by

Not applicable (Not reproducible - no bug to attribute via the real trigger path). For context: the
`lookup1_values` function itself (including the `assert!(value.pow(...) <= entries)` line) has been
present, unchanged in this respect, since af34e68 "vorbis: Initial Vorbis decoder commit." - the
bounds check at `codebook.rs:239-244` that makes it unreachable was already present at that same
commit.

## The fix

Not applicable - no bug found via the real trigger path, no patch produced. (The isolated
pow-overflow behavior in Approach B is real but only reachable by calling the private
`lookup1_values` function directly with `entries` values no real caller can produce - not a
justification for a patch to production code.)

- Project test suite (`cargo test -p symphonia-codec-vorbis`): 6 passed, 0 failed before this
  investigation (no code was changed, so there is no "after" - the main project's working tree was
  never modified, per this skill's own rule).

## What's not done / open questions

- If `symphonia-codec-vorbis`'s own bounds (`codebook_dimensions <= 32`, `codebook_entries <= 128 *
  1024`, `codebook.rs:239-244`) were ever relaxed or removed in a future change, this exact latent
  behavior (Approach B) would become live again - a defense-in-depth fix (e.g. replacing the
  `assert!(value.pow(...) <= entries)` with a `checked_pow`-based comparison, matching the file's
  own `naive_lookup1_values` test helper) would remove the *possibility* rather than relying on the
  caller's bounds check remaining in place forever, but wasn't applied since there is no reachable
  bug in the current code and this skill only patches confirmed, reachable issues found this
  session.
- Only this one call site of `lookup1_values` exists in the whole workspace (checked via
  `grep -rn lookup1_values`); no other decoder shares this helper.
- Not investigated: whether any *other* private helper in this file (or `unpack_vq_lookup_type1`/
  `unpack_vq_lookup_type2`, which also multiply attacker-influenced sizes) has a similar
  isolation-only latent issue - out of scope for this specific candidate.
