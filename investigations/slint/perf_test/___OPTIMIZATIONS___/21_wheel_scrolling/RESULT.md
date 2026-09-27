# 21 - Wheel scrolling through `Flickable`: null result, plus one thing that does nothing

Every other scene scrolls by writing a property, which bypasses `Flickable` entirely. `--wheel`
dispatches a real `PointerScrolled` over the viewport instead.

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

## Reproduce

```sh
tools/matrix.sh \
  "property|--mode list --count 100000" \
  "wheel|--mode list --count 100000 --wheel --scroll-step 0"
```
