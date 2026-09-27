# `vello_cpu::fill_path` glyph batching: one call per text run instead of one per glyph

Backlog entry from `../IDEAS.md` (top of the size-ranked table, ported from candidate `004`'s
investigation): combine multiple glyphs from the same text run into a single
`vello_cpu::fill_path` call, targeting `draw_glyph` (38.7-38.8% of `render()`'s time for
`img.pdf`, of which candidate `004` found ~89% is the `fill_path` call itself). Implemented in
`hayro/src/glyph.rs`'s `fill_outline_run` for the `Paint::Color` case (the common case for real
text): instead of looping over each glyph, setting `ctx.set_transform` and calling
`ctx.fill_path` once per glyph, every glyph's cached outline is transformed into the run's shared
coordinate space (`glyph.transform() * outline`) and appended into one combined `BezPath`, which
is then filled with a single `fill_path` call under the existing `NonZero` fill rule.

## Per-call cost data gathered first (per the backlog entry's own caveat)

No system `perf` in this environment (as in every other candidate this session); used
`valgrind --tool=callgrind` on the existing `render_bench`-style harness instead (3 iterations of
`img.pdf`, `callgrind_annotate` on the resulting profile). `vello_cpu::render::RenderContext::fill_path`'s
own function body is a negligible fraction of total instructions (~0.09%) - essentially all of
`draw_glyph`'s cost is inlined/attributed into its callees (`vello_common::flatten::fill` /Bezier
flattening, `Tiles::make_tiles_analytic_aa`, `strip::render`, `CommandBucketer`), which are
proportional-to-geometry, not obviously fixed-per-call-dominated. This is consistent with, not a
refutation of, candidate `004`'s code-reading conclusion ("no obviously wasteful fixed-cost
step found") - it does not show a large fixed per-call cost that batching would eliminate; it
just confirms the cost is spread through the pipeline stages already described in `004`.

## What was tried

One variant: batch every outline glyph in a `Paint::Color` fill run into a single combined
`BezPath` + single `fill_path` call (see `patch.diff`, touches only `hayro/src/glyph.rs`'s
`fill_outline_run`). The `Paint::Pattern` case and the stroke path were left untouched (already
lower-traffic; `Paint::Pattern`'s `fill_outline_run` arm already builds one transformed `BezPath`
per glyph via `transformed_outlines`, just without merging them into one path/call, and was not
touched to keep this candidate to one change).

Bench: same `bench/src/main.rs` harness pattern as candidate `007` (embeds `../testdata/img.pdf`,
300 iterations, 10% warmup discarded, min/p50/mean/p95/max + FNV-1a hash of the produced
`Pixmap`'s premultiplied RGBA8 bytes every iteration).

| Variant | single-core p50 (`taskset -c 2`) | multi-core p50 (unpinned) | pixmap hash |
|---|---|---|---|
| baseline (run 1) | 2.8999ms | 2.9279ms | `0x9622bedd5d3c7d1c` |
| baseline (run 2) | 2.8966ms | 2.8939ms | `0x9622bedd5d3c7d1c` |
| patched (run 1) | 2.9917ms | 3.0195ms | `0x304764aed9d52113` |
| patched (run 2) | 2.9810ms | 2.9914ms | `0x304764aed9d52113` |

## Result

**~+3% slower** (single-core p50 ~2.90ms -> ~2.98-2.99ms; multi-core/unpinned ~2.90-2.93ms ->
~2.99-3.02ms), reproduced across two repeated runs each side (noise floor well under 1%
comparing run 1 vs run 2 within each variant). This is a small, real regression, not a win - and
the pixmap hash also changed (see below), so it is not a free/neutral change either.

## Does it change behavior/output?

- **Bit-exact?** No. `img.pdf`'s pixmap hash changes (`0x9622bedd5d3c7d1c` ->
  `0x304764aed9d52113`), stable across all 300 iterations of both variants (so it's a
  deterministic difference, not run-to-run noise).
- Characterized: `img.pdf` at 595x841, **47/500395 pixels differ (0.0094%), max channel
  deviation 1/255** - see `correctness/img_pdf_baseline.png`, `correctness/img_pdf_patched.png`,
  `correctness/img_pdf_diff_visible.png` (auto-leveled so the tiny diff is visible; raw
  ImageMagick `compare -metric AE` reports 47 differing pixels for this file). For `img.pdf`
  specifically this is a sub-visible antialiasing-edge rounding difference, in the same spirit as
  candidate `002`/`008`'s FMA-vs-non-FMA precision differences.
- **Project test suite** (`cargo test --release -p hayro-tests --test tests -- render`, run
  twice per variant to confirm stability before comparing): baseline **1259 failing** (stable,
  identical set both runs - matches candidate `007`'s note that most failures are `NotFound` for
  missing local corpus assets, not render failures); patched **1331 failing** (also stable,
  identical set both runs) - **72 new failures** relative to baseline, listed in
  `correctness/hayro_tests_newly_failing.txt`. Spot-checked several of the newly-failing tests
  directly (not inferred): every one is a genuine `pixel diff was N` panic (the test harness's
  strict exact-pixel-diff check), not a pre-existing `NotFound` masked differently:
  - `font_cid_3`: 2399/500395 pixels differ, **max channel deviation 63/255** - a real, visible
    difference, not sub-LSB rounding (`correctness/hayro_tests_font_cid_3_diff_triptych.png`,
    the `[reference | diff | actual]` triptych `hayro-tests` saves on failure).
  - `issue994`: 6/484704 pixels differ, max channel deviation 10/255.
  - `password_1`, `xref_invalid_offsets`, `color_space_lab`: each reported as a small (5-6 pixel)
    but nonzero diff by the test harness's own `eprintln!("pixel diff was {pixel_diff}")`.
  - The newly-failing set spans essentially every embedded font format exercised by the local
    corpus (`font_cid_*`, `font_truetype_*`, `font_type1_*`, `font_type1_cff_*`,
    `font_standard_1`, `fonts_type1_latex`, `font_vertical`) plus several unrelated-looking
    documents that also happen to render text (`password_*`, `xref_invalid_offsets`,
    `color_space_lab`, `issue994`, several `pdftc_*` corpus files) - i.e. this is not a narrow
    edge case limited to synthetic bold/oblique transforms as originally hypothesized in
    candidate `004`; it reproduces broadly whenever a document's glyph runs have any pair of
    outlines that touch or overlap by even a fraction of a pixel after transform, which is common
    across real fonts/hinting, not rare.
- **Is this a breaking change?** No public API change, but it changes rendered pixel output for
  a substantial fraction of real documents (as shown by the test-suite delta above), which is a
  meaningful behavior change for any caller doing exact-pixel comparison against this version.

## Why the result looks the way it does

Merging a whole text run's glyph outlines into one `BezPath` and filling it once with `NonZero`
is only equivalent to filling each glyph separately when no two glyph outlines overlap at any
pixel. Candidate `004` flagged this as a risk for "synthetic bold/oblique transforms, unusual
kerning, Type3 glyphs" specifically; measuring it here shows the risk is broader than that -
ordinary embedded fonts (TrueType, Type1, CFF, CID/Type0) routinely have adjacent glyphs whose
anti-aliased edges touch or slightly overlap after transform (hinting, subpixel positioning,
tight kerning pairs), and `hayro-tests`' snapshot suite has zero tolerance for any resulting
pixel difference. Where outlines actually overlap by more than a fraction of a pixel (as in
`font_cid_3`'s 63/255 max deviation), the combined `NonZero` fill can show real coverage
differences (double-covered regions look identical whether filled once or twice for opaque
paint, but partially-overlapping anti-aliased edges compose differently as one shape's coverage
than as two independently-composited coverages).

Performance moved the wrong way for a different reason: batching trades the baseline's per-glyph
loop (a cheap `ctx.set_transform` + a `fill_path` call against the pre-existing, unmodified,
`Rc`-shared cached outline - zero allocation) for a per-glyph `Affine * &BezPath` multiply that
heap-allocates a new transformed `BezPath` for every glyph, plus growing one shared `BezPath`
across the whole run via `Extend`. The callgrind data gathered before this patch (see above)
already showed `fill_path`'s cost is spread proportionally through flatten/tile/strip rather than
concentrated in a fixed per-call setup cost that fewer, larger calls would amortize away - so
there was little fixed cost to recover, and the batching's own allocation overhead (plus a larger
combined bounding box touching more of vello's sparse tile grid in one pass instead of several
smaller localized passes) outweighed it.

## What's not done / open questions

- Only the `Paint::Color` fill path was changed; `Paint::Pattern` fills and all stroke paths
  (`stroke_outline_run`) were left as-is to keep this candidate to one change - not measured.
- Didn't attempt a "detect non-overlapping bounding boxes first, only batch when provably safe"
  variant - given the *baseline* per-call cost already isn't fixed-cost-dominated (see the
  callgrind data above), such a variant would still carry the same batching-allocation overhead
  measured here while only rarely triggering (advance-box overlap is common even when actual
  glyph-outline overlap isn't), so it isn't expected to turn this into a net win; not built or
  measured this session. See `../IDEAS.md` for this as an open idea if someone wants to check it
  anyway.
- The `../IDEAS.md` backlog entry's own suggestion (get per-call `fill_path` profiling data
  before attempting the rewrite) is not fully honored - only `callgrind`'s instruction-count view
  was available (no `perf` in this environment, same limitation as every other candidate this
  session), which cannot distinguish cache-miss/branch-misprediction-driven fixed overhead from
  proportional work the way real hardware counters could. A machine with `perf` available could
  narrow this further, though the correctness result above is decisive on its own regardless of
  what a sharper perf breakdown would show.
