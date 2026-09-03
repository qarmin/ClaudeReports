# imageops::thumbnail - thumbnail_sample_fraction_horizontal/_vertical/_both get_pixel fast path

`src/imageops/sample.rs`'s `thumbnail_sample_fraction_horizontal`/`_vertical`/`_both` (the three
helpers `thumbnail`'s output loop calls instead of `thumbnail_sample_block` whenever a dimension
is being enlarged, or an output pixel lands exactly on an integer boundary) still read source
pixels via `image.get_pixel(x, y)` through the generic `GenericImageView` trait method, unlike
`thumbnail_sample_block` after candidate 006. This candidate converts all three to the same
`to_pixel_view()`-gated direct-slice-indexing fast path used by candidates 001/004/005/006/007,
per the `IDEAS.md` entry recorded while scoping candidate 006 - with the explicit prediction from
that entry that a workload dominated by *upscaling* (rather than 006's downscale scenario) is
where these three helpers, not `thumbnail_sample_block`, do essentially all the work.

## What was tried

Base commit: `44ce9226c541dd2b11897b9ef07156a28871cdb7`. Two new criterion cases added to the
crate's own `benches/imageops.rs` (bundled into this candidate's `patch.diff`, no prior bench
exercised `imageops::thumbnail` at all - candidate 006's own bench addition was reverted along
with the rest of its patch at the end of that session, so this candidate reintroduces an
equivalent one from scratch):

- `"thumbnail downscale 4032x3024 -> 200x150"` - the same scenario candidate 006 measured
  (`thumbnail_sample_block`-dominated), included as a control: this candidate's patch does not
  touch `thumbnail_sample_block`'s own logic, but does reintroduce the same `fast_view`
  computation in `thumbnail()` that feeds it (necessary plumbing since 006's patch is not present
  in this pristine base), so it should reproduce 006's win as a sanity check that nothing broke.
- `"thumbnail upscale 200x150 -> 4032x3024"` - a 200x150 `RgbImage` enlarged to 4032x3024. Since
  both `x_ratio` and `y_ratio` are `<1` here, `left == right` and `bottom == top` for essentially
  every output pixel, routing it through `thumbnail_sample_fraction_both` - the scenario this
  candidate actually targets, per `IDEAS.md`'s note that "every output pixel routes through
  `thumbnail_sample_fraction_both` when both dimensions are being enlarged."

All numbers are from alternating A/B runs of two prebuilt `--release` criterion binaries
(baseline vs. patched, both built with the bench additions present so only the source patch
under test differs), `--warm-up-time 1 --measurement-time 2` (criterion auto-extended this to
~35s/100 samples for the upscale case, since one iteration there costs ~170-340ms - each printed
number is that run's median over its own 100 internal samples, not a single iteration), via this
workspace's `tmp/run_bench.sh` helper.

**`thumbnail downscale 4032x3024 -> 200x150`, single-core (`taskset -c 3`):**

| Variant | run 1 | run 2 | run 3 | run 4 | run 5 |
|---|---|---|---|---|---|
| baseline | 24.972 ms | 25.988 ms | 25.663 ms | 25.505 ms | 25.443 ms |
| patched | 9.674 ms | 9.884 ms | 10.155 ms | 10.236 ms | 9.538 ms |

Patched faster in 5/5 pairs. Median baseline 25.505 ms, median patched 9.884 ms (~2.58x faster) -
reproduces candidate 006's win, as expected (no logic in `thumbnail_sample_block` itself changed
by this candidate).

**`thumbnail downscale 4032x3024 -> 200x150`, multi-core (unpinned):**

| Variant | run 1 | run 2 | run 3 |
|---|---|---|---|
| baseline | 13.251 ms | 13.543 ms | 13.290 ms |
| patched | 5.324 ms | 5.403 ms | 5.446 ms |

Patched faster in 3/3 pairs. Median baseline 13.290 ms, median patched 5.403 ms (~2.46x faster).

**`thumbnail upscale 200x150 -> 4032x3024`, single-core (`taskset -c 3`):**

This machine went through two distinct "modes" during this measurement (see `IDEAS.md`'s existing
entry on this machine's noise floor); both are reported rather than discarding the first as
inconvenient.

| Variant | 1 (slow mode) | 2 (slow mode) | 3 (slow mode) | 4 (slow mode) | 5 | 6 | 7 | 8 | 9 |
|---|---|---|---|---|---|---|---|---|---|
| baseline | 342.62 ms | 341.51 ms | 341.10 ms | 342.06 ms | 174.93 ms | 176.92 ms | 176.20 ms | 176.88 ms | 174.82 ms |
| patched | 276.00 ms | 333.08 ms | 339.79 ms | 343.18 ms | 173.10 ms | 178.74 ms | 172.74 ms | 172.27 ms | 174.16 ms |

Patched faster in 6/9 pairs, slower in 3/9 (runs 4, 6, and the "slow mode" run 4 essentially tied
at +0.3%). The first pair (run 1, -19.4%) is the largest single delta observed anywhere in this
session, but is not corroborated by any of the following 8 pairs (which cluster around ±0.3% to
-2.6%, i.e. noise-floor-sized) - most plausibly a cold-cache/first-invocation artifact of that
specific process launch rather than a reproducible effect of the patch, since criterion's own
internal 1s warmup (before any of the 100 timed samples that produce the printed median) did not
prevent it. Excluding that first pair, median baseline (runs 2-9) ~208 ms is meaningless to
compute across two different machine "modes" - looking at the two modes separately instead: slow
mode (runs 2-4) baseline 341.1-342.06 ms vs patched 333.08-343.18 ms, no consistent direction;
fast mode (runs 5-9) baseline 174.82-176.92 ms vs patched 172.27-178.74 ms, patched faster in 4/5,
median difference ~-1.5%.

**`thumbnail upscale 200x150 -> 4032x3024`, multi-core (unpinned):**

| Variant | run 1 | run 2 | run 3 |
|---|---|---|---|
| baseline | 174.15 ms | 180.04 ms | 173.25 ms |
| patched | 171.18 ms | 171.32 ms | 174.34 ms |

Patched faster in 2/3 pairs; median baseline 174.15 ms, median patched 171.32 ms (~1.6% faster) -
within this workspace's established noise floor for this machine.

## Result

Two clearly different outcomes depending on scenario, both reproducible:

- **Downscale** (`thumbnail_sample_block`-dominated, unaffected by this candidate's own new code
  but sharing the reintroduced `fast_view` plumbing): ~2.5-2.6x faster, single-core and
  multi-core, 5/5 and 3/3 paired wins - this is a restatement of candidate 006's result on a
  fresh patch, not a new finding.
- **Upscale** (`thumbnail_sample_fraction_both`-dominated, the code this candidate actually
  changes): no clear, consistent win. Single-core paired wins were 6/9 (with one large but
  uncorroborated outlier and the rest within a few percent either direction); multi-core was 2/3,
  ~1.6% median. This is within the noise floor this workspace has established elsewhere (see
  `README.md`/`IDEAS.md`), not a measured performance improvement.

## Does it change behavior/output?

- **Bit-exact?** Yes. `cargo test --release --lib` passes 331/331 both before and after (same
  count, no new failures). `cargo clippy --release --lib --benches` and `rustfmt --check` on both
  touched files report no warnings/diffs.
- Additionally (session-only, not part of `patch.diff`): wrote and ran a temporary standalone
  check (`examples/_thumbnail_fraction_check_tmp.rs`, removed before capturing `patch.diff`)
  hashing `imageops::thumbnail`'s raw output buffer between the baseline and patched builds for:
  - a 200x150 `RgbImage` enlarged to 4032x3024, 601x401, 203x151, 1x1, and 5000x1 (exercises
    `thumbnail_sample_fraction_both`/`_horizontal`/`_vertical` as the dominant path);
  - a 37x29 `RgbaImage` (4-channel) enlarged to 600x500 and 77x31;
  - a 300x200 `Luma<u8>` and a 300x200 `Luma<u16>` (16-bit subpixel) both scaled to 40x30 and
    700x500 (both directions, exercising all three fraction helpers plus the block path);
  - a 4032x3024 `RgbImage` downscaled to 200x150, 63x41, and 1x1 (control: the unaffected block
    path, reusing candidate 006's scenario);
  - a `SubImage` crop (211x133 out of a 400x300 parent `RgbImage`, accessed as
    `&*sub_image_value` to reach the `GenericImageView` impl directly, not converted to an owned
    buffer first) scaled to both 29x17 (downscale) and 500x400 (upscale) - the same "packed row,
    non-contiguous parent buffer" case candidates 001/004/005/006 checked;
  - `DynamicImage::thumbnail` and `DynamicImage::thumbnail_exact` (the public wrappers), both
    directions, on the 4032x3024 source.

  All 18 hashes were identical between baseline and patched builds (see
  `/tmp/baseline_hashes.txt` vs `/tmp/patched_hashes.txt` captured during this session, not saved
  into this folder as they're reproducible from the temp example above plus `patch.diff`).
- **Is this a breaking change?** No - `imageops::thumbnail`'s public signature, panics, and
  numeric results are unchanged. The three helpers' signatures (private, not part of the public
  API) gained a `fast_view: Option<(&[S], usize)>` parameter, matching the convention candidate
  006 established for `thumbnail_sample_block`.

## Why the result looks the way it does

`thumbnail_sample_block` (candidate 006) sums a whole rectangular window of source pixels per
output pixel - for the representative downscale ratio, hundreds of `get_pixel` calls per output
pixel - so removing per-call trait-dispatch/bounds-check overhead compounds into a large total
win. The three fraction helpers this candidate touches do the opposite: each is called once per
output pixel in the (at most) `O(width + height)`-sized boundary/upscale case, and each call reads
only 2 pixels (`_horizontal`/`_vertical`) or 4 pixels (`_both`) total, not a window. The
non-pixel-access work per output pixel - float ratio/clamp arithmetic, the `fract()`/mixing math,
`put_pixel` - is comparable in cost to, or larger than, the 2-4 `get_pixel` calls being optimized,
so removing their per-call overhead does not compound into a measurable win the way it did for
`thumbnail_sample_block`'s O(window-size) access pattern. The prediction in `IDEAS.md`'s original
entry for this candidate (that an upscale-heavy workload would make these three helpers "the
bottleneck") was about *call frequency* (every output pixel instead of `O(width+height)` of them
being called) rather than about *cost per call* - this candidate's benchmark confirms the
frequency prediction (every output pixel does go through `thumbnail_sample_fraction_both` in the
upscale scenario) but shows that being frequently-called doesn't by itself make the per-call
`get_pixel` overhead large enough to matter, unlike the block-averaging case.

## What's not done / open questions

- Did not profile with `perf`/disassembly to directly confirm "non-pixel-access work dominates
  per-output-pixel cost in the fraction helpers" - `perf` remains unavailable in this environment
  (see `README.md`); the explanation follows from the algorithm's O(1)-vs-O(window) access-count
  difference rather than needing a profiler to confirm, and is consistent with the near-zero
  measured delta.
- The single-core upscale measurement crossed two different machine "modes" mid-session (a
  ~340ms and a ~175ms regime for the identical baseline binary and bench) - not caused by this
  patch (both binaries were affected equally within each mode), but see `IDEAS.md`'s existing
  entry on this machine's noise floor for why this happens and what a future session should do
  differently (true round-robin interleaving across more reps) to avoid needing this much
  explanation in a `RESULT.md`.
- Only RGB8/RGBA8/Luma8/Luma16 pixel types and one `SubImage` case were hash-verified; other
  `Pixel` implementations (e.g. floating-point subpixel types) were not separately checked, though
  the fast path's logic does not depend on subpixel type beyond what `ThumbnailSum`/`Enlargeable`
  already require - same caveat candidate 006 recorded.
- Given the measured result (no clear win), this patch is not obviously worth carrying forward on
  its own; it is recorded here per this workspace's rule that every candidate tried gets a full
  writeup regardless of outcome (see `SKILL.md`, "the one rule that matters most").
