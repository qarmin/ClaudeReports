# unpack_overlay_bits: pdep-based bit expansion (BMI2) vs candidate 022's byte table

Follow-up to candidate 022 (`OVERLAY_UNPACK_TABLE`, -88.5% single-core over the original
bit-at-a-time loop via a 256-entry `[u8; 8]` lookup table). That candidate's own "what's not
done" section speculated, but never measured, that a wider-than-byte SIMD/PDEP-style bit
expansion might do better still, since the table version already looked close to a pure
output-write-bound `memcpy` rate. This candidate builds and measures that wider approach for
real: `_pdep_u64` (BMI2), which deposits the 8 packed bits of one input byte into the low bit of
each of the 8 bytes of a `u64` in a single instruction, reproducing the exact same LSB-first
unpacked byte sequence as the table lookup with no memory access at all.

## What was tried

Added `unpack_overlay_bits_bmi2` (`pixeldata/src/overlay.rs`), gated behind a runtime
`is_x86_feature_detected!("bmi2")` check (this box: AMD Ryzen 9 7900, Zen 4, has BMI2 with
single-uop `pdep`/`pext`). `unpack_overlay_bits` now dispatches to it when available, falling
back to candidate 022's own table-driven `unpack_overlay_bits_table` otherwise (non-x86_64, or
x86_64 without BMI2) - so the table implementation is kept in the source as the portable
fallback, not replaced.

For a direct, same-binary, same-process A/B comparison (isolating the mechanism from
process-level noise), a temporary `DICOM_RS_FORCE_TABLE` env-var toggle was added during
measurement to force the table path even when BMI2 is available, then removed again before
capturing `patch.diff` - it is **not** part of the shipped patch; `patch.diff` in this folder
contains only the real, permanent bmi2-vs-table dispatch.

Benchmark: `overlay-decode 4096 4096 <iters>` (harness mode added for candidate 022) - a
synthetic ~16.8M-pixel overlay plane (~2.1MB packed input, ~16.8MB unpacked output), isolating
`decode_overlays()` which is almost entirely `unpack_overlay_bits` for a standard overlay.

| Variant | single-core p50 (`taskset -c 0`) | multi-core p50 | notes |
|---|---|---|---|
| baseline (original bit-at-a-time, re-measured fresh this session) | 8.855-10.646 ms (2 rounds) | 8.785 ms | matches candidate 022's own baseline range |
| candidate 022 table (env-forced within the same binary) | 0.992-1.188 ms (5 rounds) | 1.023-1.042 ms (3 rounds) | min across rounds: 0.982-1.033 ms |
| this candidate: pdep/bmi2 (default dispatch) | 1.008-1.827 ms (5 rounds) | 1.049-1.870 ms (3 rounds) | min across rounds: 1.007-1.160 ms; one multi-core outlier (1.870 ms) coincided with a system load spike (other unrelated processes) |

All rounds used 23-30 iterations per invocation, alternating table/pdep back-to-back (not
grouped) to spread out any drift; checksum (`0x1085a213bc222325`) was identical across every
variant and round, including the original baseline.

## Result

**No measurable improvement from the pdep/wide-bit-expansion path over candidate 022's existing
byte table - if anything, a small, consistent disadvantage.** Across 5 single-core rounds and 3
multi-core rounds (interleaved order, alternating which variant ran first each round to control
for drift), the table's own minimum was at or below the pdep path's minimum in every single
round (5/5 single-core, 3/3 multi-core) by roughly 2-8% (e.g. single-core round 3: table
982 us vs pdep 1008 us min; multi-core round 2: table 1021 us vs pdep 1051 us min). The
difference is small relative to this box's noise floor (p95/max on both variants regularly spike
to 5-8 ms from unrelated system activity - this box was running several other concurrent
`___OPTIMIZATIONS___` candidates' own builds/benchmarks during this session, serialized only
through the shared `flock`, not otherwise isolated from background load), but the *direction* was
never inverted across 8 total interleaved rounds, so it reads as a real (if minor) preference for
the table, not pure noise.

Both variants remain overwhelmingly faster than the original bit-at-a-time baseline (~8-9x,
consistent with candidate 022's own -88.5% figure) - this candidate is purely about whether a
wider replacement beats the table, not about re-litigating candidate 022's own win.

This measures **wall-clock time only** (single-core and multi-core p50/min); peak RSS/allocation
count were not separately measured since neither implementation changes the allocation pattern
from candidate 022's (`Vec::with_capacity(num_pixels)` then `extend_from_slice` in both cases).

## Does it change behavior/output?

- **Bit-exact?** Yes. `_pdep_u64(byte, 0x0101_0101_0101_0101)` deposits bit `i` of `byte` into
  bit `0` of output byte `i` (least-significant bit first) - by construction, the identical value
  and byte order as `UNPACK_BITS_TABLE[byte][i] = (byte >> i) & 1`. Verified: identical checksum
  (`0x1085a213bc222325`) across baseline/table/pdep on the synthetic 4096x4096 overlay, and every
  existing overlay test (including `unpack_bits_is_lsb_first`, an explicit bit-order assertion,
  and `decode_overlays_from_mr_with_overlays`, a real DICOM file) passes unmodified.
- **Is this a breaking change?** No. `unpack_overlay_bits` and all three new/existing helper
  functions (`unpack_overlay_bits_table`, `unpack_overlay_bits_bmi2`, `UNPACK_BITS_TABLE`) are
  private (non-`pub`) items in `pixeldata/src/overlay.rs`. No public API changed.
- Project test suite:
  - `cargo test -p dicom-pixeldata --release --features image,native`: **71 passed / 0 failed**,
    both baseline (original code, re-verified fresh this session) and patched (bmi2 dispatch,
    with the temporary `DICOM_RS_FORCE_TABLE` toggle already removed).
  - `cargo test --workspace --release` (default features): **537 passed / 0 failed**, patched.
    (Baseline workspace-wide count not independently re-run this session beyond the
    `dicom-pixeldata`-scoped 71/0 above and candidate 022's own prior 537/0 baseline record for
    the same base commit - no reason to expect the workspace-wide count to differ since this
    patch touches only `pixeldata/src/overlay.rs`, a file candidate 022 already covered.)
  - `cargo clippy -p dicom-pixeldata --release --features image,native`: clean, patched.

## Why the result looks the way it does

This is consistent with candidate 022's own closing hypothesis: the operation is dominated by
*writing* ~16.8MB of output (8x data expansion from ~2.1MB packed input), not by the per-input-byte
compute needed to produce it. A `pdep` instruction is genuinely cheap on Zen 4 (single uop,
sub-nanosecond), but it doesn't reduce the number of output bytes written or their total volume -
it only changes *how* the 8 output bytes for a given input byte are produced (register computation
instead of an L1-resident 2KB table lookup, `UNPACK_BITS_TABLE` being `256 * 8 = 2048` bytes,
comfortably cache-resident). With the table already avoiding any real cache-miss cost at that
size, replacing its lookup with an equally-cheap register op has nothing left to win: both are
bottlenecked on the same fixed `extend_from_slice`/store traffic to the output `Vec`. The small,
consistent edge toward the table (rather than a dead heat) plausibly comes from `pdep` still
being a full ALU instruction in the critical path per byte, vs. a table load that can partially
overlap with the store of the *previous* iteration's output - but this wasn't independently
confirmed (no `perf stat`/`uop`-level profiling was run to isolate that from noise).

## What's not done / open questions

- Not tried: a genuinely wider SIMD approach that processes **multiple input bytes per
  instruction** (e.g. an SSE2/AVX2 `pshufb`+compare bit-unpack processing 2-4 packed bytes into
  16-32 output bytes per vector op, or a loop unrolled to call `_pdep_u64` on several input bytes
  between each `extend_from_slice`, reducing per-byte loop/bounds-check overhead rather than
  per-byte compute). Given this candidate's own result (the per-byte *compute* step, table or
  pdep, is not the bottleneck), such an approach would need to reduce the *store* volume/pattern
  itself to help - e.g. writing larger contiguous blocks per loop iteration - not just process
  more input faster; this wasn't attempted and its potential is unquantified.
- Not independently re-run this session: the full `cargo test --workspace --release` count for
  the unpatched baseline (relied on candidate 022's own prior record of 537/0 at the same base
  commit for the same crate coverage, plus this session's own fresh 71/0 `dicom-pixeldata`-scoped
  baseline check).
- `perf stat`/microarchitectural profiling to confirm the "ALU-in-critical-path vs. overlapped
  table load" explanation above was not run - stated as the leading hypothesis, not a confirmed
  root cause.
- This was measured on one CPU (AMD Zen 4, `pdep`/`pext` single-uop). Older AMD microarchitectures
  (pre-Zen 3) implement `pdep`/`pext` via microcode at roughly 18 cycles latency each - on such a
  CPU this patch's bmi2 path would likely be a clear *regression* against the table, not just a
  wash; the `is_x86_feature_detected!("bmi2")` gate only checks instruction *availability*, not
  its throughput on that specific microarchitecture, so this patch would need a
  microarchitecture-aware exclusion (or simply not be worth taking) on pre-Zen-3 AMD hardware.
  Not measured on any such hardware this session.
