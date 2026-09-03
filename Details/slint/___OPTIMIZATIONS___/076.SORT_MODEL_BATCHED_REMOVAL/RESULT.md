# 76 - `SortModel`'s batched `row_removed` is O(n x count) instead of O(n)

`SortModelInner::row_removed` (`internal/core/model/adapters.rs:746-783`) scans `mapping` once,
but for every row that needs removing it calls `mapping.remove(i)` - an O(n) `Vec` memmove -
inside that same scan. A single batched `row_removed(index, count)` notification (from a wrapped
model whose backing store supports a bulk delete - a DB view, a batched IPC update, `Vec::drain`)
therefore costs O(n x count) instead of O(n). `FilterModel`'s equivalent path in the same file
already does one bulk `mapping.drain(range)` for its (contiguous, since its mapping is sorted by
source index) case; `SortModel`'s mapping is sorted by *value*, so the removed positions are
scattered and a single bulk `drain` isn't directly applicable, but the O(n) *scan and compaction*
still is - notifications still have to go out per removed row (see "Why the result looks the way
it does" below), only the internal bookkeeping was O(n x count).

## What was tried

No existing project bench reaches this notification shape (`VecModel` has no public API that
emits `row_removed(index, count > 1)` - it only ever removes one row at a time), so a standalone
bench (`bench/`) drives `SortModel` directly: a minimal wrapped model with its own `ModelNotify`
and a `bulk_remove(index, count)` that does one `Vec::drain` + one `notify.row_removed(index,
count)` call, standing in for any real bulk-delete-capable source. 200 000 scrambled `u64` rows,
`SortModel::new_ascending`, mapping pre-built (untimed) before each timed call; only the
`bulk_remove` call itself is timed, isolating it from model/SortModel setup. Single-threaded, pure
CPU-bound `Rc`/`RefCell` code with no thread pool involved, so "multi-core" here means "unpinned",
not "uses multiple cores" - included per protocol, not because parallelism is expected to matter.

Raw pinned runs (`taskset -c 3`, `uptime` load < 4 throughout, count values 1/50/200/1000/5000):

- Baseline run 1: 0.223 / 0.597 / 1.879 / 7.173 / 34.633 ms (p50)
- Baseline run 2 (repeat): 0.223 / 0.591 / 1.846 / 7.146 / 34.416 ms (p50)
- Patched run 1: 0.223 / 0.203 / 0.206 / 0.218 / 0.222 ms (p50)
- Patched run 2 (repeat): 0.202 / 0.203 / 0.206 / 0.218 / 0.219 ms (p50)

Unpinned runs (2x each):

- Baseline: [0.182, 0.483, 1.507, 7.061, 34.065] and [0.186, 0.490, 1.519, 7.115, 34.392] ms (p50)
- Patched: [0.166, 0.164, 0.167, 0.178, 0.217] and [0.163, 0.166, 0.167, 0.176, 0.216] ms (p50)

## Result

At fixed n=200 000, baseline cost scales with `count` (0.22 ms at count=1 up to ~34.5 ms at
count=5000, both pinned and unpinned, reproducible across repeat runs). Patched cost is flat
across the same `count` range (~0.16-0.22 ms), confirming the O(n) vs. O(n x count) difference:

| count | baseline p50 | patched p50 | speedup |
|---|---|---|---|
| 1 | 0.223 ms | 0.202-0.223 ms | ~1x (both O(n) at count=1) |
| 50 | 0.591-0.597 ms | 0.203 ms | ~2.9-3.7x |
| 200 | 1.846-1.879 ms | 0.206 ms | ~9-9.7x |
| 1000 | 7.146-7.173 ms | 0.218 ms | ~33-40x |
| 5000 | 34.416-34.633 ms | 0.219-0.222 ms | ~157x |

The win only exists when a wrapped model actually emits `count > 1` in one notification. No
project code currently does that for `SortModel`'s only public source-model type, `VecModel` - see
"What's not done" below.

## Does it change behavior/output?

- **Bit-exact?** Yes. The rewrite is a single-pass compaction (`write`/`read` indices into the
  same `Vec`, replacing `Vec::remove` calls) that reproduces the exact same per-row
  `notify.row_removed(position, 1)` sequence as before - `position` is derived the same way
  (`write`, the count of retained entries seen so far, is exactly the live index the old
  `Vec::remove`-based loop would have had at that point). Verified by the existing
  `test_sorted_model_remove` unit test (unchanged, still passes) plus the crate's full test suite.
- Not a breaking change: no public API, file format, or documented behavior changes; the change is
  internal to `SortModelInner::row_removed`.
- Project test suite: `cargo test --release -p i-slint-core --features std` (model adapter tests):
  16/16 passed, both before and after. `cargo test --release -p i-slint-core --features
  std,ffi,rtti,testing,unicode,image-decoders,svg` (full crate + doctests): 315 lib tests + 71 doc
  tests + 1 compile-fail doctest, all passing, both before and after. No new failures.

## Why the result looks the way it does

The old loop had to call `Vec::remove` immediately so its own `i` index tracking would stay
correct for the next iteration (removing shifts everything after `i` left by one, and the loop
re-reads `mapping.borrow()[i]` on the next pass). Rewriting it as a `read`/`write` compaction pass
(the same shape as `Vec::retain`, but manual because the notified position for each removed row
still has to be recorded) removes the need to mutate the `Vec`'s length until the very end
(`mapping.truncate(write)`), turning the mutation cost from "one memmove per removed row" into
"one truncate at the end" - the scan itself was already O(n) either way.

## What's not done / open questions

- Real callers currently can't reach `count > 1` here: `VecModel::remove` only ever calls
  `notify.row_removed(index, 1)`. Neither `VecModel` nor any other `Model` implementor in this
  crate exposes a public bulk-remove API. This candidate optimizes a path that's real (the API
  contract of `ModelChangeListener::row_removed` explicitly takes a `count`, and any custom
  `Model` impl - or a future `VecModel::remove_range` - can already call it) but not yet exercised
  by any shipped Slint API. Same shape as candidate 029's finding for `row_added`: the fix belongs
  in the adapter (done here), but a source model that actually batches (`VecModel::remove_range`
  or similar) would be needed for real apps to see it end-to-end.
- The remaining cost is still nominally O(n) per batched removal (a full mapping scan), which is
  as good as it gets without an auxiliary reverse index (source-index -> mapping-position) - out of
  scope here, and would add bookkeeping cost to every other mutation path in exchange.
