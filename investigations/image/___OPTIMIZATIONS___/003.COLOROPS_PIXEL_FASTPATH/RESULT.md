# colorops.rs: get_pixel/put_pixel fast path for invert/contrast_in_place/brighten_in_place/huerotate(_in_place)

`src/imageops/colorops.rs`'s `invert`, `contrast_in_place`, `brighten_in_place`, `huerotate` and
`huerotate_in_place` all iterate every pixel via the generic `GenericImageView::get_pixel`/
`GenericImage::put_pixel` trait methods, with explicit `// TODO find a way to use pixels?`
comments on three of them. These are pure elementwise maps (the output pixel only depends on the
same-position input pixel, never on neighbors), so pixel visitation order doesn't matter. Added a
fast path to each, gated on the crate's own existing `to_pixel_view()`/`to_pixel_view_mut()`
mechanism (same one used by candidate 001's `affine.rs` fast paths and by `copy_from`/
`copy_from_samples`), firing only when the buffer is provably packed/contiguous (true for the
common `ImageBuffer<P, Vec<S>>` case) and falling back to the original per-pixel loop otherwise:

- `invert`, `contrast_in_place`, `brighten_in_place`, `huerotate_in_place`: `to_pixel_view_mut()`
  + iterate `<Pixel>::pixels_from_channels_mut(..)` calling the existing in-place `Pixel::invert`/
  `apply`/`apply_with_alpha` methods (or the same inline channel math for `huerotate_in_place`)
  directly on each `&mut P`, instead of one `get_pixel`+`put_pixel` round trip per pixel.
- `huerotate` (allocates a new `ImageBuffer`): `to_pixel_view()` on the source (`out` is always a
  freshly allocated, provably packed `ImageBuffer` from `buffer_like()`), then `zip()`s
  `pixels_from_channels`/`pixels_from_channels_mut` over both sides instead of one `get_pixel` +
  `enumerate_pixels_mut` per pixel.

## What was tried

Base commit: `44ce9226c541dd2b11897b9ef07156a28871cdb7`. Benchmark: `benches/imageops.rs`. The
existing `invert`, `contrast`, `brighten`, `huerotate` cases already exist; `contrast_in_place`,
`brighten_in_place`, `huerotate_in_place` cases were added (`iter_batched(.., BatchSize::
LargeInput)`, matching candidate 001's convention), and the existing `invert` case was changed
from cloning inside `b.iter` to the same `iter_batched` pattern (less noisy, per candidate 001's
`README.md` note). All bench additions are bundled into `patch.diff`. Source image: the file's
existing synthetic 1920x1080 `RgbImage`.

Two prebuilt `--release` `imageops` bench binaries were compared: "baseline" = unpatched
`colorops.rs` + patched `benches/imageops.rs` (so both binaries expose the same benchmark IDs;
only the function bodies under test differ), "patched" = both files patched. Correctness (see
below) was additionally checked with a separate example binary comparing bit-exact output hashes
between an unpatched-source build and a patched-source build.

**This machine's `/proc/loadavg` is host-wide, not container-scoped** (this session runs in a
Docker container) - `uptime` swung between ~3 and ~12 over the course of measurement with no
visible correspondence to any process inside the container (`ps aux` stayed idle), and a handful
of individual pairs landed in what candidate 001 already documented as a "performance mode
lottery" (a single run - either baseline or patched - randomly 1.5-2x slower than its neighbors
for no code-related reason). Because of this, single paired comparisons were not trusted; each
function was measured over **8 paired runs single-core (`taskset -c 3`) and 6 paired runs
multi-core (unpinned)**, alternating which binary ran first each pair, with `--warm-up-time 1
--measurement-time 1.2` (shortened from criterion's defaults to fit more pairs in the same wall
time). Full raw output: `raw_pairs.txt` in this folder.

| Function | Mode | B (patched) faster in | ratio patched/baseline: min | median | max |
|---|---|---|---|---|---|
| `invert` | single-core | 7/8 | 0.384 | 0.547 | 1.071 |
| `invert` | multi-core | 6/6 | 0.543 | 0.547 | 0.556 |
| `contrast_in_place` | single-core | 3/8 | 0.627 | 1.097 | 1.508 |
| `contrast_in_place` | multi-core | 1/6 | 0.999 | 1.050 | 1.088 |
| `brighten_in_place` | single-core | 0/8 | 1.050 | 1.172 | 1.514 |
| `brighten_in_place` | multi-core | 0/6 | 1.165 | 1.172 | 1.301 |
| `huerotate` | single-core | 7/8 | 0.846 | 0.893 | 1.740 |
| `huerotate` | multi-core | 6/6 | 0.827 | 0.886 | 0.890 |
| `huerotate_in_place` | single-core | 6/8 | 0.904 | 0.962 | 1.071 |
| `huerotate_in_place` | multi-core | 5/6 | 0.902 | 0.929 | 1.150 |

("ratio" is patched-time / baseline-time for each pair; below 1.0 means the patched build was
faster for that pair. "B faster in N/M" counts pairs where the ratio was below 1.0.)

## Result

Mixed across the five functions patched, not a uniform win:

- **`invert`**: strong, consistent win. Median ~1.8x faster, both single- and multi-core, in
  13/14 total paired runs.
- **`huerotate`**: consistent win. Median ~11-17% faster, 13/14 total paired runs, though with an
  occasional individual-pair outlier (one single-core pair at 1.74x - patched slower - amid seven
  wins; see "performance mode lottery" note above).
- **`huerotate_in_place`**: small, mostly consistent win. Median ~4-7% faster, 11/14 total paired
  runs.
- **`contrast_in_place`**: no clear win - noisy, and if anything median ratios sit slightly above
  1.0 (patched marginally slower on the typical pair: +5-10% single-core median, +5% multi-core
  median), but with only 3/8 and 1/6 pairs favoring the patch. Reads as within-noise-to-mild
  regression, not a win.
- **`brighten_in_place`**: consistent regression. Patched was slower in every single one of the 14
  paired runs (8 single-core + 6 multi-core), by 5-51% depending on the pair, median ~17% slower
  both single- and multi-core. This is the only one of the five with a unanimous, unambiguous
  direction against the patch.

## Does it change behavior/output?

**No - bit-exact.** `cargo test --release` (331/331 lib unit tests, plus every other test binary)
passes identically before and after this patch, **except** `tests/regression.rs::
resizing_with_catmul`, which was independently confirmed to already fail on the unpatched base
commit (unrelated to this candidate - likely a `pic-scale-safe` version-resolution drift from
candidate 002's session, not investigated further here).

Additionally wrote and ran a temporary standalone example (`examples/_correctness_check.rs`, not
part of `patch.diff`, removed before reverting) that hashes (FNV-1a over the raw byte buffer) the
output of all five functions on both `Rgb<u8>` and `Rgba<u8>` synthetic images, plus the same five
calls made through a `SubImage` view (`GenericImage::sub_image`) of a larger parent buffer - this
exercises the *fallback* path specifically, since a `SubImage`'s row stride belongs to its parent
and is never "fully packed" by this patch's fast-path check. Every hash matched byte-for-byte
between an unpatched-source build and a patched-source build, for both the fast-path case (plain
`ImageBuffer`) and the fallback case (`SubImage`).

**Not a breaking change** - no public signature, panic behavior, or documented numeric result
changed for any of the five functions.

## Why the result looks the way it does (investigated, not fully resolved)

All five functions were converted using the *same* mechanism (candidate 001's proven
`to_pixel_view`/`to_pixel_view_mut` fast-path pattern), yet the outcome splits into a clear win
(`invert`, `huerotate`, `huerotate_in_place`) and a clear loss (`brighten_in_place`, with
`contrast_in_place` closer to a wash). Disassembly (`objdump -d` on the release bench binaries,
`nm`-located symbols) was used to look for a codegen-level explanation, bounded per SKILL.md's
guidance rather than exhaustively pursued (`perf` is not installed in this environment - see
`README.md`'s Environment section):

- `invert`'s fast-path loop compiles to a tight, fully-inlined `not byte` x3 unrolled loop with no
  calls - matching candidate 001's `flip_vertical_in` finding that eliminating the get_pixel/
  put_pixel round trip is the dominant cost when the per-pixel body itself is trivial.
- `brighten_in_place`'s fast-path loop (via `Pixel::apply_with_alpha`) also compiles to a tight,
  fully-inlined scalar loop (`movzx`/`add`/`test`/`cmovle`/`cmp`/`cmovge`/`mov` x3, unrolled per
  pixel) with no calls - structurally similar to `invert`'s, just with more per-byte work (add +
  saturating clamp instead of a single `not`). In the **unpatched** build, `brighten_in_place`
  does not appear as a separate symbol at all (`nm` finds no
  `colorops...brighten_in_place...` symbol in the baseline binary) - it was fully inlined into its
  bench call site, unlike every other function in this file. This is circumstantial evidence that
  the *baseline* version benefited from an inlining/optimization opportunity at its specific call
  site that the fast-path version's larger function body (the `to_pixel_view_mut` validation
  prologue) no longer qualifies for, rather than the fast-path loop itself being pessimized in
  isolation - consistent with the measured regression being of a similar magnitude
  (~15-30%) to what one inlining decision could plausibly cost, but this was not confirmed further
  (e.g. by forcing/denying inlining explicitly and re-measuring) given the bounded-time guidance.
- `huerotate`'s per-pixel body (9 `f64` multiplies via a matrix) is heavy enough, and its two
  variants' generated loops similar enough in shape (both scalar `mulsd`/`addsd` sequences with no
  vectorization either way), that the measured ~11-17% win is plausibly just the saved
  get_pixel-call/bounds-check overhead as a smaller fraction of a larger per-pixel cost, similar to
  `rotate180_in_place`'s smaller-than-`flip_vertical_in` win in candidate 001.
- `contrast_in_place`'s per-pixel body (float divide, multiply, clamp, two `NumCast` conversions)
  sits in between in per-pixel cost, and its result sits in between too (no clear direction) -
  consistent with the pattern above, but not independently confirmed to be the *cause* rather than
  a coincidence.

The working hypothesis - inlining-context sensitivity (whether the get_pixel/put_pixel loop or the
`to_pixel_view`-based loop ends up inlined into its specific call site) matters more than which
iteration strategy is used in isolation - was not confirmed with a controlled experiment (e.g.
`#[inline(never)]` on both variants, or a standalone `criterion` micro-bench isolating just the
loop body with no surrounding call). That would be the natural next step for anyone trying to
recover `brighten_in_place`'s regression while keeping `invert`/`huerotate`'s wins.

## What's not done / open questions

- Root cause of `brighten_in_place`'s regression (and `contrast_in_place`'s wash) is not fully
  confirmed - see above. Added to `IDEAS.md` as a follow-up (inlining-sensitivity investigation),
  since a partial revert (keep the fast path only for `invert`/`huerotate`/`huerotate_in_place`,
  drop it for `contrast_in_place`/`brighten_in_place`) is one obvious option a reader could take
  from this data, but per this skill's rules that decision is left to the reader, not made here.
- Did not try `#[inline(always)]`/`#[inline(never)]` annotations, nor a standalone
  micro-benchmark isolating just the loop body from its call site, to confirm the inlining
  hypothesis above.
- The host's `/proc/loadavg` being container-external made every measurement noisier than
  candidate 001/002's sessions already reported; the 8+6 paired-run-per-function protocol used
  here is heavier than those candidates' typical 2-7 pairs specifically because of this - worth
  carrying forward as the default for future candidates on this machine rather than assuming 2-3
  pairs suffice.
