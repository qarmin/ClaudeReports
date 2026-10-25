# `load_resource_pack()`'s res:// directory indexing ignores `.gdignore`, unlike every other engine scan

Root-causes real, currently-open issue **godotengine/godot#122438** ("First `load_resource_pack()`
recursively indexes the entire project and can stall for seconds"). Found via this session's
once-per-workspace GitHub issue/PR inspiration sync, then verified independently by reading the
actual code path (not just trusting the issue report) and reproducing headlessly.

`ProjectSettings::_load_resource_pack()` (`core/config/project_settings.cpp`), the first time any
pack is loaded while running from source (editor or `--path`, not an exported build), unconditionally
registers a `PackedSourceDirectory` and calls `add_pack("res://", ...)` on it *before* even
attempting to load the actual requested pack file - this indexes the entire project resource tree
so direct filesystem access keeps working alongside the loaded pack. `PackedSourceDirectory::
add_directory()` (`core/io/file_access_pack.cpp`) recurses into every subdirectory with
`set_include_hidden(true)` and **no `.gdignore` check at all** - unlike `EditorFileSystem::
_should_skip_directory()` (`editor/file_system/editor_file_system.cpp:3482`), which has skipped
`.gdignore`-marked directories in the *editor's own* filesystem scan for a long time. This means a
`.git`, build-output, or any other huge non-resource directory a project author has already marked
with `.gdignore` for the editor's own scan gets walked anyway, file by file, the very first time
`load_resource_pack()` is called - independent of whether that call ever succeeds.

Fix: add the same `.gdignore` check `EditorFileSystem` already uses, in `add_directory()`'s
subdirectory recursion loop, before recursing into (or indexing files under) a subdirectory.

## What was tried

Harness: a synthetic repro (`bench/generate_repro_project.py`) - a bare project with a
`big_excluded_dir/` marked `.gdignore` containing 50 subdirectories x 300 empty files (15,000
files total, chosen to be a tractable but still realistic stand-in for the issue's own reported
47,203-file project/`.git` directory - not the reporter's literal project), plus an unrelated
`normal_dir/` with one real file to confirm normal resources are unaffected. Timed via
`bench/bench_pck.gd` (`Time.get_ticks_usec()` around a single `ProjectSettings.load_resource_pack
("res://nonexistent.pck")` call - the indexing happens regardless of whether the target pack path
is real, since it runs before the actual pack file is opened). `--headless --path <repro> --script
bench_pck.gd`, no display server needed.

Toolchain: SCons build (runtime-behavior measurement, not a compiled-artifact/disassembly claim -
no compiler version recorded, per the template's rule).

| Variant | `load_resource_pack()` time (single call, us) | notes |
|---|---|---|
| baseline (no `.gdignore` check) | 25937 / 23836 / 24437 (3 runs), mean 24736.7us | scans all 15,000 files under `big_excluded_dir/` despite its `.gdignore` marker |
| patched (`.gdignore` check added) | 83 / 81 / 80 / 69 / 67 / 67 / 66 / 64 (8 runs), mean 72.1us | `big_excluded_dir/` skipped entirely at the first `.gdignore` hit, only `normal_dir/` and top-level files are scanned |

## Result

**Dimension measured: wall-clock time of a single `load_resource_pack()` call.**

Mean time drops from 24736.7us to 72.1us - a **~343x reduction (-99.7%)** for this repro shape.
This is not a noise-floor-sensitive result: baseline runs (23.8-25.9ms) and patched runs (64-83us)
don't overlap by more than two orders of magnitude, and the mechanism (entirely skipping a
15,000-file/50-directory recursive walk) fully explains the size of the effect. The real-world
speedup for any given project scales with however many files exist under directories already
marked `.gdignore` (typically `.git`, `.godot`, build output, etc.) - the issue reporter's own
47,203-file project would see a proportionally larger absolute saving than this synthetic 15,000-file
repro.

## Does it change behavior/output?

- **Bit-exact?** N/A (not a computation whose output is hashed - a directory-scan-skipping change).
- **What actually changes**: files/subdirectories under a directory marked `.gdignore` are no
  longer added to `PackedData`'s in-memory pack-file index (`PackedData::files`/`PackedDir` tree)
  when `res://` is registered as a pack source. Verified with `bench/bench_pck_listing.gd` and
  `bench/bench_pck_verify.gd`: in this dev-mode/editor run configuration, **`FileAccess.file_exists()`
  and `DirAccess.get_files_at()` on files under the `.gdignore`-marked directory return identical
  results before and after the patch** (both fall through to real filesystem access rather than
  consulting the pack index in this mode - confirmed by running both scripts against baseline and
  patched builds). The only observed effect is the elimination of the unnecessary scan time (and
  the now-avoided `PackedData` index entries for that subtree) - no case was found where a
  legitimately-needed resource became inaccessible.
- **Is this a breaking change?** Narrow and consistent with existing engine behavior, not new: a
  project author who marks a directory `.gdignore` already expects the editor's own filesystem scan
  to skip it entirely (`EditorFileSystem`); this makes `load_resource_pack()`'s internal `res://`
  indexing consistent with that same, already-documented convention rather than introducing new
  semantics. The only way this changes behavior is if some project relied on `load_resource_pack()`
  specifically (not the editor scan, not direct file access) treating `.gdignore`-marked directory
  contents as part of the indexed pack tree - not observed in this session's testing, and not
  consistent with `.gdignore`'s documented purpose.
- Project test suite: 1427 cases, 1426 passed, 1 failed (known pre-existing `test_ip.cpp` IPv6
  failure, no IPv6 in this container), 3 skipped - identical before and after.

## Why the result looks the way it does

`add_directory()` was written as a plain unconditional recursive walk (`set_include_hidden(true)`,
no early-outs) with no awareness of the `.gdignore` convention that `EditorFileSystem` introduced
for exactly this purpose (skip non-resource directory trees) - the two code paths evolved
independently and only one of them got the exclusion rule. Since this indexing pass runs
unconditionally on the *first* `load_resource_pack()` call regardless of whether that call
succeeds, any project with a sufficiently large `.gdignore`-marked directory (most commonly `.git`
in a version-controlled project) pays this cost exactly once per process lifetime, in full, the
first time any dynamic pack is mounted - matching the issue's own observation that the stall
scales with total project file count rather than the pack's own size.

## What's not done / open questions

- **Not tested against the literal 47,203-file project** from the issue report - this session's
  15,000-file synthetic repro demonstrates the same mechanism and scaling behavior, but the exact
  absolute number from the original report was not reproduced 1:1.
- **`DirAccessPack`/`FileAccessPack`'s fallback-to-real-filesystem behavior in dev/editor mode**
  (which is what made `file_exists`/`get_files_at` show no observable difference in this session's
  testing) was not itself audited for whether an *exported build* (where there's no real filesystem
  fallback, only the loaded pack(s)) could observe a difference - this candidate's harness only
  exercises the dev/editor-mode code path (`_load_resource_pack`'s own comment: "Add the project's
  resource file system to PackedData so directory access keeps working when the game is running
  without a main pack, like in the editor or on Android" - i.e. this specific `res://`-as-pack-source
  registration is itself dev/editor/Android-only machinery, not exercised by a normal exported-PCK
  game boot). Worth a maintainer's second look before merging, though the mechanism strongly
  suggests it's scoped correctly (only affects the auxiliary `res://` directory source, never the
  actual requested pack file's own contents).
- Did not check whether other recursive directory walks in the engine (outside `EditorFileSystem`
  and this one) have the same missing `.gdignore` check - not searched exhaustively this session.
