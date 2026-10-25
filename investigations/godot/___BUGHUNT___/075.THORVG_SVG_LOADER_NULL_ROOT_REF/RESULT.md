# ThorVG SVG loader: null `root` dereference (`SvgLoader::run()`), then a second crash on the retry, for truncated/malformed SVGs

Found by the GDScript mutational fuzz loop (`Image.load_svg_from_buffer()` on byte-mutated SVG text), ASan
editor build, seed 2. ddmin (`tmp/bh/ddmin.py`) reduced the crashing input to `<svg ""><`; `<svg><` crashes too.
Code is in `thirdparty/thorvg` (1.0.3), reached through Godot's SVG import/`Image.load_svg_from_buffer()`.

## What was tried

| Approach/tool | Config | Outcome |
|---|---|---|
| Mutational fuzz of an SVG seed | ASan editor build `b8420aa3a6`, seed 2, <100 iterations | SIGSEGV in `tvg::Paint::ref()` from `SvgLoader::run()` (tvgSvgLoader.cpp:3778) |
| ddmin to a minimal input | `tmp/bh/ddmin.py`, re-running godot per candidate | `<svg ""><` (8 bytes); `<svg width="5" height="5"><`, `<svg viewBox="0 0 5 5"><`, `<svg><rect width="5" height="5"` also crash; `<svg ""></svg>`, `<svg "">` do not |
| Repro `repro/project/repro.gd` (`<svg><`) | unfixed build | SIGSEGV 10/10 runs |
| Fix v1 (`if (root) root->ref();` only) | rebuilt | first crash gone, but a **second** SIGSEGV appears in the worker thread (`_createSvgNode`, `loader->svgParse` already freed): `read()` re-runs the parse because `root` is still null. Rejected |
| Fix v2 (v1 + `header()` returns false when `run(0)` left `root` null) | rebuilt, final `patch.diff` | repro returns `Parse error`, 0/10 crashes; all the variant inputs above return `Parse error` (or load fine for the viewBox case) instead of crashing; valid SVGs still load |

## Status

**Confirmed.** Deterministic, 10/10 crashes before, 0/10 after.

**Fully reproducible**: Yes - real `Image.load_svg_from_buffer()` -> `ImageLoaderSVG` -> ThorVG path, input is
just a truncated SVG string (no second participant involved).

## Class

`crash` (null-pointer dereference; a truncated or garbled `.svg` file crashes the process, including the editor import)

## Root cause

`SvgLoader::run()` ends with an unconditional `root->ref();`. `root` is only assigned inside the
`xmlParse(...)` success + `loaderData.doc` branch (via `svgSceneBuild`), so when parsing the document
fails (e.g. a dangling `<`) it stays `nullptr` and `Paint::ref()` dereferences it. `header()` calls
`run(0)` synchronously when the root element has no `viewBox`; when a `viewBox` is present `run()` is
executed later on a worker thread from `read()`. In the second case, once the null `ref()` is guarded,
`read()` sees `root == nullptr` and schedules `run()` again after `clear(false)` already freed `svgParse`,
which crashes in `_createSvgNode` (`loader->svgParse->node = ...`). So the failure has to be reported
from `header()` rather than only guarded in `run()`.

Not checked: other `Paint::ref()`/`root` uses in other thorvg loaders (lottie, png, jpg), and whether upstream
ThorVG newer than 1.0.3 already fixes this.

## Introduced by

7770103858 thorvg: Update to 1.0.3 (the `root->ref();` line in its current form; the structure goes back to
5db751832d Update ThorVG to v0.9.0 - best-effort `git log -S`)

## The fix

`patch.diff` (+2/-1, in `thirdparty/thorvg`): guard `root->ref()` and `return false` from `header()` when
`run(0)` produced no root. Applied to the main tree, rebuilt, verified, reverted. Godot keeps thirdparty
patches in `thirdparty/thorvg/patches/`, so a real application should be added there too (not done).

Tests: fixed build `--headless --test`: 1431 cases, 1430 passed, 1 failed (`[IP] resolve_hostname`,
needs network in this sandbox, unrelated); no clean unfixed full-suite baseline (see 073).

## What's not done / open questions

`just test` not run from a fresh clone (full ASan scons build per variant); `repro/run.sh` run against the
unfixed and fixed binaries built from the main tree.
