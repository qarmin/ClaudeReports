# egui bug-hunt workspace

Base commit 86099aca1 (egui 0.37.0). Findings are in `SUMMARY.md`; each candidate folder has `RESULT.md`, `repro/`, `patch.diff` and a self-contained `justfile` (`just test` clones the repo at the base commit, runs the repro, applies the patch, runs it again).

## Toolchain availability

| tool | works here? | notes |
|---|---|---|
| debug-assertions build | yes | most findings come from debug asserts or from checking invariants on output |
| miri / ASAN | not used | emath/epaint/ecolor/egui core contain no `unsafe`; backends do but are not headless-testable |
| headless egui | yes | `Context::default().run_ui(RawInput{..}, ..)`, then `out.textures_delta.clear()` (dropping it panics); `focused: true` is needed for key events; `TextEdit` needs `.lock_focus(true)` for Tab |
| epaint lib tests | need `--features default_fonts` | otherwise `unique_test_key` is missing (pre-existing) |

Use a shared `CARGO_TARGET_DIR` (e.g. `~/.cache/bh_target`); the disk was nearly full. Repros that can run away use `ulimit -v 6000000` and a watchdog child process.

## Randomized probes used (not kept; scratch in tmp/)

Tessellator shape fuzz with output-finiteness check, visual_bounding_rect vs mesh bounds, TextEdit random event sequences (no failure in 30000), LayoutJob/galley invariants (no failure in 20000), Slider range grid, TextBuffer editing ops with in-range cursors (no failure).
