# 22 - Prepending to a `VecModel` is O(model), and it dominates at real update rates

Ported from `perf_test/___OPTIMIZATIONS___/22_vecmodel_prepend/RESULT.md` on 2026-08-23 (original perf_test-relative session; source and any extra notes remain there untouched).

Candidate 013 measured model churn at 20 rows/frame and found `insert(0, ..)` only ~0.75 ms worse
than `push`. At the rates a feed, chat or log view actually updates, that turns out to have been
hiding the shape of the cost.

## Measured

`--mode list --mutate 200`, 200 rows/frame, medians of 3 runs:

| model size | `push` (append) | `insert(0, ..)` (prepend) | cost of prepending |
| --- | --- | --- | --- |
| 10 000 | 2.98 ms | 3.62 ms | +0.64 ms |
| 50 000 | 2.30 ms | 5.05 ms | +2.75 ms |
| 100 000 | 2.44 ms | 7.00 ms | +4.56 ms |
| 200 000 | 2.34 ms | **11.07 ms** | **+8.73 ms** |

Appending is **flat** in model size. Prepending is **linear**: double the model, double the cost.
That is `Vec::insert(0, ..)` - every existing element moves - and at 200 000 rows a batch of 200
prepends moves 40 million elements per frame.

The shaping is *not* the difference: both report 67 paragraph shapes per frame, capped at the
visible rows, which is correct. The whole gap is memory movement.

## Why it was invisible before

At 20 rows/frame (candidate 013) prepending cost +0.75 ms against append's +0.0 - easy to read as
"prepending is a bit worse". The cost is `rows_inserted x model_size`, so it only separates from
the noise when *both* are large. A benchmark that varies one and fixes the other at a small value
sees nothing.

## What to do

- **Append and display reversed** where the semantics allow it - `push` is flat at any model size.
- For genuine prepend-heavy data, a `VecModel` is the wrong container; a deque-backed `Model`
  implementation makes it O(1) amortized. Nothing in Slint provides one today, but `Model` is a
  trait and implementing it over `VecDeque` is a small amount of code.
- Batch: 200 prepends done as one `insert` of a slice would move the tail once instead of 200
  times. `VecModel` has no batch insert, which is the thing worth adding upstream.

## Reproduce

```sh
for c in 10000 50000 100000 200000; do
  tools/matrix.sh \
    "push $c|--mode list --count $c --mutate 200 --mutate-mode push" \
    "prepend $c|--mode list --count $c --mutate 200 --mutate-mode insert-front"
done
```

## Does it change behavior/output?

N/A - measured/quantified only, no patch applied to real source in the original session.

## Why the result looks the way it does

`Vec::insert(0, ..)` shifts every existing element down by one slot, so a batch of N prepends into
a model of size M costs O(N x M) element moves, while `push` is amortized O(1) regardless of M.
The behavior is exactly what `Vec`'s documented complexity predicts - this is a container-choice
issue, not a bug.

## What's not done / open questions

- No deque-backed `Model` implementation exists in Slint; this would need to be written and
  proposed upstream if genuine prepend-heavy use cases are common enough to justify it.
- No batch-insert API was added to `VecModel` - only identified as the fix that would turn N moves
  into 1.
