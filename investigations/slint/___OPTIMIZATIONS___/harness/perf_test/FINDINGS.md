# ListView vs ScrollView with many rows - measured

All numbers are from this project's own benchmark modes (1280x860, `rdebug` profile, this
machine), reproducible with the commands next to each table. "later frames" is the average
of the frames after the first one while scrolling 200 px per frame, so it is the steady
state cost of scrolling, not the one-off setup.

Both renderers were measured: `--bench` (headless software renderer, no display needed)
and `--frames` (real window, FemtoVG) under `xvfb-run` with Mesa llvmpipe. FemtoVG needs
`libxkbcommon-x11-0` and `libxcb-xkb1` installed, plus `xvfb` and `libgl1-mesa-dri`:

```sh
just frames-xvfb 100000 8            # FemtoVG in a virtual X server
```

llvmpipe rasterizes on the CPU, so FemtoVG's *drawing* is slower here than on a GPU; the
item-tree traversal, layout and shaping costs are the same either way, and the allocation
counts are identical to the millisecond-independent numbers below.

## 100 000 records, FemtoVG (window under Xvfb/llvmpipe)

| variant | first frame | later frames | allocations/frame |
| --- | --- | --- | --- |
| `ListView`, row with a layout | 109 ms | **5.3 ms** (187 fps) | 3 937 |
| `ScrollView` + `for`, row with a layout | **2.73 s** | **55.6 ms** (18 fps) | 703 101 |
| `ScrollView` + `for`, flat row (no layout inside) | 171 ms | 46.4 ms (22 fps) | 503 396 |
| `ScrollView` + `for`, flat row + constant child geometry | 111 ms | 36.2 ms (28 fps) | 303 396 |
| text mode, 100 000 lines | 146 ms | 4.8 ms | 1 978 |

## 100 000 records, software renderer (headless `--bench`)

Same shape, ~2.7x higher per-item cost because the software renderer always builds a
`PartialRenderer` (`internal/renderers/software/lib.rs:615`) whose per-item cache FemtoVG
does not have. The allocation counts per row are identical on both renderers.

| variant | first frame | later frames | allocations/frame | RSS |
| --- | --- | --- | --- | --- |
| `ListView`, row with a layout | 3.4 ms | **2.8 ms** | 2 131 | **36 MB** |
| `ScrollView` + `for`, row with a layout | **3.10 s** | **148 ms** | 701 063 (7/row) | **1542 MB** |
| `ScrollView` + `for`, flat row (no layout inside) | 131 ms | 120 ms | 501 330 (5/row) | 318 MB |
| `ScrollView` + `for`, flat row + constant child geometry | 96 ms | 90 ms | 301 355 (3/row) | - |
| `ScrollView` + `for`, row with a layout + `clip: true` | 103 ms | 97 ms | 801 665 (8/row) | - |
| `ScrollView` + `for`, row with no children at all | 35 ms | 29 ms | 300 459 (3/row) | - |

```sh
../target/rdebug/perf_test --bench --mode list   --count 100000 --frames 6
../target/rdebug/perf_test --bench --mode scroll --count 100000 --frames 6
../target/rdebug/perf_test --bench --mode scroll --count 100000 --frames 6 --flat-rows
```

For reference, 10 000 records: `ListView` 3.0 ms/frame, `ScrollView` 11.9 ms/frame
(first frame 306 ms), `ScrollView --flat-rows` 8.7 ms/frame (first frame 16 ms).
The text mode is not affected at all: 100 000 lines of ~110 characters render in
3.6 ms/frame because `TextInput` only shapes what is visible (RSS 541 MB for the
10 MB string, which is its own story).

## What actually costs what

### 1. The 3-second first frame is text shaping of off-screen rows - and 1.2 GB of cache

`render_item_children` (`internal/core/item_rendering.rs:210-230`) asks `filter_item`
whether an item is visible, and when it is not, `render_result` stays
`ContinueRenderingChildren` - so **the traversal descends into the children of culled
items anyway**. Reaching the `HorizontalLayout` inside a row means evaluating its
geometry, which needs the children's `layout_info`, which for `Text` means
`ComplexText::layout_info` -> `text_size` -> parley shaping. That is exactly the stack
in the report: every one of the 100 000 rows gets shaped even though ~25 are visible,
and the shaped paragraphs stay in the text layout cache - hence 1542 MB vs 318 MB.

Two independent one-line fixes, both measured above:

- **No layout inside the repeated row** (`--flat-rows`): 3.10 s -> 131 ms first frame,
  1542 MB -> 318 MB. Position the children explicitly instead.
- **`clip: true` on the row** (not shipped in the app, this is the whole patch):

  ```diff
  --- a/ui/shared.slint
  +++ b/ui/shared.slint
   component RowBackground inherits Rectangle {
       in property <bool> alternate;

       height: 34px;
  +    clip: true;
       border-radius: 3px;
  ```

  3.10 s -> 103 ms first frame, 148 ms -> 97 ms later frames. This one is interesting because it is
  the *only* subtree pruning the renderer has today: `visit_clip`
  (`item_rendering.rs:519-539`) returns `ContinueRenderingWithoutChildren` when the clip
  region is empty, so an off-screen row's children are never visited. It costs one extra
  item per row (allocations go 7 -> 8 per row), which is why it does not win in the
  steady state.

### 2. Steady state: on FemtoVG the allocator really is about half of it

Every tracked property read of a non-constant property allocates one `DependencyNode`
into the window's redraw tracker; constant properties skip the registration entirely
(the `const_sentinel` check in `properties.rs:764-781`). That is where the 5-7
allocations per row per frame come from, and it is measurable both ways:

- Making the children's geometry constant removes exactly 2 allocations/row
  (503 396 -> 303 396 per frame) and saves 10.2 ms of 46.4 ms on FemtoVG, i.e.
  **51 ns per removed tracked read**.
- A standalone micro-bench of the same churn (`tools/allocbench.rs`, glibc) costs **13.4 ns per
  alloc+free pair** when the free list is hot, which is what per-frame `DependencyNode` traffic
  looks like. (An earlier version of this file said 34 ns and estimated the arena at 1.6-1.8x;
  that micro-bench used a grow-then-free pattern and overestimated by 2.6x. See
  `___OPTIMIZATIONS___/07_allocator_ceiling/`.)
- Adding 3 `Text` items per row with *constant* geometry costs 61 ms/frame on the
  software renderer at **zero** extra allocations, which is the traversal cost itself.

**How much is the allocator really worth here?** Measured directly by swapping glibc for
mimalloc (3.3x faster on this pattern, verified with a control bench):

| scene | renderer | glibc | mimalloc | delta |
| --- | --- | --- | --- | --- |
| `scroll` 100 000 | software | 148.17 ms | 130.22 ms | **-12.1%** |
| `scroll` 100 000 | FemtoVG | 59.69 ms | 62.59 ms | +4.9% (worse) |

So the arena/slab (#11292 / #12255) has a ceiling of roughly **17% on the software renderer**
and **single digits, if anything, on FemtoVG** - not the 1.6-1.8x estimated before. Most of the
~51 ns per tracked read is not the allocation but the registration itself: the thread-local
current-binding lookup, two linked-list insertions, and the cache misses from touching ~700 000
scattered dependency-list heads per frame. Reducing the *number* of tracked reads is worth more
than making each allocation cheaper - and `ListView` remains 10x faster than the best
`ScrollView` variant because it does neither.

### 3. What could speed up `ScrollView` without the arena

- **Prune subtrees of culled items.** Today only `clip: true` does that, by accident of
  `visit_clip`. If the renderer knew a subtree's bounding box (cached in
  `CachedRenderingData`, invalidated with the geometry), `render_item_children` could
  skip the recursion for culled items and the per-row cost would collapse to one
  geometry check - measured proxy: the "no children at all" row, 29 ms instead of 148 ms.
  The information is not free to obtain, but it is cacheable across frames, and it would
  fix the pathological first frame and the 1.2 GB of shaping cache for everyone, without
  requiring `clip: true` or hand-positioned children.
  A cheaper approximation with the same effect for the common case: let the *compiler*
  mark repeated components whose children provably stay inside the root's geometry
  (constant `x`/`y`/`width`/`height` expressions), and let the renderer prune those.
- **Do not track geometry properties of culled items.** This would remove the
  allocations *and* the registration cost, but it is not correct as-is: nothing else
  would schedule a repaint when an off-screen item moves into view.
- Even with all of the above it stays O(items): 100 000 rows x ~50-100 ns is still
  5-10 ms/frame, so virtualization (`ListView`) remains the answer for large models.
  What the app can do today is exactly the two fixes in section 1.

## Reproducing

```sh
just bench                       # software renderer, scroll, 100k, 6 frames
just bench-all                   # the whole software-renderer table above
just samplyrd-bench              # same workload under samply
just frames 100000 8             # FemtoVG on a real display
just frames-xvfb 100000 8        # FemtoVG on a virtual display (llvmpipe)
```

`--bench` counts allocations with a wrapping global allocator (feature `alloc-stats`,
on by default; two relaxed atomics per allocation). Turn it off with
`--no-default-features --features femtovg,software` when profiling malloc-heavy paths.
