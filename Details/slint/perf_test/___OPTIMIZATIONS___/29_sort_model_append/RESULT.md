# 29 - `SortModel` turns an O(1) append into O(model), half of it for nothing

**Status: partially fixed, measured, tested; the obvious follow-up was tried and rejected on
measurement.** Patch: `sort_model_skip_index_scan.diff`.

Real apps put a filter/sort/map adapter between the model and the view. No scene had one, so
`--model-adapter none|filter|sort|map` was added.

## Static cost: nothing

`list` 100 000, no mutation: 2.22 ms bare, 2.21 map, 2.25 filter, 2.23 sort. An adapter that is
only read costs nothing measurable.

## Mutating through one: `SortModel` is the expensive case

200 rows/frame into 100 000, medians of 5 runs:

| adapter | `set_row_data` | `push` | `insert(0,..)` |
| --- | --- | --- | --- |
| none | 3.55 ms | 2.31 ms | 7.06 ms |
| map | 2.90 ms | 2.37 ms | 7.10 ms |
| filter | 3.25 ms | 2.38 ms | 10.08 ms |
| **sort** | 5.66 ms | **5.88 ms** | 11.32 ms |

`push` is the striking one: appending is *flat in model size* on a bare `VecModel` (candidate 22),
and a `SortModel` makes it cost **2.5x** - and scale:

| model | bare `push` | sorted `push` | adapter cost |
| --- | --- | --- | --- |
| 25 000 | 2.88 ms | 3.22 ms | +0.34 ms |
| 50 000 | 2.31 ms | 4.05 ms | +1.74 ms |
| 100 000 | 2.32 ms | 5.76 ms | +3.44 ms |
| 200 000 | 2.34 ms | 9.16 ms | **+6.82 ms** |

Doubling the model doubles the cost. Appending to a sorted list - a sorted log, a leaderboard, a
ranked feed - is O(model) per appended row.

## Cause: two O(n) passes, one of which cannot do anything

`SortModelInner::row_added` (`internal/core/model/adapters.rs`):

```rust
// Adjust the existing sorted row indices to match the updated source model
for row in self.mapping.borrow_mut().iter_mut() {
    if *row >= index { *row += count; }
}
```

then, per added row, a `partition_point` over the mapping (each comparison clones a row through
`wrapped_model.row_data`) and a `mapping.insert(..)`, which is an O(n) memmove.

The scan above runs **once per `row_added` call**. When rows are *appended*, `index` is the old row
count, so no existing mapping entry can be `>= index` and the scan cannot change anything - it just
walks the whole mapping to find that out. 200 appends into a 200 000-row model is 200 full scans
finding nothing.

## The fix, and what it does not fix

Skipping the scan when the rows were appended:

```rust
let appended_at_end = index >= self.wrapped_model.row_count().saturating_sub(count);
if !appended_at_end { ...adjust... }
```

| model, sorted `push` 200/frame | before | after |
| --- | --- | --- |
| 100 000 | 5.76 ms | **3.17 ms (-45%)** |
| 200 000 | 9.16 ms | **3.78 ms (-59%)** |

Nearly flat in model size where it previously doubled.

**The remaining cost is the `mapping.insert`**, ~+0.91 ms at 100 000 rows and +1.54 ms at 200 000 -
still scaling, but a sixth of what it was.

### The obvious next step was tried and is worse

`row_added` receives `count`, so the natural follow-up is to place all `count` rows and shift the
tail once instead of `count` times. Implemented and measured:

| sorted `push` 200/frame | one-at-a-time | "batched" merge |
| --- | --- | --- |
| 50 000 rows | 4.05 ms | 17.22 ms |
| 100 000 rows | 5.76 ms | 33.69 ms |
| 200 000 rows | 9.16 ms | **65.70 ms** |

**Ten times worse**, because `count` is almost always 1: `VecModel::push` emits
`row_added(index, 1)` per row, so the "batch" allocates a fresh mapping and copies all *n* entries
to insert a single row - strictly more work than a `Vec::insert` that moves only the tail.

Reverted. The lesson is where the batching has to live: **the source model has to notify in
batches** (a `push_slice`/`extend` on `VecModel` that emits one `row_added(index, count)`) before
any adapter can take advantage of one. Batching inside the adapter optimizes a `count` it never
receives. A comment in `row_added` now records this so the next person does not re-derive it.

## Validation

- **2 028 tests, 0 failures, 0 compile errors** (`i-slint-core --all-features`, `slint`, both
  build-time drivers).
- A new test, `test_sort_model_row_added_index_adjustment`, pins the path the optimization could
  break: inserting at index 0, 3, 7 and 10 (where the scan *is* needed) still produces the correct
  sorted order, checked against `ModelChecker`, plus the append path it skips.

## Reproduce

```sh
for c in 50000 100000 200000; do
  tools/matrix.sh \
    "bare $c|--mode list --count $c --mutate 200 --mutate-mode push" \
    "sorted $c|--mode list --count $c --model-adapter sort --mutate 200 --mutate-mode push"
done
```
