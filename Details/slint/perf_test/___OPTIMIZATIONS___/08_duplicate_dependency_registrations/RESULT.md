# 08 - Skip duplicate dependency registrations

**Status: WIN, implemented in `i-slint-core`, measured and validated** (`VALIDATION.md`:
291 + 77 + 818 + 745 tests pass, rendering pixel-identical).
Plan and hypotheses as written before measuring: `PLAN.md`.

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

## The change

`internal/core/properties.rs`, `BindingHolder::register_self_as_dependency`: skip the
registration when this binding is already the **front** of that property's dependency list.

```rust
if unsafe { (*property_that_will_notify).front_is(self_ptr) } {
    return;
}
```

plus `DependencyListHead::front_is()`. Patch: `dedupe_registrations.diff`.

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

## Measured (medians of 3 runs, `rdebug`)

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
both measurements and closes the loop with candidate 07: the dependency bookkeeping really does
cost ~13 ns each, and the remaining ~40 ns per tracked property read is the read itself.

## Bonus: less retained memory

The skipped registrations are not just cheaper, they are also *not kept*: each duplicate used to
leave a live `DependencyNode` in the property's list for as long as the frame's dependencies
lived. In the `scroll` 100 000 scene the resident set drops from **1 542 MB to 1 424 MB** (-118 MB)
with no other change.

## Relation to the arena (#11292 / #12255)

This removes 57% of the allocations the arena would have made cheaper, and it does so without
changing any data structure or per-property/per-binding memory. The two are complementary, but
this one is strictly smaller in scope and lands the same order of win as candidate 07 measured
for the arena's *ceiling*.

## Validation

- Allocation counts drop exactly as predicted by the duplicate measurement (-57.1% / -47.2%).
- `VALIDATION.md`: `i-slint-core` 291 tests, `slint` 77, interpreter driver 818, Rust driver
  745 - all pass. Rendering after 5 frames of moving content is byte-identical to the baseline
  on the deterministic `rects` scene. The screenshot suite could not be built in this
  environment (pre-existing `softbuffer` compile errors).
