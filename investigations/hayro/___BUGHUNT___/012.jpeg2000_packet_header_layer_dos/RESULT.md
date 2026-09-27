# jpeg2000_packet_header_layer_dos

From `___BUGHUNT___/IDEAS.md`'s backlog (noted while root-causing candidate 011): whether
`hayro-jpeg2000/src/j2c/segment.rs::parse_inner`'s packet-header-parsing cost - walking every
resolution level's packets, including skipped ones, once per quality layer - is a *separate*,
meaningful DoS surface on top of candidate 011's already-confirmed `build_decompositions`
structural-build cost, decoupled from candidate 011's own `MAX_RAW_CODE_BLOCK_ESTIMATE` cap
(which only estimates a single, layer-independent code-block count).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read `segment.rs::parse`/`parse_inner`/`resolve_segments`, `progression.rs::IteratorInput::new`, `build.rs::build_code_blocks`, `tag_tree.rs::read_tag_node` | Manual code read | Confirmed `IteratorInput::new` always clamps to the codestream's *raw* declared `num_resolution_levels` (from COD), never `decoded_resolutions` - so `segment::parse`'s packet loop always walks every raw resolution level, once per layer, regardless of `skipped_resolution_levels`/`target_resolution`. Also found `build_code_blocks` pushes one `Layer` bookkeeping entry (`storage.layers`) per code-block *per layer*, unconditionally, for every raw resolution level - a second, closely related cost that scales with `total_code_blocks * num_layers` and is likewise untouched by candidate 011's code-block-count-only estimate. Read `read_tag_node` closely: a whole precinct's inclusion tag-tree root can be kept "not included" for an entire layer using a single shared bit, cached across all of that precinct's code-blocks - meaning a *tiny* number of real payload bytes (independent of code-block count) is enough to drive `resolve_segments`'s and `parse_inner`'s per-code-block loops through their full length. |
| Hand-built (from-scratch, via a small Python bit-level generator) a minimal, legitimately-parseable raw J2C codestream (SOC/SIZ/COD/QCD/SOT/SOD) with a *real*, non-empty tile-part body: one zero-length packet per layer for the small (decoded) resolution level, and one 1-byte non-zero-length packet per layer for the huge (skipped) raw resolution level, whose 4 header bits keep every sub-band's inclusion tag-tree root at "not yet included" | Standalone `hayro-jpeg2000` example harness calling the crate's real public `Image::new`/`.decode()` with `DecodeSettings{target_resolution: Some((20,20))}` | At 3000x3000/1-decomp/4x4-code-blocks (structurally cheap to build): empty-body baseline 65ms/110MB (layers=1) -> 182ms/510MB (layers=32, `build_decompositions` only); real minimal packets 51ms/110MB (layers=1) -> 261ms/510MB (layers=32) - i.e. `segment::parse`'s own packet loop adds ~79ms at layers=32, using only 64 extra real payload bytes, on top of `build_decompositions`'s own layers-driven cost. |
| Same shape, scaled to candidate 011's own PoC dimensions (15000x15000, 1 decomp level, 4x4 code blocks) | Same harness | Pristine tree: empty-body/layers=1 reproduces candidate 011's own numbers almost exactly (1.12s/2.69GB vs their reported 1.15s/2.75GB). Empty-body/layers=32: 4.25s/12.66GB (`build_decompositions` alone, `storage.layers` scaling). Real-packets/layers=32: 7.71s/12.66GB - `segment::parse`'s own additional cost: ~3.46s, with identical peak memory to the empty-body case (confirms the ~12.66GB is entirely from `build_decompositions`, and `segment::parse` adds *time*, not additional peak memory, in this shape). |
| Isolated specifically against candidate 011's own (unmerged) fix: applied their `patch.diff` to a fresh checkout, then re-ran at 7900x7900 (1 decomp level, 4x4 code blocks) - `raw_code_block_estimate` for this shape computes to ~3.9M, just under their 4,000,000 cap, so their fix does not reject it | Same harness, candidate 011's fix applied | layers=1: 346ms/748MB. layers=32: 1.89s/3.52GB (~5.5x time, ~4.7x memory from num_layers alone, entirely un-addressed by candidate 011's code-block-count-only estimate). Empty-body/layers=32 (isolates `build_decompositions` only, fix applied): 1.11s/3.52GB - confirms `segment::parse`'s own packet loop still adds ~0.78s on top, even with candidate 011's fix present. |
| Control: realistic large image, realistic code-block size (20000x15000, 64x64 code blocks), all 32 layers, real minimal packet stream | Same harness | 3.4ms/7.5MB - confirms the cost is driven specifically by combining a large raw declared size, a small code-block size *and* a large layer count, not by any one of those alone. |
| Built a real, standalone, end-to-end `repro/` (own `Cargo.toml`, real `hayro`/`hayro-syntax` public API): an 829-byte real PDF (classic xref table) with the 15000x15000/1-decomp-level/4x4-code-block/32-layer/real-minimal-packets codestream as a real `/JPXDecode` image XObject, placed as a **20x20-on-page thumbnail** (`cm 20 0 0 20 90 90` on a 200x200-`/MediaBox` page), driven through the real, unmodified, public, top-level `hayro::render()` | `cargo run --release`, 3 separate runs against a fresh clone of the pristine `base_commit` tree | 3/3 runs: `render()` took **7.47-7.86 seconds** and peaked at **~12.66 GB** RSS to render a 20x20 thumbnail - see `output.txt`. |

## Status

**Confirmed**. Reproduces 3/3 runs (`--release`, no sanitizer/miri needed - this is a real,
measured CPU-time/memory cost, not memory-safety UB) against the pristine `base_commit` tree via
the real, public `hayro::render()` entry point, and 3/3 runs complete in ~230-290µs (and ~6.5 MB
peak RSS, down from ~12.66 GB) against the patched tree. See `output.txt` for the verbatim
captured output of all 6 `repro/` runs, plus the full boundary-sweep isolating `segment::parse`'s
own contribution from candidate 011's already-confirmed `build_decompositions` cost, both without
and with candidate 011's own (unmerged) fix applied.

**Fully reproducible**: **Yes** - `repro/` builds a syntactically real PDF (classic xref table,
`%PDF-1.7` header, catalog/pages/page/image XObject) with a real, legitimately SIZ/COD/QCD-marker-
parseable JPEG2000 codestream (built from scratch via a small bit-level generator, not
hex-patched from an existing fixture, but structurally the same kind of thing any encoder could
legitimately emit for these header field values, including a genuine, minimally-encoded, real
non-empty packet-header bitstream - not zero bytes of tile-part body like candidate 011's own
PoC) as its `/JPXDecode` stream, placed via an ordinary `cm` operator, and drives it through the
real, public, top-level `hayro::render()` - the exact same entry point `hayro/examples/render.rs`
calls, with the real CTM-derived downscale hint computed by the unmodified renderer. There is no
second live participant to fake here (a malformed/adversarial file handed to a renderer's own
public entry point is the genuine external input), same carve-out as candidates
003/004/006/008/009/010/011 - see `CONFIRMATION.md` section 11's "single-process repro remains
completely fine".

## Class

`security` (resource-exhaustion DoS - CPU time and memory both scale with a codestream's *raw*,
pre-downscale declared dimensions combined with an attacker-chosen small code-block size,
*multiplied further* by the codestream's own quality-layer count, up to the format's 32-layer
maximum - independent of both the requested/CTM-derived downscale hint and of candidate 011's own
code-block-count-only cap).

## Root cause

Two closely related mechanisms, both scaling with `total_code_blocks * num_layers` rather than
`total_code_blocks` alone, and both entirely unaddressed by candidate 011's own fix (which caps
only a single, layer-independent code-block-count estimate):

1. **`build.rs::build_code_blocks`** pushes one `Layer` bookkeeping entry (`Option<Range<usize>>`,
   `storage.layers`) per code-block *per layer*, unconditionally, for every raw resolution level
   of a tile (not just the ones `decoded_resolutions` will actually decode) - so
   `storage.layers.len()` is exactly `total_raw_code_blocks * tile.num_layers`. This vector is
   allocated and populated as part of `build_decompositions`, before any packet data is even
   read, and its size is not bounded by candidate 011's `MAX_RAW_CODE_BLOCK_ESTIMATE`, which only
   estimates a layer-independent code-block count.

2. **`segment.rs::parse_inner`/`resolve_segments`**: `progression.rs::IteratorInput::new` always
   clamps to the codestream's *raw* declared `num_resolution_levels` (from COD), never to
   `decoded_resolutions`, so the packet-header-parsing loop in `segment::parse` walks every
   resolution level's packets - including ones `skipped_resolution_levels` will later skip for
   coefficient storage - once per layer, for the codestream's *entire* declared layer count (up
   to the format's own 32-layer maximum). For a non-zero-length packet at the huge (skipped) top
   resolution level, `resolve_segments` iterates every code-block of the relevant precinct twice
   per packet (once in `resolve_segments` itself for inclusion/pass-count parsing, once again in
   `parse_inner`'s own body-reading loop) - a cost of `O(code_blocks * num_subbands)` per packet,
   times `num_layers` packets. Crucially, `tag_tree.rs::read_tag_node` lets an entire precinct's
   inclusion tag-tree root be resolved to "nothing in this whole sub-band is included yet" using
   a *single* bit, shared/cached across every code-block that queries it that layer (the
   per-code-block recursion short-circuits immediately once an ancestor node's cached value
   already proves non-inclusion) - so an attacker needs only a handful of real bits per layer
   (independent of code-block count) to drive millions of cheap-but-real loop iterations.

Both mechanisms are triggered by the exact same codestream shape candidate 011 already found
(large raw declared size + small, e.g. 4x4, code-block size), simply with a non-empty tile-part
body (a real, minimal packet stream, not the 0-byte body candidate 011's own PoC used) and a
large declared layer count. Measured impact at candidate 011's own PoC scale (15000x15000, 1
decomp level, 4x4 code blocks): decode time 1.12s -> 7.71s and peak memory 2.69GB -> 12.66GB
(pristine tree, layers 1 -> 32); with candidate 011's own fix applied at a shape just under its
cap (7900x7900), decode time still goes 346ms -> 1.89s and peak memory 748MB -> 3.52GB purely
from the layer count, of which `segment::parse`'s own loop contributes ~0.78s on its own.

## Introduced by

b56542a "hayro-jpeg2000: Complete restructuring and refactoring of JP2 processing (#610)" (2025-12-03) - same origin commit as candidate 011: both `build_code_blocks`'s per-layer `Layer` bookkeeping and `progression.rs`'s raw-resolution-level-clamped `IteratorInput::new` were created fresh by this rewrite.

## The fix

Extended candidate 011's own estimate to also account for the layer multiplier: instead of
capping a layer-independent `raw_code_block_estimate`, this candidate's fix computes
`raw_code_block_layer_estimate` - the same per-component "raw top resolution level area / actual
configured code-block area" estimate candidate 011 already used, now additionally multiplied by
`tile.num_layers` before being summed across components and compared against the same
`4_000_000` ceiling (renamed `MAX_RAW_CODE_BLOCK_LAYER_ESTIMATE`). Since `tile.num_layers >= 1`
always, this generalizes (rather than duplicates) candidate 011's own check: for the common
`num_layers == 1` case the estimate is numerically identical to theirs, so it also independently
catches candidate 011's own original PoC shape; for higher layer counts it becomes
proportionally stricter, directly bounding both mechanisms above (since both scale with the same
`total_code_blocks * num_layers` product this estimate now approximates). This candidate's
`patch.diff` is self-contained (implemented directly against the pristine tree, not layered on
top of candidate 011's own still-unmerged patch), so it stands on its own regardless of whether
candidate 011's separate fix is ever applied.

```diff
+const MAX_RAW_CODE_BLOCK_LAYER_ESTIMATE: u64 = 4_000_000;
...
     let mut total_coefficients = 0_usize;
+    let mut raw_code_block_layer_estimate = 0_u64;

     for component_tile in tile.component_tiles() {
         ...
+        let raw_num_resolution_levels = component_tile.component_info.num_resolution_levels();
+        let raw_top_resolution = ResolutionTile::new(component_tile, raw_num_resolution_levels - 1);
+        let cb_area = u64::from(raw_top_resolution.code_block_width())
+            * u64::from(raw_top_resolution.code_block_height());
+        let component_estimate = raw_top_resolution.rect.area()
+            .saturating_div(cb_area)
+            .saturating_mul(u64::from(tile.num_layers));
+        raw_code_block_layer_estimate = raw_code_block_layer_estimate.saturating_add(component_estimate);
     }
+
+    if raw_code_block_layer_estimate > MAX_RAW_CODE_BLOCK_LAYER_ESTIMATE {
+        bail!(ValidationError::ImageTooLarge);
+    }
```

Verified against more than the one triggering input, per `CONFIRMATION.md` section 7: re-ran the
malicious 15000x15000/4x4/32-layer PoC (now rejected in ~14µs/~3MB, down from 7.7s/12.7GB),
candidate 011's own original 15000x15000/4x4/1-layer PoC shape (also now rejected, in ~8µs/~3MB,
confirming this fix subsumes candidate 011's own check), and the 20000x15000/64x64/32-layer
control case (still accepted, still fast: 3.4ms/7.5MB, identical to the pristine-tree run); re-ran
the real end-to-end `repro/` 3 times against the pristine tree (3/3 slow, ~7.5-7.9s/~12.66GB) and
3 times against the patched tree (3/3 fast, ~230-290µs/~6.5MB) - see `output.txt`.

- Project test suite:
  - `cargo test -p hayro-jpeg2000 --release`: unit tests **1 passed / 0 failed** before and after
    (identical). The crate's separate `asset_suite` integration test fails identically before and
    after (`Failed to read asset directory: missing test input serenity/buggie-gray.jpf`) - a
    pre-existing environment limitation (downloaded test assets not present in this checkout),
    same as candidates 004/006/008/009/011, not a regression.
  - `cargo test -p hayro-tests --release --test tests -- load::`: **112 passed / 0 failed**,
    identical before and after - includes `load::issue391`, the specific regression candidate
    011's own first (reverted) fix attempt broke; this fix does not touch `segment::parse`'s
    structural reliance on precinct/code-block bookkeeping for skipped resolution levels (it only
    adds an earlier, purely size-based rejection), so it does not reintroduce that regression.
  - `cargo test -p hayro-tests --release --test tests -- render::`: **266 passed / 1259 failed**,
    identical count and identical failing-test set before and after (confirmed via `git stash`
    and re-running the exact same command against the pristine tree) - the failures are the same
    pre-existing environment limitation (missing downloaded reference PDFs/snapshots) candidates
    004/006/008/009/011 already documented, not caused by this change; the JPEG2000-specific
    `stream_jpx_3`/`stream_jpx_5`/`stream_jpx_6` tests fail identically before and after too.

## What's not done / open questions

- `MAX_RAW_CODE_BLOCK_LAYER_ESTIMATE = 4_000_000` is a judgment call, same status as candidate
  011's own identically-named-in-spirit constant - not derived from a spec limit or a precise
  "largest cost a legitimate caller should tolerate" measurement. Chosen to numerically match
  candidate 011's own already-reasoned threshold (so a `num_layers == 1` legitimate image is
  treated identically to before), while now also bounding the layer-multiplied cost.
- Like candidate 011's own estimate, this only accounts for each component's **top** raw
  resolution level's area (not summing the smaller, geometrically-decaying contributions from
  intermediate raw resolution levels), and is checked **per tile** - a codestream with many tiles
  and/or many components, each individually just under this cap, could still sum to a much larger
  cumulative cost across a whole `decode()` call. This is the same "per-call, not
  per-request-budget" caveat candidate 011's own "What's not done" section already documented for
  its cap; not separately re-verified for this specific (layer-multiplied) estimate either, and
  not fixed by this candidate.
- This fix caps the *estimated* cost via the same closed-form code-block-count approximation
  candidate 011 already used; it does not add a separate, direct bound inside
  `segment::parse`/`resolve_segments`/`build_code_blocks` themselves (e.g. a hard iteration
  counter). A maintainer who wants a guarantee independent of the accuracy of this geometric
  estimate (see the two caveats above) would need such a direct bound instead.
- Only a single-tile, single-component codestream shape was tested (same limitation candidate 011
  already noted for its own cap).
