# 47 - a `Path` costs 3x a rectangle and re-parses its command string every frame

The software renderer has a whole tessellation and scanline-fill implementation for `Path`
(`software/path.rs`, `#[cfg(feature = "path")]`) that nothing in this workspace had ever reached.
Added `--path` to the `rects` scene: the same grid, each item a `Path` with a fill, a 1px stroke and
a two-triangle command string, instead of a `Rectangle`.

## The cost

`--repaint-buffer new`, `RUNS=7`:

| items | `Rectangle` | `Path` | | allocations/frame |
| --- | --- | --- | --- | --- |
| 300 | 0.81 ms | **2.27 ms** | **2.8x** | 19 -> **2 719** |
| 1 000 | 1.85 ms | **6.81 ms** | **3.7x** | 21 -> **9 021** |
| 3 000 (about 1 200 visible) | 2.78 ms | 8.41 ms | 3.0x | 25 -> 10 015 |

**Exactly 9.0 heap allocations per path per frame** (2 719/300 and 9 021/1 000). Every other scene in
this workspace runs at 6-25 allocations *per frame in total*, after candidates 09, 10, 11 and 14 -
one `Path` undoes all of that on its own.

## Where they come from

`--alloc-profile 20 --profile-phase frames` attributes **98.7%** of them to one stack:

```
RawVecInner::grow_amortized
Vec<Point2D<f32>>::push_mut
lyon_path::builder::...::begin
PathData::iter                   [core/graphics/path.rs:307]
Path::fitted_path_events         [core/items/path.rs:165]
SceneBuilder::draw_path          [software/lib.rs:3136]
```

`PathData::Commands` **re-runs `lyon_extra::parser::PathParser` over the command string on every
call** (`graphics/path.rs:304-318`), and `draw_path` calls it once per item per frame. The geometry
never changes; the string is parsed 60 times a second per path.

This is the same shape as candidates 06, 10 and 11 - a path rebuilt per primitive per frame - except
those were in the FemtoVG renderer and this is in `i-slint-core`, on the path every renderer takes.

## The prize is *not* the parsing

Memoizing the parsed path by its command string (thread-local, 64 entries), pixel-identical:

| | base | memoized | |
| --- | --- | --- | --- |
| 300 paths | 2.19 ms | 2.09 ms | **-4.6%**, 7/7 paired |
| 1 000 paths | 6.66 ms | 6.24 ms | -6.3%, 6/7 |
| allocations/frame at 300 | 2 719 | **2 119** | -22% |

**-22% of the allocations, not -98.7%** - because `LyonPathIteratorVariant::FromPath` owns its
`lyon_path::Path`, so the memo hands out a **clone**, and cloning it allocates roughly seven of the
nine. Parsing was only about two of them.

So two things are now known and one is not:

- the parse is worth about 5%;
- the remaining ~7 allocations per path are the clone, and this experiment cannot remove them -
  a real fix has to hand out a shared reference (`Rc<lyon_path::Path>` inside
  `LyonPathIteratorVariant`), not a copy;
- **whether removing all 9 would buy much** - answered below, and the answer is no.

## Where the 3x actually is: the zeno conversion and the fill

Ablating `path::convert_path_data_to_zeno` to an empty command list, so the path is set up in full
and then draws nothing (a cost attribution, not a fix - the pixels are gone):

| | full | conversion + fill removed | |
| --- | --- | --- | --- |
| 300 paths | 2.17 ms | **0.967 ms** | **-55.4%**, 5/5 paired |
| 1 000 paths | 6.48 ms | **2.40 ms** | **-63.0%**, 5/5 |
| allocations/frame at 300 | 2 719 | 1 819 | -33% |

So a path's cost breaks down roughly as:

| part | share |
| --- | --- |
| converting to zeno commands + rasterizing them | **55-63%** |
| re-parsing the command string | ~6% (measured by the memo above) |
| everything else - `fitted_path_events`, item overhead, the clone | the rest |

And the sharpest way to put it: **at 300 items a path costs 0.97 ms before it draws a single pixel,
while a rectangle costs 0.81 ms in total.**

### This closes the question the right way round

The obvious next move after the allocation profile was to refactor `LyonPathIteratorVariant` to hold
an `Rc<lyon_path::Path>` and stop cloning. That would target the ~7 allocations per path which,
between them, are worth a few percent - while **60% of the cost sits in code no cache touches**.

Ablating before refactoring cost one build and saved a redesign of a public-ish type for a
single-digit gain. The allocation count was the loudest signal and the wrong one to follow: 2 719
allocations per frame is genuinely startling next to the 6-25 every other scene manages, and it is
still not where the time goes.

## Reproduce

```sh
RUNS=7 tools/matrix.sh \
  "rects|--mode rects --count 1000 --repaint-buffer new" \
  "paths|--mode rects --count 1000 --path --repaint-buffer new"

perf_test --bench --mode rects --count 300 --path --repaint-buffer new --frames 8 \
          --alloc-profile 20 --profile-phase frames
```
