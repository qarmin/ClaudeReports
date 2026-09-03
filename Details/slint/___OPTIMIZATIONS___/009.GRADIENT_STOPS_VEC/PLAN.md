# 09 - Gradient paints allocate a Vec of stops per primitive per frame (plan)

## Observation

`GLItemRenderer::brush_to_paint` (`internal/renderers/femtovg/itemrenderer.rs:1519-1580`) builds

```rust
let mut stops: Vec<_> = gradient.stops().map(|stop| (stop.position, to_femtovg_color(&stop.color))).collect();
if let Some(last_stop) = stops.last().cloned() && last_stop.0 != 1.0 { stops.push((1.0, last_stop.1)); }
femtovg::Paint::linear_gradient_stops(start.x, start.y, end.x, end.y, stops)
```

for **every gradient-filled primitive of every frame**, and the same for radial gradients.
femtovg's signature is `stops: impl IntoIterator<Item = (f32, Color)>` and it copies them into
`GradientColors::from_stops`, so the `Vec` exists only to let the code look at the last stop.

Gradients are common in real UIs (buttons, panels, headers), so this is a per-primitive
allocation on a very ordinary path.

## Plan

1. Add a `--gradient` flag to the `rects` scene so the grid is filled with a linear gradient
   instead of a solid color, and measure allocations/frame and time (FemtoVG, Xvfb).
2. Replace the `Vec` with a `chain` of the extra stop: `gradient.stops().last()` gives what the
   fix-up needs without collecting, since the stops iterator can be walked twice.
3. Measure again; expect exactly -1 allocation per gradient-filled primitive.
4. Validate: pixel comparison of the gradient scene (deterministic, no text), plus the
   `i-slint-core`/`slint` test suites (gradient rendering is covered by the layout/rendering
   tests, and by the screenshot suite that cannot build here).

## Note

This is in the femtovg renderer only. The same pattern is worth checking in the Skia and
anyrender renderers - noted as a follow-up rather than done here.
