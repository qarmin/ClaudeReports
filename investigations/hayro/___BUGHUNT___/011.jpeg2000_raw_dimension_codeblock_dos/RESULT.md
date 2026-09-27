# jpeg2000_raw_dimension_codeblock_dos

From `___BUGHUNT___/IDEAS.md`'s backlog (noted while root-causing candidate 010): whether a
JPEG2000 codestream can be crafted so that, after `skipped_resolution_levels` downscaling, the
*decoded* (post-downscale) sample count lands *under* candidate 008's `MAX_TOTAL_SAMPLE_COUNT` cap
(256M samples) - unlike candidate 010's own PoC, which lands far over the cap and is rejected
near-instantly - while the actual decode work is still a meaningful, uncapped DoS on its own.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read `hayro-jpeg2000/src/j2c/build.rs::build_decompositions` and its call graph (`decode.rs::decode_tile`, `tile.rs::ComponentTile::resolution_tiles`/`ResolutionTile::new`) | Manual code read | Found that `resolution_tiles()` iterates `0..num_resolution_levels` - the codestream's *raw* declared resolution-level count, not reduced by `decoded_resolutions` (`num_resolution_levels - skipped_resolution_levels`) - and that `build_sub_band`/`build_precincts`/`build_code_blocks` run **unconditionally** for every one of those levels, including ones above `decoded_resolutions` that `skipped_resolution_levels` is meant to skip. The `decoded_resolutions` check only gates whether `coefficients` storage is reserved for a given level (`added_coefficients = 0` for skipped ones), not whether the (potentially far larger, since it uses the *raw*, un-downscaled `component_tile.rect`) precinct/code-block structures for that level get built at all. |
| Hand-built (from scratch, via a small Python generator - not a hex-patched fixture) a minimal, legitimately-parseable raw J2C codestream (SOC/SIZ/COD/QCD/SOT/SOD, 0 bytes of tile-part body data) declaring a raw `Xsiz=Ysiz=15000`, 1 decomposition level (2 resolution levels), `QCD` using `ScalarDerived` style (sidesteps a per-resolution-band exponent-count validation check unrelated to this bug), and the smallest legal code-block size (4x4, via `SPcod`'s code-block-width/height byte = 0) | Standalone `hayro-jpeg2000` example (`examples/scratch_boundary.rs`, since removed), calling the crate's real public `Image::new`/`.decode()` with `DecodeSettings{target_resolution: Some((1,1))}` | Header parsed instantly, reporting a **decoded** size of `7500x7500 = 56,250,000 samples` (22% of the 256M cap - well under it). `decode()` took **1.15 seconds** and peaked at **~2.75 GB** RSS before erroring out downstream (`InvalidExponents`, from the deliberately-minimal QCD/empty tile-part data - irrelevant, since the expensive work happens in `build_decompositions`, which runs *before* that point and had already fully executed). |
| Same, but raw `Xsiz=Ysiz=30000` (decoded `15000x15000 = 225,000,000` samples - 88% of the 256M cap, deliberately close to it without exceeding it) | Same harness | `decode()` took **4.37 seconds**, peaked at **~11.0 GB** RSS. |
| Control: same `15000x15000` raw size, but a realistic `64x64` code-block size (not attacker-tuned) instead of `4x4` | Same harness | `decode()` took **4.9 ms**, peaked at **~14 MB** RSS - confirms the cost is driven specifically by combining a huge *raw* declared size with a small code-block size, not by raw size alone. |
| Applied candidate 008's own `patch.diff` (`MAX_TOTAL_SAMPLE_COUNT` cap in `DecoderContext::reset`, checked against the *decoded* sample count) on top of the pristine tree and re-ran the `30000x30000`/4x4 case | Same harness, candidate 008's fix applied | **No change**: still 4.37s / ~11.0 GB, `Err(InvalidExponents)` after the fact - candidate 008's cap never rejects this, since the decoded sample count (225M) it checks stays under the 256M threshold regardless of how expensive `build_decompositions` itself is. |
| Built a real, standalone, end-to-end `repro/` (own `Cargo.toml`, real `hayro`/`hayro-syntax` public API): a 764-byte real PDF (classic xref table) with the `15000x15000`/1-decomp-level/4x4-code-block codestream as a real `/JPXDecode` image XObject, placed as a **20x20-on-page thumbnail** (`cm 20 0 0 20 90 90` on a 200x200-`/MediaBox` page) - i.e. a placement any real integrator would consider "small, should decode fast" - driven through the real, unmodified, public, top-level `hayro::render()` (the same function `hayro/examples/render.rs` itself calls) | `cargo run --release`, 3 separate runs against a fresh clone of the pristine `base_commit` tree | 3/3 runs: `render()` took **1.29-1.40 seconds** and peaked at **~2.69 GB** RSS to render a 20x20 thumbnail - see `output.txt`. |

## Status

**Confirmed**. Reproduces 3/3 runs (`--release`, no sanitizer/miri needed - this is a real,
measured CPU-time/memory cost, not memory-safety UB) against the pristine `base_commit` tree via
the real, public `hayro::render()` entry point, and 3/3 runs complete in well under a millisecond
(and ~6.5 MB peak RSS, down from ~2.7 GB) against the patched tree. See `output.txt` for the
verbatim captured output of all 6 runs, plus the direct-decode boundary sweep (56.25M/22%-of-cap
and 225M/88%-of-cap decoded-sample cases, both before and after this candidate's fix, plus the
control case with a realistic code-block size).

**Fully reproducible**: **Yes** - `repro/` builds a syntactically real PDF (classic xref table,
`%PDF-1.7` header, catalog/pages/page/image XObject) with a real, legitimately SIZ/COD/QCD-marker-
parseable JPEG2000 codestream (built from scratch, not hex-patched from an existing fixture, but
structurally the same kind of thing any encoder could legitimately emit for these header field
values) as its `/JPXDecode` stream, placed via an ordinary `cm` operator, and drives it through the
real, public, top-level `hayro::render()` - the exact same entry point `hayro/examples/render.rs`
calls, with the real CTM-derived downscale hint computed by the unmodified renderer, not a
hand-picked `target_resolution` value. There is no second live participant to fake here (a
malformed/adversarial file handed to a renderer's own public entry point is the genuine external
input), same carve-out as candidates 003/004/006/008/009/010 - see `CONFIRMATION.md` section 11's
"single-process repro remains completely fine".

## Class

`security` (resource-exhaustion DoS - CPU time and memory both scale with a codestream's *raw*,
pre-downscale declared dimensions combined with an attacker-chosen small code-block size,
independent of both the requested/CTM-derived downscale hint and of candidate 008's own
decoded-sample-count cap).

## Root cause

`hayro-jpeg2000/src/j2c/build.rs::build_decompositions` builds precinct and code-block bookkeeping
(`build_precincts`/`build_code_blocks`, called via `build_sub_band`) for **every** resolution level
`0..num_resolution_levels` of a tile - the codestream's raw, undegraded resolution-level count -
regardless of `skipped_resolution_levels`. The `decoded_resolutions` check
(`resolution_tile.resolution < decoded_resolutions`) only gates whether coefficient *storage* is
reserved for a given sub-band (`added_coefficients`); it does not gate whether that sub-band's
precincts/code-blocks get built at all. Each `ResolutionTile`'s `rect` for a level above
`decoded_resolutions` is computed from the tile's **raw** declared dimensions (`Tile::rect`, built
from `SIZ`'s own `Xsiz`/`Ysiz`/`XTsiz`/`YTsiz`, never reduced by `x_resolution_shrink_factor`/
`skipped_resolution_levels`), so its code-block count scales with `raw_area / (code_block_width *
code_block_height)` - and `code_block_width`/`code_block_height` are themselves attacker-controlled
down to the format's legal minimum (4x4, `SPcod`'s code-block-size byte = 0), with no additional
minimum enforced beyond that. The result: a codestream that declares a large raw size, a small
`skipped_resolution_levels`-eligible number of decomposition levels (so most of that raw area ends
up in the *skipped*, not-decoded top resolution level), and a 4x4 code-block size drives real,
measured multi-second CPU time and multi-gigabyte memory use in `build_decompositions` - entirely
before any of the codestream's actual entropy-coded packet data is read, and entirely independent
of how small a `target_resolution` hint (or a real, small on-page placement, via `hayro::render()`'s
own CTM-derived hint) requests. This directly bypasses candidate 008's own
`MAX_TOTAL_SAMPLE_COUNT` cap, since that cap is checked against the *decoded* sample count in
`DecoderContext::reset` - a value this bug's cost is completely decoupled from.

Note: building these structures for skipped resolution levels is not *purely* wasted work in
general - `segment::parse` (`hayro-jpeg2000/src/j2c/segment.rs`) must structurally walk every
resolution level's packets (via the progression iterator, which is built from the tile's full,
un-skipped resolution-level count) to correctly parse the codestream's fixed packet order and stay
in sync with the bitstream, and it does so by indexing into the very precinct/code-block/tag-tree
structures `build_decompositions` builds - so simply not building them for skipped levels breaks
real files (see "What's not done" below, and this candidate's own first, reverted, incorrect fix
attempt). Only their *cost* is unbounded, not their existence.

## Introduced by

b56542a "hayro-jpeg2000: Complete restructuring and refactoring of JP2 processing (#610)" (2025-12-03) - `git blame`/`--diff-filter=A` confirm `build.rs` (and `build_decompositions`) was created fresh by this large rewrite, with resolution levels above `decoded_resolutions` already being fully built rather than skipped.

## The fix

Added a cheap, closed-form ceiling check (`MAX_RAW_CODE_BLOCK_ESTIMATE = 4_000_000`) on the
estimated total code-block count `build_decompositions` is about to build, computed from each
component's **raw** (un-skipped) top resolution level's area divided by its **actual configured**
code-block area (not a worst-case 4x4 assumption) - so ordinary large images using realistic
code-block sizes (e.g. the default-ish 64x64) are unaffected, and only a codestream that combines a
large raw declared size with an unusually small code-block size is rejected. This is checked
*before* the per-resolution-level build loop runs, using the same `ResolutionTile`/
`code_block_width()`/`code_block_height()` machinery already used elsewhere in this file, so no new
parsing logic is introduced.

An initial fix attempt (simply not building precinct/code-block structures for resolution levels
`>= decoded_resolutions`, i.e. skipping the "wasted"-looking work entirely) was tried first and
**reverted**: it broke a real, pre-existing test fixture (`hayro-tests/tests/load.rs::issue391`,
part of the project's own existing test suite) with an index-out-of-bounds panic in
`decode.rs::sub_band_iter`, because `segment::parse` genuinely needs those structures for skipped
levels too (see "Root cause" above) - this is recorded here so a future session doesn't retry the
same broken approach.

```diff
+const MAX_RAW_CODE_BLOCK_ESTIMATE: u64 = 4_000_000;
...
     let mut total_coefficients = 0_usize;
+    let mut raw_code_block_estimate = 0_u64;

     for component_tile in tile.component_tiles() {
         let decoded_resolutions = ...;
         let top_resolution = ResolutionTile::new(component_tile, decoded_resolutions - 1);
         let component_samples = ...;
         total_coefficients = total_coefficients.checked_add(component_samples)...;
+
+        let raw_num_resolution_levels = component_tile.component_info.num_resolution_levels();
+        let raw_top_resolution = ResolutionTile::new(component_tile, raw_num_resolution_levels - 1);
+        let cb_area = u64::from(raw_top_resolution.code_block_width())
+            * u64::from(raw_top_resolution.code_block_height());
+        raw_code_block_estimate = raw_code_block_estimate
+            .saturating_add(raw_top_resolution.rect.area().saturating_div(cb_area));
     }
+
+    if raw_code_block_estimate > MAX_RAW_CODE_BLOCK_ESTIMATE {
+        bail!(ValidationError::ImageTooLarge);
+    }
```

Verified against more than the one triggering input, per `CONFIRMATION.md` section 7: re-ran both
the `15000x15000` (56.25M decoded samples) and `30000x30000` (225M decoded samples) 4x4-code-block
cases, and the `15000x15000`/64x64-code-block control case, all 3 rejected/passed identically
before and after in the expected direction (see `output.txt`); re-ran the real end-to-end `repro/`
3 times against the pristine tree (3/3 slow, ~1.3s/~2.7GB) and 3 times against the patched tree
(3/3 fast, ~200µs/~6.5MB).

- Project test suite:
  - `cargo test -p hayro-jpeg2000 --release`: unit tests **1 passed / 0 failed** before and after
    (identical). The crate's separate `asset_suite` integration test fails identically before and
    after (`Failed to read asset directory: missing test input serenity/buggie-gray.jpf`) - a
    pre-existing environment limitation (downloaded test assets not present in this checkout, same
    as candidates 004/006/008/009 already documented), not a regression.
  - `cargo test -p hayro-tests --release --test tests -- load::`: **112 passed / 0 failed**,
    identical before and after - critically, this includes `load::issue391`, which the initial
    (reverted) fix attempt broke (see "The fix" above) and this final fix does not.
  - `cargo test -p hayro-tests --release --test tests -- render::`: **266 passed / 1259 failed**,
    identical count and identical failing-test set before and after (confirmed via `git stash` and
    re-running the exact same command against the pristine tree) - the failures are the same
    pre-existing environment limitation (missing downloaded reference PDFs/snapshots) candidates
    004/006/008/009 already documented, not caused by this change; the JPEG2000-specific
    `stream_jpx_3`/`stream_jpx_5`/`stream_jpx_6` tests fail identically before and after too.

## What's not done / open questions

- `MAX_RAW_CODE_BLOCK_ESTIMATE = 4_000_000` is a judgment call, not derived from a spec limit or a
  precise "this is the largest cost a legitimate caller should tolerate" measurement - a
  maintainer with a different real-world ceiling in mind should treat this as adjustable. It's
  chosen to sit comfortably below the cost observed in this candidate's own PoCs (which used
  10.5M-56.25M-scale code-block counts) while staying well above what a realistic large image with
  a realistic code-block size would ever need (a 20000x15000 raw image at the default-ish 64x64
  code-block size is ~73k code blocks, tested and unaffected - see `output.txt`'s "Control" run).
- This only estimates the cost using each component's **top** raw resolution level's area; it
  does not sum the (smaller, geometrically-decaying) contributions from intermediate raw
  resolution levels between `decoded_resolutions` and the top. This is a deliberate
  simplification (the top level dominates the true total by roughly a factor of 4/3 for a typical
  wavelet decomposition), so the estimate is a slight *underestimate* of the true cost in the
  general case - a maintainer wanting an exact bound would need to sum across all skipped levels,
  not just the top one.
- Does not address the (structurally necessary, see "Root cause") fact that `segment::parse` must
  still walk every resolution level's packets for a codestream that legitimately has real
  entropy-coded packet data at every level - this candidate's fix only caps the *structural*
  precinct/code-block bookkeeping cost in `build_decompositions`, not any packet-parsing cost that
  might exist in `segment::parse` itself for a codestream that passes this new cap but still has a
  large number of real packets across many resolution levels. This wasn't separately investigated
  this session (out of scope: this candidate's PoC never got far enough to exercise real packet
  parsing, since it fails before reaching it).
- Only a single-tile, single-component codestream shape was tested. A codestream with many tiles
  and/or many components, each individually just under this new per-tile-loop cap, could still sum
  to a much larger cumulative cost across the whole `decode()` call - the same "per-call, not
  per-request-budget" caveat candidate 008's own cap already carries, not newly introduced by this
  fix, but not separately re-verified for this specific cap either.
