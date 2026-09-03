# Lut::new_rescale_and_normalize: fuse min/max fold (candidate 018) + rayon threshold-guard it (candidate 012's pattern)

`Lut::new_rescale_and_normalize` (`pixeldata/src/lut.rs`, used whenever `VoiLutOption::Normalize`
is requested) scans every decoded pixel sample to find its min/max before building the LUT.
Candidate 018 (2026-08-26) already fused the original two-pass `.clone().fold()`/`.fold()` into
one pass but left it strictly sequential. `IDEAS.md` flagged the natural follow-up: since
candidate 012 already established that `Lut::new_with_fn`'s table-*construction* loop benefits
from a rayon threshold+guard, does the same pattern help this per-*sample* min/max scan too. This
candidate reimplements 018's fusion (not yet in real source - each candidate is reverted after its
own session) and adds a `rayon`-guarded parallel fold/reduce above a 16,384-sample threshold on
multi-core hosts, as one combined mechanism (per `IDEAS.md`'s own framing of this as "parallelize
the fused fold", not two independent fixes).

To make the fold parallelizable without an intermediate collect, the function's generic
`I: IntoIterator` parameter (which every real call site fed via `.iter().copied()` on an
already-owned `&[u8]`/`Vec<u16>`) was changed to a plain `samples: &[S]` slice - a `pub(crate)`-only
signature change, all 7 call sites in `pixeldata/src/lib.rs` updated to pass the slice directly
(`data` unchanged where already `&[u8]`; `&samples` where the local was an owned `Vec<u16>`).

## What was tried

Harness: `___OPTIMIZATIONS___/harness/bench_pipeline`'s existing `colorspace-normalize` mode
(added by candidate 018 - forces `ConvertOptions::new().with_voi_lut(VoiLutOption::Normalize)`),
against `testdata/mono_above.dcm` (MONOCHROME2, 2048x512 = 1,048,576 samples - above the 16,384
threshold, so the rayon path is exercised).

Harness change made this session (kept, not reverted - it's harness-only, not real project
source): `harness/bench_pipeline/Cargo.toml`'s `dicom-pixeldata` dependency now has
`default-features = false` (was implicitly on, pulling in `rayon` unconditionally), and a new
`bench_pipeline` `default = ["dicom-pixeldata/rayon"]` feature added, so `cargo build --release`
behaves exactly as before but `cargo build --release --no-default-features` now produces a binary
with zero `rayon_core` symbols (confirmed via `nm target/release/bench | grep -c rayon_core` = 0),
matching the "no-rayon build" methodology candidates 012/018's own prose describes but which this
session found the current harness `Cargo.toml` could not actually produce as shipped - worth
keeping for any future single-core rayon-guard candidate on this project.

| Variant | single-core p50 (`taskset -c 0`, no-rayon build) | multi-core p50 (unpinned, rayon build) | notes |
|---|---|---|---|
| baseline (two-pass fold, unconditional... N/A, sequential) | 1.264 ms | 1.608 ms | round 2: 1.628 ms |
| patched (fused fold + rayon threshold-guard) | 0.970 ms | 0.886 ms | round 2: 0.915 ms |

## Result

**Multi-core: -44.9%/-43.8% across two independent rounds, 9/9 paired wins both rounds, fully
consistent.** **Single-core (no-rayon build, `taskset -c 0`): -23.2%, 9/9 paired wins, consistent**
- this isolates the fusion-alone contribution (the rayon-guarded branch compiles out entirely
under `#[cfg(feature = "rayon")]` absence), and lands close to candidate 018's own standalone
fusion result (-15.4% single-core on the same fixture) - the gap is plausibly explained by this
candidate's slice-based signature avoiding the generic `IntoIterator`/`NumCast` machinery
candidate 018 still went through, though this wasn't isolated further. The multi-core number is
larger than candidate 018 alone (-18.1%/-18.4%) and larger than a naive sum of "fusion + dispatch
cost" would suggest, consistent with rayon's parallel reduce doing real work on ~1M samples well
above the 16,384 threshold. `uptime` load average was 0.6-0.9 throughout, low.

Peak RSS was not measured for this candidate - the mechanism (fewer redundant scans, no extra
allocation) is not expected to move peak RSS in either direction (no buffer added or removed,
just fewer passes/dispatch to existing structures), consistent with candidate 018's own reasoning
for not measuring it.

## Does it change behavior/output?

- **Bit-exact?** Yes - `checksum=0x96db96ecb3a17025` (FNV-1a of the decoded+converted image bytes)
  is identical between baseline and patched builds on `mono_above.dcm`. Expected: min/max
  reduction is order-invariant (same reasoning as candidate 018), and rayon's parallel
  fold/reduce recombines partial (min, max) pairs the same associative way regardless of how work
  is partitioned across threads.
- **Is this a breaking change?** No public API/type changes - `new_rescale_and_normalize` is
  `pub(crate)`, not exposed outside the `dicom-pixeldata` crate. The generic parameter rename
  (`I` -> `S`) and signature change (`IntoIterator` -> `&[S]`) only affects this one internal
  function and its 7 call sites, all updated in the same patch.
- Project test suite: `cargo test --release -p dicom-pixeldata`: **39 passed / 0 failed / 2
  ignored** (lib), plus 1 (image_reader) + 1 (overlay) + 10 (doctests) = same totals, both before
  and after the patch, no new failures. `cargo build --release` and `cargo build --release
  --no-default-features` (both crate-level and via the harness) confirmed clean with no warnings
  in either feature configuration.

## Why the result looks the way it does

Same reasoning as candidates 012 and 018 individually, compounded: fusing the two folds into one
halves the number of `to_f64()` conversions and comparisons per sample (018's contribution), and
for ~1M samples - well above rayon's fixed per-call dispatch overhead - splitting that single pass
across available cores via a guarded parallel fold/reduce adds further real wall-clock savings
(012's pattern, applied to a per-sample fold instead of a per-table-entry map this time). The
threshold guard means this never regresses a small-sample-count caller (all real call sites here
pass a full frame's worth of samples - hundreds of thousands to low millions in practice - so the
below-threshold branch is mostly a safety net for hypothetically tiny frames, not something
exercised by any real fixture in this session).

## What's not done / open questions

- The exact single-core-vs-fusion-alone gap (-23.2% here vs. candidate 018's own -15.4%) was
  noticed but not root-caused - plausibly the slice-based signature (avoiding generic
  `IntoIterator`/`Clone`/`NumCast` iterator-adapter machinery) contributes something beyond the
  fold fusion itself, not isolated as a separate measurement.
- Peak RSS not measured (see "Result" above for why this wasn't expected to matter).
- The crossover point of the 16,384-sample threshold was not independently re-verified for this
  specific fold (reused candidate 012's already-established order-of-magnitude value, per
  `IDEAS.md`'s own note that no real DICOM frame size lands near this boundary either way).
- Only `mono_above.dcm` (1,048,576 samples, above threshold) was measured - no below-threshold
  fixture was tested to directly confirm "no change" there for this specific fold (inferred from
  the `#[cfg(feature = "rayon")]`-gated, `if len >= threshold` structure being identical in shape
  to candidates 003/004/006/012, all of which confirmed this pattern holds).
