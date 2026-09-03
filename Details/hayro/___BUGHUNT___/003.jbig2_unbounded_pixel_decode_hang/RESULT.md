# jbig2_unbounded_pixel_decode_hang

`hayro-jbig2::Image::decode` has no cap on the *total* number of pixels it will walk through the
arithmetic decoder, only per-object per-dimension caps (`MAX_DIMENSION = u16::MAX`). Pre-session
triage found ~30 stale `hayro-fuzz/artifacts/fuzz_jbig2/timeout-*` files (from a May-2026 local
fuzz run) still hang (>15s) against current HEAD (commit `5a5f0e247c970df948505ee0bb36e2df2504bf86`),
and an unmerged local `Fuzzers` branch had already worked around this at the fuzz-target level with
a `width()/height() > 4096` bail, without fixing the library itself.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Replay stale corpus against current HEAD | `cargo build --release --example probe_timing` (ad hoc example calling the public `Image::new`/`decode` API), all 35 `timeout-*` artifacts, `timeout 5`-`20`s per file | All 35 still hang past their wrapper timeout on pristine HEAD - confirms the pre-session triage's "still hangs" claim was real, not stale/already-fixed |
| Manual boundary/instrumentation debugging | Temporary `eprintln!` timing instrumentation in `lib.rs`'s `decode_segments` and `decode/symbol.rs`'s per-symbol loop (reverted before this candidate folder was written), release build, 3 representative small `timeout-*` files | Found two distinct, independently-hanging mechanisms under the same missing-cap root cause (see Root cause) - not the single "huge page width/height" mechanism the pre-session triage guessed; the page's own declared width/height was actually tiny (1x1 and 13x2070) in every file that hung |
| Full corpus sweep before/after the fix | Same `probe_timing` example, all 35 `timeout-*` + all 22 `crash-*` artifacts, `timeout 5`s per file, release build | Before: corpus known to hang (per triage). After: 35/35 `timeout-*` files now complete within 5s each; 22/22 `crash-*` files still exit cleanly (no new panics) |

## Status

**Confirmed**. Reproduces 100% (every run, every file in the corpus) under a plain release build -
no sanitizer/miri needed, this is a wall-clock/CPU-time hang, not memory-safety UB. The standalone
`repro/` (built from the project's own public API, on a pristine clone) hung on 3/3 separate runs
against pristine code (`timeout 30`, `timeout 10`, and the `just test` cycle's own `timeout 20`);
after the fix it completed in <1s on 3/3 separate runs (2 direct + 1 via `just test`).

**Fully reproducible**: **Yes** - `repro/` calls `hayro_jbig2::Image::new`/`.decode` directly, the
exact same public entry point a real caller (hayro's own PDF `JBIG2Decode` filter, or any other
consumer of this crate) uses for an untrusted JBIG2 stream. A malformed/adversarial file handed to
a parser's own public decode entry point is the genuine external input here - there is no second
participant to fake (see `CONFIRMATION.md` section 11's own "single-process repro remains
completely fine" carve-out).

## Class

`security` (algorithmic-complexity / resource-exhaustion DoS - unbounded CPU work from a tiny
untrusted input, reachable from a real PDF via the `JBIG2Decode` filter).

## Root cause

Two sibling manifestations of the same missing safeguard, both fixed by the same patch:

1. **Generic region decode ignores the page's own size entirely.** `parse_region_segment_info`
   (`hayro-jbig2/src/decode/mod.rs`) reads a region's own `width`/`height`/`x_location`/
   `y_location` straight off the stream with no bound at all. `generic::decode`/`decode_into`
   (`hayro-jbig2/src/decode/generic.rs`) then unconditionally allocates and fully decodes a bitmap
   of that declared size via the arithmetic decoder - regardless of whether the region even
   overlaps the page, and regardless of how tiny the page itself is. `Bitmap::new_with`
   (`hayro-jbig2/src/bitmap.rs`) does cap each *dimension* individually at `MAX_DIMENSION =
   u16::MAX = 65535`, but that still permits up to `65535 x 65535` (~4.3 billion pixels) per
   region. The confirmed 44-byte trigger file declares a 1x1 page but a single
   `ImmediateLosslessGenericRegion` segment claiming `width=34304 height=49920`
   (`x_location=2241268250 y_location=168449792`, nowhere near the page) - ~1.7 billion pixels,
   fully decoded (and mostly discarded) before ever touching the 1x1 page bitmap, taking ~4.5s.
2. **Symbol-dictionary per-symbol width accumulates without any cumulative budget.**
   `decode/symbol.rs`'s symbol-decoding loop reads a delta-encoded `symbol_width` that keeps
   accumulating (`checked_add_signed`) across every symbol *within the same height class* - it is
   only reset when a new height class starts, not per symbol. Combined with `num_new_symbols` up
   to `MAX_SYMBOLS = 65535`, a height class with many symbols and a small positive width delta
   makes each successive symbol's own bitmap wider than the last, so the *sum* of
   `symbol_width * height_class_height` across the dictionary grows roughly quadratically with the
   symbol count - even though each individual symbol's own dimensions stay well under
   `MAX_DIMENSION` (and, after the fix, under the new per-bitmap `MAX_PIXELS`) for a long stretch.
   The confirmed 22-byte trigger file declares `num_new_symbols=59881`; observed per-symbol cost
   already reached ~17.5ms by the 600th symbol (width already 7482, still growing ~13/symbol),
   putting total decode time at many minutes before any single symbol's dimensions alone would
   ever trip a per-object cap.

`gray_scale.rs`'s `decode_bitplanes` had a related-but-distinct gap noticed while root-causing this
(not itself confirmed to hang, but fixed defensively as part of the same patch): it allocated a
`Vec<u32>` sized by `width * height` *before* calling `Bitmap::new` (which is where the per-object
caps live), so a large halftone grid could force a multi-hundred-MB allocation ahead of any check.
`decode/halftone.rs`'s `grid_width`/`grid_height` (used to size `compute_skip_bitmap`'s `hskip`
vec) had the same per-dimension-only gap as the generic-region case.

## Introduced by

Two separate mechanisms, two separate commits:
- Generic region decode ignoring the page's own size: 874a734 "hayro-jbig2: Add initial skeleton and support decoding immediate generic with MMR bitmaps (#708)" (2025-12-17), which added `parse_region_segment_info`/`generic::decode` from scratch.
- Symbol-dictionary per-symbol width accumulating without a cumulative budget: feb29ed "hayro-jbig2: Add support for decoding basic symbol dictionaries (#720)" (2025-12-21), which added the symbol-decoding loop from scratch.

## The fix

Add a single new constant, `bitmap::MAX_PIXELS = 64_000_000` (a real-world-generous ceiling for
scanned pages/glyphs actually embedded in PDFs - even a full-page scan at 600dpi is well under
this), and check `width as u64 * height as u64 <= MAX_PIXELS` at every bitmap-shaped allocation
site that wasn't already centrally covered by `Bitmap::new`/`Bitmap::new_with`:

- `Bitmap::new_with`/`Bitmap::reinitialize` (`bitmap.rs`) - the central choke point; covers generic
  regions, individual symbol bitmaps, pattern-dictionary collective bitmaps, and refinement
  regions in one place.
- `decode/symbol.rs` - a new `total_symbol_pixels: u64` running sum on `SymbolDecodeContext`,
  checked against `MAX_PIXELS` on every symbol (not just each symbol's own dimensions), which
  directly bounds the quadratic-blowup mechanism above regardless of any single symbol's size.
- `gray_scale.rs::decode_bitplanes` - the same `width * height` check moved *before* the
  `Vec<u32>` allocation it currently happens after.
- `decode/halftone.rs::parse` - the same check added next to the existing
  `grid_width/grid_height > MAX_DIMENSION` check.

Verified against more than the one triggering input per `CONFIRMATION.md` section 7: swept the
full 35-file `timeout-*` corpus (all distinct crafted inputs, not just the two `repro/` uses) and
confirmed 35/35 now complete within a 5s wrapper (previously all hung >15s), and the full 22-file
`crash-*` corpus still exits cleanly (0 new panics/regressions).

- Project test suite (`cargo test -p hayro-jbig2 --release`, plus `cargo test -p hayro-tests
  --release --test tests -- load`/`render`, workspace `cargo build --workspace --release`):
  - `hayro-jbig2` unit tests: **2 passed / 0 failed**, identical before and after (the crate's
    `tests/mod.rs` `asset_suite` integration test errors out both before and after with "No test
    inputs were found. Run `python sync.py` to download them." - a pre-existing environment
    limitation, not caused by this change; not counted as a pass/fail here).
  - `hayro-tests`' `load`/`render` filtered subset: **112 passed / 1 failed**, identical before and
    after. The one failure, `render::pdfjs_shavian_load`, panics with `Os { code: 2, kind:
    NotFound, ... }` (a missing reference asset file) on the *unpatched* tree too - confirmed
    pre-existing and unrelated to this patch by running it in isolation against a stashed
    (unpatched) working tree first.
  - Full workspace build (`cargo build --workspace --release`): succeeds both before and after.

## What's not done / open questions

- `decode/pattern.rs`'s collective-bitmap and per-pattern-bitmap allocations were checked and are
  already fully covered by the central `Bitmap::new`/`new_with` fix (its `collective_width`/
  `pattern_height` and per-pattern `pattern_width`/`pattern_height` all route through those
  constructors) - no separate change was needed there.
- `MAX_PIXELS = 64_000_000` is a judgment call, not derived from the JBIG2 spec (which places no
  upper bound on region/symbol size at all). It comfortably covers realistic scanned-page/glyph
  sizes actually seen in PDFs, but a maintainer with a different real-world ceiling in mind
  (smaller for stricter DoS resistance, or larger if some legitimate corpus needs more) should
  treat this as adjustable, not load-bearing at this exact value.
- The symbol-dictionary fix bounds the *cumulative* pixel budget across the whole dictionary but
  does not change the per-symbol algorithm's own asymptotic behavior; a dictionary designed to
  land just under the new `MAX_PIXELS` budget could still take a fraction of a second (observed:
  ~0.5s for the confirmed trigger file, post-fix) - a real improvement (was 10+ minutes
  unbounded) but not literally O(1). Not treated as its own bug given the multi-order-of-magnitude
  improvement and the fact JBIG2 has no problem-scale-independent decode bound in the first place.
- Did not check `hayro-ccitt` for an analogous missing-cap pattern, even though the same unmerged
  `Fuzzers` branch applied an identical `MAX_DIM` workaround to `fuzz_ccitt.rs` too (suggesting the
  branch author suspected the same class there) - added to `IDEAS.md`'s backlog for a future
  session rather than checked here, to keep this candidate to one focused fix.
- Did not attempt to reduce `hang_input_symbol_dict.jb2`'s post-fix ~0.5s runtime further (e.g. a
  tighter `MAX_PIXELS`, or a smarter per-height-class cap) - 0.5s is well within any reasonable
  per-file budget for a real PDF renderer and far below what would register as a "hang" in
  practice or in a fuzzer's own timeout.
