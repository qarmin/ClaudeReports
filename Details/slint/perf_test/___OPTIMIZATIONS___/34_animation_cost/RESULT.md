# 34 - what a per-item animation costs, and what it costs to animate the *wrong* property

No scene here had ever measured animation. `--animate` binds every rectangle in the `rects` scene to
`animation-tick()`; `--animate-count` limits how many rectangles animate, so the per-animated-item
cost separates from the per-item cost by comparing two runs of the **same** scene.

This candidate adds `--animate-color`, which animates the fill instead of the position: the same
clock, the same number of bindings re-evaluated per frame, but the geometry never changes.

## Result

50 000 rectangles, all visible, medians of `RUNS=7`, each pair differing only in `--animate-count`:

| | none animated | all animated | per animated item |
|---|---|---|---|
| **position** (`x` offset) | 35.80 ms | **50.97 ms** | **~300 ns** |
| **fill** (`background`) | 31.92 ms | 30.76 ms | **below the noise floor** |

The position figure reproduced across five passes on two different builds of the scene
(+14.5, +13.9, +14.8, +14.0, +15.2 ms), so it is solid. The fill figure did not resolve at all: the
paired runs bracket it between -1.2 ms and +1.3 ms, i.e. under ~30 ns per item.

**Animating a position costs at least ten times what animating a fill costs.** Both re-evaluate one
binding per item per frame; the difference is everything that a *geometry* change sets in motion -
the dirty-region walk takes its `geometry_changed` branch, marks two rectangles dirty instead of
none, and writes the geometry cache (candidates 32 and 33).

This is the practical form of the same structural fact those candidates measured, and it is
actionable without any change to Slint: **animate colour, opacity or a transform in preference to
`x`/`y`/`width`/`height` when the visual result is the same.**

## Four hypotheses this refuted along the way

Worth recording, because each was plausible and each was wrong - and three of them were *mine*,
proposed after seeing a number and before testing it.

1. **"Animations are free"** - the first sweep (3 000 rectangles, 10/100/1000/all animated) was
   completely flat, which read as "the animation system costs nothing". It was too small: the
   per-item cost only clears the noise floor above ~50 000 items. A flat curve at one size is not a
   null result.

2. **"Fluent's translucent `control-background` makes every widget blend"** - `control-background`
   really is `#FFFFFF0F` / `#FFFFFFB3`, and the inference that half the scene therefore takes the
   software renderer's blending path is reasonable. Measured with an explicit `--item-alpha` sweep
   over the same colour: 1.000 -> 34.52 ms, 0.996 -> 35.75, 0.900 -> 36.63, 0.500 -> 36.09,
   0.050 -> 36.22. **~4%, at the edge of the noise, and no discrete opaque fast path.** Fill alpha
   is not a significant software-renderer cost.

3. **"A long chain of pass-through properties costs per link"** - a `--binding-depth` knob inserted
   0 to 8 pass-through properties between the position expression and `x`. Flat: 40.32, 38.23,
   36.96, 38.76, 37.95 ms. **Clean properties are not re-evaluated**, so a chain that nothing
   dirties costs nothing at all. Do not flatten bindings for performance. (The knob was removed
   again afterwards - see below.)

4. **"`animation-tick()` in a non-taken branch still dirties the property every frame"** - this
   would have been a significant bug. Checked against the generated code rather than by inference:
   the conditional compiles to a real Rust `if` and `sp::animation_tick()` is called only in the
   taken branch. Laziness is correct.

## One reading retracted

An earlier pass appeared to show the **fill colour alone** changing frame time by 19% (35.92 ->
28.92 ms) with identical geometry and no animation, reproducibly across three passes. It did not
survive: bisecting the two things `--animate-color` changes put the effect at 2.0-3.5 ms against
spreads of 3.5-6.4 ms, and it moved substantially when the binary changed. Three consistent passes
were not enough, because all three shared the same contaminated comparison. Recorded rather than
deleted, since "reproducible three times" is exactly the evidence that felt sufficient.

## A harness lesson: the `--binding-depth` knob was removed after it answered

The chain added 10 properties to **every** rectangle, and the `rects` scene is the baseline for
candidates 06, 09, 10, 11 and 25. It moved that baseline by 14 MB of RSS (103.9 -> 118.4 at 50 000)
and several ms per frame - so every future `rects` number would have been silently incomparable with
every recorded one. It was removed once it had produced its null result, and the baseline was
re-checked against the recorded values before moving on:

| | recorded before | after removal |
|---|---|---|
| `rects` 50 000 | 35.92 ms / 103.9 MB | 35.69 ms / 104.0 MB |
| `rects` 3 000 | 2.68-3.05 ms / 20.4 MB | 2.68 ms / 21.2 MB |

A measurement knob that is cheap to add can be expensive to leave in.

## Reproduce

```sh
RUNS=7 tools/matrix.sh \
  "no animation|--mode rects --count 50000 --animate --animate-count 0" \
  "animate position|--mode rects --count 50000 --animate" \
  "fill, none animated|--mode rects --count 50000 --animate-color --animate-count 0" \
  "animate fill|--mode rects --count 50000 --animate-color"

RUNS=7 tools/matrix.sh \
  "alpha 1.000|--mode rects --count 50000 --item-alpha 1.0" \
  "alpha 0.500|--mode rects --count 50000 --item-alpha 0.5"
```
