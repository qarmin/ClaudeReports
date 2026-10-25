# GDScript syntax highlighter: `Vector<ColorRegion>` -> `LocalVector<ColorRegion>`

Third independent hunk of the same already-open PR
[godotengine/godot#123320](https://github.com/godotengine/godot/pull/123320) checked in this
workspace (see candidates 001 and 002 for the other two). `color_regions` is a private,
never-shared member of `GDScriptSyntaxHighlighter` - swapping its type from the reference-counted,
copy-on-write `Vector<T>` to the non-shared `LocalVector<T>` removes the `CowData` indirection
entirely for every access. Tested in isolation on top of a pristine, unpatched tree (neither
candidate 001's nor 002's fix applied).

Toolchain: `scons` 4.11.1, GCC 15.2.0, `platform=linuxbsd target=editor tests=yes debug_symbols=yes`.

## What was tried

Same scratch benchmark and same 8000-char synthetic line as candidates 001/002
(`bench/scratch_benchmark.patch`, identical), same invocation, 5 runs of 10 iterations. `uptime`
load average 1.09-1.64 for all runs (uptime marginally higher than candidates 001/002's runs but
still well under the 24-core budget).

| Variant | 8000 chars, us/iter (5 runs) |
|---|---|
| baseline (unpatched, same numbers as candidates 001/002's baseline row) | 23939.7, 23795.9, 20181.7, 19947.0, 21323.6 - mean 21837.6 |
| patched (`LocalVector<ColorRegion>`) | 24449.2, 24212.0, 21381.0, 21446.7, 21459.2 - mean 22589.6 |

## Result

**Time.** Mean 21837.6us -> 22589.6us - a **+3.4% difference, within the noise floor** (the
baseline's own run-to-run spread is ~18%, larger than this difference). Same conclusion as
candidate 002: no measurable change. Peak RSS/allocation count not measured - `LocalVector` avoids
a refcount+possible-fork check per access but doesn't change the number or size of allocations for
a vector that's never shared/copied in the first place (it's a private member, never aliased), so
no plausible allocation-count effect either.

## Does it change behavior/output?

- **Bit-exact?** Yes. `LocalVector<T>` supports every operation this class uses on
  `color_regions` (`clear()`, `insert(pos, val)`, range-`for`, `operator[]`, `size()`) with
  identical semantics to `Vector<T>` for a private, non-shared, non-serialized member - no
  observable behavior change. One mechanical fix was needed: `LocalVector::size()` returns an
  unsigned type, so the existing `for (int c = 0; c < color_regions.size(); c++)` loop needed an
  explicit `(int)` cast to avoid a signed/unsigned comparison (added in `patch.diff`) - purely a
  compile-time fix, not a behavior change.
- **Is this a breaking change?** No - private member, no public API surface.
- Project test suite (`--headless --test`, `XDG_DATA_HOME` set, scratch benchmark excluded via
  `--test-case-exclude="*SCRATCH*"`): **1427 test cases, 1426 passed, 1 failed, 3 skipped** - the
  same one pre-existing `test_ip.cpp` IPv6 failure, identical before and after.

## Why the result looks the way it does

Same story as candidate 002: `color_regions` is a small vector (order of 10-15 pre-registered
`ColorRegion` entries for a language's whole set of string/comment/region markers), and neither
`Vector`'s CowData indirection nor `LocalVector`'s direct storage meaningfully differs in cost for
read-only access at this size - both are effectively "array of small structs, indexed in a tight
loop," and the measured difference is well inside noise. The dominant costs in this function remain
the O(n) per-character scan and (in this unpatched baseline) the O(n^2)-ish `color_map.sort()`
candidate 001 already found and fixed - a container-type swap on a ~15-element vector cannot
plausibly compete with either of those at any line length this workspace tested.

## What's not done / open questions

- Not tested combined with candidate 001's fix (i.e., does the container-type swap matter more once
  the dominant `sort()` cost is already removed and the remaining O(n) scan is what's left to
  optimize further)? Plausible the *relative* weight of this micro-optimization grows once the
  bigger cost is gone, but the absolute numbers here (a few percent, inside noise) suggest even then
  it would stay negligible. Not verified - left as a note rather than re-tested, given two
  independent "no change" results already point the same direction.
- This closes out all three of PR #123320's `gdscript_highlighter.cpp`-side hunks that this
  workspace identified as separately testable (candidates 001, 002, 003). The PR's fourth change
  (the `TextEdit`-side caching layer avoiding a full `Vector` copy on every
  `_get_line_syntax_highlighting` cache read) is a different file/mechanism entirely and remains
  untested - still in `IDEAS.md`.
