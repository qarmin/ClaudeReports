# GDScript syntax highlighter: hoist `color_map.sort()` out of the per-character loop

`GDScriptSyntaxHighlighter::_get_line_syntax_highlighting_impl()` (`modules/gdscript/editor/gdscript_highlighter.cpp`)
re-sorts its whole `color_map` `Dictionary` every time it exits a color-region match inside its
`for (int j = 0; j < line_length; j++)` loop - i.e. up to once per character on a line with many
short symbol/region transitions - instead of sorting once after the loop finishes. This looked
worth trying because it directly explains a real, currently-open bug report,
[godotengine/godot#123246](https://github.com/godotengine/godot/issues/123246) ("Text editor
incredibly slow with very long lines of code"), and matches one hunk of an already-open,
unmerged fix, [godotengine/godot#123320](https://github.com/godotengine/godot/pull/123320)
("Improve performance for GDScript Syntax Highlighting") - this candidate independently verifies
*only* that one hunk (the `sort()` hoist) in isolation, not PR #123320's other changes (a
`Vector`->`LocalVector` swap for `color_regions`, hoisting the `strip_edges()+split_spaces()`
"#region" check out of the per-region-candidate loop, and a caching-layer change in `TextEdit`
itself to stop copying the whole highlighting result out of the cache on every read) - per this
skill's "one fix per candidate folder" rule, those are separate, independently-measurable
candidates, left in `IDEAS.md`.

Toolchain: `scons` 4.11.1 (via `pip3 install scons --break-system-packages`), GCC 15.2.0 (Ubuntu
15.2.0-16ubuntu1), build flags `platform=linuxbsd target=editor tests=yes debug_symbols=yes`.

## What was tried

Measured via a temporary scratch `doctest` `TEST_CASE` added to
`modules/gdscript/tests/gdscript_test_runner_suite.h` (see `bench/scratch_benchmark.patch` - not
part of `patch.diff`, reverted before finishing this candidate per SKILL.md). It builds a
`CodeEdit` + `GDScriptSyntaxHighlighter`, sets a single line of length N built from a repeating
`"var x = 1 + 2 - 3 * 4 / 5 % 6 & 7 | 8 ^ 9 . "` pattern (heavy on symbols/operators, matching the
PR's own description of "particularly noticeable... with many spaces" - our pattern maximizes
region-transition-adjacent symbol checks the same way), then times
`clear_highlighting_cache(); get_line_syntax_highlighting(0);` in a loop, discarding nothing (each
iteration is a full cold computation since the cache is explicitly cleared first). Run via
`bin/godot.linuxbsd.editor.x86_64 --headless --test --test-case="*SCRATCH syntax highlighting*"`.
`uptime` load average was 0.9-1.7 (well under the 24-core budget) for every run below.

First, scaling behavior was characterized against the **unpatched** tree to confirm the mechanism
(all single-sample, `n_chars` varied, 1 iteration each after the same clear-cache pattern):

| line length (chars) | unpatched, single sample | ratio vs. half the length |
|---|---|---|
| 1000 | 538.8 us | - |
| 2000 | 1498.6 us | 2.78x |
| 4000 | 5576.4 us | 3.72x |
| 8000 | 20542 us | 3.68x |
| 16000 | 80080 us | 3.90x |

A 2x growth ratio would mean O(n); the observed 2.8-3.9x growth ratio confirms clearly
super-linear (roughly quadratic-ish, consistent with an O(k log k) sort repeated up to O(n) times
as the dictionary grows to size k up to n) scaling, before any fix.

Then, paired before/after measurement at two fixed sizes (5 runs at 8000 chars/10 iterations per
run, 3 runs at 16000 chars/5 iterations per run - fewer iterations at 16000 purely to keep total
session time bounded, not for any measurement reason):

| Variant | 8000 chars, us/iter (5 runs) | 16000 chars, us/iter (3 runs) |
|---|---|---|
| baseline (unpatched) | 23939.7, 23795.9, 20181.7, 19947.0, 21323.6 - mean 21837.6 | 81520.6, 79896.6, 75335.0 - mean 78917.4 |
| patched (`sort()` hoisted) | 13951.0, 13901.2, 11733.4, 11305.6, 11448.7 - mean 12468.0 | 40224.2, 40308.8, 40769.8 - mean 40434.3 |

## Result

**Time.** At 8000 chars: mean 21837.6us -> 12468.0us, a **42.9% reduction** (1.75x speedup). At
16000 chars: mean 78917.4us -> 40434.3us, a **48.8% reduction** (1.95x speedup). The improvement
*grows* with line length (1.75x at 8000 chars vs. 1.95x at 16000 chars) rather than staying constant,
which is the expected signature of removing a super-linear (not just constant-factor) cost - moving
a fixed amount of O(k log k) work from "done once per region-transition, k growing up to n" to "done
once, k = n at the very end" removes strictly more total work as n grows. Not measured: peak
RSS/allocation count - this candidate's mechanism (reordering an existing sort call, not
adding/removing allocations) has no plausible memory-footprint effect worth a separate measurement.

## Does it change behavior/output?

- **Bit-exact?** Yes. `color_map.sort()` sorts the `Dictionary`'s existing entries by key
  (character index) - it does not change which entries exist or what colors they map to, only when
  the sort happens relative to insertions. Since nothing between the loop's various early `sort()`
  call sites and the function's final `return color_map;` ever reads `color_map` assuming it's
  already sorted (verified by reading every line of the function between the removed call sites and
  the `return` - the only other uses of `color_map` are further insertions via `color_map[j] = ...`),
  sorting once at the very end instead of opportunistically during the scan produces an identical
  final `Dictionary`, key-for-key and color-for-color.
- **Is this a breaking change?** No - `_get_line_syntax_highlighting_impl` is a `protected`/internal
  override, not public API; its return value's *content* is unchanged, only the internal ordering
  of when it becomes sorted.
- Project test suite (`--headless --test`, `XDG_DATA_HOME` set, scratch benchmark `TEST_CASE`
  excluded via `--test-case-exclude="*SCRATCH*"` - see "What's not done" below for why the scratch
  case itself needs excluding): **1427 test cases, 1426 passed, 1 failed, 3 skipped** (the one
  pre-existing `test_ip.cpp` IPv6 failure documented in `___OPTIMIZATIONS___/README.md`) - identical
  before and after this patch. No new failures, no newly-passing tests.

## Why the result looks the way it does

`Dictionary::sort()` (backed by a hash map, sorted into a temporary ordered structure on every
call) costs roughly O(k log k) for a dictionary that currently holds k entries. The original code
calls it every time the per-character scan exits a "we just finished matching/failing a color
region" branch - which on a line dominated by short operators/symbols (each is its own
region-transition check) can happen close to once per character. Across a full line-length scan,
that sums to calling an O(k log k) sort at (roughly) every k from small up to n, whose total cost
integrates to something between O(n^2) and O(n^2 log n) - matching the observed super-linear
scaling table above almost exactly (ratios growing from 2.78x to 3.90x as n doubles repeatedly).
Moving the identical `sort()` call to run exactly once, after the loop, replaces that sum with a
single O(n log n) call - the dominant remaining cost of the whole function becomes the O(n)
per-character scan plus one O(n log n) sort, which is why the *relative* improvement keeps growing
with n (the removed super-linear term matters more, not less, as n grows) rather than plateauing at
a fixed percentage the way a constant-factor optimization would.

## What's not done / open questions

- This candidate deliberately measures *only* the `sort()` hoist in isolation. PR #123320's other
  three changes (see above) were not measured here - they're plausible independent wins worth their
  own candidates, added to `IDEAS.md`.
- The scratch benchmark `TEST_CASE` crashes with SIGSEGV when the *full* `--headless --test` suite
  runs it back-to-back with every other test case (but passes cleanly every time when run alone via
  `--test-case="*SCRATCH syntax highlighting*"`, which is how every timing number above was
  produced) - this looks like the scratch harness leaking or depending on some global editor-theme/
  `EditorSettings` state relative to whichever real test happens to run immediately before it in
  full-suite order, not a bug in the engine fix itself (a 2-line reordering of an existing `sort()`
  call cannot plausibly cause a segfault in an unrelated, later test case). Not root-caused this
  session since the scratch harness is disposable - the real product's own test suite was verified
  clean via `--test-case-exclude="*SCRATCH*"` instead (see "Does it change behavior/output?" above).
  If this harness is ever reused/extended by a future candidate, root-causing this first would be
  worthwhile.
- Did not verify the fix against real-world GDScript source (only a synthetic symbol-heavy line) -
  the mechanism (a `Dictionary` sort moved outside a loop with no change to its inputs) makes this
  low-risk, but a real large `.gd` file with a genuinely huge single line was not tried.
- Did not check whether the same "sort/re-sort inside a per-character or per-token loop" pattern
  appears in any other syntax highlighter in the codebase (e.g. `CodeHighlighter` in
  `scene/resources/syntax_highlighter.cpp`) - flagged in `IDEAS.md` as a lead, not checked here.
