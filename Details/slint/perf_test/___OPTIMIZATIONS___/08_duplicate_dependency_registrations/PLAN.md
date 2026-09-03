# 08 - Are the per-frame dependency registrations redundant? (plan, before measuring)

## Why

Candidate 07 showed the allocator is not the main cost of the ~700 000 dependency
registrations per frame in the non-virtualized `ScrollView` scene; the registration itself is
(thread-local lookup + two linked-list insertions + cache misses). That makes the *number* of
registrations the lever.

`PropertyHandle::register_as_dependency_to_current_binding` ->
`BindingHolder::register_self_as_dependency` (`internal/core/properties.rs:461-473`) allocates a
`DependencyNode` and pushes it **on every read**, with no check for "this binding already
depends on this property". So if a render pass reads the same property twice, two nodes are
allocated and two list insertions happen, for no additional information.

## Hypothesis

A single rendering pass reads some geometry properties more than once per item (for example
`item_geometry()` during culling and again while drawing, or `x`/`y` read by both the parent
layout and the renderer), so a meaningful fraction of the registrations are duplicates that
could be skipped.

## Plan

1. Instrument `i-slint-core` behind a new `dep-stats` feature: count registrations and, using a
   `HashSet<(binding, property)>`, how many are duplicates within one evaluation of that
   binding. `pub fn reset()` / `pub fn report()` so the benchmark can bracket a frame.
2. Report per frame for `scroll` 100k, `rects` 3000, `list` 100k: total registrations, distinct
   (binding, property) pairs, duplicate share.
3. If duplicates are a meaningful share, look at what a dedupe would cost: the cheapest correct
   check is on the binding side (the property's list head is not indexed by binding), e.g. a
   small ring of recently registered properties per `BindingHolder`, or a generation stamp on
   the property. Prototype only if the measurement justifies it.

## Expected

If duplicates are <5%, this is a null result and the remaining lever is fewer *reads* (i.e.
subtree culling from candidate 01, or virtualization). If they are 30-50%, a dedupe is worth
prototyping: it would cut both allocations and cache misses proportionally.
