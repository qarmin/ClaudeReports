# jpeg2000_estimate_precinct_clamp_bypass

From `___BUGHUNT___/IDEAS.md`'s backlog (noted while root-causing candidate 012): candidate 012's
`raw_code_block_layer_estimate` (like candidate 011's own estimate before it) only accounts for
each component's *top* raw resolution level's area, not the smaller, geometrically-decaying
contributions from lower raw resolution levels - candidate 012's own "What's not done" section
already flagged this as a "deliberate underestimate (~4/3 factor)" but never re-verified it stays
a safe margin. This candidate checks that claim directly (both the plain geometric-decay case, and
whether the format's own per-resolution-level precinct-size mechanism can break the "~4/3" bound
entirely, independent of - and compounding with - `num_layers`).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read `build.rs::build_decompositions`/`build_precincts`/`build_code_blocks`, `tile.rs::ComponentTile::resolution_tiles`/`ResolutionTile::code_block_width`/`code_block_height`/`precinct_exponent_x`/`precinct_exponent_y`, `codestream.rs::coding_style_parameters` | Manual code read | Confirmed `resolution_tiles()` always iterates every *raw* resolution level (`0..num_resolution_levels`, from COD), not just the ones `decoded_resolutions`/`skipped_resolution_levels` will actually decode - so `build_decompositions` structurally builds code-blocks/precincts/tag-trees for every raw level regardless, matching candidate 012's own premise. Also confirmed `code_block_width()`/`code_block_height()` (B-17/B-18) clamp a resolution level's code-block size using **that level's own** `precinct_exponent_x()`/`precinct_exponent_y()` (`precinct_exponents[resolution]`), which `codestream.rs::coding_style_parameters` reads as one independent byte *per resolution level* straight off the COD marker whenever precincts are defined (Scod bit 0) - i.e. a codestream can legitimately set a large precinct (and therefore a large, "normal" code-block size) at its top resolution level while setting a tiny precinct (and therefore a code-block size clamped down to 1x1) at any lower resolution level, entirely independently. `raw_code_block_layer_estimate`'s divisor (`cb_area`) is computed once from the top level only, so this directly breaks the assumption behind the claimed "~4/3" margin, which implicitly assumes a constant code-block area across all levels. |
| Manual boundary-value construction: a small Python bit-level generator (`gen3.py`, extending candidates 011/012's own `gen.py`/`gen2.py`) producing a real, from-scratch, legitimately SIZ/COD/QCD-parseable J2C codestream with an empty (0-byte) tile-part body (isolates `build_decompositions`'s own structural cost from `segment::parse`'s, same pattern candidate 011's own "empty-body baseline" used - `build()` runs unconditionally before `segment::parse` even looks at the body), COD precincts defined with a distinct `(PPx, PPy)` per resolution level | Standalone `hayro-jpeg2000` example harness calling the crate's real public `Image::new`/`.decode()` with `DecodeSettings{target_resolution: Some((20,20))}`, candidate 012's own (unmerged) `patch.diff` applied | 6000x6000 raw, 3 decomposition levels, `Xcb=Ycb=6` (64x64 code blocks) everywhere, `PPx=PPy=15` (unbounded) at every level except the one directly below the top (`PPx=PPy=1`, clamping that single level's code-block size to 1x1): per-component `raw_code_block_layer_estimate` = 8,789 (top level only) at `num_layers=1`, ~455x under candidate 012's own 4,000,000 cap - **not rejected**. Real `decode()` cost: 560ms / ~1.39GB RSS. At `num_layers=32`: estimate = 281,248, ~14x under the cap - still not rejected; real cost: 2.18s / ~6.19GB RSS - confirming `num_layers` compounds with this bypass on top of, not instead of, the precinct-clamp trick. |
| Control: candidate 011/012's own original PoC shape (7900x7900, 1 decomposition level, 4x4 code blocks, default `PPx=PPy=15` everywhere, no precinct trick), `num_layers=1` | Same harness | Rejected by *this candidate's own* fix (see below), because summing every raw level's own code-block count for this shape totals ~4.88M - already over the 4,000,000 cap even without any precinct trick, showing candidate 011/012's own top-only estimate (~3.9M, "just under" their cap) was itself already a real, if modest (~25%), underestimate of the true structural-build cost for their own reference shape - consistent with, and roughly matching, their own claimed ~4/3 factor. |
| Control: realistic large image (20000x15000, 1 decomposition level, 64x64 code blocks, 32 layers, default precincts) and a realistic multi-level image (4000x3000, 5 decomposition levels, 64x64 code blocks, 20 layers, default precincts) | Same harness | Both accepted by this candidate's own fix, both fast/low-memory (3.4ms/~70MB and 1.2ms/~5MB respectively) - confirms summing every level instead of just the top one doesn't penalize ordinary, non-adversarial content. |
| Built a real, standalone, end-to-end `repro/` (own `Cargo.toml`, real `hayro`/`hayro-syntax` public API): a 766-byte real PDF (classic xref table) with the 6000x6000/3-decomp/precinct-clamp-bypass codestream as a real `/JPXDecode` image XObject, placed as a **20x20-on-page thumbnail** (`cm 20 0 0 20 90 90` on a 200x200-`/MediaBox` page), driven through the real, unmodified, public, top-level `hayro::render()` | `cargo run --release`, 3 separate runs against the pristine `base_commit` tree (no cap of any kind - candidates 008/011/012/013's own fixes are all unmerged), and 3 more against the same tree with this candidate's own `patch.diff` applied. Also ran the full `just test` cycle (`clone` -> `reproduce-before` -> `apply` -> `reproduce-after`) from a completely fresh scratch copy of this candidate folder, cloning from the real `repo_url`/`base_commit` over the network | Pristine: 3/3 runs, 2.38-3.56s / ~6.19GB RSS to render a 20x20 thumbnail. Patched: 3/3 runs, 160-227µs / ~6.5-6.6MB RSS. The independent scratch-copy `just test` run reproduced the same before/after shape (3.56s/~6.19GB -> 199µs/~6.5MB) with no dependency on the main checkout - see `output.txt`. |

## Status

**Confirmed**. Reproduces 3/3 runs (`--release`, no sanitizer/miri needed - this is a real,
measured CPU-time/memory cost, not memory-safety UB) against the pristine `base_commit` tree via
the real, public `hayro::render()` entry point, and 3/3 runs complete in ~160-227µs (and ~6.5MB
peak RSS, down from ~6.19GB) against the patched tree. The specific claim this candidate set out
to check - that candidate 012's own top-resolution-level-only estimate stays a safe margin - does
**not** hold: it is trivially bypassed (independent of `num_layers`, and compounding with it) by a
legitimate, spec-compliant use of the format's own per-resolution-level precinct-size mechanism to
force a small code-block size at an intermediate resolution level while keeping the top level's
code-block size large. See `output.txt` for the full boundary sweep isolating this from candidate
012's own already-confirmed `num_layers` multiplier.

**Fully reproducible**: **Yes** - `repro/` builds a syntactically real PDF (classic xref table,
`%PDF-1.7` header, catalog/pages/page/image XObject) with a real, legitimately SIZ/COD/QCD-marker-
parseable JPEG2000 codestream (built from scratch via a small bit-level generator, not hex-patched
from an existing fixture, but structurally the same kind of thing any encoder could legitimately
emit for these header field values, including a real COD marker defining precincts with a distinct
size per resolution level - an ordinary, spec-defined mechanism this exact crate already
implements and exercises for legitimate progressive images) as its `/JPXDecode` stream, placed via
an ordinary `cm` operator, and drives it through the real, public, top-level `hayro::render()` -
the exact same entry point `hayro/examples/render.rs` calls, with the real CTM-derived downscale
hint computed by the unmodified renderer. There is no second live participant to fake here (a
malformed/adversarial file handed to a renderer's own public entry point is the genuine external
input), same carve-out as candidates 003/004/006/008/009/010/011/012/013 - see `CONFIRMATION.md`
section 11's "single-process repro remains completely fine". `just test` was actually run (not
just read) from a fresh scratch copy and genuinely reproduces both states from a real `git clone`,
not a description of what it would do.

## Class

`security` (resource-exhaustion DoS - the finding is specifically that candidate 012's own
unmerged size-estimate cap, if adopted as-is, would not actually bound this cost: a codestream
combining a large raw declared size with a per-resolution-level precinct-size mechanism that
clamps code-block size independently per level can cost orders of magnitude more real CPU
time/memory than its own `raw_code_block_layer_estimate` suggests, entirely bypassing the cap).

## Root cause

`build.rs::build_decompositions` structurally builds code-blocks, precincts and tag-trees for
**every raw resolution level** of a tile (`ComponentTile::resolution_tiles()` always iterates
`0..num_resolution_levels`, the codestream's raw declared count from COD, never
`decoded_resolutions`), and each resolution level's code-block size is independently determined by
that level's own precinct size (`ResolutionTile::code_block_width`/`code_block_height`, B-17/B-18:
`min(configured_Xcb/Ycb, precinct_exponent - (1 if resolution > 0 else 0))`), which
`codestream.rs::coding_style_parameters` reads as one independent byte per resolution level
straight off the COD marker whenever precincts are defined (Scod bit 0).

Candidate 012's `raw_code_block_layer_estimate` computes only the **top** raw resolution level's
`area / cb_area`, multiplied by `num_layers`, reasoning (in its own "What's not done" section)
that lower levels' geometrically-decaying areas contribute at most a converging ~4/3 factor on top
of that - a reasoning that implicitly assumes every resolution level uses the *same* code-block
area as the top level. That assumption holds for an ordinary codestream (no precincts, or a
uniform precinct size across levels), but is not required by the format: an attacker can set a
large precinct (and therefore a large, cheap-looking code-block size) specifically at the top
level - keeping the top-level-only estimate tiny - while setting a tiny precinct at any lower
level, forcing that level's own code-block size down to 1x1 independently. Since
`build_decompositions` still structurally builds that lower level in full (one `CodeBlock` +
`num_layers` `Layer` bookkeeping entries per 1x1 pixel, plus one `Precinct` with two `TagTree`s per
tiny precinct), the real total cost is driven by that level's own area divided by 1, not by the
top level's area divided by its own large `cb_area` - a divisor mismatch that can differ by
several orders of magnitude, not just the claimed ~4/3. Measured impact at a moderate scale
(6000x6000, 3 decomposition levels, only *one* of the four raw resolution levels using the
precinct trick): candidate 012's own cap computes an estimate 14-455x under its own 4,000,000
threshold (depending on `num_layers`) while the real cost is 560ms/~1.4GB (`num_layers=1`) to
2.18s/~6.19GB (`num_layers=32`) - comparable to, or worse than, candidate 012's own PoC shape
sitting right at its cap boundary.

## Introduced by

Not applicable in the usual sense (no commit hash) - the affected code
(`raw_code_block_layer_estimate` and the reasoning behind it) exists only as candidate 012's own
unmerged `patch.diff` from `___BUGHUNT___/012.jpeg2000_packet_header_layer_dos/`, never applied to
the actual `main` branch. This candidate is an audit of that proposed fix's adequacy, not of code
currently shipping.

## The fix

Implemented directly against the pristine tree (not stacked on candidate 012's own still-unmerged
patch, so it stands on its own regardless of whether that separate fix is ever adopted): instead
of estimating cost from only the top raw resolution level's `area / cb_area`, the fix iterates
**every** raw resolution level of a component (`component_tile.resolution_tiles()` - the same
iterator `build_decompositions` itself already uses to do the real work), computes **that level's
own** `code_block_width()`/`code_block_height()` (so a precinct-driven per-level clamp is reflected
correctly instead of assumed away), and sums each level's own `area / cb_area` estimate (each
multiplied by `tile.num_layers`) into `raw_code_block_layer_estimate`, compared against the same
`4_000_000` ceiling (`MAX_RAW_CODE_BLOCK_LAYER_ESTIMATE`). This directly approximates the actual
cost driver (`total_code_blocks_across_all_raw_levels * num_layers`) instead of extrapolating it
from a single level under an assumption the format does not enforce.

```diff
+const MAX_RAW_CODE_BLOCK_LAYER_ESTIMATE: u64 = 4_000_000;
...
     let mut total_coefficients = 0_usize;
+    let mut raw_code_block_layer_estimate = 0_u64;

     for component_tile in tile.component_tiles() {
         ...
+        for resolution_tile in component_tile.resolution_tiles() {
+            let cb_area = u64::from(resolution_tile.code_block_width())
+                * u64::from(resolution_tile.code_block_height());
+            let level_estimate = resolution_tile.rect.area()
+                .saturating_div(cb_area)
+                .saturating_mul(u64::from(tile.num_layers));
+            raw_code_block_layer_estimate =
+                raw_code_block_layer_estimate.saturating_add(level_estimate);
+        }
     }
+
+    if raw_code_block_layer_estimate > MAX_RAW_CODE_BLOCK_LAYER_ESTIMATE {
+        bail!(ValidationError::ImageTooLarge);
+    }
```

Verified against more than the one triggering input, per `CONFIRMATION.md` section 7: re-ran the
6000x6000/1-precinct-clamped-level PoC at both `num_layers=1` and `num_layers=32` (both now
rejected in ~8-9µs/~3MB, down from 560ms/~1.4GB and 2.18s/~6.19GB respectively); re-ran candidate
011/012's own original 7900x7900/4x4/1-layer PoC shape (also now rejected - its true summed-level
cost was already over the cap, see "What was tried" above); re-ran the 20000x15000/64x64/32-layer
and 4000x3000/5-decomposition-level/64x64/20-layer control cases (both still accepted, both still
fast: 3.4ms/~70MB and 1.2ms/~5MB, unchanged from the pristine-tree run); re-ran the real end-to-end
`repro/` 3 times against the pristine tree (3/3 slow, ~2.4-3.6s/~6.19GB) and 3 times against the
patched tree (3/3 fast, ~160-227µs/~6.5MB) - see `output.txt`.

- Project test suite:
  - `cargo test -p hayro-jpeg2000 --release`: unit tests **1 passed / 0 failed** before and after
    (identical). The crate's separate `asset_suite` integration test fails identically before and
    after (`Failed to read asset directory: missing test input serenity/buggie-gray.jpf`) - a
    pre-existing environment limitation (downloaded test assets not present in this checkout),
    same as candidates 004/006/008/009/011/012, not a regression.
  - `cargo test -p hayro-tests --release --test tests -- load::`: **112 passed / 0 failed**,
    identical before and after - includes `load::issue391`, the specific regression candidate
    011's own first (reverted) fix attempt broke; this fix does not touch `segment::parse`'s
    structural reliance on precinct/code-block bookkeeping for skipped resolution levels (it only
    adds an earlier, purely size-based rejection), so it does not reintroduce that regression.
  - `cargo test -p hayro-tests --release --test tests -- render::`: **266 passed / 1259 failed**,
    identical count and identical failing-test set before and after (confirmed via `git checkout
    --` and re-running the exact same command against the pristine tree) - the failures are the
    same pre-existing environment limitation (missing downloaded reference PDFs/snapshots)
    candidates 004/006/008/009/011/012 already documented, not caused by this change; the
    JPEG2000-specific `stream_jpx_3`/`stream_jpx_5`/`stream_jpx_6` tests fail identically before
    and after too.

## What's not done / open questions

- Like candidates 011/012's own estimates, this is still a **per-tile** cap, not a whole-`decode()`
  call cumulative budget - candidate 013 already confirmed this exact gap for candidate 011/012's
  own caps (a codestream with many tiles, each individually under the cap, can still sum to a much
  larger cumulative cost). This candidate's fix has the identical limitation and was not re-checked
  against candidate 013's own cross-tile cumulative-budget fix.
- The fix still caps an *estimated* cost via a closed-form code-block-count approximation (now
  summed correctly across all raw levels using each level's own code-block dimensions), not a
  separate, direct bound inside `build_precincts`/`build_code_blocks` themselves (e.g. a hard
  iteration counter). A maintainer wanting a guarantee independent of the accuracy of this
  estimate would need such a direct bound instead.
- Not checked whether an even more extreme version of this trick (e.g. many resolution levels each
  independently precinct-clamped, rather than just one) could still find headroom under the same
  4,000,000 threshold in some other component/tile-count combination - only single-tile,
  single-component shapes were tested, same limitation candidates 011/012/013 already noted for
  their own caps.
- `segment.rs::parse_inner`'s own packet-header-parsing cost (candidate 012's other, separately
  confirmed finding) was not re-examined against this same precinct-size trick - a tiny precinct
  at an intermediate level also multiplies the number of *packets* (one per precinct per
  resolution per layer) `segment::parse` would need to walk if the tile-part body were non-empty,
  which this candidate's empty-body PoC deliberately did not exercise (isolating
  `build_decompositions`'s own cost, same as candidate 011's own "empty-body baseline" pattern).
