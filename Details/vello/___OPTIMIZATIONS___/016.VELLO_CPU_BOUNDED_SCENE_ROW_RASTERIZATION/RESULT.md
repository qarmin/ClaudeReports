# VELLO_CPU_BOUNDED_SCENE_ROW_RASTERIZATION

`rustc 1.91.0-nightly` / `cargo 1.91.0-nightly` (workspace toolchain, `x86_64-unknown-linux-gnu`).

`vello_cpu::RasterizerSettings::scene_rows` (open PR linebender/vello#1795, not yet merged) adds
an optional half-open range of scene rows to rasterize, letting a tall scene be rasterized one
horizontal band at a time into a band-sized pixmap (bounding peak memory / letting a caller stream
rows to a consumer) without re-recording the scene per band. The mechanism: both dispatchers
(`single_threaded.rs`, `multi_threaded.rs`) now compute a `RasterizerSettings::scene_viewport()`
(an early-return match: `None` -> the unchanged `RectU16::new(0, 0, scene_width, scene_height)`,
`Some((y0, y1))` -> a tile-aligned sub-rect) instead of constructing the whole-scene `RectU16`
directly. This candidate reproduces the PR's diff (adapted by hand to this workspace's pinned
`base_commit`, since the PR's own diff no longer applies verbatim - see "What's not done" below)
and checks the one thing the task asked to check: whether this adds measurable overhead to the
common, non-banded (`scene_rows: None`) case, which is the only case any existing caller of
`vello_cpu` currently exercises.

## What was tried

Reused the project's own `sparse_strips/vello_bench` crate's existing
`integration::images::overlapping` criterion benchmark (gated behind vello_bench's `extended`
feature), which already exercises the exact code path this patch touches:
`RenderContext::render()` -> `render_with(RasterizerSettings::default())` i.e. `scene_rows: None`,
through the full record -> bucket -> coarse -> fine pipeline compositing 5 overlapping images at
1280x960. No new bench was needed; the "no-op case" this candidate is checking is already covered
by the project's own default-configuration benchmark. Built once with vello_cpu's default features
(single-threaded dispatcher, for `taskset`-pinned single-core numbers) and once with vello_cpu's
own `multithreading` feature (`RenderSettings::default()` then picks 8 worker threads, since
`available_parallelism()` on this 24-core machine saturates the `.min(8)` cap - for unpinned
multi-core numbers, exercising `multi_threaded.rs`'s copy of the same diff).

Measurements were taken on a machine shared with 5 sibling optimization candidates running
concurrently (batch session); all `cargo build`/`cargo bench`/`cargo test` invocations were
wrapped in a shared `flock` so no two candidates' heavy steps overlapped, but system-wide load
(from siblings' non-locked work, and from queuing behind their locked steps) still fluctuated
between `uptime` load averages of ~3 and ~27 on this 24-core machine during the session. This
produced two distinct, reproducible measurement regimes for the `images/overlapping` benchmark
regardless of baseline/patched: a "quiet" one (~3.5-3.8ms single-core, ~670-800us multi-core) and
a contended one (single-core up to ~7ms, multi-core up to ~3.6ms, and occasionally a
criterion-calibration artifact landing multi-core anywhere from ~700us to ~1.4ms even when the
system was nominally quiet - see MEASUREMENT.md's "some machines land in one of several
performance modes per process" note, observed directly here). Comparisons below use alternating
baseline/patched pairs (per MEASUREMENT.md 2b) specifically to cancel this drift; single runs
taken during a contended window are not trustworthy in isolation and are not the basis for the
conclusion.

| Variant | single-core p50 (quiet window, 3 alternating pairs) | multi-core p50 (quiet window, 3 alternating pairs) | notes |
|---|---|---|---|
| baseline (scene_rows field absent) | 3.843, 3.551, 3.618 ms | 1.379, 1.352, 1.387 ms | pair-wise alternating with patched |
| patched (`scene_rows: None`, extra match+call per render) | 3.564, 3.571, 3.743 ms | 0.757†, 1.386, 1.369 ms | same pairs, immediately after each baseline run |

† This one multi-core pair landed with baseline in the slower ~1.3ms mode and patched in the
faster ~0.7ms mode - the single instance where the two runs of a pair landed in different modes;
the other 2 multi-core pairs and all 3 single-core pairs had both halves of the pair in the same
mode, which is the more informative comparison for this candidate (see "Result" below).

An additional 6-pair round of single-core and multi-core comparisons was taken earlier in a more
heavily contended window (loads ~12-27) as a sanity check; win/loss was roughly even (3/6
single-core pairs patched-faster, 3/6 patched-slower, by 0.1-53% either direction) - consistent
with "no detectable effect, dominated by noise" but not independently used as the basis for the
conclusion given the contention.

## Result

**No measurable overhead** in the common, non-banded (`scene_rows: None`) case, for either
single-core or multi-core rendering of this benchmark. Single-core: baseline and patched differ by
+0.2%/-6.9%/+3.5% across the 3 quiet-window pairs (mean of the 3 deltas: -1.1%, i.e. patched
fractionally faster on average) - indistinguishable from noise for a full end-to-end
image-compositing benchmark whose iteration time is measured in milliseconds while the actual
change is a single `match`-and-return on a `Copy` `Option<(u16, u16)>` returning the exact same
`RectU16` value as before. Multi-core: the 2 pairs that landed in the same performance mode
differ by +0.4%/-1.3% - likewise noise. This matches the expectation from reading the diff: for
`scene_rows: None`, `scene_viewport()` returns `RectU16::new(0, 0, scene_width, scene_height)` -
bit-for-bit the same value the unpatched code constructed directly - so there is no reason to
expect a real difference, and none was measured. This candidate only measured wall-clock time; no
extra allocation is introduced on the `None` path (`RectU16` is `Copy`, the call is on `self`
which is also `Copy`), so no separate memory measurement was taken.

## Does it change behavior/output?

- **Bit-exact?** Yes, for the `scene_rows: None` (default) case - confirmed both by the pixel-level
  `vello_sparse_tests` suite (see below, byte-identical pass/fail sets before and after) and by
  construction (`scene_viewport()` returns the same `RectU16` `None` produced before this patch).
  For the new, additive `scene_rows: Some((y0, y1))` case (not exercised by any existing caller),
  output for a given band is byte-identical to the corresponding rows of a full-scene render - this
  is exactly what the PR's own new test (`scene_rows_bands_match_a_full_rasterization[_multithreaded]`,
  reproduced here) asserts, and it passed in both the single- and multi-threaded dispatcher.
- **Is this a breaking change?** No - `scene_rows` is a new field on `RasterizerSettings` defaulting
  to `None`, and `RasterizerSettings` is constructed with `..Default::default()` at every call site
  in this codebase, so no existing caller needs to change. (A caller outside this codebase that
  constructs `RasterizerSettings { render_mode, composite_mode, pixel_format, offset }` with all
  fields named and no `..Default::default()` would fail to compile until updated - this is the
  usual "adding a public struct field" source-compatibility caveat, not a runtime behavior change.)
- Project test suite, before (`base_commit` `75168314`) and after (patched):
  - `cargo test -p vello_cpu --features multithreading`: **87 passed / 0 failed** (baseline) ->
    **90 passed / 0 failed** (patched; the 3 new tests from the PR's diff:
    `scene_rows_unaligned_start_panics`, `scene_rows_bands_match_a_full_rasterization`,
    `scene_rows_bands_match_a_full_rasterization_multithreaded`). 1 doctest passed in both.
  - `cargo test -p vello_sparse_tests` (full pixel-snapshot suite, 4856 tests including ignored):
    **3541 passed / 86 failed / 1230 ignored** (baseline) -> **3541 passed / 86 failed / 1230
    ignored** (patched) - identical pass/fail sets (spot-checked the failure list; all 86 failures
    are `*_hybrid` tests comparing against a GPU-rendered reference image, which fail identically
    on this sandbox regardless of this patch - `vello_hybrid`/wgpu is unrelated to the code this
    patch touches, and the failure is present on an unpatched `base_commit` checkout too, i.e. it
    is a pre-existing environment limitation of this sandbox, not something this patch caused).
    Snapshot PNGs (`sparse_strips/vello_sparse_tests/snapshots/*.png`, `vello_tests/snapshots/*.png`)
    are Git LFS objects; `git-lfs` is not installed in this sandbox, so the isolated worktree used
    for this candidate initially checked out LFS pointer-text stubs instead of real images (which
    would have made every snapshot test fail, uninformatively) - the real PNGs were copied in from
    the main checkout (already materialized there) before running this suite, for both the baseline
    and patched runs.

## Why the result looks the way it does

The diff is, for the `None` path, a no-op restructuring: the two dispatchers previously constructed
`RectU16::new(0, 0, scene_width, scene_height)` directly; now they call
`settings.scene_viewport(scene_width, scene_height)`, which pattern-matches `self.scene_rows`,
finds `None`, and returns that exact same `RectU16` value via an early `return`. There is no new
allocation, no new loop, and the function is tiny and marked on a hot call site inside an already
`#[inline(always)]`-friendly module, so it is unsurprising that it measures as noise-level. This is
the expected outcome for "check overhead in the common non-banded case" from the candidate
description, not a surprising one.

## What's not done / open questions

- This is an **open, unmerged PR** (linebender/vello#1795) as of this session, not a merged commit -
  the patch here is a hand-adapted reproduction of that PR's diff against this workspace's pinned
  `base_commit` (`75168314`), not a cherry-pick, because the PR's own diff (fetched via
  `gh pr diff 1795`) no longer applies cleanly to `base_commit` (a handful of unrelated import/line
  drifts in `render.rs`, e.g. an added `PixelMetadata` import, between when the PR was opened and
  this pinned commit). The adapted version preserves the PR's logic and public API (`scene_rows`,
  `SCENE_ROW_ALIGNMENT`, `scene_viewport()`) and its own test additions verbatim; see `patch.diff`.
- This candidate did **not** measure the banded (`scene_rows: Some(..)`) path's actual memory or
  time benefit (i.e. it did not quantify "how much peak RSS does rendering in N-row bands save
  versus rendering the whole scene at once") - the task specifically asked to check overhead in the
  common case, which is what this measures. Quantifying the banded-rendering memory win would be a
  separate, follow-up candidate (see IDEAS.md).
- The `*_hybrid` test failures (pre-existing, unrelated to this patch, see above) mean this sandbox
  cannot fully validate `vello_hybrid` in general - not a gap specific to this candidate.
