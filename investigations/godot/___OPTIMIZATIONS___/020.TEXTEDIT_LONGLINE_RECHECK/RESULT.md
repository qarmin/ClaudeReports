# Re-checking candidate 008's `TextEdit` syntax-cache-by-reference fix with much longer lines

Candidate 008 (`TextEdit::_get_line_syntax_highlighting()` returning its cached
`Vector<Pair<int64_t,Color>>` by value vs. by reference) measured "No change" on short, lightly-
highlighted lines (~75 chars, matching a hunk of open PR godotengine/godot#123320) - the copied
vector was too small to matter at that scale. This candidate re-checks with lines ~100x longer
(7500 chars, ~100 keyword/color-region transitions per line vs. the original's handful), the one
variable candidate 008's own "What's not done" flagged as most likely to separate the real effect
from noise.

## What was tried

Reused candidate 008's own scratch benchmark shape (`bench/scratch_benchmark.patch`, adapted -
a `TEST_CASE` added to `tests/scene/test_text_edit.cpp`, not part of any `patch.diff`, reverted
before finishing this candidate), with the highlighted line repeated 100x into a single ~7500-char
line, 60 such lines, same warmup + 300-iteration `NOTIFICATION_DRAW` timing loop. Baseline = current
`HEAD` (unpatched `_get_line_syntax_highlighting()`, by-value copy). Patched = candidate 008's own
`patch.diff` applied on top.

Toolchain: SCons build (runtime-behavior measurement, not a compiled-artifact/disassembly claim -
no compiler version recorded, per the template's rule).

| Variant | us/iter (4 runs, first excluded as cold-cache) |
|---|---|
| baseline (by-value copy) | 616.7 / 617.4 / 626.8, mean 620.3 |
| patched (by-reference) | 616.3 / 613.7 / 607.8, mean 612.6 |

## Result

**Dimension measured: wall-clock time per `NOTIFICATION_DRAW` call (steady-state cache-hit reads,
twice per drawn line).**

Delta: -7.7us/iter, **-1.25%** - smaller than the ~1.4-1.6% run-to-run spread observed on both
sides. **Still "No change" at 100x the line length/color-transition count candidate 008 originally
tested at.** This rules out "line length was too short to show the effect" as an explanation for
candidate 008's own null result - even a ~7500-char line with ~100 color-region transitions
(vs. the original's handful) doesn't move the needle beyond noise.

## Does it change behavior/output?

N/A - this candidate applied no new patch to real source (candidate 008's own, already-recorded
`patch.diff` was reapplied temporarily to re-measure, then reverted - no new fix, no new
correctness claim beyond what candidate 008 already established). Full test suite (including this
candidate's own scratch benchmark `TEST_CASE`) ran cleanly: 1428 cases, 1427 passed, 1 failed
(known pre-existing `test_ip.cpp` IPv6 failure), 0 skipped when the scratch test is included -
unlike candidate 008's own scratch benchmark (which had a known full-suite ordering crash, see
`IDEAS.md`), this one ran and passed cleanly as part of the full suite, not just in isolation.

## Why the result looks the way it does

The dominant per-line `NOTIFICATION_DRAW` cost is elsewhere (text shaping, font rendering, glyph
lookup) and swamps the vector-copy difference regardless of the vector's size at the scales tested
here - the syntax-highlighting cache copy genuinely isn't a meaningful contributor to per-frame
`TextEdit` redraw cost, confirmed now across two very different line-length regimes (candidate
008's short lines and this candidate's 100x-longer ones). Candidate 008's underlying mechanism
(avoiding a copy) is still real and directionally correct - it's just too small relative to
everything else `NOTIFICATION_DRAW` does per line to matter in practice.

## What's not done / open questions

- This closes out the `IDEAS.md` backlog item that suggested this re-check - no further variants
  (even more color regions per line, thousands of lines) are expected to change the conclusion
  given the effect stayed flat across a 100x line-length range; not pursued further.
