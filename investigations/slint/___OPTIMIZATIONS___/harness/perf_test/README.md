# Slint perf test

Scratch app for measuring Slint/FemtoVG rendering performance with a lot of content.
It uses the Slint crates from this repository and the `femtovg` checkout in `../femtovg`
(via `[patch.crates-io]`), so local changes to either are picked up by a plain rebuild.

## Modes

Switch with the three buttons in the top left corner. Only the active mode exists as
elements (`if` in the .slint file), so switching frees the previous tree.

1. **TextArea** - a `TextInput` inside a `ScrollView` (like the `TextEdit` widget, but it also
   exposes the cursor/anchor offsets). `+10 ... +100000` append that many lorem ipsum lines of
   at least 100 characters. Every line is unique (index prefix + rotated word window + hash
   suffix), so glyph/text caches cannot serve them all from a single entry. The footer shows how
   many lines the current selection spans. `word wrap` and `monospace` are togglable, both change
   the text layout cost significantly.
2. **ListView** - the same `+N` buttons add records to a `VecModel`. Each row is a `Rectangle`
   with `border-radius: 3px` and three values. `ListView` is virtualized, so only the visible
   rows exist.
3. **ScrollView** - the exact same model and row component, but with a plain `for` inside a
   `ScrollView`: every record is a live element. This is the mode that hurts. The `flat rows`
   checkbox (or `--flat-rows`) swaps the row for one without an inner layout, which is worth
   25x on the first frame - see [FINDINGS.md](FINDINGS.md).

4. **Rects** - a grid of rounded, bordered rectangles, all visible, moving every frame. Added
   for renderer work: it is dominated by per-primitive cost instead of by the item tree.

Modes 2 and 3 share one model, so records added in one show up in the other.

`force repaint` reads `animation-tick()`, which keeps the animation driver alive and makes the
window redraw every frame - turn it on before profiling, otherwise the profile is mostly an idle
event loop.

The status line at the bottom shows how long the Rust side took. Layout and rendering happen on
the next frame and are *not* included there - that part is what the profiler is for.

## Backends and renderers

FemtoVG is the default. Renderers are selected at runtime, but each one has to be compiled in:

| Renderer | Build | Run |
| --- | --- | --- |
| femtovg (default) | - | `just runr` |
| software | `--features software` | `RENDERER=software just runr` |
| skia | `--features skia` | `RENDERER=skia just runr` |
| Qt backend | `--features qt` | `RENDERER=qt just runr` |

`--features all-renderers` puts femtovg, skia and software in one binary; then pick with
`--renderer <name>` (or `SLINT_BACKEND=winit-<name>`). `perf_test --help` lists what the binary
has. Every `just` recipe honours the `RENDERER` variable.

## Automatic mode

Both benchmarks fill a mode with content, render N frames while scrolling, print a per
frame table (time, allocations, RSS) and exit.

```sh
just bench                          # headless, software renderer, no display needed
just bench -- --flat-rows           # ... with the layout-free row
just bench-all                      # the table in FINDINGS.md
just frames 100000 60               # real window with the selected renderer (FemtoVG), 60 frames
just frames-xvfb 100000 8           # same in a virtual X server (no display needed)
just samplyrd-bench                 # the headless workload under samply
```

Directly:

```sh
perf_test --bench --mode scroll --count 100000 --frames 6 [--flat-rows] [--size 1280x860]
perf_test --mode scroll --count 100000 --frames 60      # windowed, needs a GPU renderer
```

`--bench` installs its own `MinimalSoftwareWindow` platform, so it needs no display and
is deterministic; `--frames` uses the real backend and a rendering notifier, which is the
only way to measure FemtoVG or Skia. Allocation counting comes from a wrapping global
allocator (feature `alloc-stats`, on by default).

`frames-xvfb` runs FemtoVG against Mesa llvmpipe inside `Xvfb`, which needs
`xvfb`, `libxkbcommon-x11-0`, `libxcb-xkb1`, `libgl1-mesa-dri` and `libglx-mesa0`.
Rasterization then happens on the CPU, so drawing is slower than on a real GPU, but the
item traversal, layout, shaping and allocation counts are unaffected.

## Measurement extras

- `--alloc-profile <n>` samples 1 in n allocations and prints a ranking of allocation *sites*
  plus the heaviest stacks. `--profile-phase fill|show|frames` picks what to sample. Works
  because `perf`/`valgrind`/`heaptrack` are not always available.
- The per-frame table also reports **text shapes**, **paragraph shapes** and **line breaks** (from
  the shaped-text cache counters), which is how you tell whether scrolling re-shapes text. `text
  shapes` counts cache entries rebuilt; `paragraph shapes` counts the parley layouts actually
  built, which is the one to read for a large document, whose paragraphs are shaped on demand.
- `--snapshot <file.ppm>` writes the last frame, for before/after pixel comparisons. Works with
  `--bench` (headless, no display or GL needed) as well as with `--frames`.
- `text` mode: `--word-wrap` lays the document out with `wrap: word-wrap` (Slint's own `TextEdit`
  default) instead of `no-wrap`. The two take different paths through the shaped-text cache.
- `--resize-step <px>` alternates the window width by that many pixels each frame instead of (or as
  well as) scrolling. A resize re-runs the layout at a new width rather than reusing it, which is a
  different question from what scrolling costs, and the only way to reach the width-dependent paths
  of the text layout.
- `just allocbench` prints the cost of one malloc+free pair, to turn allocation counts into
  milliseconds.

## Findings

[___OPTIMIZATIONS___/SUMMARY.md](___OPTIMIZATIONS___/SUMMARY.md) is the index of every
optimization candidate investigated here (wins, null results and open items), each with its
own folder, measurements and patches. [___OPTIMIZATIONS___/PIPELINE.md](___OPTIMIZATIONS___/PIPELINE.md)
describes the harness.

[FINDINGS.md](FINDINGS.md) has the measured comparison of `ListView` (virtualized) vs
`ScrollView` + `for` at 100 000 records, what the time and the 1.5 GB of RSS actually go
into, and which of it can be fixed from the .slint side.

## Profiling

```sh
just samplyrd            # optimized + full debug info, recorded with samply
RENDERER=software just samplyrd
just heaptrack           # allocations
```

The `rdebug` profile is `release` plus `debug = "full"`, `strip = "none"` and no debug
assertions. If samply shows truncated stacks, rebuild with
`RUSTFLAGS="-C force-frame-pointers=yes"`.
