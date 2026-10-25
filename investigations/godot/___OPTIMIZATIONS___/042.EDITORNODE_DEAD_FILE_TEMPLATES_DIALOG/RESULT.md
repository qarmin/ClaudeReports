# 042. `EditorNode::file_templates` is built at every editor startup and can never be shown

- **Project**: godotengine/godot
- **Base commit**: `9552dfb6859a1aaba1e570b8e0ef5c599b830f19`
- **Files**: `editor/editor_node.cpp`, `editor/editor_node.h` (**+0/-11 lines**, pure deletion)
- **Metric(s) measured**: Godot's tracked peak heap (primary) and peak RSS (secondary)
- **Workload**: empty-project editor startup,
  `--headless --editor --path harness/empty_project --quit-after 1`

## What was tried

Candidate 024 noticed in passing that `EditorNode::file_templates` - an `EditorFileDialog` titled
"Import Templates From ZIP File" - is constructed, configured, parented and signal-wired at every
editor startup but never shown from anywhere, and flagged it as worth reporting upstream on its own.
This candidate verified and measured that.

The claim holds. `grep -rn file_templates` over the entire repo returns **only**:

- its declaration (`editor/editor_node.h:413`),
- its construction and configuration block (`editor_node.cpp:9413-9420`),
- one `connect("file_selected", ...)` (`editor_node.cpp:9444`).

There is no `popup()`, `popup_file_dialog()`, `popup_centered*()` or `show()` call on it anywhere, so
the dialog cannot ever appear, and therefore its `file_selected` signal can never fire either.

**How it was orphaned**: `.tpz` template import is now owned by `ExportTemplateManager`, which has
its own `_tpz_file_selected()` handler and `tpz_button` ("Install from a TPZ file."). The last commit
to touch a `file_templates->popup` call is **b5a06cecff "Export template management dialog. Missing
download (need to discuss this!)"** - i.e. that dialog took over the job and left this one behind.

The fix is deletion, not deferral: there is no call site to defer to.

## Result

**Tracked peak heap: -2286416 bytes (-2.18 MB, -0.51%)**, 448282957 -> 445996541 B. Byte-identical
across 3 runs per arm.
Tracked live heap at `Main::cleanup()`: -329948 B (-0.31 MB), also byte-identical.

**Peak RSS: -2.50 MB (-0.36%)**, 701.23 -> 698.73 MB mean, 6 interleaved pairs, patched lower in
**5/6** (the one exception is a baseline run that landed in the harness's known low mode - see
candidate 035). RSS/tracked ratio 1.15x, so the saving reaches the process.

This is the best MB-per-line result in the workspace after candidates 023 and 029: **11 deleted
lines, no added lines**, for ~2.2-2.5MB.

## Does it change behavior/output?

**No, and uniquely for this workspace it cannot.** Every other memory candidate here defers
construction, which leaves real behavioral caveats (translation timing, internal child counts, the
first-use cost). This one removes an object that no code path can reach, so there is no flow to
break: no popup call exists, so the dialog was never visible and its only signal could never fire.

**Test suite, both arms, identical**: 1427 cases, 1426 passed, 1 failed, 3 skipped (the known
`test_ip.cpp` IPv6 failure).

**Xvfb startup count** (candidate 037's counting plugin, both arms): startup `EditorFileDialog`
**64 -> 63**, exactly the one instance removed, confirming both that it really was being built and
that nothing else changed.

## Why the result looks the way it does

~2.2MB of tracked peak for one `EditorFileDialog` is in line with candidate 036's figure of ~2.0MB
per *configured* `EditorFileDialog` (filters, an `OptionButton` in the dialog's vbox, and in this
case a title and an access mode), and above candidate 021's 1.45MB for a bare `FileDialog`
construction - this one sets a file mode, an access mode and a `*.tpz` filter.

## What's not done / open questions

- **Whether upstream would rather re-wire it than delete it** is a judgment call this file does not
  make. If the intent was ever for `EditorNode` to offer template import directly, the fix would be
  to add the missing popup call instead - but `ExportTemplateManager` already provides that UI, so
  the dialog is duplicated functionality either way.
- Nothing was checked about the other `EditorFileDialog` members of `EditorNode`; candidate 024
  deferred four of them and deliberately left `file` (the general scene open/save dialog) eager.
- The same "constructed but never shown" audit has **not** been run across other editor classes -
  this instance was found incidentally. `grep`ing for members that are `memnew`'d and connected but
  never popped would be a cheap systematic sweep, and is recorded in `IDEAS.md`.
