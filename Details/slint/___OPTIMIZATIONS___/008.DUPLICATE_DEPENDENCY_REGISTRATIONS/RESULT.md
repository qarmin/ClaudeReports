# 08 - Skip duplicate dependency registrations

Ported from `perf_test/___OPTIMIZATIONS___/08_duplicate_dependency_registrations/RESULT.md` on 2026-08-23 (original perf_test-relative session; source and any extra notes remain there untouched).

Implemented in `i-slint-core`, measured and validated (`VALIDATION.md`: 291 + 77 + 818 + 745
tests pass, rendering pixel-identical).
Plan and hypotheses as written before measuring: `PLAN.md`. Re-checked after candidate
`014.DEPENDENCY_NODE_RECYCLING` landed: `RECHECK_AFTER_14.md`.

## The measurement that started it

Instrumented `i-slint-core` (feature `dep-stats`, still in the tree) to count registrations and
how many repeat a `(binding, property)` pair within one evaluation:

| scene | registrations/frame | distinct pairs | duplicates |
| --- | --- | --- | --- |
| `scroll` 100 000 | 700 914 | 300 568 | **400 346 (57.1%)** |
| `list` 100 000 | 1 632 | 835 | **797 (48.8%)** |
| `text` 100 000 | 427 | 204 | 223 (52.2%) |
| `rects` 3 000 | 18 487 | 18 400 | 87 (0.5%) |

More than half of the dependency bookkeeping in scenes built from repeated components is
redundant. The reason is visible in the generated code: the renderer walks the children of a
layout and every child's `item_geometry` reads the *same* `..._layout_cache` (and the same
`height`) property, once per child - four children, four identical registrations.

`rects` has no duplicates because those rectangles have no shared layout cache: each one reads
its own x/y/width/height once.

## What was tried

`internal/core/properties.rs`, `BindingHolder::register_self_as_dependency`: skip the
registration when this binding is already the **front** of that property's dependency list.

```rust
if unsafe { (*property_that_will_notify).front_is(self_ptr) } {
    return;
}
```

plus `DependencyListHead::front_is()`. Patch: `patch.diff`.

Why this is safe:

- `DependencyListHead::append` inserts at the **front**, so "front carries this binding" means
  this binding's most recent registration for this property.
- A binding's nodes are owned by its `dep_nodes` list and unlinked when they are dropped, which
  happens when the binding is re-evaluated. So a node found at the front is live, and the
  dependency it represents is in effect: skipping the second registration cannot lose an
  invalidation.
- Even if a node survived from an earlier evaluation, skipping is conservative: the dependency
  exists, so the binding still gets notified.
- It reads the real list instead of remembering a property address, so a destroyed and
  reallocated property cannot be mistaken for an already-registered one - the hazard that a
  "last registered property" cache in the `BindingHolder` would have had.
- The check touches the old head node, which `append` was about to write to anyway, so it is
  practically free.

It skips **all** duplicates, not just consecutive ones (400 345 of 400 346 in the `scroll`
scene), because between two reads of the same property the other registrations go to *other*
properties, leaving our node at the front.

## Result

Measured (medians of 3 runs, `rdebug`).

FemtoVG (Xvfb/llvmpipe):

| scene | baseline | with dedupe | time | allocations/frame |
| --- | --- | --- | --- | --- |
| `scroll` 100 000 | 57.53 ms | **53.03 ms** | **-7.8%** | 703 200 -> **302 853 (-57%)** |
| `rects` 3 000 | 45.40 ms | 45.31 ms | - | 51 423 -> 50 231 |
| `list` 100 000 | 5.01 ms | 5.27 ms | +5% (noise) | 3 976 -> 3 410 (-14%) |

Software renderer (`--bench`, deterministic):

| scene | baseline | with dedupe | time | allocations/frame |
| --- | --- | --- | --- | --- |
| `scroll` 100 000 | 148.17 ms | **142.94 ms** | **-3.5%** | 701 063 -> **300 743 (-57%)** |
| `rects` 3 000 | 2.17 ms | 2.22 ms | - | 18 487 -> 18 414 |
| `list` 100 000 | 2.19 ms | 2.25 ms | - | 2 131 -> 1 394 (-35%) |

Removing 400 000 allocations per frame bought 5.2 ms, i.e. ~13 ns per removed
allocation+registration - exactly the churn figure from `tools/allocbench.rs`, which confirms
both measurements and closes the loop with candidate `007.ALLOCATOR_CEILING`: the dependency
bookkeeping really does cost ~13 ns each, and the remaining ~40 ns per tracked property read is
the read itself.

### Bonus: less retained memory

The skipped registrations are not just cheaper, they are also *not kept*: each duplicate used to
leave a live `DependencyNode` in the property's list for as long as the frame's dependencies
lived. In the `scroll` 100 000 scene the resident set drops from **1 542 MB to 1 424 MB** (-118 MB)
with no other change.

### Re-checked after candidate 014 (node pooling) landed

Candidate `014.DEPENDENCY_NODE_RECYCLING` makes a registration cheap by reusing a pooled node
instead of allocating one - they are in the same file and interact by construction. Re-measured
with `tools/isolate.py`, which turns exactly one of them off (both binaries built in the same
session, each checked to be genuinely different before its row was read):

Allocations per frame:

| | `rects` 3 000 | `scroll` 100 000 |
| --- | --- | --- |
| both applied | 14 | 182 |
| 08 off, 14 on | 14 | 357 |
| 08 on, 14 off | 26 637 | 300 837 |
| both off | 26 727 | 701 187 |

08 alone (bottom two rows) is still the -57% on `scroll` it was originally credited with. With 14
applied, the same comparison is 182 against 357 - still half, but of a number that no longer
matters: the duplicates now come from the pool rather than the allocator.

Frame time, paired and interleaved (unpaired medians disagreed with each other, so both were
re-run as interleaved A/B pairs to cancel machine drift):

| | 08 on | 08 off | 08 on won |
| --- | --- | --- | --- |
| `rects` 3 000, 12 pairs | 2.57 ms | 2.54 ms | **3 of 12** |
| `scroll` 100 000, 6 pairs | **116.87 ms** | 132.39 ms | **6 of 6** |

So, with 14 applied: on `scroll`, 08 is still worth **-12%**, unambiguously (every pair). On
`rects` it is now a **~1% loss** (0.03 ms, losing 9 of 12 pairs) - the `front_is` check costs a
dereference per registration and the allocation it avoids has become nearly free. Kept: -12% on
the scene that needs it against ~1% on the scene that does not is a clear trade, but its headline
number is no longer -57% allocations - that figure belongs to a tree without 14.

## Does it change behavior/output?

- **Bit-exact?** Yes. Allocation counts drop exactly as predicted by the duplicate measurement
  (-57.1% / -47.2%). Pixel comparison, `rects` scene moving content for 5 frames:
  byte-identical PPM before vs after. For the `scroll` scene (contains text, not bit-deterministic):
  baseline vs patched differ in 159 bytes of 3 302 400, while two runs of the *same baseline*
  binary differ in 291 bytes - the patch is well inside the scene's own noise.
- **Is this a breaking change?** No - purely an internal dependency-tracking optimization, no
  public API or documented behavior changes.
- Project test suite (`VALIDATION.md`): `i-slint-core` 291 tests (+69 doc tests, +1 compile-fail),
  `slint` 77, interpreter driver 818, Rust driver 745 - all pass, 0 failures. The screenshot
  comparison suite (`test-driver-screenshots`) **could not be run**: pre-existing `softbuffer`
  compile errors in that environment, unrelated to this change.

## Why the result looks the way it does

Generated code for a repeated layout (e.g. a row's `HorizontalLayout`) has every child's
`item_geometry` read the *same* shared `..._layout_cache` and `height` properties, once per
child - so N children produce N identical dependency registrations where one would do. Since
`DependencyListHead::append` inserts at the front, checking "is this binding already at the
front" is a cheap, correct way to detect and skip the redundant ones without changing the
dependency semantics.

## What's not done / open questions

This removes 57% of the allocations a `DependencyNode` arena (upstream #11292 / #12255) would
have made cheaper, and it does so without changing any data structure or per-property/per-binding
memory. The two are complementary, but this one is strictly smaller in scope. The
`isolate.py`/`dep-stats` re-check tooling itself is a useful lesson: an earlier fix's headline
number can be silently consumed by a later one landing in the same file, and only re-measuring
the earlier fix in isolation after the later one lands catches it.

## Reproduce

```sh
just run       # the duplicate-registration measurement (dep-stats build; timings meaningless there)
just run-ab    # before/after timing of the change itself (needs git to stash the core patch)
```

Fully reproducing the bench additionally needs `perf_test/` copied in manually, since it is
untracked in the main repo's git history.

## Reproducibility check after upstream update (2026-08-23)

`patch.diff` still applies cleanly against slint HEAD `cdb6c43f7` (no upstream commits touched `internal/core/properties.rs` since this patch was captured). Not rebuilt/re-benched this round - re-verified via `git apply --check` only.
