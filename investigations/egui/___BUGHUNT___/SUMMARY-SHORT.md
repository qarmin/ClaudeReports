
# Confirmed bugs (Status: Confirmed only)

Filtered from `SUMMARY.md` - see that file for every candidate investigated (including
Not-reproducible and Inconclusive ones) and the full methodology behind each finding.

| # | Class | Candidate | Fix lines | Root cause | Introduced by |
|---|---|---|---|---|---|
| 001 | logic | TextBuffer::decrease_indentation caret adjustment (text_buffer.rs) | +4/-9 | caret moved by 4 though fewer spaces removed at end of buffer | not determined |
| 002 | logic | ecolor HexColor::from_str_without_hash leading-plus check | +5/-0 | from_str_radix accepts leading + | 669473ccd |
| 003 | crash | epaint Galley::cursor_end_of_row and cursor_end_of_paragraph zero-row guard | +7/-2 | unchecked rows[0] index when max_rows = 0 gives zero rows | 267485976 |
| 004 | logic | emath format_with_minimum_decimals range upper bound | +1/-1 | hard-coded ..=6 upper bound inverts the range for decimals > 6 | not determined |
| 005 | crash | epaint Shape::dashed_line pattern-length guard in dashes_from_line | +5/-0 | no progress/emptiness guard on dash pattern | fc18d6f8f |
| 006 | logic | epaint Path::add_line_loop miter normal for doubled-back closed paths | +3/-0 | 0/0 miter normal when closed path turns 180 degrees | not determined |
| 007 | crash | epaint Tessellator::tessellate_ellipse point-count cap | +2/-1 | uncapped point count proportional to radius | bce257adb |
| 008 | crash | epaint Bezier::flatten(None) default tolerance for equal-x end points | +14/-3 | default tolerance zero when end points share x | not determined |
| 009 | logic | epaint QuadraticBezierShape flattening of hairpin curves | +48/-7 | flattening count too low for arcs containing the parabola apex | not determined |
| 010 | logic | egui logarithmic_zero_cutoff for -1..=1 log sliders | +7/-1 | 0/0 zero cutoff when both magnitudes are log10(1) | not determined |
| 011 | logic | egui_extras Sizing::to_lengths remainder minimum handling | +19/-7 | stale average when pinning remainders at their minimums | 8cdffc4e2 |
| 012 | logic | epaint CircleShape/EllipseShape::visual_bounding_rect stroke width | +4/-2 | stroke drawn outside but bounding rect uses centered-stroke width | bce257adb |
| 013 | logic | epaint PathShape::visual_bounding_rect stroke kind | +6/-1 | bounding rect ignores StrokeKind::Outside/Inside width | not determined |
| 014 | logic | emath Rect::translate for infinite rects | +5/-1 | min + size arithmetic gives inf - inf = NaN | not determined |
| 015 | crash | egui Ui::columns negative column width | +2/-2 | column width not clamped at zero | not determined |
| 016 | crash | egui_extras TableBuilder remainder column at_most | +1/-1 | min.max(max_used) can exceed width_range.max before clamp | 8ef0e85b8 |
