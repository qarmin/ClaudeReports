# glyph_batching_overlap_aware

Follow-up to candidate `009` (`vello_cpu::fill_path` glyph batching), asked for explicitly
because `009`'s own "What's not done" section speculated that an always-batch variant's 72
new `hayro-tests` failures might be avoidable by only merging glyphs whose transformed outlines
are *provably* non-overlapping, and left it as an untested idea. This candidate builds and
measures that variant: `hayro/src/glyph.rs`'s `fill_outline_run` (`Paint::Color` arm) now
walks the glyph run accumulating a pending batch, and only merges a new glyph into the pending
`BezPath` if its transformed bounding box does not overlap the pending batch's own running
bbox (`kurbo::Rect::overlaps`, checked on `Affine::transform_rect_bbox` of the glyph's
*untransformed* cached-outline bbox - a conservative, cheap-to-compute enclosing box, so a
"no overlap" verdict is always sound even though it never requires materializing the full
transformed path just to check). Any bbox overlap flushes the pending batch (as one combined
`fill_path` call if it has >1 glyph, or the original per-glyph `set_transform`+`fill_path` call
if it only has 1) and starts a new one.

`rustc 1.98.0`, `cargo 1.98.0`.

## What was tried

Bench: `009`'s own bench (`bench/src/main.rs`, copied verbatim, unmodified) - `render()` on
`img.pdf`, 300 iterations, 10% warmup discarded, FNV-1a hash of the produced `Pixmap`'s
premultiplied RGBA8 bytes every iteration. Measured with `../tools/paired.sh`, adapted in place
to parse this bench's `p50=` output (the host's `uptime` was fluctuating between ~10 and ~17
during this session, sibling candidates' worktrees building/benchmarking concurrently, so the
naive-twice-run method candidate `035` flagged as unreliable under load was not trusted alone -
`RUNS=9`, 100 iterations/run, alternating baseline/patched):

| Variant | single-core p50 (`taskset -c 2`, `paired.sh`) | multi-core p50 (unpinned, `paired.sh`) | pixmap hash |
|---|---|---|---|
| baseline (this candidate's own two `taskset -c 2` runs, informal, before switching to `paired.sh`) | 3.9334ms / 3.1504ms | - | `0x9622bedd5d3c7d1c` |
| baseline (`paired.sh` median of 9) | 3.0975ms | 3.1114ms | `0x9622bedd5d3c7d1c` |
| patched (`paired.sh` median of 9) | 3.3596ms | 3.3778ms | `0x304764aed9d52113` |

`paired.sh` verdict: **consistent** in both pin modes - baseline faster in **9 of 9** pairs
single-core, **9 of 9** pairs multi-core (see raw per-pair numbers below).

```
single-core: A: 3.0931 3.0955 3.0975 3.0910 3.1148 3.1190 3.1322 3.1381 3.0899
             B: 3.3596 3.3476 3.3481 3.3545 3.3671 3.3639 3.3930 3.3683 3.3448
             median 3.0975 -> 3.3596  (+8.5%)   B faster in 0 of 9 pairs   verdict: consistent

multi-core:  A: 3.1070 3.1114 3.1173 3.1179 3.0971 3.0914 3.1429 3.1603 3.0319
             B: 3.3844 3.3662 3.3881 3.3498 3.3778 3.4192 3.4058 3.3494 3.2867
             median 3.1114 -> 3.3778  (+8.6%)   B faster in 0 of 9 pairs   verdict: consistent
```

## Result

**~+8.5% slower single-core, ~+8.6% slower multi-core** (p50, both directions consistent across
9/9 paired runs, well above noise) - a larger regression than candidate `009`'s naive
always-batch variant (~+3%). Time only; no memory/allocation-count instrumentation was added.

The pixmap hash for `img.pdf` (`0x304764aed9d52113`) is **bit-identical to candidate `009`'s own
patched hash** - i.e. for this test document, the overlap-aware guard never actually declines to
batch a glyph pair; the whole run merges into one combined `fill_path` call exactly like the
naive variant did, so this file demonstrates no correctness improvement over `009` while running
slower (see "Why" below for the per-glyph bbox-check overhead this variant adds on top of `009`'s
own per-glyph transform-allocation cost).

## Does it change behavior/output?

- **Bit-exact?** No, same as candidate `009`. `img.pdf`'s pixmap hash changes identically to
  `009`'s patched hash (`0x9622bedd5d3c7d1c` -> `0x304764aed9d52113`), stable across all 300
  iterations of both variants in every run performed (bench + the standalone `just test`
  verification below).
- Characterized: same 47/500395 pixels differ (0.0094%), max channel deviation 1/255 as
  candidate `009` reported for `img.pdf` - not re-derived pixel-by-pixel this session since the
  hash is bit-identical to `009`'s own patched output; `correctness/img_pdf_baseline.png`,
  `correctness/img_pdf_patched.png`, `correctness/img_pdf_diff_visible.png` are copied verbatim
  from `009`'s own `correctness/` folder (same bytes, verified by hash - not regenerated).
- **Project test suite** (`cargo test --release -p hayro-tests --test tests -- render
  --test-threads=1`, run twice per variant to confirm a stable failing set before comparing -
  `--test-threads=1` used because an initial default-parallelism run showed the failing set
  itself was not stable run-to-run on this loaded host, see "Why" below):
  - Baseline: **274 passed, 1252 failed** (`correctness/hayro_tests_failing_baseline.txt`),
    identical failing set across 2 runs.
  - Patched: **201 passed, 1325 failed** (`correctness/hayro_tests_failing_patched.txt`),
    identical failing set across 2 runs - **73 new failures**
    (`correctness/hayro_tests_newly_failing.txt`), 0 newly-passing.
  - This is essentially the same regression candidate `009` measured (72 new failures there):
    diffing the two newly-failing lists shows **65 of the 73 (009's 72) test names are
    identical** between the two candidates (`font_cid_3/4/5/7`, `font_truetype_*`,
    `font_type1_*`, `password_*`, `issue994`, `xref_invalid_offsets`, `color_space_lab`, etc.) -
    the small remainder differs by a handful of `pdftc_*`/`stream_jpx_*`/`tile_clamp_bug`
    entries, plausibly baseline corpus-asset availability drift between the two sessions rather
    than a real difference in what the two patches actually do.
  - Not spot-checked individually with pixel triptychs this session (candidate `009` already did
    that for `font_cid_3`, `issue994`, `password_1`, `xref_invalid_offsets`, `color_space_lab` -
    same failure mechanism applies here since the bbox guard essentially never triggers, see
    below).
- **Is this a breaking change?** No public API change, but - same as `009` - it changes rendered
  pixel output for a substantial fraction of real documents.

## Why the result looks the way it does

The bbox-overlap guard was meant to only batch glyphs where doing so is provably safe, avoiding
`009`'s correctness regression while still capturing some of its (hoped-for) performance win.
Neither half of that held up:

- **Correctness: the guard essentially never fires on real text.** Candidate `009`'s own
  postmortem already predicted this ("advance-box overlap is common even when actual
  glyph-outline overlap isn't") - measuring it here confirms it directly: on `img.pdf`, every
  glyph's transformed bbox overlaps the running batch bbox closely enough that the *entire* text
  run still gets merged into one `fill_path` call, producing the identical pixmap hash to `009`'s
  unconditional-always-batch variant. Ordinary text lays glyphs edge-to-edge (by design, for
  legibility/kerning) - two adjacent glyphs' *bounding boxes* almost always touch or overlap even
  when their actual ink (the curves inside those boxes) doesn't, so a bbox-level test is far too
  coarse to distinguish "safe to batch" from "not safe to batch" for typical text layout. The
  `hayro-tests` suite result (73 new failures, largely the same set as `009`) is the direct
  consequence: this variant is not meaningfully safer than always-batching in practice, despite
  being provably conservative in principle.
- **Performance: worse than `009`, not better.** `009`'s own callgrind-based investigation
  already showed `fill_path`'s cost is spread through flatten/tile/strip roughly proportionally
  to geometry, not concentrated in a fixed per-call setup cost - so there was little fixed
  per-call cost available to recover by batching in the first place, and `009` measured the
  batching allocation overhead (one heap-allocated transformed `BezPath` per glyph, `Extend`ed
  into a growing shared path) outweighing it by ~+3%. This variant does everything `009`'s
  variant does (same per-glyph `Affine * &BezPath`-shaped transform work when it does batch) plus
  extra work on **every** glyph regardless of whether batching happens: computing the
  untransformed outline's own bbox (`BezPath::bounding_box()`, an O(elements) scan) and mapping
  its 4 corners through `Affine::transform_rect_bbox` and an `overlaps` test, all before deciding
  whether to batch at all. Since the guard turns out to almost always say "yes, batch" in
  practice (per the correctness finding above), this extra bbox bookkeeping is close to pure
  overhead added on top of `009`'s own already-net-negative mechanism, consistent with the larger
  measured regression (~+8.5%/+8.6% vs. `009`'s ~+3%).
- Separately (unrelated to this candidate's own result, noted for whoever re-runs the test suite
  next): the default-parallelism `cargo test -p hayro-tests` run showed a genuinely unstable
  failing-test count between two back-to-back runs of the *same, unpatched* baseline tree (one
  run: 274 passed/1252 failed; another: 0 passed/1526 failed) on this session's heavily loaded
  host (multiple sibling `perf-optimize` candidates building/testing concurrently). Serializing
  with `--test-threads=1` made the failing set fully stable across repeated runs both before and
  after the patch, which is what all counts in this file use - a flakiness worth keeping in mind
  for any future candidate's correctness check on this project, especially under a loaded host.

## What's not done / open questions

- Only the `Paint::Color` fill path was changed, same scope as `009` - `Paint::Pattern` fills and
  stroke paths were left untouched.
- Didn't try a tighter (non-bbox) overlap test - e.g. an actual polygon/curve intersection check
  - since the bbox test was already the cheapest plausible option and still turned out to almost
    always approve batching for real text; a more precise (and necessarily more expensive) test
    would only make the per-glyph overhead worse while the observed failure mode (bboxes
    routinely touching in ordinary text layout) would very likely still defeat it just as often -
    not measured, so this is a plausibility argument, not a ruled-out result.
- Didn't attempt shrinking the guard to something coarser-but-cheaper (e.g. only batch within a
  single "word" break, or key off `glyph_run`-level metadata already available cheaply) - out of
  scope for this candidate, which was specifically about the bbox-provable-non-overlap approach
  `009` itself proposed.
- 8 of the 73 patched failures' test names don't appear in `009`'s own 72-line list (and vice
  versa for a few of `009`'s) - plausible corpus-asset-availability drift between sessions per
  the note above, not independently confirmed by re-running `009`'s patch on this same host.
