# Empty-project editor startup: where the 716.5MB peak RSS actually goes

User-requested investigation: "opening even an empty project preallocates a lot of stuff (dialogs
etc.) - a lot of memory." Verified the hypothesis is broadly correct and precisely bisected which
phases of `EditorNode`'s constructor dominate, down to which of the 74 built-in `EditorPlugin`s
individually cost the most. Toolchain: SCons build (not a compiler-version-sensitive candidate -
no toolchain version recorded per the template's rule, this is a runtime-behavior measurement, not
a compiled-artifact/disassembly claim).

## What was tried

Harness: `bin/godot.linuxbsd.editor.x86_64 --headless --editor --path
___OPTIMIZATIONS___/harness/empty_project --quit-after 1`, `XDG_DATA_HOME`/`XDG_CONFIG_HOME` set to
writable dirs (see `README.md`). Peak RSS via `/proc/<pid>/status`'s `VmHWM` (last successful read
before the process exits - kernel-tracked, monotonic peak, no `/usr/bin/time` binary in this
container). Engine-internal tracked heap via temporary `print_line(vformat(...,
Memory::get_mem_usage()))` probes added at 17 points through `EditorNode::EditorNode()`
(`editor/editor_node.cpp`) - see `bench/instrumentation.diff` for the exact temporary diff (applied,
measured, then reverted - not a real fix, see "Does it change behavior/output?" below).

| Variant | Peak RSS (VmHWM) | Engine-tracked heap at end of `EditorNode()` ctor | notes |
|---|---|---|---|
| Editor, empty project (baseline, 3 runs) | 716472 / 716520 / 716480 kB (~716.5MB, <0.01% spread) | 323151271 B (~308.2MB) | `--editor --path <empty project> --quit-after 1` |
| Editor, empty project (re-verified after reverting instrumentation, 2 runs) | 717220 / 716764 kB | n/a (probes removed) | confirms instrumentation itself added no measurable overhead |
| Bare runtime, same empty project (calibration, NOT a fair baseline) | 54948 / 8520 kB | n/a | `--headless` (no `--editor`) exits in a few ms with `Error: Can't run project: no main scene defined` - barely initializes anything, so this is NOT "engine baseline vs. editor overhead", just confirms the empty project itself defines zero runtime content |

## Result

**Dimension measured: peak RSS (primary) and engine-internal tracked heap usage (`Memory::get_mem_usage()`, for attribution/bisection only).**

Peak RSS for editor startup on a maximally empty project: **~716.5MB**, reproducible to <0.01%
across runs. Of this, Godot's own allocator tracking accounts for **~308.2MB** by the end of the
`EditorNode` constructor (before the first frame draws) - the remaining **~408MB** is RSS the
engine's own `Memory::get_mem_usage()` counter does not attribute to anything (see "What's not
done" below - likely GPU/rendering-driver-side allocations, ICU/font data, shared library pages,
or third-party allocations that bypass Godot's wrapped allocator).

Bisecting the ~308.2MB engine-tracked total by `EditorNode()` constructor phase (17 checkpoints,
each a `Memory::get_mem_usage()` delta against the previous checkpoint):

| Phase | Delta | Cumulative | What it covers |
|---|---|---|---|
| ctor start -> before theme | +0.5MB | 45.98MB | early flags/menu bar skeleton |
| `EditorThemeManager::generate_theme()` | **+5.7MB** | 51.65MB | icon/theme generation |
| before theme -> before docks | **+83.7MB** | 135.32MB | main menu bar, toolbar, ~100+ `ED_SHORTCUT`/`ED_SHORTCUT_AND_COMMAND` registrations, `EditorCommandPalette` singleton creation - **not bisected further this session**, see "What's not done" |
| docks instantiation | **+33.8MB** | 169.13MB | `SceneTreeDock`, `FileSystemDock`, `ImportDock`, `InspectorDock`, `SignalsDock`, `GroupsDock`, `HistoryDock` |
| remaining dialogs/menus before plugin registration | +12.6MB | 181.71MB | ~40 `ConfirmationDialog`/`EditorFileDialog`/`PopupMenu` instances (see candidate context - individually small, this is their sum) |
| `CanvasItemEditorPlugin` | +14.4MB | 196.09MB | 2D editor |
| `Node3DEditorPlugin` | +10.6MB | 206.67MB | 3D editor |
| `ScriptEditorPlugin` | +12.6MB | 219.28MB | script editor |
| game view + text/shader editor registration + `AssetLibraryEditorPlugin` | +6.5MB | 225.82MB | |
| Animation plugins (Player/TrackKeyEdit/MarkerKeyEdit) | +12.1MB | 237.90MB | |
| VersionControl + AudioBuses plugins | +0.02MB | 237.93MB | negligible |
| **`EditorPlugins::create(i)` loop (74 built-in plugins)** | **+83.4MB** | 321.30MB | **bisected per-plugin below** |
| GDExtension plugins + tail of ctor | +1.9MB | 323.15MB | none loaded in this environment, expected ~0 |

Per-plugin breakdown of the 83.4MB `EditorPlugins::create(i)` loop (top 15 of 74, full data in
`bench/plugin_iter_full.txt`):

| Plugin | Memory delta at construction |
|---|---|
| `AnimationTreeEditorPlugin` | 11.49MB |
| `ThemeEditorPlugin` | 11.07MB |
| `ShaderEditorPlugin` | 9.26MB |
| `TileSetEditorPlugin` | 6.32MB |
| `SpriteFramesEditorPlugin` | 5.31MB |
| `MeshLibraryEditorPlugin` | 2.62MB |
| `GPUParticles2DEditorPlugin` | 2.53MB |
| `CPUParticles2DEditorPlugin` | 2.51MB |
| `MultiplayerEditorPlugin` | 2.46MB |
| `SceneExporterGLTFPlugin` | 1.98MB |
| `ResourcePreloaderEditorPlugin` | 1.96MB |
| `TileMapEditorPlugin` | 1.90MB |
| `LightmapGIEditorPlugin` | 1.72MB |
| `GPUParticlesCollisionSDF3DEditorPlugin` | 1.66MB |
| `VoxelGIEditorPlugin` | 1.66MB |

The top 5 plugins alone account for **43.45MB of the 83.4MB loop total (~52%)**, from just 5 of 74
plugins - the cost is concentrated, not evenly spread.

**Root cause identified for the #2 entry (`ThemeEditorPlugin`, 11.07MB):** its constructor
(`editor/scene/gui/theme_editor_plugin.cpp:4147`) unconditionally builds a full `ThemeEditor` dock
(`theme_editor_plugin.cpp:3994`), which unconditionally constructs a `DefaultThemeEditorPreview`
(`theme_editor_plugin.cpp:4102`, ~34-41 `memnew()` calls inside `ThemeEditorPreview`'s own
constructor in `editor/scene/gui/theme_editor_preview.cpp:238`) - a gallery of sample UI controls
(buttons, sliders, trees, etc.) used only to preview how a theme resource renders. This is built
**even though the dock starts closed** (`theme_editor->close()` is called immediately after, in
`ThemeEditorPlugin`'s own constructor) and even though an empty project has no theme resource open
to preview. The other 4 top entries (`AnimationTreeEditorPlugin`, `ShaderEditorPlugin`,
`TileSetEditorPlugin`, `SpriteFramesEditorPlugin`) were not root-caused to the same line-level
detail this session (see "What's not done") but share the same general shape on inspection: each
constructs a full dock/editor UI eagerly regardless of whether the corresponding resource type is
ever touched.

## Does it change behavior/output?

**N/A - measurement/characterization only, no patch applied to real source.** The temporary
`Memory::get_mem_usage()` probes used to produce the phase/plugin breakdown above were reverted
before finishing this candidate (`bench/instrumentation.diff` is kept for reference/reproducibility
only, not a patch to apply - see `SKILL.md`'s "patch.diff is mandatory..." section: this is
correctly the "pure measurement/investigation" state, not "a real fix to the tested project's own
source", so there is intentionally no `patch.diff` in this folder).

A bounded code fix (deferring `DefaultThemeEditorPreview`'s construction until the Theme dock is
first actually shown) was investigated but **not attempted this session** - see "What's not done"
for why.

- Project test suite: 1427 cases, 1426 passed, 1 failed (known pre-existing `test_ip.cpp` IPv6
  failure, this container has no IPv6), 3 skipped - identical before and after (before/after here
  meaning "with vs. without the reverted instrumentation", since no real patch was applied).

## Why the result looks the way it does

The ~408MB gap between peak RSS (716.5MB) and Godot's own tracked heap (308.2MB) is real and not
yet explained - `Memory::get_mem_usage()` only counts allocations routed through Godot's own
allocator wrapper (`core/os/memory.h`/`.cpp`); anything allocated by a third-party library directly
via the system allocator (rendering driver/GPU context even in a "headless"/dummy-renderer
configuration, ICU, FreeType, mbedTLS, glslang, etc.), shared library `.text`/`.data` pages, or
thread stacks (the editor spins up several `WorkerThreadPool` threads) would show up in RSS without
ever incrementing this counter. This session did not have time to attribute that ~408MB further -
see "What's not done".

Within the tracked 308.2MB, the two largest single blocks (menu/shortcut/command-palette setup at
+83.7MB, and the 74-built-in-plugin loop at +83.4MB) both fit a consistent story: Godot's editor
architecture instantiates essentially all of its UI machinery unconditionally at startup,
regardless of what the opened project actually contains or what the user will actually interact
with in a given session. This is architecturally deliberate in most of the codebase (a plugin needs
to exist to answer `handles()`/`edit()` calls when the user selects a matching node type later), not
an obvious bug - but for at least the `ThemeEditorPlugin` case, the dock content itself (not just
the dock shell) is built eagerly with no such justification, since a closed dock cannot be
`handles()`-queried by anything.

## What's not done / open questions

- **The ~408MB RSS-vs-tracked-heap gap was not attributed.** Worth a follow-up session with
  `valgrind --tool=massif` or a similar external heap profiler (not just the engine's own counter)
  to see whether it's dominated by the rendering driver, ICU/font data, or something else - `perf`
  alone (cycle-based sampling) is the wrong tool for this, an allocation/memory profiler is needed.
- **The +83.7MB "before theme -> before docks" block was not bisected further** (menu bar, ~100+
  shortcut registrations, `EditorCommandPalette::get_singleton()`). Individual `ED_SHORTCUT` calls
  are unlikely to be expensive; `EditorCommandPalette`'s own first-construction cost was not
  isolated - a natural next candidate for a future session, same bisection technique as used here
  (temporary `Memory::get_mem_usage()` probes, already proven to work cleanly in ~20s incremental
  rebuild cycles).
- **The other 4 of the top-5 plugins (`AnimationTreeEditorPlugin` 11.49MB, `ShaderEditorPlugin`
  9.26MB, `TileSetEditorPlugin` 6.32MB, `SpriteFramesEditorPlugin` 5.31MB) were not root-caused to
  the same line-level detail as `ThemeEditorPlugin`** - flagged in `IDEAS.md` for a follow-up session,
  since the pattern found in `ThemeEditorPlugin` (eager construction of preview/gallery content for a
  dock that starts closed) is a plausible hypothesis worth checking in each.
- **A bounded fix for the `ThemeEditorPlugin` finding was investigated but not attempted**, for a
  concrete correctness reason: `ThemeEditor::_change_preview_tab()`
  (`editor/scene/gui/theme_editor_plugin.cpp:3822`) indexes `preview_tabs_content`'s children
  directly by tab index (`preview_tabs_content->get_child(p_tab)`, with an
  `ERR_FAIL_INDEX_MSG(p_tab, preview_tabs_content->get_child_count(), ...)` guard). Deferring
  `DefaultThemeEditorPreview`'s construction (currently always child index 0, matching the
  "Default Preview" tab always at index 0) until the dock is first shown would leave this
  tab-index-to-child-index correspondence temporarily broken (tab exists, child doesn't) unless
  every code path that can trigger `_change_preview_tab(0)` before first-show is intercepted and
  made to construct the content synchronously first. This is a normal, boundable fix in principle
  (add a `_ensure_default_preview()` helper, call it from `_change_preview_tab` and from a
  `visibility_changed` signal connection), but verifying it doesn't break interactive dock/tab
  behavior would require actually clicking through the Theme dock in a running editor with a real
  display server - not possible to verify in this headless container, and the project's existing
  automated test suite does not exercise this interactive UI path at all (test suite passing
  before/after would be true but would not actually validate the fix). Left as a characterized,
  reasoned-through lead in `IDEAS.md` rather than shipped as an unverifiable patch.
- No attempt was made to check whether the ~43 dialogs/popups enumerated separately (the "remaining
  dialogs/menus" +12.6MB row) have any single outsized contributor - that block's cost is modest
  enough (12.6MB across ~40 objects, ~300KB average) that it looked like normal small-Control
  overhead rather than a concentrated bug, so it wasn't bisected per-object.
