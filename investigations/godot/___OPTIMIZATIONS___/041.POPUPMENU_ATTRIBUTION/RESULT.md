# 041. The ~148 "unattributed" startup PopupMenus: 64 are already fixed, the rest is a thin tail

- **Project**: godotengine/godot
- **Base commit**: `9552dfb6859a1aaba1e570b8e0ef5c599b830f19`
- **Metric(s) measured**: instance counts and ownership (no timing/memory patch measured)
- **Workload**: real editor startup under Xvfb on a bare project
- **No `patch.diff`**: measurement/attribution only, no patch applied to real source. This candidate
  exists to bound a lead, not to change code.

## What was tried

Candidate 027's widget census counted **520 `PopupMenu` instances / 49.02MB** at startup and noted
that 372 belong to `MenuButton`/`OptionButton`, whose owners populate the popup immediately at
construction (so deferring only re-times the allocation - candidate 015's null-result shape). That
left **~148 instances (~14MB at ~96.5KB each)** as an open lead in `IDEAS.md`, with the 120 explicit
`memnew(PopupMenu)` sites known by file but never counted per instance or attributed to owners.

Following the lesson candidate 032 wrote into the playbook - attribute before grinding through call
sites - this walked the real startup tree under Xvfb and printed each `PopupMenu`'s ancestor class
chain, with no rebuild or C++ instrumentation needed. Full output in
`bench/attribution_popupmenus.txt`, harness in `bench/attribution_plugin.gd`.

## Result

513 `PopupMenu` instances at startup (candidate 027's headless counter said 520; this is a real
Xvfb editor session, close enough to confirm the figure). By direct parent:

| direct parent | count | assessment |
|---|---|---|
| `MenuButton` | 213 | populated at owner construction - not addressable (candidate 027) |
| `OptionButton` | 152 | same |
| **`EditorFileDialog`** | **64** | **already deferred by candidate 029** (`FileDialog::item_menu`) |
| `PopupMenu` (submenus) | 31 | menu structure, built and populated with the parent menu |
| `MenuBar` | 5 | the main menu bar itself |
| everything else | 48 | long tail across ~30 owner classes, 1-4 each |

So of the ~148 "other" popups the lead was about:

- **64 (~6.2MB) are `FileDialog::item_menu`** - the right-click context menu candidate 029 already
  defers (029 measured -8.63MB, more than the 6.2MB this census implies, because its cost includes
  the menu's own nested children). This lead was largely **already harvested** before it was
  written down.
- **31 are submenus of other popups** and 5 are the `MenuBar`'s own menus: menu structure, populated
  when the parent menu is built, so deferring them is candidate 015's null-result shape.
- **48 (~4.6MB) are a genuine long tail**, spread across ~30 owner classes at 1-4 instances each:
  `Node3DEditorViewport` 4, `TileAtlasView` 3, `Tree` 3, `ScrollContainer` 3, `VBoxContainer` 3,
  then `AnimationNodeStateMachineEditor`, `EditorAudioBus`, `SceneTreeDock`, `FileSystemDock`,
  `DocumentList`, `CanvasItemEditor` at 2 each, and ~20 classes with exactly 1.

**There is no second systemic `PopupMenu` cluster.** The one that existed was `FileDialog`'s, and it
is fixed. At ~96.5KB per instance, no remaining owner class is worth more than ~0.4MB, against the
audit surface of a separate correctness review per class - which is exactly the ratio the playbook's
rule 2 says to reject.

Control counts from the same run, matching candidate 027's census closely (it used a headless
constructor counter, this walks a live Xvfb tree): `LineEdit` 1171 (027: 1184), `ItemList` 238 (238),
`TextEdit` 113 (118), `Tree` 110 (113).

## Does it change behavior/output?

N/A - nothing was patched. No test-suite comparison applies.

## Why the result looks the way it does

`PopupMenu` is cheap individually (~96.5KB) and expensive only in aggregate, so it only pays to
attack it where one owner is instantiated many times. `FileDialog` was that owner (64 instances, one
context menu each), which is why candidate 029 got -8.63MB from a +15/-3 line change. Every other
owner in the editor exists once or twice, so the same fix applied to any of them returns ~0.1-0.4MB.

This also re-confirms the in-codebase precedent worth citing upstream: `LineEdit` and `TextEdit`
already build their context menus lazily via `_generate_context_menu()`, which is why 1171 `LineEdit`
instances cost ~0.5KB each in candidate 027's census rather than ~96KB.

## What's not done / open questions

- The 48-instance tail was **not** audited per class, deliberately - see above for why. If a future
  session wants one of them anyway, `Node3DEditorViewport` (4) and `TileAtlasView` (3) are the
  largest, and `bench/attribution_popupmenus.txt` has the full ancestor chain for each.
- The 3 popups parented directly to `Tree` are a different thing from candidate 028's `Tree` inline
  cell editor (that is a `Popup` + `LineEdit` + `TextEdit` cluster, 8.20MB across 110-113 Trees, and
  remains the largest single unfixed widget-level prize with its blockers documented in 028).
- This measures a **bare project**; a project with assets, scripts and a populated scene tree would
  add more of the long-tail owners (each editor dock that exists once still exists once, but
  per-item context menus could multiply).
