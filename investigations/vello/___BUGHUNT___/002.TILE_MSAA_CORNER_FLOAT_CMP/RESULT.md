# tile.rs exact float equality for tile-corner detection (float_cmp)

`sparse_strips/vello_common/src/tile.rs`'s `make_tiles_msaa` (~lines 1099-1245, the `push_edge!`/
`process_row!` macros) computes `row_top_x = p0_x + (row_top_y - p0_y) * x_slope` and compares it
with exact `f32` equality (`row_top_x == x_right_f` / `== x_left_f`) to detect whether a line
touches a tile corner, feeding the `trc`/`tlc`/`brc`/`blc` corner bits of the tile's winding/AA
bitmask. Flagged by `cargo clippy --workspace --all-targets --all-features -- -W clippy::pedantic`
(`clippy::float_cmp`); the open question for this session was whether the resulting wrong branch
produces visibly different (wrong) *rasterization output* for a real path, not just a differing
internal bitmask.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value construction against the project's own public API (`Tiles::make_tiles_msaa`), release build | Concrete integer-pixel line `p0=(0,0)`, `p1=(248,56)` (tile-space dx=62, dy=14 - the exact case cited in the candidate report), `k=7` row boundary | Confirmed: `7.0_f32 * (62.0/14.0)` computes `30.99999809`, not the mathematically exact `31.0` (`(7*62) as f64 / 14.0 == 31`). `as u16` truncates this to `30`. Reproduces deterministically, 3/3 runs (once via `just test`, twice more via direct `cargo run --release`) - pure floating-point arithmetic, no randomness/seed involved. |
| Whole-workspace reachability grep (`grep -rln make_tiles_msaa`, `grep -rln "msaa\|Msaa\|MSAA" sparse_strips/`) | Static, no build | `make_tiles_msaa` is called from exactly two places in the entire workspace: its own `#[cfg(test)]` module in `tile.rs` (as an msaa "ground truth" oracle compared against `make_tiles_analytic_aa`), and `vello_bench/src/tile.rs` (a Criterion timing benchmark that never inspects the tile output for correctness). No feature flag, no shader, no other crate (`vello_cpu`, `vello_hybrid`, `vello_toy`) references it. `StripGenerator::generate_filled_path`/`generate_stroked_path` (the only place that turns a `Tiles` buffer into actual pixels) calls `make_tiles_analytic_aa` exclusively. |
| Real end-to-end render via the project's own public API (`RenderContext::fill_path` -> `flush` -> `render_with` -> `Pixmap`), release build | A right-triangle `BezPath` whose hypotenuse is exactly the `p0=(0,0)`,`p1=(248,56)` edge from above (`EXACT`), rendered side-by-side with the same triangle nudged to `p1=(248.3,56)` (`CONTROL`, no exact tile-grid alignment); alpha values dumped for pixel columns 112-144, rows 26-33 | No visible artifact: both renders show a smooth, monotonic antialiasing ramp along the diagonal edge; the "phantom" tile (30,7) region (pixel x=120-123, y=28-31) has alpha `0` in both cases, matching the correct geometric answer. Deterministic, 2/2 repeat runs plus the original `just test` run (3/3 total). |

Existing project unit tests for this file (`cargo test -p vello_common --lib tile:: --release`, in
the pristine worktree, before any change) were also run once as a sanity baseline: **60 passed; 0
failed** (`sparse_strips/vello_common`'s full `cargo test -p vello_common --lib --release` at the
same commit: **159 passed; 0 failed; 1 ignored**) - included here for completeness even though no
patch was attempted (see "Status" below).

## Status

**Not reproducible** (as a visible rasterization bug for a real rendered path).

The underlying `float_cmp` claim itself *is* real and was independently confirmed with the
project's own code (`Tiles::make_tiles_msaa`, public API): for `dx=62, dy=14, k=7`, the f32
computation of `row_top_x` at the row-7 boundary is `30.99999809` instead of the mathematically
exact `31.0`, one ULP short, which does make the exact-equality corner checks (`trc`/`tlc`/`brc`/
`blc`) at `tile.rs:1136-1139` miss the corner and take a different internal branch - this matches
the ~4.8% ULP-off rate and the specific `dx=62,dy=14,k=7` example cited in the candidate report.

However, that wrong branch cannot currently produce a visible rendering difference for any real
path, because `make_tiles_msaa` (the only function containing this exact-equality corner logic) is
not reachable from any image-producing code path in this workspace - see the reachability grep
above. The actual rendering pipeline (`StripGenerator` -> `strip::render` -> `Pixmap`) exclusively
uses `make_tiles_analytic_aa`, which computes pixel coverage via a fundamentally different,
continuous-math algorithm (`calc_fractional_coverage`, sampling four sub-rows per tile from the
raw, unrounded `row_left_x`/`row_right_x` floats) and never evaluates the flagged `trc`/`tlc`/
`brc`/`blc` expressions at all - those lines exist only inside `make_tiles_msaa`. A real end-to-end
render of the exact triggering edge through the actual public rendering API confirmed this
directly: no artifact appears at the affected tile.

As a secondary check (see the second row of the table above), the same `x_slope`-based
tile-column truncation mechanism (not the flagged exact-equality bits themselves, but the shared
`row_top_x`/`row_bottom_x` computation and its `as u16` truncation) *is* structurally shared
between `make_tiles_msaa` and `make_tiles_analytic_aa`, and both independently place a tile entry
at column 30 of row 7 for this exact input (a column the true continuous geometry of row 7 never
touches). This looked initially like a second, deeper bug, but the real-render check in step 3
shows it doesn't translate into a wrong pixel either: `make_tiles_analytic_aa`'s tile entries here
carry only the winding bit (no L/R/T/B semantics - `check_analytic_aa_matches`'s own existing test
helper only ever compares the winding bit for exactly this reason), and the actual per-pixel
coverage number for that phantom column comes from the continuous `calc_fractional_coverage`
computation, not from the discrete tile label, so it correctly evaluates to zero there regardless
of which column the bookkeeping tile happened to be filed under.

**Fully reproducible**: N/A (no bug to reproduce - see Status; the *investigation itself* is fully
reproducible on demand via `just test`, which was verified to run end-to-end against a pristine
clone of the base commit, 1/1 clone-and-run plus 2 additional direct reruns of the built binary,
3/3 total, all producing byte-identical output).

## Class

N/A (not Confirmed).

## Root cause

Not a bug in the sense originally suspected. The `clippy::float_cmp` lint is a legitimate style/
robustness observation about `make_tiles_msaa`'s corner-detection code (exact equality on a value
derived from a division is inherently ULP-fragile, and the reporter's own independent brute-force
sweep and this session's targeted repro both confirm real misfires at grid-aligned crossings). It
does not currently manifest as a rendering defect because `make_tiles_msaa` is prep/reference code
for a not-yet-implemented downstream consumer: its own doc comment states its purpose is to
"establish a source of ground truth for line-tile intersections" for "downstream rasterization
(MSAA)" that "occurs in parallel" - i.e. a future MSAA sample-point rasterizer this codebase
doesn't have yet (introduced by PR #1319, "Separate make_tiles into AA and MSAA versions. Fully
cases out MSAA version", commit 4ea47b47). Today it is exercised only by its own unit tests (whose
own cross-check helper, `check_analytic_aa_matches`, deliberately only compares the winding bit,
not the corner bits this candidate is about) and by a Criterion benchmark that never inspects
correctness.

## Introduced by

Not determined for the specific `float_cmp` lines - not worth a full `git blame`/bisect this
session given the Status is Not reproducible (per SKILL.md step 5, this lookup is optional and
skipped once a candidate resolves to a non-bug). For context only: the surrounding `make_tiles_msaa`
function itself was introduced by 4ea47b47 "Separate make_tiles into AA and MSAA versions. Fully
cases out MSAA version (#1319)".

## The fix

No fix attempted or needed - Status is Not reproducible (no visible defect exists to fix in the
current codebase). If a future downstream MSAA consumer of `make_tiles_msaa`'s bitmask is ever
added, the exact-equality checks at `tile.rs:1136-1139` should be revisited then (e.g. compare
`row_top_x` against a `.round()`-based reconstruction, or derive the corner test from the same
integer-domain quantities used to decide `y_top_tiles`/`p0_tile_x` rather than from re-deriving it
through `x_slope` multiplication).

- Project test suite: N/A, no fix was applied. Baseline (`cargo test -p vello_common --lib
  --release`, at the investigated commit, no changes): **159 passed; 0 failed; 1 ignored**.

## What's not done / open questions

- This candidate only checked the `make_tiles_msaa` code path named in the original report. It did
  not attempt to fix the `clippy::float_cmp` lint itself (e.g. by silencing it or restructuring the
  comparison) since doing so with no observable behavior change and no test coverage of the
  affected bits would be an unverifiable, purely cosmetic patch - out of scope for what this
  candidate loop verifies (a reproduced *bug*, not a lint).
- The secondary observation (both tilers place a "phantom" bookkeeping tile at a column the
  continuous line geometry doesn't touch, for exact tile-grid-aligned crossings) is real and
  reproducible in this session's `repro/`, but was checked only for this one input shape/slope
  ratio; it wasn't swept across other `dx,dy` ratios or width/height combinations, and - as shown
  in step 3 above - doesn't currently produce a wrong pixel for `make_tiles_analytic_aa` since
  coverage there comes from continuous math, not the tile's column label. It's plausible a
  *future* consumer that trusts the tile stream's column labels more literally (exactly the kind of
  consumer the `make_tiles_msaa` doc comment anticipates) would need to care about this. Noted as a
  suspect for a future session rather than chased further here.
- Did not attempt `miri`/ASAN (no `unsafe`/UB-shaped code in the affected macros - pure safe-Rust
  float arithmetic) or a fuzz harness (the failure mode is a deterministic, algebraically-derivable
  rounding property of specific `dx`/`dy` ratios, not something a blind fuzzer would characterize
  better than the targeted boundary-value construction already used).
