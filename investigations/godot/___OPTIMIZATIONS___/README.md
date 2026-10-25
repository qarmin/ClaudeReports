# What is under test, and how it is measured

Read this first if picking the work up cold - it explains the harness, `SUMMARY.md` lists the
candidates measured with it.

## Workload

Godot Engine (C++, SCons build) has no bundled benchmark suite - only a doctest-based unit test
suite (`tests/`). No demo-content repo is checked out here (that's a separate repo,
`godotengine/godot-demo-projects`). Discovery/harness work for this workspace therefore starts by
building small, headless, script-driven or C++ doctest-based micro-benchmarks that isolate one
engine subsystem at a time (String/Array/Dictionary/Variant COW behavior, math library, GDScript
VM dispatch, regex, JSON, scene tree/signal overhead, physics step) - see `harness/` once one
exists, and extend it (new mode/flag) rather than multiplying one-off benches, per SKILL.md's
"Bench harnesses: grow a few, don't multiply".

| mode/scene | what it stresses |
|---|---|
| `harness/gdbench/bench.gd` modes (`string_concat`, `string_format`, `array_append`, `array_cow_pass`, `dict_insert`, `packed_array`, `vector_math`, `variant_call`, `regex`, `json_stringify`, `json_parse`, `node_instance`, `signal_emit`, `if_logic_baseline`, `if_logic_and`, `if_logic_or`, `if_logic_nested`, `resource_set_path`) | GDScript-level core container/Variant/String micro-costs, run via `--headless --script bench.gd -- <mode> [<mode> ...]`. The four `if_logic_*` modes (added 2026-09-11 for candidate 048) time a raw `for` loop directly around the `if` statement (no `_timed()` closure wrapper - a closure call's own overhead would dominate the few-ns effect being measured), matching PR godotengine/godot#120660's own benchmark methodology; `if_logic_baseline` is the empty-loop control to subtract per-iteration loop overhead from the other three. `resource_set_path` (added 2026-09-11 for candidate 049) creates a fresh `Resource` and assigns a unique `resource_path` each iteration (counter-based, never a path collision) to measure `Resource::set_path()`'s no-collision-case cost. `json_from_native_packed` (added 2026-09-11 for candidate 050) times a single `JSON.from_native()` call on a 1,000,000-element `PackedVector3Array` (not amortized over many iterations, matching this operation's own single-call O(n) shape). `chained_operator_call` (added 2026-09-11 for candidate 052) calls a dedicated `_chained_add()` function (10-operand chained `+`) 5,000,000 times to measure GDScript function-call/stack-frame setup cost. `get_path_to_deep`/`get_path_to_cross_branch` (added 2026-09-11 for candidate 053) build a 300-node-deep chain / two 150-node-deep branches under the real `SceneTree` root and time `Node.get_path_to()` between the endpoints. `get_property_list` (added 2026-09-11 for candidate 054) times `Node2D.get_property_list()`. `while_logic_and`/`while_logic_or` (added 2026-09-11 for candidate 055) time a 3-operand `and`/`or` `while` loop condition over 20,000,000 iterations. `dict_string_key_read`/`dict_string_key_write`/`dict_string_key_contains` (added 2026-09-11 for candidate 056) time fixed-string-key `Dictionary` operations over 5,000,000 iterations. `callable_bindv_large`/`callable_get_bound_arguments_large` (added 2026-09-11 for candidate 057) time `Callable.bindv()`/`get_bound_arguments()` on a 2000-element `Array`, 20,000 iterations |
| `harness/gdbench/packed_scene_instantiate_probe.gd` (+ `prop_list_node.gd`) | Added 2026-09-11 for candidate 051 - `prop_list_node.gd` is a `@tool` `Node` script overriding `_get_property_list()` with 100 `Dictionary` properties (PR godotengine/godot#120100's own "Scenario B"); the probe packs one into a `PackedScene` and times 5000 `instantiate()`+`free()` calls. Correctness companions `packed_scene_correctness_test.gd`/`_test2.gd` (property-value equality + cross-instance no-aliasing, incl. the editor-only `GEN_EDIT_STATE_INSTANCE` mode) live in that candidate's own `bench/` folder, not here, since they're one-shot checks rather than reusable bench modes |
| `harness/gdbench/textedit_probe.gd` | `TextEdit::insert_text()` cost vs. line length (confirmed O(n) per edit, dominated by `TextServerAdvanced` full-line reshaping - see "Instrumentation" below; not a quick-fix candidate, see `IDEAS.md`) |
| scratch `TEST_CASE` in `modules/gdscript/tests/gdscript_test_runner_suite.h` (candidate 001's `bench/scratch_benchmark.patch`) | `GDScriptSyntaxHighlighter::get_line_syntax_highlighting()` cost vs. line length - used because `GDScriptSyntaxHighlighter` is editor-only and cannot be `.new()`'d from a GDScript script (`ClassDB` blocks scripting-side instantiation of editor-only classes), so a C++-level doctest `TEST_CASE` was the only way to reach it directly |
| `harness/gdbench/projection_determinant_probe.gd` | `Projection.determinant()` cost - a normal bound method, reachable directly from GDScript (no editor-only restriction like the syntax highlighter above) |
| `harness/gdbench/object_instance_timing_probe.gd` | `Object.new()`/`.free()` cost - deliberately uses plain `Object`, not `Node` (`bench.gd`'s existing `node_instance` mode uses `Node`, which overrides `_uses_signal_mutex()` to skip the heap-allocated signal mutex entirely - it cannot show candidate 005's fix) |
| `harness/gdbench/object_instance_memory_probe.gd` | `Object.new()` memory cost via `OS.get_static_memory_usage()` delta (deterministic, no `/usr/bin/time`/RSS needed - see Environment below) |
| `harness/gdbench/physics_area_gravity_probe.gd` | Real `SceneTree`/`_physics_process` physics stress test - 2000 `RigidBody2D`s inside one gravity `Area2D`, us/frame over a 300-frame window after a 10-frame settle. Added 2026-09-11 to check `GodotBody2D::integrate_forces()`'s cost (candidate 048's discovery pass, `areas.sort()` lead - checked and ruled out, see `IDEAS.md`) - reusable for any future physics-step candidate needing a real multi-body scene rather than a single direct API call |
| `harness/gdbench/many_nodes_process_probe.gd` (+ `mover_empty.gd`/`mover_whole_read.gd`/`mover_component_read.gd`/`mover_simple.gd`/`mover.gd`) | Real `SceneTree`/`_process` steady-state cost of 5000 `Node2D`s, parametrized by which of the 5 small mover scripts each node runs (pass the mover filename as the first `--` cmdline arg, default `mover.gd`). Added 2026-09-11 for the native-property-read finding in `IDEAS.md` (`OPCODE_GET_NAMED` vs `OPCODE_GET_NAMED_VALIDATED`) - **must call `OS.set_low_processor_usage_mode_sleep_usec(0)` before measuring** (already done in the probe's own `_initialize()`), or every measurement silently reads ~6900us/frame regardless of actual script cost (see the probe's own header comment for why - `OS::add_frame_delay()`'s headless-mode branch) |
| `harness/empty_project/` (bare `project.godot`, no scenes/scripts) | Editor startup peak RSS on a maximally empty project, via `bin/godot.linuxbsd.editor.x86_64 --headless --editor --path harness/empty_project --quit-after 1` + `VmHWM` polling (see Environment below); also the target of candidate 006's temporary `Memory::get_mem_usage()` checkpoint probes through `EditorNode`'s constructor for phase/per-plugin attribution |

## Measurement harness

Two entry points are available for anything reachable from GDScript or engine init:
- `bin/godot.linuxbsd.editor.x86_64 --headless --script <bench>.gd` - runs a GDScript file
  headless, no window/display server needed. Use `Time.get_ticks_usec()` around the timed loop,
  `print()` the result, discard warmup iterations per usual discipline.
- `perf record -g -- bin/godot.linuxbsd.editor.x86_64 --headless --script <bench>.gd` then
  `perf report --stdio` / `perf script` - real profiling is usable in this environment
  (`perf_event_paranoid` is `-1`), so prefer profiling a representative GDScript workload over
  guessing hot spots by inspection alone when a harness exists to profile.

For lower-level C++-only code with no natural GDScript entry point, add a scratch `TEST_CASE` with
manual timing to a `tests/` TU temporarily (never commit it for real - revert before finishing the
candidate, same as any other source change) or a standalone small `main()` using `{ path =
"../clone" }`-equivalent - just linking the relevant `.a` static libs from `bin/obj/` - if that's
lighter than a full engine binary link.

## Instrumentation added for this workspace

- `harness/gdbench/` (`project.godot` + `bench.gd` + two scratch probe scripts): a minimal headless
  GDScript project, invoked via `bin/godot.linuxbsd.editor.x86_64 --headless --path
  harness/gdbench --script bench.gd -- <mode>`. `bench.gd`'s `_timed()` helper discards a 10%
  warmup fraction and reports mean us/iteration; each `bench_*` function is a standalone mode
  selected by a string argument after `--` (`OS.get_cmdline_user_args()`).
- **`debug_symbols=yes` is required for any `perf`-based profiling to resolve real function names**
  - without it the default build is fully stripped (confirmed via `nm`: "no symbols") and `perf
    report` shows raw addresses instead of demangled C++ symbols. Add `debug_symbols=yes` to the
    normal `scons ... target=editor tests=yes` build command whenever profiling is needed this
    session; it roughly doubles binary size (~170MB -> ~2.3GB) and the FIRST build after adding the
    flag is a full rebuild (not incremental, ~7-8 minutes on 22 threads) since it changes every
    TU's compile flags - subsequent single-file changes stay fast (~15-40s incremental) same as
    without the flag.
- **`GDScriptSyntaxHighlighter` (and other editor-only `GDREGISTER_CLASS`es) cannot be
  `.new()`'d from a GDScript script even in a `--headless --script` run of the editor binary** -
  `ClassDB::_instantiate_internal` rejects it ("can only be instantiated by editor"). Worked around
  by adding a temporary C++ `TEST_CASE` directly (candidate 001's approach - see its
  `bench/scratch_benchmark.patch`), which uses `memnew`/`Ref<>::instantiate()` directly and bypasses
  the scripting-only gate entirely. This is the general pattern for reaching any editor-only C++
  class from this workspace's benchmarks going forward.
- Candidate 006 added 17 temporary `print_line(vformat("PERFPROBE <label> %d",
  Memory::get_mem_usage()))` checkpoints through `EditorNode::EditorNode()`
  (`editor/editor_node.cpp`) to bisect editor-startup memory growth by construction phase, plus a
  per-iteration variant inside the `EditorPlugins::create(i)` loop
  (`print_line(vformat("PERFPROBE plugin_iter %d %s %d", i, plugin->get_class(), delta))`) to
  attribute cost to each of the 74 built-in editor plugins individually. Reverted before finishing
  the candidate (see `006.../bench/instrumentation.diff` for the exact diff, kept for
  reproducibility only - not a real patch, this is a measurement-only candidate). Incremental
  rebuild after editing just this one file: ~20-22s, cheap enough to iterate the checkpoint
  placement several times in one session.
- Investigated (but did NOT turn into a candidate/patch) whether `TextEdit`'s per-edit cost on a
  very long single line comes from *redundant* re-shaping calls: added a temporary static call
  counter inside `TextServerAdvanced::_shaped_text_update_breaks` (reverted immediately after, never
  part of any patch) and confirmed exactly ONE real reshape happens per `insert_text()` call (a
  second call correctly short-circuits via its own `line_breaks_valid` cache). The O(n)-per-edit
  cost is the inherent, architectural cost of this text server's full-line-reshape design, not an
  avoidable redundant call - see `IDEAS.md` for why this was left as a characterized dead end rather
  than a candidate folder.

**Interactive (Xvfb) verification of deferred-construction candidates** (added 2026-09-10, used by
candidates 013/017/018/019). Deferred-construction fixes can't be validated by the headless test
suite - the code path only runs when a user actually opens the deferred UI. The reusable pattern:
a throwaway `@tool` `EditorPlugin` autoload in a scratch project (never committed) that

1. recursively searches `EditorInterface.get_base_control()` for the target `PopupMenu` by name
   (e.g. `"Help"`, `"Project"`, `"Editor"`), or for a submenu via
   `get_item_submenu(idx)` + `get_node(path)`;
2. finds the menu item by `get_item_text(i)` substring match (avoids hardcoding internal C++
   `MenuOptions` enum values, which aren't exposed to GDScript);
3. emits that menu's own `id_pressed`/`index_pressed` signal with `get_item_id(i)`/the index - the
   exact signal path a real click takes, so it runs the real `EditorNode::_menu_option[_confirm]`
   handler;
4. finds the resulting dialog by recursively collecting `Window`-typed nodes under
   `get_tree().root` and matching on `.title`/`get_class()` - **not** by screenshotting the main
   viewport, since an editor dialog is its own `Window` node and does not appear in the main
   viewport's texture;
5. screenshots that window's own `get_texture().get_image()` and saves it into the candidate's
   `bench/` folder as the visual evidence.

**Scratch `TEST_CASE` benches for C++-only APIs** (used by candidates 008/014/020). When the code
under test isn't reachable from GDScript (`ProjectZIPPacker::pack_project_zip()`,
`TextEdit::_notification(NOTIFICATION_DRAW)`), add a temporary `TEST_CASE` under `tests/` (globbed
automatically by `tests/SCsub`, so a new file anywhere under it is picked up - no manual
registration), run it in isolation with `--test-case="<pattern>"`, and verify the real suite
separately with `--test-case-exclude="<pattern>"`. Two traps found the hard way: (a) `--test` mode
does **not** initialize `ProjectSettings`' resource path from `--path` the way `--editor`/`--script`
do, so such a bench must call `ProjectSettings::setup()` itself; (b) a bench that does call
`setup()` is order-dependent and can fail inside the full suite (stale resource path from an
earlier test) while passing perfectly in isolation - always check both ways before reading anything
into a failure count.

## Reproducibility bugs found by actually running `just test` (2026-09-10)

`just test` was run end-to-end for real, in a scratch location, for candidate 018 (chosen as
representative: 5 candidates share its clone+harness-copy+RSS-bench shape). It found **three real
bugs that affected most candidates' justfiles** - every one of them would have made `just test`
fail for any reader who tried it. All are fixed across the workspace now; listed here because the
same mistakes are easy to reintroduce:

1. **`repo_url` used the SSH form** (`git@github.com:godotengine/godot.git`), which fails with
   `Host key verification failed` for anyone without GitHub SSH keys and a trusted host key -
   including this project's own dev container. Fixed to the HTTPS form
   (`https://github.com/godotengine/godot.git`), which needs no auth for a public repo. Applied to
   all 20 candidate justfiles.
2. **The bench recipes ran `./bin_baseline` / `./bin_patched` after `cd clone`**, but the build
   recipes copy those binaries to `{{justfile_directory()}}` (the candidate folder), not into
   `clone/` - so the bench step died with `No such file or directory` (exit 127). Fixed to invoke
   them via the absolute `"{{justfile_directory()}}/bin_baseline"` path. Affected 13 candidates.
3. **The `VmHWM` polling loop was not `set -e`-safe.** `V=$(grep VmHWM /proc/$PID/status 2>/dev/null)`
   followed by `[ -n "$V" ] && PEAK="$V"` returns non-zero whenever `V` is empty (the process
   exited between the `kill -0` check and the `grep`, or `VmHWM` isn't published yet on the first
   poll), and the recipes run under `set -euo pipefail`, so the whole recipe aborted - timing
   dependent, which is exactly why it survived interactive testing and only failed inside the
   recipe. Fixed to `V=$(... || true)` plus an `if` instead of `&&`. Affected the 6 RSS-polling
   candidates (006/007/013/017/018/019).

Two useful side facts from the same run: a **full from-scratch Godot build on this machine takes
~5 minutes** (not the 15-25 min the justfile headers estimate - those headers are conservative and
were never corrected), and an incremental rebuild after applying a single-file patch takes ~1
minute. So a full `just test` for one candidate is roughly 8-12 minutes wall-clock including the
~1.7GB clone, not the 30-50 min implied by the headers.

**Green end-to-end confirmation (2026-09-10, candidate 004).** After those three fixes, a second
candidate's `just test` was run through to completion to confirm the pipeline actually works rather
than just that the known bugs were gone: fresh clone -> baseline build -> baseline bench -> `git
apply patch.diff` -> patched build -> patched bench, **exit 0 with numbers from both arms**
(`004.PROJECTION_DETERMINANT_COFACTOR/just_test_run.log`, ~8 minutes wall-clock, confirming the
timing note above). Candidate 004 was chosen as the other justfile shape in this workspace (the
GDScript-harness/throughput shape, `harness/gdbench`, rather than the RSS-bench shape candidate 018
covered), so between the two runs both shapes have now been executed for real.

The reproduction also produced a measurement lesson, recorded in that candidate's RESULT.md: the
**patched** arm reproduced tightly (0.0146 vs 0.0159 us/call) but the **baseline** arm came out
0.0232 us/call against the 0.0199 originally recorded, i.e. the reproduced win was -36.8% where the
RESULT.md claims -20.1%. The recorded claim was left at the conservative figure. Do not compare a
fresh arm against a figure from an earlier session on this container - the spread across sessions
(~17 percentage points here) is far larger than the within-session spread.

## Environment

- **`/usr/bin/time` is NOT installed in this container** (only the shell builtin `time`, which
  can't report memory) - two working alternatives depending on what's being measured:
  - For a *GDScript-internal* memory delta (e.g. "how much does allocating N objects cost"), use
    Godot's own `OS.get_static_memory_usage()` called before/after from GDScript. Fully
    deterministic run-to-run in that workload shape (confirmed identical across 3 repeat runs in
    candidate 005).
  - For the *whole process's* peak RSS (e.g. "how much memory does starting the editor use"),
    poll `/proc/<pid>/status`'s `VmHWM` in a tight loop (`sleep 0.02`) while the process runs in
    the background; the last successful read before it exits is the peak (kernel-tracked,
    monotonic, no need to compute a running max yourself) - see candidate 006's `RESULT.md`/
    `justfile` for the exact snippet. Confirmed extremely low noise (<0.01% spread across repeat
    runs) for editor-startup-shaped workloads.
- Running inside Docker (Ubuntu 26.04, 24 cores, 29 GiB RAM). `$HOME` (`/home/rafal`) is
  **read-only** in this container except for the project directory itself
  (`/home/rafal/Projekty/Obce/godot`, on a separate writable mount). Godot's default `user://` data
  dir lives under `$HOME/.local/share` by XDG convention, which is not writable here - **every**
  invocation of the built binary (tests, benches, anything that touches `user://`) must set
  `XDG_DATA_HOME` to a writable path, e.g.:
  `export XDG_DATA_HOME=/home/rafal/Projekty/Obce/godot/tmp/xdg_data` (create the dir first).
  Without this, `OS::ensure_user_data_dir()` fails to create the app_userdata dir, logging opening
  fails, and (as observed directly in this session) the full `--test` run then **crashes with
  SIGSEGV** in `tests/core/io/test_logger.cpp` instead of failing gracefully - this looked like a
  real defensive-coding gap worth a quick look for `___BUGHUNT___/`, not just an environment quirk
  to route around silently (see note below).
- **Running `--editor` (not just `--test`) also needs `XDG_CONFIG_HOME` set**, separately from
  `XDG_DATA_HOME` above - the editor writes to `$HOME/.config/godot` (editor settings, feature
  profiles), equally unwritable here. Without it: `ERROR: Could not create editor config
  directory` and a failed editor-settings save on every run (non-fatal, but noisy and means
  editor settings never persist between runs, which could confound anything that depends on them).
  Set both: `export XDG_DATA_HOME=.../tmp/xdg_data XDG_CONFIG_HOME=.../tmp/xdg_config` (create
  both dirs first).
- No IPv6 in this container (no `ip` tool even present, no `::1` loopback). This makes exactly one
  known, pre-existing, environment-caused test failure:
  `tests/core/io/test_ip.cpp:44: CHECK( "0:0:0:0:0:0:0:1" == String(IPV6) )`. Expected baseline with
  `XDG_DATA_HOME` set correctly: **1427 test cases, 1426 passed, 1 failed** (that same IPv6 one),
  424066 assertions, 423066 passed / 1000 failed (all attributable to that one test case's
  sub-assertions in a loop). Any candidate's before/after `--test` run should show this exact same
  single known failure in both - a *different* failure appearing is a real regression; a *new*
  passing IPv6 test would be surprising and worth double-checking the environment didn't change.
- Build must use `tests=yes` to get a `--test`-capable binary - `scons -j<N> platform=linuxbsd
  target=editor tests=yes`. This is the **same binary** used for both benchmarking and correctness
  checks in this workspace (no separate non-test build) - `tests=yes` only adds `tests/` TUs and a
  test-runner entry point gated behind the `--test` flag; it does not change production code paths,
  so it's safe to use for timing too.
- `scons` is not preinstalled - `pip3 install scons --break-system-packages` (installs to
  `~/.local/bin`, must be on `PATH`). All other Linux/X11/Wayland dev headers (X11, Xcursor,
  Xinerama, Xi, Xrandr, Wayland, ALSA, PulseAudio, udev, Mesa GL/GLU, OpenSSL, zlib, FreeType) were
  already present in this Docker image - a from-scratch clone build only needs the `scons` pip
  install as an extra step, nothing else.
- Incremental rebuild after a small source change (bin/obj/ cache warm): ~35 seconds to 2 minutes
  on 22 threads. A from-scratch clone build (candidate justfile's clone-based repro path): **~4-5
  minutes measured on this machine** (2026-09-10, twice) - the "15-25 minutes" in the justfile
  headers is a conservative estimate for slower hardware, not what this container does.
- Full `--test` run: ~50-55 seconds wall clock (with `XDG_DATA_HOME` set).
- `perf` is installed and usable without extra privileges (`perf_event_paranoid=-1`).
- `gh` is authenticated (account `qarmin`) - the GitHub issue/PR inspiration sync
  (`gh-issues-prs-check/fetch_issues.py`) can run normally.

## Ground rules used in this workspace

1. No claim without a command that reproduces it - every candidate folder has the numbers and the
   exact command.
2. Every candidate that touches real source gets a real `patch.diff`, applied and built for real,
   plus `--headless --test` (with `XDG_DATA_HOME` set) before and after - compare against the
   known baseline (1426/1427 passed, the one IPv6 failure) rather than expecting 100%.
3. Null results are written down with the same care as wins - a null result with no file is
   indistinguishable from "never checked".
4. If a later candidate changes the baseline enough that an earlier candidate's numbers no longer
   hold, don't silently rewrite that candidate's `RESULT.md` - add a short `CORRECTION.md` in its
   folder explaining what changed and why, and update its `SUMMARY.md` row to point at it.
