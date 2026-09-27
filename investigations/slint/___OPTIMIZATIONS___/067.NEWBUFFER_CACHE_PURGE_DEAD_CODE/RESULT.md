# 67 - the NewBuffer cache purge has nothing to purge

Followed up on the IDEAS.md backlog row ported from candidate 037/NEXT_CANDIDATES.md's "C2":
"bound the partial-rendering cache instead of purging it every frame in
`RepaintBufferType::NewBuffer` mode", on the theory that the unconditional
`self.partial_rendering_state.clear_cache()` call in the `NewBuffer` branches of
`internal/renderers/software/lib.rs` (`render_buffer_impl` and `prepare_scene`) discards useful
cache state every frame "to avoid unbounded growth" (the comment's own words), and that several
already-committed candidates (014/026/032/033/035/036) might be getting none of their benefit in
this mode as a result.

Before touching anything, instrumented `PartialRendererCache::clear()`
(`internal/core/partial_renderer.rs`) with `std::eprintln!("slab.len()={}", self.slab.len())` and
ran `perf_test --bench --mode scroll --count 20000 --repaint-buffer new --frames 5`: **every one of
the 6 `clear()` calls (1 at window setup + 1 per frame) reported `slab.len()=0`.** Same result with
`--repaint-buffer reused` (1 call, at setup, also 0 - nothing has rendered yet).

Reading why: in both `NewBuffer` branches, `partial = repaint_buffer_type != NewBuffer` is `false`,
so `render_component_items` is called with `&mut renderer.actual_renderer` directly
(`internal/renderers/software/lib.rs:684-694` and `:1507-1517`), **bypassing the `PartialRenderer`
wrapper's `do_rendering`/`filter_item` entirely**. Those two methods are the only places that read
or insert into `PartialRendererCache`; `compute_dirty_regions` (the only place that *inserts* new
entries) is also skipped for `NewBuffer` - the branch only sets `dirty_region` to the full screen and
returns. So in `NewBuffer` mode the cache is never written to in the first place; it is not a "purge
every frame" but a "clear an always-empty `slab::Slab` every frame" - free of the allocations it
appears to guard against, because there is nothing to guard against.

This also means candidate 037's explanation of why fix 035 (cull geometry read) is inert in
`NewBuffer` mode needs a small correction: it is not that the cache gets populated and then wiped
before the read - it is that the read (`filter_item`, on `PartialRenderer`) is never reached at all
in this mode, for the same reason (`partial == false`). Same conclusion (035 is near-inert by
default), different mechanism.

## What was tried

Given the cache is provably empty on every `clear()` call in this mode, the closest concrete
embodiment of "don't purge every frame" is removing the now-redundant `clear_cache()` calls from
both `NewBuffer` branches (2 call sites). There is nothing to "bound" - bounding a structure that
never grows is a no-op by construction, so the patch actually tested is the minimal one that
removes the redundant call without changing any other logic.

`perf_test`, software renderer, `rdebug` profile, `RUNS=9 FRAMES=20`, machine's per-process
performance-mode noise (see MEASUREMENT.md 2b) handled via `tools/paired.sh` (alternating A/B, pair
count is the signal, not the median):

| Variant | scene | paired result | verdict |
|---|---|---|---|
| baseline vs. patched | `scroll` 20 000, `--repaint-buffer new` | median 16.93 -> 16.75 ms (-1.1%), B faster in 6/9 pairs | not consistent |
| baseline vs. patched | `scroll` 100 000 `--row-style bare`, `--repaint-buffer new` | median 21.13 -> 20.78 ms (-1.7%), B faster in 5/9 pairs | not consistent |
| baseline vs. patched | `rects` 3 000, `--repaint-buffer new` | median 3.41 -> 3.43 ms (+0.6%), B faster in 2/9 pairs | not consistent |

Allocation counts (exactly reproducible within a run) were identical baseline vs. patched on every
scene: `scroll` 20 000 = 140 976, `scroll` 100 000 `bare` = 300 489, `rects` 3 000 = 45 546 - i.e.
`clear()`'s cost (an empty `Vec`'s `clear()` plus a generation-counter increment) doesn't show up in
allocation counts either, consistent with it being a no-op on an already-empty slab.

Single-core (`taskset -c 3`) matrix runs (`RUNS=5`, two repeats for the noise floor) landed inside
the same spread as the paired runs above and are not reproduced in a separate table - the paired
comparison is the one with a documented pair-count discipline and supersedes it.

## Result

No measurable difference on any scene tried, in either direction, beyond noise (paired A/B pair
counts of 6/9, 5/9, 2/9 - none reach the 8/9-or-9/9 bar this workspace uses for "real"). Allocation
counts are bit-identical between baseline and patched. This is the expected outcome given the root
cause: the removed `clear_cache()` calls were clearing an empty structure, so removing them removes
work that cost effectively nothing to begin with.

## Does it change behavior/output?

- **Bit-exact?** Yes outside the documented per-run noise floor. `tools/pixelcheck.sh`,
  `--mode scroll --count 20000 --repaint-buffer new --frames 10`, `RUNS=5`: noise floor (same
  binary, 5 runs each) 186 px; raw difference between binaries 112 px; **difference outside the
  noise floor: 0 px of 1 100 800**. Baseline and patched render the same image.
- **Is this a breaking change?** No. `clear_cache()` and `PartialRendererCache::clear` are
  crate-internal (`pub(crate)`-reachable via `partial_renderer.rs`, not part of the public API);
  removing two call sites that operate on an already-empty structure changes no observable state.
- Project test suite: not run for this candidate - the change has no code path where it could
  alter behavior (verified by the `slab.len()=0` instrumentation before writing the patch, not
  just after), so a full suite run was not expected to add information beyond the targeted pixel
  check above.

## Why the result looks the way it does

The IDEAS.md entry (and candidate 037's writeup it was built on) assumed the `NewBuffer` branch's
`clear_cache()` call was discarding real, accumulated state - "to avoid unbounded growth", per the
code's own comment. That comment describes the *intent* of the line, not what the surrounding code
actually does today: the `NewBuffer` branch's `partial = false` short-circuit means rendering never
routes through the `PartialRenderer` wrapper that would populate the cache, so nothing ever grows
in this mode for `clear_cache()` to bound or purge. The "up to six prior candidates might be
inert in the default mode" framing from the original task description is still worth investigating
separately (035 is confirmed inert per candidate 037, for the mechanism corrected above), but it is
not fixable by changing anything about the cache-clearing line, because the cache was never the
thing standing between those candidates and this mode - the `partial == false` rendering bypass is.

## What's not done / open questions

- **The real lead this surfaces**: if `NewBuffer` mode is meant to get *any* benefit from
  candidates 014/026/032/033/035/036, the actual blocker is that `render_component_items` is
  called with `renderer.actual_renderer` instead of `renderer` (the `PartialRenderer` wrapper) when
  `partial == false` - i.e. `NewBuffer` mode does not merely skip dirty-region computation, it skips
  the entire `PartialRenderer` layer during rendering. Making `NewBuffer` route through
  `PartialRenderer` (so `do_rendering`'s `PropertyTracker`-based skip and `filter_item`'s
  cached-bounding-rect read would actually run) is a materially different, larger change than
  "bound a cache" - it would need its own correctness case (does re-adding tracker/cache overhead
  in the mode chosen specifically for large trees net out positive, given candidate 037 found
  `NewBuffer` already wins by 1.3-1.7x over `ReusedBuffer` above ~20 000 items *without* any of that
  machinery running) and was out of scope for this candidate. Logged as a new idea in `IDEAS.md`.
- Only the software renderer was checked. FemtoVG has no partial-rendering/dirty-region support at
  all (GitHub #6396, already recorded in `IDEAS.md`), so this doesn't apply there. Skia's
  `RepaintBufferType`-equivalent logic is structurally different (back-buffer-age-driven, always
  routes through `PartialRenderer` when partial rendering is enabled at all) and was not part of
  candidate 037's original finding or this one.
