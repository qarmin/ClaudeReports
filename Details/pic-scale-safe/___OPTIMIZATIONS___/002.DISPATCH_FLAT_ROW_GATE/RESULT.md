# Extend the flat row-count gate from resize_nearest to the fixed/floating-point dispatch functions

**Ported from `image` repo's `___OPTIMIZATIONS___/034.PIC_SCALE_SAFE_FILTER_DISPATCH_VENDORED_ROW_GATE/`.**
Originally measured in 2026-08 via a vendored `pic-scale-safe` 0.1.11 copy inside the `image`
project. **Not re-measured in this workspace** - numbers below are copied verbatim. The patch was
rebuilt by hand against this repo's current `HEAD` (`b50e020`); `git apply --check` passes,
`cargo check --features rayon` compiles, and `cargo test --release --features rayon` passes 5/5
(verified this session). Builds on top of candidate 001 (includes its `resize_nearest.rs` gate).

Candidate 001 fixed `resize_nearest`'s own small-output regression. This candidate applies the
*same* `dst_height >= 32` gate mechanically to the six dispatch call sites in
`fixed_point_dispatch.rs`/`floating_point_dispatch.rs` (`convolve_trampoline_*`/`convolve_row_*`/
`convolve_column_*` in both files): below the threshold, fall back to the existing serial loop;
at/above it, unchanged parallel dispatch.

## What was tried

Wrapped each dispatch function's existing `#[cfg(feature = "rayon")]` parallel block in `if
destination_size.height >= 32 { ...; return; }`, with the existing `#[cfg(not(feature =
"rayon"))]` serial block made unconditional as the fallback. Measured at this line's usual
`400x300->32x24` scenario (`taskset -c 3` single-core, unpinned multi-core, 24-thread host).

| Filter | vs. true serial (S) | vs. ungated `rayon` (A) |
|---|---|---|
| Triangle | wash both axes | single: **-11.6%** (6/6); multi: **-3.2%** (6/6) |
| CatmullRom | wash both axes | single: **-5.8%** (6/6); multi: **+32.5%/+32.8%** regression (1/6, two runs) |
| Gaussian | wash both axes | single: **-7.8%** (6/6); multi: **+25.0%** regression (0/6) |
| Lanczos3 | wash both axes | single: **-4.1%** (6/6); multi: **+41.2%** regression (0/6) |

## Result

**Mixed, filter-dependent.** Single-core: decisive win vs. ungated `rayon` for all four filters,
wash vs. true serial for all four - consistent with candidate 001's `Nearest` result, since
single-core has no parallelism to lose from the serial fallback. Multi-core diverges sharply:
`Triangle` keeps a small decisive win vs. ungated `rayon`, but `CatmullRom`/`Gaussian`/`Lanczos3`
all show a **decisive regression** (+25% to +41%) vs. ungated `rayon` - for these three filters,
the serial fallback below `dst_height=32` throws away a real multi-core win the ungated dispatch
was already providing (these filters have much wider convolution kernels than `Nearest`, so there
is enough real work at `dst_height=24` to productively use multiple threads). Vs. `main` as it
ships today (no `rayon` feature forwarded downstream at all), this candidate is a wash on both
axes for every filter - it only regresses relative to the *ungated* alternative, not relative to
what ships today.

## Does it change behavior/output?

- **Bit-exact?** Yes, per the original measurement (16 hashes across 4 filters x 4 scenarios,
  matched exactly between gated and ungated builds, including scenarios both below and at/above
  the gate's threshold).
- **Is this a breaking change?** No public API change.
- Project test suite, re-verified this session against current `HEAD`: `cargo test --release
  --features rayon` - **5 passed, 0 failed**. `cargo check --features rayon` compiles clean.

## Why the result looks the way it does

A single flat `dst_height` threshold reused across filters asks the wrong question for filters
with substantial per-row cost: `Nearest`'s regression (candidate 001) was "is there enough total
work to be worth dispatching to any pool at all" (answer: no, below ~32 rows), while
`CatmullRom`/`Gaussian`/`Lanczos3` already have enough real per-row convolution work at the same
row count that the right question is "how many threads should absorb this work" - a tuning problem
a binary serial-or-full-pool gate cannot express. See candidate 003 for the fix (a per-filter
capped-size thread pool instead of a binary gate).

## What's not done / open questions

- This candidate is superseded in this workspace by candidate 003 (per-filter capped thread pool)
  and candidate 004 (that plus a low row-count floor) - both fix the multi-core regression found
  here. This candidate's own folder is kept as a record of the regression, not as a recommended fix.
- `Triangle` was the only one of the four filters where the flat gate was a genuine (if small)
  multi-core win, not just single-core - not root-caused beyond "narrower/cheaper kernel than the
  other three" (candidate 018/019 in `image`'s own workspace made the same observation).
- Not independently re-benchmarked against current `HEAD` in this session - only correctness and
  build/test were re-verified here; timing numbers are historical and directional only.
