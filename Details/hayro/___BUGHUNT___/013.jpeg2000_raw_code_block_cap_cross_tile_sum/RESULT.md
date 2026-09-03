# jpeg2000_raw_code_block_cap_cross_tile_sum

From `___BUGHUNT___/IDEAS.md`'s backlog (noted while root-causing candidate 011): candidate 011's
`MAX_RAW_CODE_BLOCK_ESTIMATE` cap (and candidate 012's own extension,
`MAX_RAW_CODE_BLOCK_LAYER_ESTIMATE`) is checked once per `build_decompositions` call - i.e. once
per *tile* - not once per whole `decode()` call. Neither cap's own patch is merged into the
pristine tree, so this checks whether that specific design choice (per-tile, not per-call) would
leave a real gap if either fix were adopted as-is: a codestream with many tiles, each individually
staying under the per-tile cap, summing to a much larger, effectively unbounded total cost across
one `decode()` call.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read `decode.rs::decode`'s tile loop (`for tile in &tiles { ... decode_tile(...)? ... }`) and `build.rs::build_decompositions` (with candidate 012's own patch.diff applied on top of the pristine tree, since that patch is the most complete of the two unmerged candidate fixes and numerically subsumes candidate 011's) | Manual code read | Confirmed `build_decompositions` is called once per tile (via `decode_tile`, once per loop iteration in `decode()`), and candidate 012's own cap check happens entirely inside that per-tile call, comparing a purely local `raw_code_block_layer_estimate` (computed fresh, from scratch, on every call) against the fixed threshold - nothing carries any running total across tiles. `DecompositionStorage::reset()` (called at the top of every `decode_tile`) does clear all of `storage`'s other bookkeeping vectors per tile, but nothing in the pre-fix code tracks a cumulative cost anywhere. |
| Wrote a small multi-tile J2C codestream generator (`gen_multi_tile.py`, extending candidate 011/012's own `gen.py`/`gen2.py`) producing a real, from-scratch, legitimately SIZ/COD/QCD-parseable codestream with `N` real SOT/SOD tile-parts (not one), each tile 7900x7900 (the same per-tile shape candidate 012 already established sits just under their own 4,000,000-estimate cap at `num_layers=1`), 1 decomposition level, 4x4 code blocks, and a real (not hex-patched, not empty) minimal packet-header bitstream per tile, plus a QCD with a non-zero base exponent (`8`) so `ScalarDerived` quantization's `e_0 - n_ll` subtraction doesn't underflow - unlike candidate 011/012's own PoCs (which deliberately triggered `InvalidExponents` immediately after their own expensive code ran, since a single tile was enough to make their point), this candidate needs `decode()` to *fully succeed* across every tile, since `decode()`'s tile loop propagates any tile's `Err` immediately via `?` and would otherwise mask whether later tiles ever ran their own `build_decompositions` at all | Standalone `hayro-jpeg2000` example harness, `Image::new`/`.decode()` with `DecodeSettings{target_resolution: Some((20,20))}`, candidate 012's own patch.diff applied (no cross-tile fix) | 1/5/20/40-tile codestreams (all built from identical 7900x7900 per-tile shapes) all `decode()` **successfully** (no rejection at any tile count), with time and peak memory both growing ~linearly with tile count: 536ms/926MB (1 tile) -> 1.35s/1.16GB (5 tiles) -> 3.56s/2.06GB (20 tiles) -> 6.91s/3.25GB (40 tiles) - directly confirming the suspected gap: each tile's own estimate independently passes the per-tile check, so the cap never fires regardless of how many tiles are declared. |
| Control: same generator, realistic content (`20000x15000`, 64x64 code blocks, 32 layers, single tile) | Same harness | `decode()` succeeded in 354ms/866MB, both before and after this candidate's own fix (see "The fix" below) - confirms ordinary, non-adversarial content isn't affected by either cap. |
| Implemented a bounded fix (see "The fix"), re-ran the same 1/5/20/40-tile sweep against it | Same harness | 1 tile (always under the cap alone) unaffected: 424ms/926MB, `decode()` still succeeds. 5/20/40 tiles: all now `Err(ValidationError::ImageTooLarge)` in 423-444ms/~942MB - bounded to roughly one tile's worth of work, not scaling with declared tile count. |
| Built a real, standalone, end-to-end `repro/` (own `Cargo.toml`, real `hayro`/`hayro-syntax` public API): a 1071-byte real PDF (classic xref table) with a 20-tile (158000x7900 raw, 7900x7900 per tile) codestream as a real `/JPXDecode` image XObject, placed as a **20x20-on-page thumbnail** (`cm 20 0 0 20 90 90` on a 200x200-`/MediaBox` page), driven through the real, unmodified, public, top-level `hayro::render()` (the same function `hayro/examples/render.rs` itself calls) | `cargo run --release`, 3 separate runs against a fresh clone of the pristine `base_commit` tree (no cap of any kind - candidates 008/011/012's own fixes are all unmerged), and 3 more against the same clone with this candidate's own `patch.diff` applied | Pristine: 3/3 runs, 4.41-5.66s / ~2.36GB RSS to render a 20x20 thumbnail. Patched: 3/3 runs, 505-515ms / ~945MB RSS. Also ran the full `just test` cycle (clone -> reproduce-before -> apply -> reproduce-after) from a completely fresh scratch copy of this candidate folder, confirming the same before/after numbers (4.33s/2.36GB -> 482ms/945MB) with no dependency on the main checkout - see `output.txt`. |

## Status

**Confirmed**. Reproduces 3/3 runs (`--release`, no sanitizer/miri needed - this is a real,
measured CPU-time/memory cost, not memory-safety UB) against the pristine `base_commit` tree (no
cap of any kind present) via the real, public `hayro::render()` entry point, and 3/3 runs complete
in ~505-515ms (and ~945MB peak RSS, down from ~2.36GB/4.4-5.7s) against a tree with this
candidate's own patch applied. The specific finding this candidate set out to check - that a
per-tile-only cap (as candidate 011's and candidate 012's own unmerged patches implement it) does
not bound the cumulative cost across many tiles in one `decode()` call - is separately confirmed
via the boundary sweep above (candidate 012's patch applied *alone*: cost scales ~linearly with
tile count, 536ms/926MB at 1 tile up to 6.91s/3.25GB at 40 tiles, with the cap never firing at any
tile count tested).

**Fully reproducible**: **Yes** - `repro/` builds a syntactically real PDF (classic xref table,
`%PDF-1.7` header, catalog/pages/page/image XObject) with a real, legitimately SIZ/COD/QCD-marker-
parseable JPEG2000 codestream (built from scratch via a small generator script, not hex-patched
from an existing fixture, but structurally the same kind of thing any encoder could legitimately
emit for these header field values, including a genuine, minimally-encoded, real non-empty
packet-header bitstream per tile, and a non-degenerate QCD so decode genuinely succeeds rather than
erroring out early) as its `/JPXDecode` stream, placed via an ordinary `cm` operator, and drives it
through the real, public, top-level `hayro::render()` - the exact same entry point
`hayro/examples/render.rs` calls, with the real CTM-derived downscale hint computed by the
unmodified renderer. There is no second live participant to fake here (a malformed/adversarial
file handed to a renderer's own public entry point is the genuine external input), same carve-out
as candidates 003/004/006/008/009/010/011/012 - see `CONFIRMATION.md` section 11's "single-process
repro remains completely fine". The candidate's `justfile`'s `just test` was actually run (not just
read) from a fresh scratch copy and genuinely reproduces both states from a real `git clone`, not a
description of what it would do.

## Class

`security` (resource-exhaustion DoS - CPU time and peak memory both scale with a codestream's
declared *tile count*, independent of both the requested/CTM-derived downscale hint and of any
single-tile-scoped cost cap, since each tile's own contribution is checked, and passes,
independently).

## Root cause

`hayro-jpeg2000/src/j2c/decode.rs::decode` loops over every declared tile
(`for tile in &tiles { ... decode_tile(tile, ...)? ... }`), calling `build::build` ->
`build_decompositions` once per tile via `decode_tile`. Both candidate 011's
`MAX_RAW_CODE_BLOCK_ESTIMATE` and candidate 012's `MAX_RAW_CODE_BLOCK_LAYER_ESTIMATE` (neither
merged into the pristine tree) compute their cost estimate and compare it against the threshold
entirely *within* that single `build_decompositions` call, using a `let mut ... = 0` local that is
recomputed from scratch on every call - nothing persists or accumulates across the tile loop in
`decode()`. Because `Tile::new` derives each tile's own rect purely from the codestream's `SIZ`
marker (`XTsiz`/`YTsiz`/`XTOsiz`/`YTOsiz`, independent of tile *count*), and `SIZ`'s own `num_tiles()`
(`num_x_tiles() * num_y_tiles()`, both computed by dividing the image's declared size by the tile
size) is only guarded against integer overflow, not against being large, a codestream can legally
declare many tiles, each individually small enough that its own `raw_code_block_layer_estimate`
sits comfortably under either cap's threshold, while the codestream's *total* declared tile count
(and therefore the total real work `decode()` performs, since `decode_tile` fully executes
`build::build` -> `segment::parse` -> bitplane decode -> IDWT -> store for every declared tile in
sequence) is unbounded. `DecompositionStorage::reset()` clears the per-tile bookkeeping vectors
(`layers`/`code_blocks`/`precincts`/etc.) at the start of every `decode_tile` call, which keeps
*peak* memory closer to "one tile's worth" than a naive sum in some cases, but does nothing to
bound the *cumulative CPU time* spent redoing that same per-tile work once for every one of an
attacker-chosen number of tiles - and, as measured above, peak RSS itself still grows measurably
with tile count too (926MB at 1 tile up to 3.25GB at 40 tiles), likely from `Vec` capacity growth
across tiles of otherwise-identical size not being reclaimed by `clear()`.

## Introduced by

b56542a "hayro-jpeg2000: Complete restructuring and refactoring of JP2 processing (#610)" (2025-12-03) - `decode.rs` (including the per-tile `decode_tile` loop with no cross-tile accumulation) was created fresh by this rewrite; a later commit, 78056f2 "hayro-jpeg2000: Allow decoding images using an existing buffer and context (#1048)" (2026-03-07), only changed how the loop's context is threaded through, not its lack of cross-tile bookkeeping.

## The fix

Extended candidate 012's own per-tile estimate into a cumulative one that persists for the
duration of a whole `decode()` call, not just one tile:

- Added a new field, `raw_code_block_layer_total: u64`, to `DecompositionStorage` - deliberately
  **not** cleared by `DecompositionStorage::reset()` (which runs once per tile, at the top of every
  `decode_tile` call), so it survives across the whole tile loop in `decode()`.
- Reset it to `0` exactly once per `decode()` call, in `DecoderContext::reset` (which already runs
  exactly once per `decode()` call, before the tile loop begins, to set up `channel_data`).
- In `build_decompositions`, instead of comparing the current tile's own
  `raw_code_block_layer_estimate` directly against the threshold, it's now added
  (`saturating_add`) into `storage.raw_code_block_layer_total`, and *that* running total is what's
  compared against `MAX_RAW_CODE_BLOCK_LAYER_ESTIMATE`.

This is a minimal, additive change on top of candidate 012's own (still-unmerged) patch - it does
not change the per-tile estimate formula itself (still each component's raw top resolution level's
area divided by its actual configured code-block area, multiplied by the tile's own layer count),
only *where* the running total is stored and *when* it's reset. A single legitimately large tile
still behaves identically to before (its own contribution is compared against the same threshold,
just via the running total rather than a fresh local variable); a codestream with many tiles now
has its *combined* cost bounded by the same threshold, rather than each tile getting an
independent "budget".

```diff
@@ build.rs @@
 fn build_decompositions(...) -> Result<()> {
     let mut total_coefficients = 0_usize;
     let mut raw_code_block_layer_estimate = 0_u64;
     for component_tile in tile.component_tiles() {
         ...
         raw_code_block_layer_estimate =
             raw_code_block_layer_estimate.saturating_add(component_estimate);
     }
+    storage.raw_code_block_layer_total = storage
+        .raw_code_block_layer_total
+        .saturating_add(raw_code_block_layer_estimate);
-    if raw_code_block_layer_estimate > MAX_RAW_CODE_BLOCK_LAYER_ESTIMATE {
+    if storage.raw_code_block_layer_total > MAX_RAW_CODE_BLOCK_LAYER_ESTIMATE {
         bail!(ValidationError::ImageTooLarge);
     }

@@ decode.rs @@
 impl DecoderContext<'_> {
     fn reset(&mut self, header: &Header<'_>, initial_tile: &Tile<'_>) -> Result<()> {
         self.tile_decode_context.reset();
         self.storage.reset();
+        self.storage.raw_code_block_layer_total = 0;
         ...

 pub(crate) struct DecompositionStorage<'a> {
     ...
+    pub(crate) raw_code_block_layer_total: u64,
 }
```

See `patch.diff` for the full, self-contained diff against the pristine `base_commit` tree (it
does not require candidate 011's or candidate 012's own patch.diff to be applied first - it
implements the equivalent of candidate 012's own check plus this candidate's cross-tile
accumulation in one combined change).

Verified against more than the one triggering input, per `CONFIRMATION.md` section 7: re-ran the
1/5/20/40-tile sweep (only the 1-tile case - already under the cap on its own - still succeeds;
5/20/40 are all now rejected, each in ~423-444ms/~942MB regardless of declared tile count) and the
realistic-content control case (still accepted, unaffected: 354ms/866MB, identical to before this
fix); re-ran the real end-to-end `repro/` 3 times against the pristine tree (3/3 slow,
4.4-5.7s/~2.36GB) and 3 times against the patched tree (3/3 fast, ~505-515ms/~945MB) - see
`output.txt`. Also actually ran `just test` from a fresh scratch copy of this candidate folder
(fresh `git clone` of the pristine tree, not the main checkout) and confirmed the same before/after
behavior end to end (4.33s/2.36GB -> 482ms/945MB).

- Project test suite:
  - `cargo test -p hayro-jpeg2000 --release`: unit tests **1 passed / 0 failed** before and after
    (identical). The crate's separate `asset_suite` integration test fails identically before and
    after (`Failed to read asset directory: missing test input serenity/buggie-gray.jpf`) - a
    pre-existing environment limitation (downloaded test assets not present in this checkout, same
    as candidates 004/006/008/009/011/012 already documented), not a regression.
  - `cargo test -p hayro-tests --release --test tests -- load::`: **112 passed / 0 failed**,
    identical before and after (including `load::issue391`, the specific regression candidate
    011's own first, reverted, fix attempt broke - this fix does not touch which resolution levels
    get precinct/code-block structures built, only adds an earlier, cumulative, purely size-based
    rejection, so it doesn't reintroduce that regression).
  - `cargo test -p hayro-tests --release --test tests -- render::`: **266 passed / 1259 failed**,
    identical count before and after - the failures are the same pre-existing environment
    limitation (missing downloaded reference PDFs/snapshots) candidates 004/006/008/009/011/012
    already documented, not caused by this change.

## What's not done / open questions

- `MAX_RAW_CODE_BLOCK_LAYER_ESTIMATE = 4_000_000` (unchanged from candidate 012's own choice) is
  now a **whole-image** budget rather than a per-tile one. This is a meaningfully different
  real-world tradeoff a maintainer should be aware of: a *legitimate* multi-tile image whose
  individual tiles are each modest but whose combined raw-top-resolution-level/code-block-size/
  layer-count estimate exceeds 4,000,000 in total would now be rejected, even though no single
  tile would have been rejected under candidate 011/012's own per-tile design. This wasn't
  measured against a corpus of legitimate real-world multi-tile JPEG2000 images this session (none
  were available in this checkout - same missing-test-asset limitation noted above) - a maintainer
  adopting this fix should re-check the threshold against real multi-tile content before shipping
  it, not just against the control case's single-tile 20000x15000/64x64/32-layer shape tested here.
- Like candidate 011/012's own estimate, this still only accounts for each component's **top**
  raw resolution level's area (not summing the smaller, geometrically-decaying contributions from
  intermediate raw resolution levels) - not changed by this candidate, same caveat both prior
  candidates already documented.
- This candidate's fix is against a **hypothetical** state of the tree (candidate 011's/012's own
  caps having been merged) - as of the pristine `base_commit` tree checked in this session, no
  cap of any kind exists yet for this code path (candidates 008/011/012's own fixes are all
  unmerged), so `patch.diff` here stands alone as a complete, self-contained replacement for both
  candidate 012's per-tile check and this candidate's cross-tile accumulation, not as a diff meant
  to be layered on top of an already-applied 012 patch.
- Only checked JPEG2000/`hayro-jpeg2000`; did not check whether any of this project's other
  format decoders (JBIG2, CCITT, DCT/JPEG - several already have their own confirmed
  dimension/resource caps from earlier candidates in this session, see candidates 003/004/006/008)
  have an analogous per-call-but-not-per-request-total gap for a multi-"tile"/multi-strip/
  multi-segment format shape - not investigated this session.
