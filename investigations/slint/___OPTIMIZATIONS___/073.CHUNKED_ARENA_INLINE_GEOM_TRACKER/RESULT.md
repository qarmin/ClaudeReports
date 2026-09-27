# 73 - A chunked/stable-address arena for `PartialRendererCache::slab`, to let `geom_tracker`/`tracker` live unboxed

IDEAS.md's backlog (added while closing out candidate 072) named this as the only path left to
ship candidate 071's diagnostic-only upside for real: candidate 065 added a per-item
`geom_tracker: Pin<Box<PropertyTracker>>` to let `compute_dirty_regions` skip re-reading geometry
for clean items, but the extra heap indirection made it a net regression on scenes with real
per-item property reads; candidate 071 showed that storing it inline (unboxed) in
`slab::Slab`'s backing `Vec` crashes (`slab::Slab`'s `Vec` can reallocate and move every existing
entry, and `PropertyTracker` is `!Unpin`/self-referential - other properties hold raw pointers
back into it once a dependency is registered) but that an *address-stable* inline version, if one
existed, would recover the win (-5.5%/-16.9% in a diagnostic build using a hardcoded
`with_capacity` to sidestep, not fix, the hazard); candidate 072 confirmed the lower-risk
alternative (merge the two trackers into one *box* instead of two) does not recover it either.
This candidate builds the actual, sound storage backend: a hand-rolled chunked arena
(`chunked_arena::ChunkedArena<T>`) that replaces `PartialRendererCache::slab`'s `slab::Slab`,
giving every occupied entry a permanent, never-relocated address (growth only ever appends a new,
independently heap-allocated fixed-size chunk; removal drops the entry in place via
`ptr::drop_in_place` instead of moving it out) - so `geom_tracker` (and the existing `tracker`)
can be plain, unboxed `PropertyTracker`/`Option<PropertyTracker>` fields with zero allocation
overhead of their own, sidestepping 071's crash and 072's box-size trade-off entirely rather than
working around either.

## What was tried

One implementation: `ChunkedArena<T>` (256-entry chunks, `Vec<Box<[Slot<T>; 256]>>` plus a
free list) providing the same four operations `PartialRendererCache` needs from `slab::Slab`
(`insert`, `get_mut`, a removal that never moves a live entry, `clear`), plus
`PartialRenderingCachedData` gaining `geom_tracker: PropertyTracker` (unboxed, matching
candidate 065's mechanism) and `tracker` changing from `Option<Pin<Box<PropertyTracker>>>` to
`Option<PropertyTracker>` (also unboxed, since it no longer needs its own allocation for the
same address-stability reason). New-item insertion places a not-yet-evaluated placeholder first,
then evaluates the tracker through a `&mut` reference to its now-final, permanent slab address
(the same reordering candidate 071's diagnostic attempt used, but here it's actually sound rather
than papering over the hazard with a fixed capacity - see "Why the result looks the way it does").

| Variant | scenario | single-core (`taskset -c 2`) | multi-core | notes |
|---|---|---|---|---|
| baseline (plain `master`, `cdb6c43f7`) | `scroll`, `layout`, static (`--scroll-step 0`, count 100000) | 152.68ms median | 152.33ms median | |
| **this candidate** | same | **148.29ms median (-2.9%, 9/9 paired)** | **147.31ms median (-3.3%, 8/9 paired)** | consistent win |
| baseline | `scroll`, `layout`, scrolling (count 100000) | 156.03ms median | not measured | |
| **this candidate** | same | **156.19ms median (+0.1%, 3/9 paired)** | not measured | NOT consistent - wash |
| baseline | `scroll`, `flat`, scrolling (count 100000) | 109.14ms median | not measured | |
| **this candidate** | same | **112.69ms median (+3.3%, 3/9 paired)** | not measured | NOT consistent - wash |
| baseline | `scroll`, `bare`, scrolling (count 100000) | 26.83ms median | 27.13ms median | |
| **this candidate** | same | **24.84ms median (-7.4%, 9/9 paired)** | **23.00ms median (-15.2%, 9/9 paired)** | consistent win |

All paired via `tools/paired.sh` (`RUNS=9 FRAMES=20`), same protocol as candidates 065/071/072.
`uptime` load average stayed at/under 2 (24-core machine) for every run.

## Result

A real, reproducible win on the two scenes candidate 065 originally won on (`bare` - almost no
per-item properties to read, so the recompute-skip is nearly free money - and `layout` static,
065's own worst *regression* case): -2.9%/-3.3% (`layout` static, single-/multi-core, 9/9 and
8/9 paired) and -7.4%/-15.2% (`bare` scrolling, single-/multi-core, 9/9 both). This recovers
`layout` static from a +13.3% regression (candidate 065's boxed version) to a real win, i.e. the
concrete outcome IDEAS.md's backlog entry asked for - though the win is smaller than candidate
071's own diagnostic-only figure for the same scene (-5.5%/-5.7%; see "Why" below for the likely
gap). `layout`/`flat` scrolling are washes (3/9 both directions) rather than the small wins
candidate 071's diagnostic saw there (-2.9%/-1.1%) or the small regressions candidate 065's boxed
version saw there (+10.9%/+5.6%) - a clear improvement over 065, but not a decisive win the way
`layout` static and `bare` are.

## Does it change behavior/output?

- **Bit-exact outside the noise floor.** `tools/pixelcheck.sh` (`RUNS=6`), baseline vs. patched,
  0 px difference outside the measured same-binary noise floor on 9 scene/mutation combinations:
  `scroll` x (`layout`/`flat`/`bare`/`clipped`), `layout --scroll-step 0`,
  `layout --mutate 50 --mutate-mode push`, `layout --mutate 50 --mutate-mode insert-front`
  (exercises the arena's slot-reuse/free-list path directly), `list --count 5000`,
  `rects --count 3000` (0 px noise floor and 0 px raw difference there).
- `cargo test -p i-slint-core --lib`: 307/307 passed, unchanged.
- `cargo test -p slint` (default features): unchanged, including `tests/partial_renderer.rs`'s
  own dedicated 24-test suite for this exact module (candidate 065/071/072 didn't report running
  this file specifically) - all 24 passed with the patch applied.
- **Is this a breaking change?** No public API changes - `chunked_arena` and every field it
  affects are private to `partial_renderer.rs`. No new dependency was added (the arena is
  hand-rolled, not a crate); `slab` remains a dependency of `i-slint-core` for `timers.rs`, which
  is unaffected by this change.

## Why the result looks the way it does

**Soundness, worked through explicitly since this is the whole point of the candidate.**
`PropertyTracker` is self-referential via `BindingHolder::register_self_as_dependency`: once
`register_as_dependency_to_current_binding`/`evaluate_if_dirty` runs, the *property being read*
stores a raw pointer directly back into this tracker's own memory (`DependencyNode::binding: *const
BindingHolder`, pushed into that property's `DependencyListHead`). Moving a live one anywhere
leaves that pointer dangling. Three things had to hold for the inline fields to be sound, and the
design was checked against each one field-by-field rather than assumed:

1. **No occupied slot ever moves due to arena growth.** `ChunkedArena` only ever appends a new,
   independently heap-allocated `Box<[Slot<T>; 256]>` chunk; existing chunks (and their
   contents) are never touched again. Growing the *outer* `Vec<Box<[Slot<T>; 256]>>` can
   reallocate and move `Box` pointers around, but never the boxed memory they point to - unlike
   `slab::Slab`'s single, contiguous, reallocating `Vec<T>` (candidate 071's crash cause).
2. **`insert()` only ever moves a value before it has any dependency registered against it.**
   Both new-item paths (`geom_tracker` and `tracker`'s `get_or_insert_with`) construct a fresh
   `PropertyTracker::default()` (a plain, not-yet-pinned value with an empty dependency list) and
   only evaluate/register it *after* it has already been moved to its final address - a
   deliberate reordering (matching candidate 071's own diagnostic-attempt fix, but here needed
   for real rather than as a workaround) rather than the natural "compute geometry, then
   construct-and-insert the whole entry" order the original (boxed) code used.
3. **Removal (`free()`) never moves a live value out either.** Unlike `slab::Slab::remove`
   (hands the removed value back by value - fine when the payload is just a pointer, unsound if
   the payload itself is self-referential), `ChunkedArena::free` runs `ptr::drop_in_place` on the
   occupied value *at its permanent address*, then overwrites the (now logically dead) bytes with
   a `Vacant` marker. `PartialRendererCache::remove()` (called from `CachedRenderingData::release`)
   only needs the `data` field back afterward, which is a plain, non-self-referential, cheaply
   `Clone`-able type - so it's cloned out *before* calling `free()`, and the trackers are dropped
   in place along with the rest of the entry, never relocated.

Point 3 in particular is a materially different design decision than 071/072 needed to make (they
kept `tracker`/`geom_tracker` boxed, so this concern never arose for them - only the box
*pointer* moved on removal, never the tracker itself); getting it right is what makes this
candidate's storage backend actually sound rather than another "still relies on a hazard,
just harder to trigger" attempt.

**Why the measured win (-2.9%/-3.3%) is smaller than candidate 071's diagnostic figure
(-5.5%/-5.7%) on the same scene.** Two plausible, non-exclusive contributors, neither isolated
further under this candidate's budget: (a) `ChunkedArena::get_mut`/`slot_mut` does a
division/modulo to locate an index's chunk and offset, plus one extra pointer indirection through
the chunk's `Box`, versus `slab::Slab`'s single direct index into one flat `Vec` - a real, if
small, per-access cost paid on every cache hit, that 071's `with_capacity`-reserved plain
`Vec<PartialRenderingCachedData>` never paid; (b) `pin_ref`'s `Pin::new_unchecked` and the
now-generic `Option<PropertyTracker>` accesses may inline less predictably than 071's version,
which read a single hardcoded field through one fewer layer of abstraction. This is the expected,
reasonable cost of doing it soundly rather than with a hardcoded-capacity shortcut - the
`layout`/`flat` scrolling washes (vs. 071's small wins there) point the same direction.

**Why `layout`/`flat` scrolling land as washes rather than wins.** These scenes constantly bring
new rows into view and push others out as the list scrolls, both of which force actual
recomputation (`sibling_index_changed`/`moved`/`geometry_changed`) regardless of the tracker -
the skip-recompute fast path this candidate is built to make cheap applies to items that stay
put, and scrolling item-heavy scenes deliberately don't have many of those. `bare` (few enough
properties per item that the whole point is "read almost nothing either way") and `layout` static
(nothing moves, so nearly every item takes the skip path every frame) are exactly the two
scenarios where the mechanism has the most to gain, and they're where the wins land.

## What's not done / open questions

- No `perf`/allocation-profiler attribution beyond the bench's own allocation counter was
  attempted (`perf_event_open` remains blocked in this environment) - confirmed via
  `--alloc-profile`-adjacent counters that `--count 20000 --row-style layout --scroll-step 0`
  drops from 140510 to 80390 allocations/frame in steady state (-42.8%), consistent with removing
  the recomputation this mechanism is meant to skip, but the single-access-cost explanation above
  (division/modulo vs. flat index) is reasoned from the measured numbers, not a confirmed trace.
- `CHUNK_SIZE = 256` was chosen as a reasonable default (a few hundred bytes on the stack while
  building a chunk via `core::array::from_fn`, a few hundred chunks for a 100k-row scene) and not
  swept - a different value might change the single-access-cost gap noted above, untested.
- Only the software renderer (`--bench`) was measured, same limitation as candidates 049-051 and
  065/071/072 (no GPU display available this session).
- `layout`/`flat` scrolling were only measured single-core (matching 072's own scope for its
  secondary scenes); `clipped`/`touch` row styles were pixel-checked but not timed.
- This candidate does not attempt the "worth trying" idea noted while investigating 071/072
  (adding a debug-only tally of how often the geom-tracker skip path actually fires per scene) -
  would help explain the `layout`/`flat` scrolling wash more precisely than the reasoning above.
