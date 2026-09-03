# ANALYTIC_AREA_TILE_BOUNDARY_HALF_PIXEL_BUMP

Investigates https://github.com/linebender/vello/issues/1556 ("Incorrect edge pixel rendering at
some places"): on a maze-like demo slide, long straight wall edges show periodic "bumps" of about
half a pixel, only when using the "Analytic Area" AA method (goes away with 8x/16xMSAA). The
candidate's own stated hypothesis was that this is caused by "a per-tile-boundary discontinuity in
the analytic coverage computation reused across adjacent tiles" - i.e. a defect specific to how
`vello`'s sparse-strip/tiled analytic-coverage renderer carries state across its internal tile
grid.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read the actual GitHub issue and its comment thread (`gh api repos/linebender/vello/issues/1556` and `/comments`) | n/a | Maintainer `raphlinus`: "The first image is explicitly crafted to highlight conflation artifacts, which appear when doing analytic AA but not MSAA. So that's working as intended." Maintainer `LaurenzV`: "The artifacts... are not desired behavior, but they are inevitable if you use analytic anti-aliasing, so it's not a bug." Both attribute the effect to **conflation artifacts** (a known, general limitation of compositing independently-antialiased shapes), not to anything tile-grid-specific. |
| Read `sparse_strips/vello_common/src/strip.rs`'s `render_impl` (the CPU sparse-strip analytic-area coverage algorithm) end to end, focused on state carried across a tile-row/tile-location transition (`accumulated_winding`, `location_winding`, `winding_delta`) | Code reading only | The algorithm carries `accumulated_winding` forward continuously across tile-row transitions (splatted into the next `location_winding`) and accumulates `winding_delta` from every line crossing a tile's top edge for the *whole path*; nothing is reset or discarded specifically "at a tile boundary" for a single, continuous path. No structural discontinuity found by inspection. |
| Manual boundary-value construction, standalone `vello_cpu` program (`repro/src/main.rs`, "Test 1"): built a maze-like concave wall T-junction (`union_path`, one horizontal + one vertical rect touching edge-to-edge, not overlapping) two ways - (a) as ONE closed path (ground truth) and (b) as TWO separate `fill_path` calls with the same opaque color composited normally (as independently-drawn wall segments would be) - then diffed the two renders while sweeping the junction across sub-pixel x offsets (0.0-0.9) **and** across every possible offset relative to the 4px tile grid (`tile_shift` 0-8 and 12, i.e. every residue mod 4 plus a full extra tile) | `vello_cpu` (real SIMD `Level`, release build), 40x40 px target, deterministic (no randomness) | A real discrepancy exists between (a) and (b): max per-byte diff of **54/255** at the concave junction. Critically, this diff is **exactly 54 for every single `tile_shift` value tested** (0 through 8, and 12) - i.e. it makes zero difference whether the junction lands exactly on a tile-row boundary or squarely mid-tile. This directly refutes "reused across adjacent tiles" as the mechanism: a per-tile-boundary bug would be expected to vary with alignment to the tile grid; this doesn't, at all. |
| Manual boundary-value construction, same program ("Test 2"): rendered a SINGLE path (one `fill_path` call, no compositing of separate shapes at all) with a fractional (`x=5.37`) vertical edge spanning a target 40px tall (10 tile rows), then read the alpha byte at the boundary column for every one of the 40 rows | Same build | All 40 rows produced the **exact same alpha byte (161)**, with **zero variation** at any tile-row boundary (rows 4, 8, 12, ..., 36). This rules out even a *plain* (non-conflation) per-tile-boundary coverage bug for a single continuous path. |

## Status

**Not reproducible** (as the specific hypothesis stated: a per-tile-boundary discontinuity in the
analytic coverage computation reused across adjacent tiles).

Reproduced the underlying *visual* artifact itself deterministically (3/3 runs, byte-identical
output each time, release build, `vello_cpu`, real SIMD dispatch via `Level::new()`) - the "bump"
described in the issue is real and was reconstructed here as a concave-corner discrepancy of
54/255 in the alpha channel between a geometrically-correct single-path render and the same shape
built from two independently-composited touching rects. But the *mechanism* is not what this
candidate hypothesized: the discrepancy's magnitude is bit-for-bit identical across every tested
offset relative to the 4x4 tile grid, so it cannot be a defect in how coverage state is "reused
across adjacent tiles." It matches, instead, exactly the mechanism the project's own maintainers
already identified on the linked issue: a **conflation artifact** - an inherent property of
compositing two separately-antialiased shapes that meet at a concave (reflex) corner without being
boolean-unioned into one path, present in essentially any analytic/exact-coverage AA renderer, not
specific to `vello`'s tiling.

**Fully reproducible**: Yes. `repro/` runs the real, public `vello_cpu::RenderContext` API
end-to-end (real path flattening, tiling, strip generation, and compositing) - there is no second
participant to fake here (this is a single-process rendering call on caller-supplied geometry, the
same shape of input a real scene-building caller would hand it), so `CONFIRMATION.md` section 11's
two-participant requirement does not apply.

## Class

N/A (not Confirmed as a bug against this candidate's stated hypothesis). For the record, the
underlying visual phenomenon investigated here would be classified **logic**/**visual-artifact**,
but it is the project's own maintainers' documented, intentional trade-off of analytic-area AA
(conflation artifacts), not a defect.

## Root cause

The candidate's hypothesized mechanism (state incorrectly reused/discontinuous across the internal
tile grid) was checked directly and does not hold - see "What was tried" above (both a code
read of the exact state-carrying variables across tile-row transitions, and the tile-alignment
sweep in Test 1 producing an identical diff regardless of alignment).

The actual mechanism behind the real, reproducible artifact is a **conflation artifact**: when two
axis-aligned rectangles are drawn as *separate* fills that only touch (share a boundary,
non-overlapping) and meet at a concave/reflex corner - exactly how two maze wall segments meeting
at a T-junction would be authored - each shape's analytic (exact fractional) edge coverage is
computed independently, then composited with ordinary "source-over" alpha blending. For a pixel
straddling the concave corner, both shapes' *own* coverage fractions correctly describe their own
contribution, but combining two independent partial coverages `a1`, `a2` via `over` compositing
(`a2 + a1*(1-a2)`) understates the true union coverage of two disjoint sub-pixel regions relative
to what a single, correctly-unioned path would compute directly (which instead sums crossing
counts across the whole boundary in one signed-area accumulation, per `strip.rs`'s comment: "The
fractional area coverage of a pixel is the integral of the winding within it"). This is a
general property of analytic/exact-coverage AA plus independent per-shape compositing - it is not
particular to `vello`'s 4x4 (`vello_cpu`)/tile-based (GPU `vello`) internal grid, which is exactly
why the diff in Test 1 doesn't move at all when the corner is shifted relative to that grid. MSAA
(mentioned in the original issue as making the bumps disappear) sidesteps this because it samples
a shared, discrete sample lattice per pixel across both shapes rather than computing an exact
analytic area per shape independently.

## Introduced by

Not applicable - not a code defect (see Status/Root cause above); nothing to bisect to an
introducing commit. This is the intended behavior of a signed-area/exact-coverage AA algorithm
composited without global path merging, present by design since analytic-area AA was implemented.

## The fix

No fix applicable. Per the linked issue's own maintainer responses, avoiding this artifact for
scenes that touch multiple shapes at shared edges (like the maze demo) requires either (a) using
MSAA instead of Analytic Area AA, or (b) constructing the scene as a single, correctly-unioned path
per contiguous region instead of many separately-filled adjacent shapes - both are scene-authoring
or renderer-selection decisions for a caller to make, not a bounded code change to `vello`/
`vello_cpu` itself.

- Project test suite (base_commit 7516831426fb70f36d62c521c5477f6a7265b36a, release, no changes
  made - nothing to fix): `cargo test -p vello_common --release` -> **159 passed; 0 failed; 1
  ignored**. `cargo test -p vello_cpu --release --lib` -> **81 passed; 0 failed**.
  `cargo test -p vello_sparse_tests` (the real rendering-snapshot suite): **N/A this session** -
  its reference PNGs are Git-LFS-tracked and this sandbox has no `git-lfs` binary (see
  `___BUGHUNT___/README.md`'s documented workaround/limitation).

## What's not done / open questions

- Only `vello_cpu` (the CPU sparse-strip backend) was directly exercised. The actual web demo
  referenced in the issue uses the GPU compute-shader renderer (`vello`/`vello_shaders`, e.g.
  `fine.wgsl`'s `fill_path`/`backdrop.wgsl`'s prefix-sum winding propagation across tiles), which
  was only checked by reading the WGSL source (the same continuous-state-carrying pattern as the
  CPU path, no discontinuity found by inspection) - not exercised with a real GPU/software Vulkan
  adapter, since none was available in this sandbox (no `vulkaninfo`, no `nvidia-smi`, no
  registered ICD). The conflation-artifact mechanism identified here is a property of *composited
  independent shapes*, which applies equally to both backends' algorithms in principle, but this
  was not independently confirmed end-to-end on the GPU path.
- Slide 14 of the same web demo (mentioned in the same GitHub issue, "flowers" with too-dark edges
  and stray red/purple/yellow pixels) was not investigated here - the issue reporter and
  maintainers treat it as a possibly-separate phenomenon (lack of gamma-correct blending for the
  "too dark" part; the stray colored pixels remain unexplained even by the reporter). Not part of
  this candidate's scope (which was specifically the maze wall "bumps").
- Did not attempt to quantify whether 54/255 is the worst case across all possible junction angles/
  geometries - only right-angle (T-junction) corners at a handful of sub-pixel offsets were swept.
  A different concave-corner angle could plausibly show a larger or smaller discrepancy, but that
  wouldn't change the finding that the effect is independent of the tile grid.
