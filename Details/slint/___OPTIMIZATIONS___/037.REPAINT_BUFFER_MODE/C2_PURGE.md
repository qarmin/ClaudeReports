# 37 / C2 - the purge the default mode does every frame is a no-op

`prepare_scene` in `NewBuffer` mode ends with:

```rust
// NewBuffer always redraws the full screen, so skip dirty region
// tracking to avoid unbounded growth of the partial rendering cache.
renderer.dirty_region = LogicalRect::from_size(logical_size).into();
software_renderer.partial_rendering_state.clear_cache();
```

Plan item C2 was: bound that cache instead of purging it, the way candidate 026 bounded the text
cache. Measured first, and the answer is that there is nothing to bound.

## Removing the purge changes nothing

| scene, `--repaint-buffer new` | with purge | without | |
| --- | --- | --- | --- |
| `scroll` 20 000 | 11.77 ms | 12.59 ms | +7.0%, but **2 of 5 pairs** - not consistent |
| `scroll` 100 000 `bare` | 11.46 ms | 11.27 ms | -1.7%, 2/5 - noise |
| `rects` 3 000 | 2.11 ms | 2.12 ms | +0.5%, 0/5 |

## And the growth it guards against does not happen

| frames rendered | with purge | without |
| --- | --- | --- |
| 10 | 111.6 MB | 111.9 MB |
| 60 | 111.7 MB | 111.7 MB |
| 200 | 111.6 MB | 111.9 MB |

Flat either way, over twenty times as many frames.

## Why: the cache is empty

Instrumented `clear_cache` to print the slab size at the moment it purges:

```
new      PARTIAL_CACHE_AT_PURGE entries=0
```

**Zero, every time.** In `NewBuffer` mode `compute_dirty_regions` never runs, and that is the only
thing that *creates* entries - `filter_item` and `do_rendering` both only read
(`rendering_data.get_entry(..)`). So the cache is never populated, the purge clears an empty slab,
and the comment describes an unbounded growth that cannot occur.

## It also explains candidate 037's other result mechanically

Candidate 037 measured fix 35 (serve a culled item's origin from the cache) at **-8.8% in
`ReusedBuffer` and only -1.7% in `NewBuffer`**, and attributed it to the per-frame purge. The real
reason is one step earlier: in that mode the cache is never filled, so `cached_offset` is `None` for
every item and the fast path is never taken. Same conclusion, firmer footing.

Two consequences worth recording:

- In `NewBuffer`, `filter_item` takes its "no cache entry" branch for **every item, every frame**,
  recomputing `item_rc.bounding_rect(..)` from scratch. That mode is still faster overall because it
  skips the whole dirty-region walk - but it is paying for a cache it never fills.
- The comment is stale and mildly misleading: it justifies a call by a risk that the code structure
  already rules out.

## Reproduce

```sh
tools/paired.sh <with purge> <without> -- --mode scroll --count 20000 --repaint-buffer new
for f in 10 60 200; do perf_test --bench --mode scroll --count 20000 --repaint-buffer new --frames $f; done
```
