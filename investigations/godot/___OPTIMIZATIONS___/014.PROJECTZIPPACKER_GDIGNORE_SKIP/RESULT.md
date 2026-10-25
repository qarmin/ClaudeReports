# `ProjectZIPPacker::_zip_recursive()` ignores `.gdignore`, same bug class as candidate 009

Found while searching `IDEAS.md`'s backlog item "search other recursive directory-walk call sites
in the engine for the same missing `.gdignore` check" (left open by candidate 009). A genuinely
different root cause from candidate 009 (a different function, different call path - "Project ->
Pack Project as ZIP" menu action / the Web editor's project download feature, not the first
`load_resource_pack()` call), but the exact same bug class: `ProjectZIPPacker::_zip_recursive()`
(`editor/export/project_zip_packer.cpp`) recurses into every subdirectory of `res://` with no
`.gdignore` check at all - only `.`/`..`/the project's own `.godot` data directory are skipped.
Unlike `EditorExportPlatform::_edit_files_with_filter()` in the same `editor/export/` area (checked
this session, already correctly calls `EditorFileSystem::_should_skip_directory()`), nothing skips
a `.gdignore`-marked directory (e.g. `.git`, a build-output folder) here - its entire contents get
walked, read, compressed, and included in the exported/downloaded zip.

Fix: add the same `EditorFileSystem::_should_skip_directory()` check candidate 009 used (and that
`_edit_files_with_filter()` already uses), in `_zip_recursive()`'s per-entry loop, before recursing
into (or zipping the entry as) a directory.

## What was tried

Harness: reused candidate 009's synthetic repro generator (`bench/generate_repro_project.py`) -
a bare project with `big_excluded_dir/` marked `.gdignore` (50 subdirs x 300 empty files = 15,000
files) plus a `normal_dir/` with one real file. Timed via a temporary scratch `TEST_CASE`
(`bench/scratch_bench_test.cpp`, added to `tests/core/io/` for this session, reverted before
finishing - not part of `patch.diff`) that manually calls `ProjectSettings::setup()` to point the
engine's resource path at the repro project (`--test` mode does not otherwise honor `--path` for
resource-path setup, unlike `--editor`/`--script`), then calls `ProjectZIPPacker::pack_project_zip()`
directly and inspects the resulting zip's entry list via `ZIPReader` to both time the call and
verify correctness (entries under `big_excluded_dir` vs `normal_dir`).

Toolchain: SCons build (runtime-behavior measurement, not a compiled-artifact/disassembly claim -
no compiler version recorded, per the template's rule).

| Variant | `pack_project_zip()` time (single call, us) | Zip contents | notes |
|---|---|---|---|
| baseline (no `.gdignore` check) | 1424491 / 1275780 / 1310266 (3 runs), mean 1336845.7us | 15055 entries: 15052 from `big_excluded_dir/`, 2 from `normal_dir/`, 1 `project.godot` | walks and compresses all 15,000 excluded files |
| patched (`.gdignore` check added) | 521 / 504 / 506 (3 runs), mean 510.3us | 3 entries: 2 from `normal_dir/`, 1 `project.godot` - `big_excluded_dir/` fully absent | `big_excluded_dir/` skipped entirely at the first `.gdignore` hit |

## Result

**Dimension measured: wall-clock time of a single `pack_project_zip()` call.**

Mean time drops from 1336845.7us to 510.3us - a **~2620x reduction (-99.96%)** for this repro
shape. Not a noise-floor-sensitive result: baseline runs (1.28-1.42s) and patched runs (504-521us)
don't overlap by more than three orders of magnitude, and the mechanism (skipping 15,000 files'
worth of read+compress work) fully explains the size of the effect - larger than candidate 009's
own ~343x, consistent with this path doing actual file I/O and zip compression per file rather than
just a directory-listing recursion. The real-world speedup scales with however many files exist
under `.gdignore`-marked directories in a given project (typically `.git`, `.godot`-adjacent
caches, build output).

## Does it change behavior/output?

- **Bit-exact?** N/A (not a computation whose output is hashed - a directory-scan-skipping change
  to a zip archive's contents).
- **What actually changes**: files/subdirectories under a directory marked `.gdignore` (or
  containing a nested `project.godot`) are no longer included in the zip produced by "Project ->
  Pack Project as ZIP" or the Web editor's project download. This is very likely the *more
  correct* behavior a project author would expect (a `.gdignore`-marked `.git`/build-output
  directory usually shouldn't end up inside a distributed project zip anyway, independent of the
  performance angle), but it is a real, user-visible content change to the exported zip's contents -
  flagged explicitly rather than silently bundled with the timing claim.
- **Is this a breaking change?** Only for someone specifically relying on the previous behavior of
  `.gdignore`-marked directories being included in the exported zip (an unlikely, unstated
  dependency - `.gdignore` is documented as an editor-scan-exclusion mechanism generally).
- Project test suite: 1427 cases, 1426 passed, 1 failed (known pre-existing `test_ip.cpp` IPv6
  failure, no IPv6 in this container), 3 skipped (4 skipped when this candidate's own scratch
  benchmark test case is present but excluded via `--test-case-exclude`) - identical before and
  after. The scratch benchmark test case itself is **not safe to run as part of the full suite**
  (it manually calls `ProjectSettings::setup()`, which conflicts with resource-path state left by
  an earlier, unrelated test when run in full-suite order - the same class of global-state leakage
  candidate 001's own scratch benchmark hit; confirmed **not** a regression from this patch by
  running it in isolation via `--test-case="*pack_project_zip*"`, where it passes cleanly and
  produces the numbers above).

## Why the result looks the way it does

Same root cause shape as candidate 009: `.gdignore` is an editor-filesystem-scan concept
(`EditorFileSystem::_should_skip_directory()`), and any *other* code path that independently walks
`res://` with its own hand-rolled `DirAccess::list_dir_begin()`/`get_next()` loop has to remember to
call it too - `EditorExportPlatform::_edit_files_with_filter()` remembered to; `ProjectZIPPacker::
_zip_recursive()` (and, per candidate 009, `PackedSourceDirectory::add_directory()`) didn't. This
is a real, recurring pattern in this codebase - candidate 009's own backlog item is now confirmed
to have a second live instance.

## What's not done / open questions

- **A systematic sweep of every other `list_dir_begin()`/`get_next()` recursive-walk call site**
  was done this session (see the full list checked: `editor_export_platform.cpp` twice - both
  ruled out, one already correct, one operates on a temp export-staging directory not `res://`;
  `project_converter_3_to_4.cpp`, `modules/gdscript/language_server/gdscript_workspace.cpp` -
  already has its own `.gdignore` check), but not exhaustively for every file in the earlier grep
  list (many are editor-tool-specific dialogs unlikely to walk large parts of `res://` - left
  unchecked as low-value, not flagged as a new backlog item since the search was reasonably
  thorough for anything walking the *whole* resource tree).
- Not verified against a real, large existing Godot project (only the synthetic 15,000-file repro) -
  the mechanism (skip a `.gdignore`-marked subtree entirely) is simple enough that this is a low
  concern, matching candidate 009's own reasoning.
