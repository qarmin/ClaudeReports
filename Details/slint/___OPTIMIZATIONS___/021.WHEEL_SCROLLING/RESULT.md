# 21 - Wheel scrolling through `Flickable`: null result, plus one thing that does nothing

Ported from `perf_test/___OPTIMIZATIONS___/21_wheel_scrolling/RESULT.md` on 2026-08-23 (original perf_test-relative session; source and any extra notes remain there untouched).

Every other scene scrolls by writing a property, which bypasses `Flickable` entirely. `--wheel`
dispatches a real `PointerScrolled` over the viewport instead.

## Measured

| scene | property scroll | wheel scroll |
| --- | --- | --- |
| `list` 100 000 | 2.25 ms | 2.36 ms |
| `scroll` 10 000 | 9.07 ms | 8.15 ms |

Within noise (the `scroll` scene's spread is 1.8-4.7 ms). **`Flickable` adds nothing measurable** to
a scroll - the cost is the traversal and the drawing, wherever the offset came from. The property
scroll used by every other scene is therefore a fair stand-in, which is worth knowing because all
the numbers in this workspace depend on it.

`list` shapes 21 paragraphs per wheel frame against 18 per property frame, i.e. the wheel moves
slightly further per notch. Not a difference in kind.

## The `text` scene does not respond to the wheel at all

0.02 ms and 35 allocations per frame: nothing is redrawn, the wheel event changes nothing. That is
the hand-rolled `TextArea` in `perf_test/ui/text-mode.slint`, not Slint - it wires the `ScrollView`
to a `TextInput` without forwarding wheel events, so this is a property of the test scene. Recorded
so the next person does not read `text --wheel` as a Slint result. Slint's own `TextEdit` was not
tested here.

## Result

No measurable difference between wheel-driven and property-driven scrolling on the scenes tested;
the `text` scene's apparent zero-cost wheel response is an artifact of the test scene's wiring, not
of Slint.

## Does it change behavior/output?

N/A - measured/quantified only, no patch applied to real source in the original session.

## Why the result looks the way it does

`Flickable`'s own overhead is dominated by the traversal and drawing cost that happens regardless
of how the scroll offset was produced, so routing the same offset change through a real pointer
event versus a direct property write is indistinguishable within measurement noise.

## What's not done / open questions

- Slint's own `TextEdit` was not tested with `--wheel`; only the hand-rolled `perf_test`
  `TextArea` scene was, and that scene doesn't forward wheel events to its `TextInput` at all.

## Reproduce

```sh
tools/matrix.sh \
  "property|--mode list --count 100000" \
  "wheel|--mode list --count 100000 --wheel --scroll-step 0"
```
