# GDScript syntax highlighter: cache `color_regions[c]` into a local reference

Same function as candidate 001 (`GDScriptSyntaxHighlighter::_get_line_syntax_highlighting_impl()`,
`modules/gdscript/editor/gdscript_highlighter.cpp`), a different, independent hunk of the same
already-open PR [godotengine/godot#123320](https://github.com/godotengine/godot/pull/123320): the
`for (int c = 0; c < color_regions.size(); c++)` loop re-indexes `color_regions[c]` up to eight
separate times per iteration (`.start_key` twice, `.end_key`, `.is_string`, `.r_prefix`, `.type`,
`.line_only` twice, `.color` twice) instead of caching the element once into a local reference.
Tested in isolation from candidate 001 (on top of a pristine, unpatched tree - candidate 001's
`sort()` fix was NOT applied here) to attribute any measured effect to this change alone.

Toolchain: `scons` 4.11.1, GCC 15.2.0, `platform=linuxbsd target=editor tests=yes debug_symbols=yes`.

## What was tried

Same scratch benchmark as candidate 001 (`bench/scratch_benchmark.patch`, identical to candidate
001's - copied here for this candidate's own self-containment), same 8000-char synthetic line, same
`--test-case="*SCRATCH syntax highlighting*"` invocation, 5 runs of 10 iterations each. `uptime`
load average 0.89-1.75 for all runs.

| Variant | 8000 chars, us/iter (5 runs) |
|---|---|
| baseline (unpatched, same numbers as candidate 001's baseline row) | 23939.7, 23795.9, 20181.7, 19947.0, 21323.6 - mean 21837.6 |
| patched (`color_regions[c]` cached into a local reference) | 25733.9, 23974.8, 20212.5, 21724.0, 20219.9 - mean 22373.0 |

## Result

**Time.** Mean 21837.6us -> 22373.0us - a **+2.5% difference, within the noise floor** established
by the run-to-run spread of the baseline samples themselves (19947.0 to 23939.7, a ~4000us/~18%
range on the baseline alone - larger than the entire measured "difference" between variants). Not
a real regression, not a real win - no change. Peak RSS/allocation count not measured (this
change's mechanism - avoiding repeated array indexing - has no plausible allocation-count effect).

## Does it change behavior/output?

- **Bit-exact?** Yes. Caching `color_regions[c]` into a `const ColorRegion &` reference and reading
  through it instead of re-indexing `color_regions[c]` each time is a pure refactor with no
  semantic change (the underlying `Vector<ColorRegion>` isn't mutated inside the loop, so the
  reference stays valid for the whole iteration).
- **Is this a breaking change?** No.
- Project test suite (`--headless --test`, `XDG_DATA_HOME` set, scratch benchmark excluded via
  `--test-case-exclude="*SCRATCH*"`): **1427 test cases, 1426 passed, 1 failed, 3 skipped** - the
  same one pre-existing `test_ip.cpp` IPv6 failure, identical before and after.

## Why the result looks the way it does

`Vector<T>::operator[]` in Godot is a plain pointer-plus-bounds-check read (no COW forking happens
on a const/read access - forking is only triggered by a *mutating* access) - so re-indexing
`color_regions[c]` several times per loop iteration costs, at most, a handful of extra pointer
dereferences and array-bounds comparisons per symbol/operator character in the line. That's
completely dwarfed by the loop's other work on the same iteration - the character-by-character
`start_key` comparison, the `str.strip_edges().split_spaces(1)` call for `TYPE_CODE_REGION`
candidates, and (in the unpatched baseline used here) the `color_map.sort()` call candidate 001
found and removed. Unlike candidate 001's fix, this doesn't remove a term that grows with input
size - it just removes a small, already-cheap, constant-factor cost, which is exactly why the
measured difference disappears into run-to-run noise instead of showing up as a real percentage
improvement.

## What's not done / open questions

- Not re-tested on top of candidate 001's fix (i.e. both patches applied together) - possible the
  combined effect is still just "no change" for the same reason (the remaining O(n log n) sort cost
  and O(n) scan cost would still dominate), but not verified. Low priority given this fix alone
  already shows nothing.
- The `Vector<ColorRegion> color_regions` -> `LocalVector<ColorRegion>` type change (also part of
  PR #123320's same hunk) was deliberately NOT included in this candidate (kept isolated per "one
  fix per candidate") - `LocalVector` avoids the `Vector`/`CowData` indirection entirely (a
  non-shared, no-refcount container), which is a different mechanism than "cache one lookup per
  iteration" and could plausibly show a different (still probably small) result. Left in `IDEAS.md`
  as its own untested lead if a future session wants to check it.
