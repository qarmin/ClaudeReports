# 006. `flatten` - four `#[inline]` hints across a module boundary

`kurbo::flatten` is the most-called entry point in the library for any renderer, and profiling put
`QuadBez::estimate_subdiv` at **40.7% of the `flatten` scene's self time** - the largest single
untouched symbol anywhere in this session's profiles.

The fact that it shows up as a *separate symbol at all* is the finding. `estimate_subdiv` and
`determine_subdiv_t` are `pub(crate)` in `kurbo/src/quadbez.rs`, and their only caller is
`bezpath::flatten` in a **different module**. Neither carries `#[inline]`, and neither do the two
private helpers they call (`approx_parabola_integral`, `approx_parabola_inv_integral`). Release
builds use 16 codegen units by default, so without an explicit hint these can land in a different
CGU from their caller and never be inlined - which is exactly what the profile shows.

This is HEURISTICS.md's "small hot function missing `#[inline]`, called across a module boundary"
pattern, the same shape as [image-rs/image#1761](https://github.com/image-rs/image/pull/1761).

Toolchain: `rustc 1.98.0 (88d9e12ae 2026-08-18)`, `cargo 1.98.0 (797e8a9bc 2026-08-05)`.

## What was tried

Four `#[inline]` attributes (plain `#[inline]`, not `#[inline(always)]` - the hint, not the
mandate), plus a three-line comment on `estimate_subdiv` explaining why it needs one. **Seven added
lines, zero deleted, no logic touched.**

| function | file | why |
|---|---|---|
| `QuadBez::estimate_subdiv` | `quadbez.rs` | only caller is `bezpath::flatten`, another module |
| `QuadBez::determine_subdiv_t` | `quadbez.rs` | same |
| `approx_parabola_integral` | `quadbez.rs` | called 2-3x per `estimate_subdiv` |
| `approx_parabola_inv_integral` | `quadbez.rs` | called 2x per `estimate_subdiv`, 1x per `determine_subdiv_t` |

### Wall clock (`paired.sh`, `RUNS=15`)

`RUNS=9` was tried first and returned `NOT consistent` on several rows at host load ~9.5; raising
the pair count to 15 resolved all four:

| Variant | single-core p50 (`taskset -c 2`) | multi-core p50 (unpinned) |
|---|---|---|
| baseline, `flatten` (200 iters) | 0.769 ms | 0.700 ms |
| patched, `flatten` | 0.648 ms (**-15.7%**), paired **14/15** `consistent` | 0.512 ms (**-26.9%**), paired 14/15 `consistent` |
| baseline, `flatten-fine` (40 iters) | 5.201 ms | 4.741 ms |
| patched, `flatten-fine` | 4.442 ms (**-14.6%**), paired 14/15 `consistent` | 3.440 ms (**-27.4%**), paired 14/15 `consistent` |

Earlier `RUNS=9` rows, for completeness: `flatten-small` -33.1% (7/9, not consistent) / -29.3%
(8/9, consistent); `mixed-flatten` -10.7% (7/9) / -21.5% (6/9), both not consistent. Directionally
the same, but only the `RUNS=15` rows above should be quoted.

Independently reproduced from a fresh clone at the pinned commit by this folder's own `just test`,
which was the strongest measurement of the set: `flatten` 0.667 -> 0.496 ms (**-25.7%**) and
`flatten-fine` 4.356 -> 3.277 ms (**-24.8%**), both **15 of 15 pairs**, `consistent`, with `CHECK`
identical between baseline and patched on both scenes.

### Deterministic instruction counts (`valgrind --tool=callgrind`, `<scene> 5`)

| scene | baseline `Ir` | patched `Ir` | delta |
|---|---|---|---|
| `flatten` | 34,685,305 | 32,845,315 | **-5.31%** |
| `flatten-fine` | 247,056,104 | 237,218,119 | **-3.98%** |
| `mixed-flatten` | 30,037,113 | 28,860,952 | **-3.92%** |

Checked for collateral damage on scenes this shouldn't touch:

| scene | baseline `Ir` | patched `Ir` | delta |
|---|---|---|---|
| `stroke` | 742,514,890 | 742,514,735 | -0.00002% (unchanged) |
| `simplify` | 17,747,681 | 17,553,575 | -1.09% (small gain; `simplify` flattens internally) |

Binary size **went down**, 8,112,680 -> 8,090,552 bytes: inlining let LLVM drop the outlined
copies, so there is no code-bloat cost to weigh against the win.

## Result

A real win of **-14.6% to -15.7% single-core** and **-26.9% to -27.4% multi-core** on kurbo's
most-called entry point, from a **7-line, logic-free, bit-exact** change, with 14 of 15 paired runs
agreeing in all four comparisons. Dimension measured: time (primary), instruction count and binary
size (secondary). Allocations unchanged.

**The wall-clock win is 3-5x the instruction-count win, and that gap needs stating rather than
smoothing over.** Plausible mechanism: `estimate_subdiv` returns a 5-field `FlattenParams` struct,
so every un-inlined call round-tripped that struct through memory, and `sqrt_tol` - loop-invariant
in `flatten` - could not be constant-propagated across the boundary. Removing a call plus a struct
return per quadratic is worth far more in cycles (store-to-load forwarding, scheduling, register
allocation) than it is in retired instructions. Candidate 001 showed the same asymmetry for a
different reason. Instruction counting systematically under-reports this class of win; the
`RUNS=15` paired wall-clock figures are the ones that describe the actual effect.

Some caution is still warranted on the exact magnitude: the host was at load ~9.5 throughout, and
the multi-core figures in particular (-27%) come from a machine whose own baseline drifted during
the session. The *direction* and *consistency* are not in doubt (14/15, four times over, plus a
deterministic instruction-count confirmation); the precise percentage would be worth re-taking on a
quiet host.

## Does it change behavior/output?

- **Bit-exact?** **Yes.** `CHECK` checksums compared across **all 25 harness scenes**: 25
  identical, 0 differing. `#[inline]` cannot change floating-point results in Rust - there is no
  fast-math, and inlining does not license reassociation or contraction.
- **Is this a breaking change?** No. No signatures, no logic, no public items changed - all four
  functions are `pub(crate)` or private. `#[inline]` on a non-generic function does increase what
  goes into crate metadata, which is a (small) compile-time cost, not a semantic one.
- Project test suite: `cargo test --workspace --all-features` - **252 passed / 0 failed before,
  252 passed / 0 failed after**; 4 `test result` lines both times, `grep -cE '^error'` = 0 both
  times.
- `cargo build -p kurbo --no-default-features --features libm`: succeeds, 0 errors.
- `cargo clippy --workspace --all-features`: only the pre-existing `unneeded late initialization`
  warning in `kurbo/src/common.rs`. No new warnings.

## Why the result looks the way it does

Rust does not inline across codegen units without a hint. A `pub(crate)` function whose only caller
sits in another module is precisely the case where LLVM has no opportunity: `cargo`'s default 16
CGUs for a release build put `quadbez.rs` and `bezpath.rs` code in potentially different units, and
`#[inline]` is what makes the function's IR available to every unit that needs it. The profile
proved this had happened - `estimate_subdiv` appearing as its own 40.7% symbol is only possible if
it was a real call.

`approx_parabola_integral` in particular is a ~3-operation expression
(`x / (1.0 - D + (D.powi(4) + 0.25*x*x).sqrt().sqrt())`) that is called two or three times per
`estimate_subdiv`; leaving it outlined meant a function call per parabola integral on the hottest
path in the library.

This also explains why the win is larger on `flatten` (-15.7%) than the raw share of
`estimate_subdiv` alone would suggest: `determine_subdiv_t` and both helpers are called from the
inner subdivision loop too, so the change removes calls at several nesting levels at once.

## What's not done / open questions

- **The exact wall-clock percentage should be re-taken on a quiet host** (load was ~9.5 throughout).
  The direction is confirmed four ways plus deterministically; the magnitude is the uncertain part.
- `CubicBez::subsegment` also appeared as a separate symbol (23.9% of the `flatten` scene) and has
  no `#[inline]` either. Candidate 002 removed half its *calls* from `to_quads`; whether adding
  `#[inline]` to `subsegment` (or to `ToQuads::next`, which does carry `#[inline]` already) gives a
  further win on top was not measured. Added to `IDEAS.md`.
- A broader sweep for the same pattern elsewhere in the crate (`pub(crate)`/private hot functions
  whose callers live in another module) was not done - this candidate only covers the four
  functions the `flatten` profile pointed at. Added to `IDEAS.md`.
- Stacks with candidate 002 (same scene, different mechanism - 002 removes calls, this makes the
  remaining ones free); the combination was not measured together.
- `#[inline(always)]` was deliberately not tried. Plain `#[inline]` was sufficient here, and
  HEURISTICS.md is explicit that the stronger form needs its own justification.
