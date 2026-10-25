# TextEdit syntax highlighting cache: read by reference instead of by value

`TextEdit::_get_line_syntax_highlighting(int)` returned `Vector<Pair<int64_t, Color>>` **by
value** from a `HashMap` cache lookup, called twice per drawn line (once for the main text, once
for the minimap) inside `_notification(NOTIFICATION_DRAW)` - once per frame, per visible line.
Matches the shape of open upstream PR godotengine/godot#123320 (a different hunk of the same PR
than candidate 001). Renamed to `_update_line_syntax_highlighting(int) -> bool`, which populates
`syntax_highlighting_cache[line]` as a side effect and returns whether highlighting exists for
that line; both call sites now read `syntax_highlighting_cache[line]` by `const &` (falling back
to a function-local `static const` empty vector when there's no highlighting for that line)
instead of copying the vector out.

## What was tried

Harness: a temporary doctest `TEST_CASE` added to `tests/scene/test_text_edit.cpp` (never part of
`patch.diff` - see `bench/scratch_benchmark.patch`), building a `TextEdit` with a `CodeHighlighter`
over 500 short lines (`"func foo(a, b): return a + b if a > b else b - a # comment 12345 var x =
1"`, a handful of keyword/number/symbol/comment-region highlights per line), sized so ~400+ lines
are visible with the minimap enabled, then timing 300 repeated `NOTIFICATION_DRAW` deliveries after
a 5-iteration cache-warming pass (so every timed draw is a pure cache-hit read, the steady-state
cost this candidate targets). Godot's own `--headless --test --test-case="*SCRATCH*"` runner, no
external timing tool needed (`OS::get_ticks_usec()`).

Toolchain: SCons build (runtime-behavior/wall-clock measurement, not a compiled-artifact/disassembly
claim - no compiler version recorded, per the template's rule).

| Variant | single-core p50 (`taskset -c 5`, 6 runs, us/iter) | unpinned p50 (8 runs, us/iter) | notes |
|---|---|---|---|
| baseline (`_get_line_syntax_highlighting`, by-value) | 3940.7 | 4040.0 | pinned: 3930.7/3937.2/3940.1/3941.3/3961.6/4312.5 (one outlier); unpinned: 3927.1/3995.4/4011.9/4020.6/4059.4/4076.1/4091.8/4226.1 |
| patched (`_update_line_syntax_highlighting`, by-ref) | 3886.6 | 3867.8 | pinned: 3870.2/3873.9/3880.3/3892.8/3941.5/4048.3 (one outlier); unpinned: 3811.6/3812.6/3820.3/3853.3/3882.4/4000.3/4276.3/4603.0 (one outlier) |

## Result

**Dimension measured: wall-clock time only** (no allocation counter available in this container;
the mechanism removes a heap allocation + copy of a small `Vector`, so an allocation-count win is
plausible but not separately instrumented here).

Median delta: -1.4% pinned (3940.7us -> 3886.6us), -4.3% unpinned (4040.0us -> 3867.8us). Mean
delta: -2.2% pinned, -1.1% unpinned. Every one of the four before/after comparisons is smaller than
the run-to-run spread observed within a single variant (e.g. baseline pinned alone ranged
3930.7-4312.5us, a 9.7% spread with no code change at all; unpinned patched alone ranged
3811.6-4603.0us, a 20.8% spread). **This is within the noise floor - not a measurable win at this
benchmark's scale**, even though the code change is real and the mechanism (avoiding two
`Vector<Pair<int64_t,Color>>` heap-copy round trips per line per frame) is sound in principle.

## Does it change behavior/output?

- **Bit-exact?** yes - this is a pure refactor of how the same cached data is retrieved, not what
  is computed.
- **Is this a breaking change?** No - `_get_line_syntax_highlighting`/`_update_line_syntax_highlighting`
  are both private members, no public API surface changes.
- Project test suite (real engine source, not counting the temporary scratch benchmark test case):
  baseline 1427 cases, 1426 passed, 1 failed (known pre-existing `test_ip.cpp` IPv6 failure, no
  IPv6 in this container), 3 skipped. Patched: identical - 1427 cases (1428 counting the temporary
  scratch benchmark case itself, which also passes), 1426/1427 passed, same single known failure,
  3 skipped. No new failures.

## Why the result looks the way it does

The copied `Vector<Pair<int64_t, Color>>` per drawn line is small - on the order of a dozen or two
`(int64_t, Color)` pairs for a realistically-highlighted line (one entry per color transition), a
few hundred bytes at most. Against the rest of what `_notification(NOTIFICATION_DRAW)` does per
line (font shaping/glyph lookups, rect fills, gutter drawing, the minimap's own separate scaled
draw pass), a single small-vector heap copy just isn't a large enough fraction of the total frame
cost to rise above this container's own measurement noise (itself several percent, apparently
dominated by shared-machine scheduling jitter rather than anything in this benchmark). This is
plausibly a bigger win on files with much longer lines and/or far more color-region transitions per
line (more entries in the copied vector, i.e. the same shape of effect candidate 001 found scales
with line length) - not measured here, see below.

## What's not done / open questions

- **Not tried with longer/heavier-highlighted lines.** This benchmark used short, lightly
  highlighted lines matching realistic GDScript. Re-running with much longer lines and/or many more
  color regions per line (closer to candidate 001's 8000/16000-char stress lines) would show
  whether the by-value copy cost scales enough with vector size to separate from noise - not done
  this session, left as a follow-up if this candidate is revisited.
- **No allocation-count instrumentation** was available in this container to directly confirm the
  removed heap allocation, only wall-clock time (which came back inconclusive). An allocation
  counter (if Godot's allocator can be wrapped/counted per the skill's `diag-mimalloc`-style
  pattern) would give a cleaner, allocation-count-based confirmation of the mechanism even without
  a measurable time delta.
- The scratch benchmark's own `TEST_CASE` (`tests/scene/test_text_edit.cpp`) is intentionally not
  part of `patch.diff` per SKILL.md's Correctness section (no permanent test files added to the
  project) - see `bench/scratch_benchmark.patch` for the harness-only diff, applied by `justfile`'s
  `clone` step, never mixed into the real fix.
