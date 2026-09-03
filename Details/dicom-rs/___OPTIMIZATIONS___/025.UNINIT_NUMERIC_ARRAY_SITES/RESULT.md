# Fresh re-derivation of candidate 009's numeric-array zero-init sites (unresolved contradiction)

`IDEAS.md` carried an open lead: candidate 009 applied its "skip zero-init before full-overwrite"
fix (`uninit_smallvec`) to 8 more numeric-array decode sites in `parser/src/stateful/decode.rs`
(`read_value_ss`/`fl`/`od`/`ul`/`us`/`uv`/`sl`/`sv`) as a broader attempt before narrowing down to
just `read_value_ob`. That broader version was originally measured as a **+14.2% regression** (0/9
paired wins, `parse liver.dcm 2000`), reproduced twice - but a later same-session re-check of the
exact same comparison came back **-1.2%, a small win, 9/9 then 15/15 paired wins**, directly
contradicting the original finding. Neither result was trusted; `IDEAS.md` asked for a fresh,
careful re-derivation. This candidate re-implements the isolated 8-site-only patch (identical in
mechanism to candidate 009's own `uninit_smallvec` helper - self-contained duplicate per this
workspace's "one fix per candidate" discipline) and re-measures from scratch.

## What was tried

Applied `uninit_smallvec::<T, A>(len)` (skip `SmallVec`'s zero-fill before a `decode_*_into` call
that immediately overwrites every element) to all 8 sites: `read_value_ss`, `read_value_fl`,
`read_value_od`, `read_value_ul`, `read_value_us`, `read_value_uv`, `read_value_sl`,
`read_value_sv`. `read_value_ob` itself was left untouched (out of scope here, already its own
separate candidate 009 finding).

Bench: `___OPTIMIZATIONS___/harness/bench_pipeline`'s `parse` mode against `pydicom/liver.dcm`
(102KB), the exact same fixture and mode (`parse testdata/liver.dcm 2000`) as both of candidate
009's own original and follow-up measurements, for direct comparability.

| Variant | round 1 (RUNS=9) | round 2 (RUNS=9) | round 3 (RUNS=15) | round 4, `--features diag-mimalloc` (RUNS=9) |
|---|---|---|---|---|
| baseline (zero-init) | 0.024 ms p50 | 0.024 ms p50 | 0.025 ms p50 | 0.024 ms p50 |
| patched (uninit) | 0.024 ms p50 (-0.7%, 5/9) | 0.024 ms p50 (+0.9%, 2/9) | 0.024 ms p50 (-0.8%, 10/15) | 0.024 ms p50 (-0.8%, 5/9) |

## Result

**No consistent change in any of the 4 rounds run this session** - none clears this workspace's
paired-comparison consistency bar (`wins >= n-1` or `wins <= 1`) in either direction: 5/9, 2/9,
10/15, 5/9. This is a **third, different outcome** from both of candidate 009's own prior
measurements (originally +14.2% regression, 0/9 twice; later -1.2% win, 9/9 then 15/15) - neither
extreme reproduces here. Given `liver.dcm`'s full `parse` takes only ~24 microseconds total, and
these 8 sites each fire a small, bounded number of times per file (numeric-VR elements like Bits
Allocated/Rows/Columns/Pixel Representation are typically single scalar values, not large
multi-thousand-element arrays, unlike `read_value_ob`'s native pixel data case) - the underlying
effect size, whatever it truly is, is very likely smaller than this box's measurement noise floor
at this timescale. **This is consistent with the swings seen across all three measurement attempts
(this session's included) being noise landing on different sides of zero each time, not a real
effect that flips direction** - see "Why the result looks the way it does" for the reasoning this
conclusion rests on.

## Does it change behavior/output?

- **Bit-exact?** Yes - `checksum` (`obj.length().0` reported by the harness's `parse` mode) is
  identical between baseline and patched builds on `liver.dcm`. `dump`-style full re-serialization
  was not separately re-checked this session (relies on the same reasoning already verified by
  candidate 009's own bit-exactness check and Miri run for the mechanically identical
  `uninit_smallvec` helper applied to a different site).
- **Is this a breaking change?** No public API/type changes - all 8 functions are private methods
  of `parser`'s internal `StatefulDecoder`.
- Project test suite: `cargo test --release -p dicom-parser`: **49 passed / 0 failed / 0 ignored**,
  both before and after - identical to candidate 009's own baseline count for this crate. Miri was
  not independently re-run this session (candidate 009 already verified the identical
  `uninit_smallvec` mechanism with Miri, 49/49, no UB, applied to a different call site in the same
  file - the soundness argument is identical for these 8 sites, same `T: Copy`, no-`Drop`,
  every-bit-pattern-valid reasoning).

## Why the result looks the way it does

The most likely explanation for three measurement attempts (0/9 twice; then 9/9 and 15/15; now
5/9, 2/9, 10/15, 5/9 across four more rounds) landing in three qualitatively different places is
that this specific comparison - a ~24-microsecond, fixed-instruction-count operation being measured
at a resolution close to this box's own timer/scheduling noise floor - is simply not a comparison
this measurement setup can resolve reliably, regardless of which direction (if any) the true
effect points. This differs from `read_value_ob` (candidate 009's own kept fix), where the fixed
per-call saving is concentrated in one large (~96KB) buffer read once per file - large enough
relative to the rest of `parse`'s work to clear the noise floor consistently (-3.2%/-3.8%, 9/9 both
rounds). The 8 numeric sites here are each small, low-call-count, low-total-byte-volume operations
whose combined saving (if `uninit_smallvec`'s per-call overhead doesn't eat it, per candidate 009's
own "generic dispatch overhead may dominate for small N" hypothesis) is plausibly on the same order
as this box's measurement noise for a file this fast to parse overall.

## What's not done / open questions

- **This candidate does not resolve which of the three measurement outcomes (if any) reflects a
  real underlying effect** - it adds a fourth, "no consistent change" outcome to the record rather
  than picking a winner. Given the timescale involved (tens of microseconds), a future attempt
  would likely need either a much larger/more numeric-VR-dense fixture (none readily available or
  easily synthesizable - `gen-seq` only produces string content) or a lower-noise measurement
  technique (e.g. `perf stat` cycle counting, or a tight in-process loop bypassing process-spawn
  overhead entirely) to have any chance of resolving this at all.
- Per candidate 009's own open item, a hand-specialized non-generic `uninit_u16_smallvec`/
  `uninit_u32_smallvec`/etc. per type (avoiding the generic function's dispatch/branching
  overhead entirely) was not attempted - still a live idea if a future session wants to pursue this
  further with a better-suited measurement technique.
- `patch.diff` is included for completeness/reproducibility (this workspace's rule: any real
  source change gets a patch, regardless of the measured outcome) but, consistent with candidate
  009's own scope decision, **this patch was not adopted as a recommendation either way** - the
  result here is genuinely inconclusive, not a confirmed win withheld out of caution.
