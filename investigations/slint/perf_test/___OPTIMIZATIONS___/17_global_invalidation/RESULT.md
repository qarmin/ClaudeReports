# 17 - Flipping a global property every frame: null result

What a theme switch does: write one global property that every item in the tree reads. No scene
here did it - the others only write properties the *renderer* reads (scroll offset, geometry),
which is a different path.

`--toggle-global` flips `Settings.alt-theme` before each frame; every row's `background` binding
reads it, so one write dirties one binding per row.

| scene | static | +global toggle/frame | delta |
| --- | --- | --- | --- |
| `scroll` 1 000 | 2.87 ms | 2.34 ms | -0.53 ms |
| `scroll` 10 000 | 8.72 ms | 9.85 ms | +1.13 ms |
| `scroll` 100 000 | 123.50 ms | 124.01 ms | +0.51 ms |
| `list` 10 000 | 2.26 ms | 2.28 ms | +0.02 ms |
| `list` 100 000 | 2.20 ms | 2.18 ms | -0.02 ms |

Nothing, at any size. The deltas are inside the spread of the `scroll` scene (1.7-6 ms) and the
sign is not even consistent.

## Why

Bindings are evaluated **lazily**: dirtying 100 000 of them costs a flag and a walk of the
dependency lists, but the `background` binding is only re-evaluated when something *reads* it,
which is when the item is drawn. The renderer visits every item (candidate 12) but only reads the
brush of the ones it paints, so ~25 rows re-evaluate and the other 99 975 stay dirty until they are
needed.

This is the one traversal-shaped thing in the engine that already behaves like it should. Worth
knowing precisely because the neighbouring passes (12, 13, 15) do not: the property system is not
where the O(items) problem lives.

## Reproduce

```sh
tools/matrix.sh \
  "static|--mode scroll --count 100000" \
  "+global|--mode scroll --count 100000 --toggle-global"
```
