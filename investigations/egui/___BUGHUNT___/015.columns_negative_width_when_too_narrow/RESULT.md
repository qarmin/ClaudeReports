# columns_negative_width_when_too_narrow

`Ui::columns_dyn` and `Ui::columns_const` in `crates/egui/src/ui.rs` (~2667, ~2725), which carry a `TODO(emilk): ensure there is space`. Found by a headless widget smoke run inside a narrow, heavily padded frame: `columns(3)` panicked with "Negative width makes no sense".

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Widget smoke run | ~20 widgets inside `Frame::inner_margin(100)` in a 50 px screen, debug | only `columns` panics (the `set_width(-5)` style panics are documented asserts) |
| Standalone repro | `ui.columns(3, ..)` and `ui.columns(20, ..)` in a 100 px wide region, default spacing 8 | 3 columns fine. 20 columns: debug panics (`set_min_width` debug_assert, width -2.6); release lays the first column out at x = -1.3..7.7, i.e. left of the region it was given |

## Status

**Confirmed** - deterministic, 3/3 runs, debug and release.

**Fully reproducible**: **Yes** (real `Ui::columns` in a headless `Context`).

## Class

crash

## Root cause

`column_width = (available_width - spacing * (n - 1)) / n` is negative when the region is narrower than the total spacing. It is passed to `set_width`, whose debug assertion panics, and in release builds the negative width shifts the columns out of the region.

## Introduced by

Not determined (the TODO comment is in the code since at least the `crates/` move, 041f2e64b).

## The fix

Clamp the column width to `>= 0` in both functions (`patch.diff`, +2/-2). After the patch 20 columns in 100 points are laid out from x = 0 with width 9 (debug and release), no panic.

- Project test suite (`cargo test -p egui --lib`): 86 passed / 0 failed before -> same after.

## What's not done / open questions

- Columns still overflow the region when it is too narrow (the TODO); only the negative width is removed.
- `Grid` and `ui.horizontal` with many items were not checked.
