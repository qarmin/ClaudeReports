# rect_translate_infinite_rect_nan

`Rect::translate` in `crates/emath/src/rect.rs:229`, written as `from_min_size(min + amnt, size())`. Looked suspicious because egui uses infinite rects as sentinels (`Rect::EVERYTHING` as "no clipping", `Rect::NOTHING` as the empty bounding box) and `size()` of those is infinite.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Sentinel probe | `translate`, `expand`, `shrink`, `rotate_bb`, `TSTransform * rect`, `intersect`, `round_to_pixels` on `Rect::EVERYTHING` and `Rect::NOTHING` | only `translate` yields NaN (`max = (min + d) + size = -inf + inf`); `rotate_bb(NOTHING)` returns an "everything" rect (separate, not analysed) |
| Standalone repro | `translate(vec2(1, 2))` on EVERYTHING, NOTHING, a finite rect and a half-infinite rect, debug build | EVERYTHING -> max NaN, NOTHING -> max NaN, half-infinite -> max.y NaN, finite is fine |

## Status

**Confirmed** - deterministic pure function, same output 3/3 runs.

**Fully reproducible**: **Yes** (public `emath::Rect` API, no second participant).

## Class

logic

## Root cause

`translate` rebuilds the rect from `min + amnt` and `size()`. For a rect whose size is infinite the new max is `-inf + inf` (EVERYTHING) or `inf + -inf` (NOTHING), i.e. NaN, so a translated "no clip" rect is a NaN rect for which every `contains`/`intersects` test is false.

## Introduced by

Not determined (the line predates the `crates/` move; `git log -S` stops at the first commit that contains it).

## The fix

Translate both corners (`patch.diff`, +5/-1). This also avoids the extra rounding of `min + size` for finite rects. After the patch EVERYTHING and NOTHING keep their values and the half-infinite rect keeps `inf`.

- Project test suites: emath 13 + 19 passed / 0 failed before -> same after; `cargo test -p egui --lib` 86 passed after (unchanged).

## What's not done / open questions

- No in-repo caller was found that translates an infinite rect (e.g. a default clip rect); the impact is on user code such as `ui.clip_rect().translate(offset)` when the clip rect is EVERYTHING.
- `Rect::NOTHING.rotate_bb(..)` returns a rect covering everything (an empty rect should stay empty); not changed here.
