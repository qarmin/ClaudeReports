# Ideas

Untested ideas only - see `SUMMARY.md` for what's already been tried.

Last GitHub issue/PR checked for inspiration: **#8737** (repo: emilk/egui)

## Backlog (not yet tried)

Consumed top-to-bottom by default when a session isn't given a specific target - see `SKILL.md`.

| Added | Idea | Why it looks promising | Source |
|---|---|---|---|
| 2026-10-18 | `PaintStats::from_shapes` runs every tessellate although only the inspection UI reads it (walks every galley row) | `PaintStats::add` 3.5-4% of demo_only_tessellate (~1% of frame); needs an API decision (lazy / opt-in) | perf |
| 2026-10-18 | Merge the ~6 `Context` lock acquisitions in `create_widget` into one `write` | 005 removed 2 of ~8 acquisitions with no measurable effect; a full merge may still reach a few % of widget cost | candidate 005 |
| 2026-10-18 | `Path::add_line_loop` + `fill_closed_path` + `cw_signed_area` for 4-point rects: dedicated non-rounded rect fast path in `tessellate_rect` | add_line_loop 7% + fill_closed_path 5.6% of demo_only_tessellate | perf |
| 2026-10-18 | `floorf` (`floor_ui`, `floor_to_pixel`) cast-based replacement | 004 showed `roundf` replacement is a wash, so low prior | candidate 004 |
| 2026-10-18 | Issue #8093 "Text layout performance regression" 0.33 -> 0.34 (zoom in egui_plot) - bisect text layout between 0.33.3 and 0.34 | open perf complaint without repro numbers | GitHub #8093 |
| 2026-10-18 | `Button::atom_ui` allocates a `String` copy of the label for every button every frame (`layout.text().map(String::from)`, button.rs:344) only to feed the lazy `widget_info` closure | heaptrack: ~117 allocs/frame of ~1100 in the demo; needs the text only if accesskit is enabled, a debug flag is set, or the response emits an event (known only after `allocate`) | heaptrack session 2 |
| 2026-10-18 | `Atoms(Vec<Atom>)` allocates for every widget (atoms.rs push_right, ~300 allocs/frame in the demo); `SmallVec<[Atom; 2]>` was tried and does NOT compile: SmallVec is invariant in the lifetime so `combo_box.rs`/`menu.rs` fail variance checks; an enum `One(Atom)\|Many(Vec)` or arena might work | heaptrack: biggest single allocation site | heaptrack session 2 |
| 2026-10-18 | `highlight()` in egui_extras returns an owned `LayoutJob` clone every frame (text + hundreds of sections) and the Label then hashes it all again | `code_view_ui` is 7% of demo_no_tessellate inclusive | perf session 2 |
| 2026-10-18 | Pack `FontId`/`VariationCoords` writes too; combine with 006 and re-measure | follow-up of 012 | candidate 012 |
| 2026-10-18 | More `Context` lock merges: `Painter::add` (`graphics_mut`) + `will_discard` (via `Painter::is_visible`, called ~2x per widget when building `WidgetRect.visible` and in `is_rect_visible`), `accesskit_node_builder` x2, `register_accesskit_parent`, `viewport_id`, theme lookup in `widget_style` | 15 lock acquisitions per button remain after 020; each ~8-10 ns | candidate 020 |
| 2026-10-18 | Extend 021: per-corner normal tables for rounded rects with unequal radii, Ellipse (`add_line_loop` 30% of `shape_ellipse_fill`), `rect_blur`, closed `PathShape`s | `Path::add_line_loop` + `fill_closed_path` dominate every closed shape | candidate 021, scenes shape_* |
| 2026-10-18 | `Path::add_open_points` (28%) + `stroke_and_fill_path` (40%) for long polylines (plots): fewer divisions would change float results by an ULP (characterized-difference); explicit SIMD not allowed without unsafe | scene_polylines_2x5000_points | candidate 019 |
| 2026-10-18 | Cubic bezier flattening (`flatten_closed`, `split_range`, `FlatteningParameters::from_curve` ~36% of 800 ns/shape) | scenes shape_cubic_bezier_1000 | perf |
| 2026-10-18 | Build the galley-cache key without allocating the `LayoutJob` (String is moved, but sections Vec + `Arc<LayoutJob>` are created and dropped on every cache hit) | ~2 allocs per label per frame, heaptrack | heaptrack session 2/3 |
| 2026-10-18 | Combined effect measurement as `APPLIED_FIXES` (001,002,003,006,007,009,012,014,015,016,017,018 apply together cleanly with `git apply --3way`; scenes with all of them: buttons_300 -18%, grid_labels -18%, text_edit 18k -59%, changing_labels -37%, painter_shapes -14%, table -9%, widget_mix -7%, windows -8.5%, collapsing -8%) - 020/021 not included in that stack | measured informally with a throw-away worktree; not written up as APPLIED_FIXES.md | session 3 |

