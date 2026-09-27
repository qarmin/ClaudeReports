# Add a minimum-row gate inside resize_nearest's own rayon dispatch

**Ported from `image` repo's `___OPTIMIZATIONS___/033.PIC_SCALE_SAFE_NEAREST_VENDORED_ROW_GATE/`.**
Originally measured in 2026-08 by vendoring `pic-scale-safe` 0.1.11 into a local copy inside the
`image` project (date not recorded more precisely than the session's own git history) - it never
touched this repo's own tree at the time. **Not re-measured in this workspace**; the numbers below
are copied verbatim from that session. What *was* redone here: the patch itself was rebuilt by
hand against this repo's current `HEAD` (`b50e020`) rather than reused byte-for-byte, since the
original diff was captured against a vendored copy nested under `image`'s own `vendor/` tree.
`git apply --check patch.diff` passes cleanly against current `HEAD`, `cargo check --features
rayon` compiles with no errors, and `cargo test --release --features rayon` still passes 5/5 -
verified in this session (see "Does it change behavior/output?").

`image` (a downstream consumer of this crate) found that `FilterType::Nearest` resizes at small
output sizes (e.g. `400x300->32x24`) are 26-37x slower than a true serial build once this crate's
`rayon` feature is compiled in, because `resize_nearest`'s dispatch calls `par_chunks_exact_mut`
unconditionally whenever the `rayon` feature is present, with no minimum-row/size gate of its own -
the dispatch/join overhead is paid regardless of how large or small the actual per-row work is.

## What was tried

Added a `dst_height >= 32` minimum-row gate inside `resize_nearest` (`src/resize_nearest.rs`):
below the threshold, the function now takes the same serial `chunks_exact_mut` loop it already
uses when the `rayon` feature is off entirely; at or above the threshold, behavior is unchanged
(`par_chunks_exact_mut`, same as today).

Original measurement (from `image`'s workspace, vendored `pic-scale-safe` 0.1.11, `--features
rayon`, `taskset -c 3` for single-core, unpinned for multi-core on a 24-thread host):

| Variant / scenario | single-core p50 | multi-core p50 | notes |
|---|---|---|---|
| S (true serial, `rayon` feature off) | 1.207 us | 0.737 us | `400x300->32x24`, Nearest |
| E (this gate, `--features rayon`) | 1.196-1.194 us (-0.6% to -0.9%, wash vs S) | 0.746-0.753 us (+2.3% to +3.6%, wash vs S) | same scenario, 2 repeat runs |
| A (ungated `rayon`, i.e. today's unconditional `par_chunks_exact_mut`) | 9.170 us (single) / 18.213 us (multi) | | same scenario |
| E vs A | 0.747 us (**-91.8%**, 8/8 pairs) | 0.745 us (**-95.9%**, 10/10 pairs) | decisive in both directions |

## Result

**Closes the regression**: vs. the ungated `rayon` dispatch pic-scale-safe ships today whenever the
`rayon` feature is enabled (`A`), the gate is a decisive ~12x (single-core) to ~24x (multi-core)
win, 8/8 and 10/10 paired runs respectively. Vs. a build with the `rayon` feature off entirely
(`S`), the gate is a wash on both axes (no comparison clears the originating workspace's own
`wins >= n-1`/`<=1` decisiveness bar) - expected, since below the threshold this patch takes
exactly the same serial code path the `rayon`-off build already takes.

## Does it change behavior/output?

- **Bit-exact?** Yes, per the original measurement (a temporary hash-based correctness check
  across `Nearest` on `RgbImage`/`RgbaImage` at `400x300->32x24`, below the gate, and two scenarios
  at/above the gate, all matched exactly between gated and ungated builds).
- **Is this a breaking change?** No public API change - the gate is entirely internal to
  `resize_nearest`'s own dispatch.
- Project test suite, re-verified in **this** session against current `HEAD`: `cargo test
  --release --features rayon` - **5 passed, 0 failed** (this crate's own suite, unrelated to
  `image`'s much larger suite the original measurement quoted). `cargo check --features rayon`
  compiles clean.

## Why the result looks the way it does

`resize_nearest`'s per-row cost is tiny (~1us total for a 24-row output) - `par_chunks_exact_mut`'s
own dispatch/join-tree setup dwarfs it completely at small sizes regardless of core count, so
gating it off below a reasonable row-count threshold is close to a pure win with no real
parallelism given up.

## What's not done / open questions

- The `32`-row threshold was picked as a round number above the one scenario (`dst_height=24`)
  this line of candidates always measured, not independently bisected across a range of sizes.
- Only `Nearest` is gated here - see candidates 002/003/004 in this workspace for the same idea
  applied to the other filters' dispatch sites (`fixed_point_dispatch.rs`/`floating_point_dispatch.rs`),
  where a flat threshold turned out to be actively harmful rather than merely suboptimal.
- This candidate was not independently re-benchmarked against current `HEAD` - only correctness
  (hash-based, historically) and the build/test check (this session) were verified here; the
  timing numbers above are historical, from a different host, and should be treated as directional
  rather than a guaranteed reproduction on any other machine.
