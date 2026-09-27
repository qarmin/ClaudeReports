# 71 - Inline (non-boxed) `PropertyTracker` for candidate 065's per-item geometry tracker

IDEAS.md's backlog (added while building candidate 065) asked whether the consistent wall-time
regression 065 measured on `Text`-bearing rows - even in the fully static case, despite a real
43% allocation reduction - came from the extra heap indirection of a separate
`Pin<Box<PropertyTracker>>` per item, rather than from the tracking mechanism itself. The idea:
store `PropertyTracker` inline in the `slab::Slab` entry instead, "if its `!Unpin` requirement
can be satisfied in-place." This candidate builds that and answers the conditional.

## What was tried

**Attempt 1 (the literal idea): replace `geom_tracker: Pin<Box<PropertyTracker>>` with a plain
inline `geom_tracker: PropertyTracker` field**, changing every call site from `geom_tracker.as_ref()`
(a `Pin<Box<T>>` deref) to a new `pin_geom_tracker(&geom_tracker)` helper that does
`unsafe { Pin::new_unchecked(tracker) }` to keep the type-level API working. This compiles cleanly
(with `#[allow(unsafe_code)]` on the new helper, `internal/core` denies `unsafe_code` by default).

It **segfaults on the very first frame** of every scene tried (`--mode scroll --row-style layout
--count 20000`, and every other scene/style below), 100% reproducible, not a rare race:

```
Program received signal SIGSEGV, Segmentation fault.
0x... in i_slint_core::properties::mark_dependencies_dirty::{closure#0} (binding=<optimized out>) at properties.rs:833
833:    (binding.vtable.mark_dirty)(binding as *const BindingHolder, was_dirty);
#0  mark_dependencies_dirty::{closure#0}
#1  DependencyListHead::for_each
#2  mark_dependencies_dirty (dependencies=0x...)
...
#8  Property<Length<f32, LogicalPx>>::set  (a two-way-bound geometry property being set)
#16 MainWindow::set_scroll_y
#17 perf_test::bench::run
```
(full backtrace captured via `gdb -batch -ex run -ex bt`, reproduced identically across two
separate builds - see "Additional finding" below for why capacity reservation didn't change it).

**Root cause, worse than the "reallocation on growth" hazard the idea's own caveat anticipated:**
`PropertyTracker` is `!Unpin` and self-referential - `register_as_dependency_to_current_binding`
stores a raw pointer *into the tracker's own `BindingHolder`* inside the dependency list of every
property it reads (`DependencyNode::prev` in `properties.rs`'s `dependency_tracker` module points
back at `&geom_tracker.holder.dependencies`). The `None` branch (first time an item is seen)
builds `geom_tracker` as a **local variable**, calls `evaluate_if_dirty` on it (registering that
raw pointer against the *local's stack address*), and only afterward moves it - twice - into
`PartialRenderingCachedData::new(...)` and then into the slab via `cache.insert(...)`. Both moves
happen *after* other properties already hold a pointer to the now-abandoned stack location. The
very next time anything sets one of those properties (here: `scroll_y`, two-way-bound to a
geometry property that a visible row read during its `evaluate_if_dirty` call), `mark_dependencies_dirty`
walks that list and dereferences the stale pointer - `BindingHolder<()>` at an address that is by
then unrelated stack memory, `vtable.mark_dirty` is garbage, and it segfaults. This is not the
Vec-reallocation hazard the idea flagged as the thing to check - it fires unconditionally, on
frame 1, for the *existing-item* re-evaluation path too, has nothing to do with the slab growing
at all. It is simply not possible to safely call anything requiring `Pin<&Self>` on a value before
its *final* resting place, and `Pin::new_unchecked` does not change that - it only removes the
compiler's ability to catch the mistake.

**Attempt 2 (diagnostic-only, not a real fix, built solely to answer the performance question in
isolation):** two changes on top of attempt 1, both explicitly commented as diagnostic-only in
`patch.diff`:
1. In the `None` (new-item) branch: compute a placeholder `CachedItemBoundingBoxAndTransform`
   untracked, `cache.insert()` it with a freshly-`Default`-constructed (still-dirty)
   `geom_tracker`, obtain a `&mut` reference back into the *slab-resident* copy via
   `cache.get_mut(index)`, and only then call `evaluate_if_dirty` on *that* reference before
   overwriting `entry.data`. This never evaluates while `geom_tracker` is at a temporary address.
2. `PartialRendererCache::default()` pre-reserves `slab::Slab::with_capacity(600_000)` so the
   slab's backing `Vec` cannot reallocate (and thus cannot move already-registered entries) for
   any scene size used in this session's measurements. This does **not** fix the underlying
   hazard for the *existing-item* re-evaluation path (`pin_geom_tracker(geom_tracker)` on a
   `&mut` reference straight into the slab) - it only prevents this specific set of runs from
   ever exercising it, which is exactly why it is diagnostic-only.

This variant does not crash on anything tried (see "Does it change behavior/output?"), and gives
the numbers below.

| Variant | scenario | single-core (`taskset -c 2`) | multi-core |
|---|---|---|---|
| baseline (plain `master`, `cdb6c43f7`) | `layout`, static (`--scroll-step 0`, count 100000) | 150.97ms median | 150.02ms median |
| candidate 065 (boxed `geom_tracker`) | same | 171.06ms median (**+13.3%** vs baseline) | not re-measured this session (065's own RESULT.md: +13.8%) |
| **attempt 2 (inline, diagnostic-only)** | same | **142.71ms median (-5.5% vs baseline, -16.9% vs 065, 9/9 paired)** | **141.50ms median (-5.7% vs baseline, 9/9 paired)** |
| baseline | `layout`, scrolling (count 100000) | 154.65ms median | 159.88ms median |
| **attempt 2** | same | **150.22ms median (-2.9%, 9/9 paired)** | **155.98ms median (-2.4%, 8/9 paired)** |
| baseline | `flat`, scrolling (count 100000) | 108.72ms median | not measured |
| **attempt 2** | same | **107.57ms median (-1.1%, 8/9 paired)** | |
| baseline | `bare`, scrolling (count 100000) | 26.96ms median | not measured |
| **attempt 2** | same | **23.38ms median (-13.3%, 9/9 paired)** | |

All paired via `tools/paired.sh` (`RUNS=9 FRAMES=20`), same protocol as candidate 065.

## Result

Attempt 1 (the literal idea as stated) is not a viable patch: it crashes deterministically on
first use, independent of scene or item count, for the reason above.

Attempt 2 (diagnostic-only, sidesteps but does not fix the underlying hazard) answers the
question IDEAS.md actually asked: **removing the extra heap indirection does eliminate 065's
regression**, and turns the static-`layout` case (065's worst regression, +13.3-13.8%) into a
**net win over plain baseline** (-5.5% single-core, -5.7% multi-core, 9/9 and 9/9 paired), and a
much larger win relative to 065's boxed version (-16.9%, 9/9 paired) on the same scene. The
`scrolling`-`layout` and `scrolling`-`flat` cases (065's other regressions) also turn into small,
consistent wins over baseline (-2.9%/-2.4% and -1.1% respectively) rather than regressions. The
`bare` case, which 065 already won, wins by roughly the same margin here (-13.3% vs 065's -12.3%),
i.e. inline storage does not cost anything extra in the case that was already fine.

This confirms the suspected cause named in the candidate description: candidate 065's regression
was the separate heap allocation/indirection, not the `PropertyTracker`-based skip mechanism
itself - but the fix as literally specified ("store inline in the slab entry") is unsound with
`slab::Slab`'s Vec-backed storage and cannot be shipped in the form measured here.

## Does it change behavior/output?

- **Attempt 1: crashes.** Not a correctness *difference* to characterize - a deterministic
  SIGSEGV on ordinary use, reproduced via `gdb -batch -ex run -ex bt` (see backtrace above). No
  pixel comparison is meaningful for a binary that does not survive one frame.
- **Attempt 2 (diagnostic-only): bit-exact against baseline**, `tools/pixelcheck.sh` (RUNS=5,
  `--frames 6`), pixel-identical outside the noise floor on: `scroll` x (`layout`/`flat`/`bare`/
  `clipped`) x (`--scroll-step 0` and default scrolling), and `scroll --row-style layout` x
  `--mutate 50 --mutate-mode push` and `--mutate-mode insert-front`. `list` and `rects` sanity-ran
  without crashing (not pixel-compared this session).
- **Attempt 2 is still not sound in general** and must not be read as "the fix" - see "Why the
  result looks the way it does". It relies on: (a) the reordering in the `None` branch (a real,
  generally-applicable fix for *that* specific bug - evaluate only through the final address), and
  (b) a hardcoded `with_capacity(600_000)` that happens to be large enough for every scene size
  used in this session's measurements and is explicitly commented in `patch.diff` as not a real
  fix - a long-lived window, or any scene with more live items than that, would still hit the
  Vec-reallocation version of the same crash.
- A stress test intended to actually trigger the still-present reallocation hazard under attempt 2
  (`--mode scroll --count 5000 --row-style layout --mutate 500 --mutate-mode insert-front --frames
  200`) did complete (after ~5 minutes real time - `insert-front` is the O(rows) reshape
  candidates 022/043 already documented, so 200 frames x growing insert-front cost compounds
  badly, up to 3.00s/frame by frame 199, 210 594 total insert-front calls) without crashing (exit
  0). This is consistent with the entry count staying under the reserved 600 000 capacity for this
  particular parameter choice, not evidence that the reallocation hazard is fixed - it was
  sidestepped by construction, exactly as documented above.
- **Is this a breaking change?** No public API changes in either attempt. Both attempts are
  private to `partial_renderer.rs`.
- No project test suite was run against either attempt this session (measurement-only session,
  same as 065); attempt 1 is disqualified by the crash regardless.

## Why the result looks the way it does

`PropertyTracker`'s dependency-tracking is a genuine self-referential, pinned data structure by
design (see `internal/core/properties.rs`'s `dependency_tracker` module) - every existing use in
the codebase (`item_rendering.rs`, `menus.rs`, `component_container.rs`, `items/path.rs`,
`items/input_items.rs`, and 065's own `tracker: Option<Pin<Box<PropertyTracker>>>` field right
next to this one) stores it behind `Pin<Box<...>>` specifically because a `Box`'s heap allocation
has a stable address regardless of what container holds the `Box` pointer - moving the `Box`
(e.g. because the surrounding `Vec` reallocated) never moves the pointee. Storing the
`PropertyTracker` value directly inside a `slab::Slab<T>` entry removes exactly that guarantee:
`slab::Slab` is backed by a plain growable `Vec<Entry<T>>`, so (a) inserting past capacity moves
every existing entry to a new buffer, and (b) even before any growth, a value must physically be
constructed *somewhere* before it lands in its final slot, and evaluating dependencies against
that intermediate address (as the straightforward "create locally, then insert" pattern does) is
already broken regardless of what container it ends up in. Attempt 2's fix for (b) - evaluate only
after the value is already in its final slab slot - is a real, generally applicable pattern (it is
exactly what the *existing-item* re-evaluation branch already did, by construction, since it works
through a `&mut` reference obtained from the slab). Its "fix" for (a) - reserve enough capacity
that growth never happens - is not a fix at all, just a way to avoid exercising the bug for the
specific runs measured here; a `slab`/arena crate that guarantees per-entry address stability
across growth (e.g. one backed by fixed-size chunks rather than a single contiguous `Vec`, such as
`sharded-slab`, or a hand-rolled `Vec<Box<PartialRenderingCachedData>>` so only the pointers move)
would be a real fix, at the cost of reintroducing one allocation per item (defeating the purpose)
unless entries are batch-allocated in chunks.

## What's not done / open questions

- **No sound, generally-applicable implementation of "inline in the slab" was built or measured.**
  Attempt 2 answers the narrower research question ("would removing the box recover the
  regression, if it could be done safely") affirmatively, but is not itself something a maintainer
  could take - see the capacity caveat above.
- A real fix would need either: a slab/arena data structure with per-entry address stability
  across growth (chunked storage), or accepting one allocation per item but merging it with
  `PartialRenderingCachedData`'s *other* boxed field (`tracker: Option<Pin<Box<PropertyTracker>>>`)
  into a single combined `Box<{ data, tracker, geom_tracker }>` per item so there is at most one
  heap object instead of two - this second option was not attempted this session and is added to
  IDEAS.md.
- The stress test intended to reproduce the *other* (reallocation-based) crash under attempt 2's
  workaround did not finish in-session (see above) - whether that path is exercised even less
  favorably than attempt 1's guaranteed-every-time crash, or behaves identically once triggered,
  is unconfirmed.
- Only the software renderer (`--bench`) was measured, same limitation as candidates 049-051/065
  (no GPU display available this session).
- `perf`/allocation-profiler attribution of *why* the inline version's per-item cost is lower than
  the boxed version's (cache-miss avoidance vs. TLS/pointer-chase savings, candidate 065's own open
  question) was not attempted this session either - `perf_event_open` remains blocked in this
  environment.

## Reproduce

```sh
# from ___OPTIMIZATIONS___/, with pt_071_base (plain master) and pt_071_inline_fixed (attempt 2,
# diagnostic-only) built - see justfile for the full clone+apply+build sequence, and note the
# WARNING there: applying patch.diff exactly as saved reproduces attempt 1's crash on frame 1.
RUNS=9 FRAMES=20 taskset -c 2 bash tools/paired.sh /tmp/pt_071_base /tmp/pt_071_inline_fixed -- \
    --mode scroll --count 100000 --row-style layout --scroll-step 0
RUNS=9 FRAMES=20 bash tools/paired.sh /tmp/pt_071_base /tmp/pt_071_inline_fixed -- \
    --mode scroll --count 100000 --row-style layout --scroll-step 0   # multi-core
RUNS=5 bash tools/pixelcheck.sh /tmp/pt_071_base /tmp/pt_071_inline_fixed -- \
    --mode scroll --count 20000 --row-style layout
```
