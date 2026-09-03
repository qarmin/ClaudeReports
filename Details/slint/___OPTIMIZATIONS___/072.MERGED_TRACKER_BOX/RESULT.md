# 72 - Merge `PartialRenderingCachedData`'s two boxed `PropertyTracker`s into one allocation

IDEAS.md's backlog (added while building candidate 071) named this the lowest-risk of two fix
shapes for shipping 071's finding soundly: candidate 065 added a second per-item
`geom_tracker: Pin<Box<PropertyTracker>>` next to the existing `tracker: Option<Pin<Box<PropertyTracker>>>`,
and 071 showed that the *extra heap indirection* (not the tracking mechanism) was 065's
regression's cause - but 071's literal "store inline in the slab" fix is unsound (`PropertyTracker`
is `!Unpin`/self-referential, segfaults on any move before its final address). This candidate
builds the safe alternative: keep both trackers boxed (so nothing about the slab's own storage
needs to change, and no reallocation-safety hazard is introduced), but merge them into **one**
`Box` instead of two, since a `PropertyTracker` doesn't need its *own* allocation - it needs *an*
allocation with a stable address, and one box can hold both.

## What was tried

Two implementations, in order (the first was found to have a real design flaw and was replaced,
not reported as the final answer):

**Attempt 1 (literal reading of the backlog's "`Box<{ data, tracker, geom_tracker }>`"):** merged
`data: CachedItemBoundingBoxAndTransform` itself into the same box as both trackers, so
`PartialRendererCache::slab` became `slab::Slab<Pin<Box<PartialRenderingCachedData>>>` instead of
`slab::Slab<PartialRenderingCachedData>`. This regressed badly - worse than candidate 065's
*two*-box version, not better. Root cause: `slab::Slab<T>`'s whole reason for existing here is a
contiguous, cache-friendly `Vec<T>` that `compute_dirty_regions` walks every frame, reading
`data` (bounding rect, transform, sibling index) for potentially every item in the tree, most of
which are never re-rendered. Wrapping the *entire* entry (data included) in a `Box` turns that
walk from "read inline, contiguous fields" into "chase one heap pointer per item, then read"
for every single item, every frame - which is exactly what the slab was supposed to avoid, and
outweighed the one-fewer-allocation saving completely.

**Attempt 2 (the one measured below, and the one in `patch.diff`): keep `data` inline in
`PartialRenderingCachedData` (so the slab stays a plain, contiguous `slab::Slab<PartialRenderingCachedData>`,
identical to master's layout), and merge only the two `PropertyTracker`s into a new
`PartialRenderingCachedDataTrackers { tracker: Option<PropertyTracker>, geom_tracker: PropertyTracker }`
struct, itself behind one `Pin<Box<...>>` field (`trackers`) on `PartialRenderingCachedData`.**
Uses the already-a-dependency `pin-project` crate (already used the same way in this file's
neighbor `window.rs` for `WindowPinnedFields`) to get safe `Pin`-projected field access
(`Option::as_pin_ref`/`as_pin_mut`, `Pin::set` for the "destroy tracker to force redirty" and
"lazily create tracker" call sites) with zero `unsafe` added (`internal/core` denies
`unsafe_code`). Compiled and passed `cargo clippy` cleanly on the first attempt for both versions.

| Variant | scenario | single-core (`taskset -c 2`) | multi-core | vs |
|---|---|---|---|---|
| baseline (plain `master`, `cdb6c43f7`) | `layout`, static (`--scroll-step 0`, count 100000) | 151.84ms median | 152.10ms median | - |
| candidate 065 (two separate boxes) | same | 170.59ms median (+13.3% vs baseline, matches 065's own RESULT.md) | not re-measured | 065 |
| **attempt 1 (whole entry incl. `data` in one box)** | same | **199.34ms median (+30.8% vs baseline, 0/9 paired, consistent)** | not measured (disqualified by single-core result) | baseline |
| **attempt 1** | same | **200.78ms median (+20.1% vs 065, 0/9 paired, consistent)** | | 065 |
| **attempt 2 (`data` inline, only trackers merged)** | same | **174.84ms median (+15.1% vs baseline, 0/9 paired, consistent)** | **174.45ms median (+14.7% vs baseline, 0/9 paired, consistent)** | baseline |
| **attempt 2** | same | **176.01ms median (+3.2% vs 065, 0/9 paired, consistent)** | | 065 |
| baseline | `layout`, scrolling (count 100000) | 156.16ms median | not measured | - |
| **attempt 2** | same | **176.79ms median (+13.2% vs baseline, 0/9 paired, consistent)** | | baseline |
| candidate 065 | same | 173.34ms median | | - |
| **attempt 2** | same | **176.97ms median (+2.1% vs 065, 3/9 paired, NOT consistent)** | | 065 |
| baseline | `flat`, scrolling (count 100000) | 108.39ms median | not measured | - |
| **attempt 2** | same | **122.28ms median (+12.8% vs baseline, 0/9 paired, consistent)** | | baseline |
| candidate 065 | same | 114.43ms median | | - |
| **attempt 2** | same | **123.52ms median (+7.9% vs 065, 0/9 paired, consistent)** | | 065 |
| baseline | `bare`, scrolling (count 100000) | 26.73ms median | not measured | - |
| **attempt 2** | same | **27.07ms median (+1.3%, 5/9 paired, NOT consistent - no real change)** | | baseline |

All paired via `tools/paired.sh` (`RUNS=9 FRAMES=20`), same protocol as candidates 065/071.
`uptime` load average stayed under 2 (24-core machine) for every run.

## Result

Neither implementation recovers candidate 065's regression, let alone reaches candidate 071's
"removing the extra indirection can beat plain baseline" result. Attempt 1 is a clear, decisive
regression relative to *both* baseline and 065 (+30.8%/+20.1%, 0/9 paired both) and was replaced
rather than reported as the finding. Attempt 2 - the sound, generally-applicable version of "one
box instead of two" that the backlog described - is a smaller but still real and consistent
regression relative to baseline on every scene with actual per-item work (`layout` static +15.1%
single-core/+14.7% multi-core, `layout` scrolling +13.2%, `flat` scrolling +12.8%, all 0/9 paired
i.e. every single pair favored baseline), and is **no better than candidate 065's own two-box
version** - flat at best (`layout` scrolling, not a decisive 8-9/9 result either way) and
measurably *worse* on two of three scenes checked (`layout` static +3.2%, `flat` scrolling +7.9%,
both 0/9 paired, i.e. every pair favored 065). Only `bare` (065's own no-regression case, almost
no properties touched per item) shows no consistent difference from baseline, matching 065's own
finding for that scene.

## Does it change behavior/output?

- **Bit-exact**, `tools/pixelcheck.sh` (attempt 2, `RUNS=6`, `--count 20000 --frames 6`),
  pixel-identical outside the noise floor on: `scroll` x (`layout`/`flat`/`bare`/`clipped`),
  `layout --scroll-step 0`, `layout --mutate 50 --mutate-mode push`, and
  `layout --mutate 50 --mutate-mode insert-front`.
- `cargo test -p i-slint-core --lib`: 307/307 passed, unchanged, for attempt 2.
- Attempt 1 was not pixel-checked beyond a quick sanity pass at `--count 20000` (also bit-exact
  there) since its timing already disqualified it before further correctness work was spent on it.
- **Is this a breaking change?** No public API changes in either attempt - both are private to
  `partial_renderer.rs`. `pin-project = "1"` was already a dependency of `i-slint-core` (used the
  same way for `window.rs`'s `WindowPinnedFields`), so no new dependency was added.

## Why the result looks the way it does

Attempt 1's failure mode is explained above (destroys the slab's contiguous-storage benefit for
`data`, which is read far more often - every visited item, every frame - than either tracker).

Attempt 2's failure mode is more subtle and is the actual answer to what the backlog asked:
**"one allocation instead of two" is true by allocation *count*, but not by allocation *size* or
*frequency* in the way that matters for this workload.** Under candidate 065's original two-box
design, for the overwhelming majority of items in an unvirtualized 100000-row `scroll` list (only
a small fraction are ever on-screen and thus ever call `do_rendering`), the cost is: one
`geom_tracker` box (~1x `PropertyTracker`'s size) per item, and the `tracker: Option<Pin<Box<...>>>`
field costs only a null pointer (8 bytes, no allocation) for every item that's never rendered.
Under attempt 2's merged design, *every* item's single allocation must be sized to hold **both**
trackers (`Option<PropertyTracker>` plus `PropertyTracker`, roughly 2x a `PropertyTracker`'s size)
up front, since the box has to exist before either tracker can be evaluated/registered - even
though the `tracker` slot sits unused (`None`) for the overwhelming majority of items that are
never actually rendered. The trade is real: attempt 2 avoids 065's *second* allocation event
entirely (writing into the already-boxed slot via `Pin::set` when an item first renders costs
nothing extra), but pays a larger allocation for every one of the many more not-currently-visible
items to do it. For this specific workload (huge total item count, small visible fraction), that
trade nets negative - the reverse of what 071's own diagnostic measurement suggested, because 071's
"attempt 2" achieved something attempt 2 here cannot: *zero* allocation overhead for `geom_tracker`
at all (stored raw, unboxed, directly in the slab) for every item, sidestepping the soundness
hazard only via a hardcoded capacity reservation rather than fixing it. This candidate's "sound"
fix keeps paying one real allocation per item either way; only its *size*, not its existence,
changed - and that saving is smaller than the up-front size cost it introduces at this item-count
ratio.

## What's not done / open questions

- **Fix shape #1 (merge into one `Box`), even done soundly, does not recover 071's result** - this
  question is now answered, and closed: the "generalized" fix shape #2 named alongside it
  (a chunked/stable-address arena, e.g. `sharded-slab` or a hand-rolled `Vec<Box<T>>`-of-chunks,
  letting `geom_tracker` live **unboxed** directly in stable per-entry storage, matching 071's
  diagnostic attempt 2's actual allocation profile without its hardcoded-capacity soundness gap)
  is the only remaining path that could realize 071's originally measured upside for real. Not
  attempted in this candidate - real, nontrivial scope of its own (a new storage backend for
  `PartialRendererCache`, plus auditing every existing `cache_index`-based access pattern against
  it), and left as a fresh backlog entry rather than attempted under this one's budget.
- No `perf`/allocation-profiler attribution of the exact allocator-side cost difference (box size
  vs. box count) was attempted - `perf_event_open` remains blocked in this environment; the
  explanation above is a reasoned account of the measured numbers, not a confirmed allocator trace.
- Only the software renderer (`--bench`) was measured, same limitation as candidates 049-051/065/071
  (no GPU display available this session).
- Multi-core was only measured for the `layout` static scene (the one attempt 1 already flagged
  as a clear loss); the `layout`/`flat` scrolling scenes were only checked single-core, consistent
  with 071's own scope for its secondary scenes.

## Reproduce

```sh
# from ___OPTIMIZATIONS___/, with pt_072_base (plain master), pt_072_065 (candidate 065's
# patch), and pt_072_merged_v2 (this candidate's patch.diff, attempt 2) built - see justfile.
RUNS=9 FRAMES=20 taskset -c 2 bash tools/paired.sh /tmp/pt_072_base /tmp/pt_072_merged_v2 -- \
    --mode scroll --count 100000 --row-style layout --scroll-step 0
RUNS=9 FRAMES=20 bash tools/paired.sh /tmp/pt_072_base /tmp/pt_072_merged_v2 -- \
    --mode scroll --count 100000 --row-style layout --scroll-step 0   # multi-core
RUNS=9 FRAMES=20 taskset -c 2 bash tools/paired.sh /tmp/pt_072_065 /tmp/pt_072_merged_v2 -- \
    --mode scroll --count 100000 --row-style layout --scroll-step 0   # vs candidate 065
RUNS=6 bash tools/pixelcheck.sh /tmp/pt_072_base /tmp/pt_072_merged_v2 -- \
    --mode scroll --count 20000 --row-style layout
```
