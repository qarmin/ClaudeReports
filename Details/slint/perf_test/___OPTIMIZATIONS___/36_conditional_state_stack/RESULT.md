# 36 - save/restore the renderer state only for items that change it (idea 9, implemented)

`render_item_children` bracketed **every** item with `renderer.save_state()` /
`renderer.restore_state()`: a push and pop of the render state onto a `Vec`, through two virtual
calls on `&mut dyn ItemRenderer`, for every item in the tree whether or not anything was drawn.

`TEN_IDEAS.md` sized it at ~12 ns per item and left it unimplemented, calling it "riskier than it
sounds: any child that touches the clip or alpha relies on the pop".

## Why it is not as risky as it looked

The state stack exists for what an item's **own** `render()` does - `combine_clip`, `apply_opacity`.
Children balance their own state through their own save/restore. So the only item that needs the
stack is one whose `render()` actually runs, and `render_item_children` already computes exactly
that condition to decide whether to call `render()` at all. For every other item the only change is
the `translate` before recursing, which is undone directly:

```rust
if renders { renderer.save_state(); }
renderer.translate(item_origin.to_vector());
...
if renders { renderer.restore_state(); } else { renderer.translate(-item_origin.to_vector()); }
```

`Coord` is `f32`, so undoing a translation by adding the negation is not exact in general - this is
the part that needed checking rather than reasoning, see below.

## Result

Measured on top of candidate 35, `tools/isolate.py off 09-conditional-state-stack` for the OFF side,
12 paired alternating runs:

| scene | OFF | ON | | paired wins |
| --- | --- | --- | --- | --- |
| `scroll` 100 000 `bare` | 17.93 ms | **16.46 ms** | **-8.2%** | 11/12 |
| `nest` depth 8 | 12.40 ms | **11.94 ms** | -3.7% | 12/12 |
| `scroll` 20 000 (layout rows) | 21.06 ms | **20.50 ms** | -2.7% | 8/12 |
| `list` 20 000 | 2.22 ms | 2.22 ms | 0.0% | 2/9 |
| `rects` 3 000 | 2.35 ms | 2.35 ms | 0.0% | 7/9 |
| `rects` 3 000 `--opacity 0.5` | 2.81 ms | 2.81 ms | 0.0% | 3/9 |
| `text` 50 000 | 3.86 ms | 3.85 ms | -0.3% | 5/9 |
| `scroll` 5 000 `clipped` | 3.56 ms | 3.57 ms | +0.3% | 3/9 |

Same shape as candidate 35: everything is in the scenes with many culled items, and nothing
regresses. `clipped` rows and `--opacity` are flat because those items *do* render, so they keep the
stack - which is the point.

## The float-exactness question, answered by testing rather than by argument

Eight scenes with round coordinates said pixel-identical, which proves little: `x + d - d == x`
holds trivially when both are small integers. The cases that could break it are fractional device
coordinates, so those were built deliberately - fractional scale factors combined with fractional
scroll steps, including 8 levels of nesting so the translations compose:

| scene | verdict |
| --- | --- |
| `--scale-factor 1.75 --scroll-step 3.7` | identical |
| `--scale-factor 1.33 --scroll-step 0.9` | identical |
| `nest --nest-depth 8 --scale-factor 1.75 --scroll-step 3.7` | identical |
| `--scale-factor 2.5 --scroll-step 0.7` | identical |

374 `i-slint-core` tests pass, `--features ffi` clean.

## A false "differs", and the tool change it forced

The 1.75 scene first came back as **rendering differs**: 259 pixels, at `RUNS=3`. It was not a
rendering difference - at `RUNS=6` the same comparison put the noise floor at 476 px and the
verdict back to identical. **The noise floor is itself a sampled quantity, and undersampling it
manufactures differences.**

`tools/pixelcheck.sh` now defaults to `RUNS=5` and, when leftover pixels all sit inside the bounding
box of the noise region, reports `VERDICT: inconclusive - re-run with RUNS=<2x>` instead of
`differs`. It was re-calibrated afterwards on a change that genuinely alters the image (the ablation
that skips the dirty-region walk): 621 626 px, correctly outside the noise region, still `differs`.

That is the second time in two candidates that a pixel verdict was wrong in the *cautious*
direction, and both times the fix was to make the tool say "I don't know" where it had been saying
"yes".

## Reproduce

```sh
tools/isolate.py off 09-conditional-state-stack   # build -> /tmp/pt_i9off
tools/isolate.py restore                          # build -> /tmp/pt_i9on

for i in $(seq 1 12); do
  /tmp/pt_i9off --bench --mode scroll --count 100000 --row-style bare --frames 20
  /tmp/pt_i9on  --bench --mode scroll --count 100000 --row-style bare --frames 20
done

RUNS=6 tools/pixelcheck.sh /tmp/pt_i9off /tmp/pt_i9on -- \
  --mode nest --nest-depth 8 --scale-factor 1.75 --scroll-step 3.7
```
