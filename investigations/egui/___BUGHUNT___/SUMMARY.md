
# Bug-hunt results

**Status**: **Confirmed** (a real, run reproduction exists) - **Not reproducible** (checked,
couldn't confirm - see the candidate's `RESULT.md` for exactly what was tried) - **Inconclusive**
(a signal was seen but fell short of `CONFIRMATION.md`'s bar). **Class** (only when Confirmed):
purely descriptive bug category - `crash` / `UB` / `logic` / `data-race` / `memory-safety` /
`security` / other. **Fully reproducible** (only when Confirmed): **Yes** if the repro actually runs
the real affected entry point end-to-end (the real binary, or the real public API a genuine caller
would use), driven by a real second participant when the finding is about one participant's
reaction to another's input (see `CONFIRMATION.md` section 11) - **No** only for the rare, explicitly
justified exception where a genuine end-to-end repro was impractical this session (the
justification lives in that candidate's `RESULT.md`, not here). **A `Confirmed` row without a
`Yes`/justified-`No` in this column should not exist** - a candidate whose only repro fakes a second
participant's behavior instead of actually running it does not get written up as `Confirmed` at all,
per `CONFIRMATION.md` section 11; it stays `Inconclusive` or goes back to `IDEAS.md` until a real
end-to-end repro is built. **Introduced by** (only when Confirmed, best-effort): the short commit
hash (`git log -S`/`git blame`/`git bisect` - see `COMMANDS.md`) that introduced the buggy code, if
it was worth tracking down and could actually be identified this session - written as **plain text,
not inside backticks** (e.g. `abc1234 "commit subject"`, not `` `abc1234` "commit subject" ``) so
GitHub/GitLab auto-links the hash to the commit instead of rendering it as inert code - or
`not determined` (with the reason briefly noted in `RESULT.md`: too costly to bisect this session,
history unavailable, present since the file's initial commit, etc.) which is a complete, honest
answer, not a gap to apologize for. **Fixed by** (only when Confirmed, best-effort, checked at
row-creation time and worth re-checking on later sessions/`git pull`s): blank/empty if still
present in current `HEAD` (the default, ordinary case) - or the short commit hash + subject of the
commit that fixed it, if the bug has since been fixed **upstream, independently of this
workspace's own (always-reverted) patch** (a maintainer applied their own fix, a dependency bump
happened to fix it, etc.), same plain-text-not-backticks format as **Introduced by**. Finding one
means the Status cell stays `Confirmed` (it was real) but a `CORRECTION.md` per `SKILL.md`'s "When
a later candidate invalidates an earlier one's confirmation" should exist alongside it stating the
fixing commit and that the original repro no longer reproduces against current `HEAD` - this
column is the at-a-glance pointer to that `CORRECTION.md`, not a replacement for it. None of these
columns is a priority ranking - see `SKILL.md`'s "the one rule that matters most".

Rows are kept sorted by the **#** column (ascending) - append in whatever order candidates get
investigated, then run `just tidy` (or `just sort-summaries`) to reorder; sorting is mechanical and
must never be done by hand. The **#** is a plain number so it sorts numerically: a string sort would
render `1, 10, 11, 2`. The order carries no meaning beyond readability - it is not a priority
ranking, see `SKILL.md`'s "the one rule that matters most". A
candidate that combines several suspected issues only appears here if explicitly requested and
investigated as its own row (this skill checks one bug per candidate by default - see `SKILL.md`).

| # | Status | Class | Fully reproducible | Candidate | Root cause (short) | Introduced by | Fixed by | Link |
|---|---|---|---|---|---|---|---|---|
| 001 | Confirmed | logic | Yes | shift_tab_trailing_spaces_cursor_jump | decrease_indentation subtracts TAB_SIZE from the caret even when only 1-3 trailing spaces were removed at end of buffer | not determined |  | [`001.shift_tab_trailing_spaces_cursor_jump/RESULT.md`](001.shift_tab_trailing_spaces_cursor_jump/RESULT.md) |
| 002 | Confirmed | logic | Yes | hex_color_accepts_plus_sign | from_str_radix accepts a leading +, so Color32::from_hex accepts strings like #+f0 as valid colours | 669473ccd Add Color32::from_hex_string and to_hex_string (#3570) |  | [`002.hex_color_accepts_plus_sign/RESULT.md`](002.hex_color_accepts_plus_sign/RESULT.md) |
| 003 | Confirmed | crash | Yes | galley_cursor_end_of_row_panics_with_zero_rows | cursor_end_of_row/cursor_end_of_paragraph index rows[0] on a zero-row galley (LayoutJob max_rows = 0); End key in TextEdit panics | 267485976 Simplify the text cursor API (#5785) |  | [`003.galley_cursor_end_of_row_panics_with_zero_rows/RESULT.md`](003.galley_cursor_end_of_row_panics_with_zero_rows/RESULT.md) |
| 004 | Confirmed | logic | Yes | format_with_minimum_decimals_above_six | decimals..=6 range is inverted for decimals > 6: debug assert panic, release returns fewer decimals than the requested minimum | not determined |  | [`004.format_with_minimum_decimals_above_six/RESULT.md`](004.format_with_minimum_decimals_above_six/RESULT.md) |
| 005 | Confirmed | crash | Yes | dashed_line_zero_lengths_hang | dashes_from_line loops forever (unbounded memory) for dash=gap=0 and panics on empty dash/gap slices | fc18d6f8f Add support for dashed lines with offset (#3720) (empty-slice panic); zero-length hang older, not determined |  | [`005.dashed_line_zero_lengths_hang/RESULT.md`](005.dashed_line_zero_lengths_hang/RESULT.md) |
| 006 | Confirmed | logic | Yes | closed_path_two_points_nan_vertices | Path::add_line_loop divides a zero miter normal by its zero squared length for paths that double back (2 points, collinear), emitting NaN vertices | not determined |  | [`006.closed_path_two_points_nan_vertices/RESULT.md`](006.closed_path_two_points_nan_vertices/RESULT.md) |
| 007 | Confirmed | crash | Yes | ellipse_tessellation_points_unbounded | tessellate_ellipse point count grows linearly with radius with no cap: 1e8 radius gives 100M vertices, 1e9 aborts on allocation failure | bce257adb epaint: Add EllipseShape (#4122) |  | [`007.ellipse_tessellation_points_unbounded/RESULT.md`](007.ellipse_tessellation_points_unbounded/RESULT.md) |
| 008 | Confirmed | crash | Yes | bezier_flatten_default_tolerance_zero | default flatten tolerance is 0.001 * x-distance of end points, zero for vertical curves: cubic hangs (release) or asserts (debug), quadratic silently becomes a straight line | not determined (predates cf7150c6a Refactor: put each shape into its own file (#5564)) |  | [`008.bezier_flatten_default_tolerance_zero/RESULT.md`](008.bezier_flatten_default_tolerance_zero/RESULT.md) |
| 009 | Confirmed | logic | Yes | quadratic_bezier_hairpin_flattened_to_chord | quadratic flattening count underestimates for arcs turning more than 90 degrees: hairpins are flattened to a single chord (880/200000 random curves exceed 2x tolerance, worst 108 units) | not determined (algorithm ported before 041f2e64b Move all crates into a crates directory (#1940)) |  | [`009.quadratic_bezier_hairpin_flattened_to_chord/RESULT.md`](009.quadratic_bezier_hairpin_flattened_to_chord/RESULT.md) |
| 010 | Confirmed | logic | Yes | log_slider_symmetric_unit_range_nan | logarithmic_zero_cutoff divides 0 by 0 for a zero-crossing log slider -1..=1: debug assert, release slider stuck at -1.0 | not determined (older than 33aa4d698 Fix typos (#2866)) |  | [`010.log_slider_symmetric_unit_range_nan/RESULT.md`](010.log_slider_symmetric_unit_range_nan/RESULT.md) |
| 011 | Confirmed | logic | Yes | sizing_remainder_min_overflows_available | Sizing::to_lengths decides which remainders sit at their minimum with a stale average, so cells can add up to more than the available length (9.5% of random strips, up to 11 points over) | 8cdffc4e2 Replace uses of RangeInclusive<f32> with emath::Rangef (#3221) |  | [`011.sizing_remainder_min_overflows_available/RESULT.md`](011.sizing_remainder_min_overflows_available/RESULT.md) |
| 012 | Confirmed | logic | Yes | circle_ellipse_visual_bounding_rect_half_stroke | visual_bounding_rect of Circle and Ellipse adds half the stroke width per side but the tessellator draws the stroke fully outside | bce257adb epaint: Add EllipseShape (#4122) (ellipse); circle older, not determined |  | [`012.circle_ellipse_visual_bounding_rect_half_stroke/RESULT.md`](012.circle_ellipse_visual_bounding_rect_half_stroke/RESULT.md) |
| 013 | Confirmed | logic | Yes | path_visual_bounding_rect_ignores_stroke_kind | PathShape::visual_bounding_rect always adds half the stroke width, so a StrokeKind::Outside path is culled although its stroke is inside the clip rect | not determined |  | [`013.path_visual_bounding_rect_ignores_stroke_kind/RESULT.md`](013.path_visual_bounding_rect_ignores_stroke_kind/RESULT.md) |
| 014 | Confirmed | logic | Yes | rect_translate_infinite_rect_nan | Rect::translate rebuilds from min + size, giving NaN max for Rect::EVERYTHING, Rect::NOTHING and any half-infinite rect | not determined |  | [`014.rect_translate_infinite_rect_nan/RESULT.md`](014.rect_translate_infinite_rect_nan/RESULT.md) |
| 015 | Confirmed | crash | Yes | columns_negative_width_when_too_narrow | Ui::columns computes a negative column width when the region is narrower than the spacing: debug assert panic, release layout starts left of the region | not determined |  | [`015.columns_negative_width_when_too_narrow/RESULT.md`](015.columns_negative_width_when_too_narrow/RESULT.md) |
| 016 | Confirmed | crash | Yes | table_remainder_column_at_most_panics | content-based minimum width is not capped at width_range.max, so Column::remainder().at_most(small) reaches f32::clamp with min > max | 8ef0e85b8 egui_extras: Improve the auto-sizing of Table (#4756) |  | [`016.table_remainder_column_at_most_panics/RESULT.md`](016.table_remainder_column_at_most_panics/RESULT.md) |
