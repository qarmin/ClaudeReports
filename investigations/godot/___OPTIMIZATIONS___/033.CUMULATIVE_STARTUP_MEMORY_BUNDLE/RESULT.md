# How much of this workspace's startup-memory wins are additive: 8 patches stacked and measured together

Thirteen candidates in this workspace each report a peak-RSS reduction at empty-project editor
startup, and every one of them was measured **alone against pristine `HEAD`**. Their individual
deltas therefore cannot be summed: several of them touch the same startup path, and two of them
attack the *same objects* from opposite sides - candidates 023/029/030 make each `FileDialog`
cheaper, while candidates 024/025/026/031/032 stop some of those dialogs being created at all. A
saving on an object that another patch deletes is counted twice in any naive total.

This candidate measures the stack instead of reasoning about it: apply the patches together to one
tree, one build, and bench that against the same baseline binary, interleaved.

**8 of the 13 are in the bundle: 007, 013, 017, 023, 025, 026, 031, 032.** The other five (018,
019, 024, 029, 030) were left out for a purely mechanical reason: each `patch.diff` was generated
against pristine `HEAD`, and 017/018/019/024 all edit the same ~30 lines of `EditorNode`'s
constructor and member block, while 023/029/030 all edit the same region of
`scene/gui/file_dialog.h`. `git apply` refuses the later ones (and `git apply -3` resolves
them only with conflict markers), so stacking them would have meant hand-merging five patches -
work that would change the diffs being measured. Their exclusion is noted in the accounting below.

## What was tried

Harness: `bin/godot.linuxbsd.editor.x86_64 --headless --editor --path
___OPTIMIZATIONS___/harness/empty_project --quit-after 1`, same as every startup-RSS candidate
here. Peak RSS via `/proc/<pid>/status`'s `VmHWM`, every run's exit code checked.

`bench/bundle_of_8_patches.diff` is the exact combined diff that was built and measured (20 files,
+492/-208), captured with `git diff` after applying the 8 `patch.diff` files in ascending candidate
order, and re-checked with `git apply --check` against pristine `HEAD`. Each individual patch also
still applies cleanly on its own; this file exists so the measured artifact is reproducible without
re-deriving the stack.

**Interleaved A/B/A/B measurement** (`bench/rss_interleaved_ab.txt`): baseline x2, bundle x2,
repeated 3 times, so any machine drift during the measurement hits both arms equally. This was
needed because the run happened while an unrelated build was finishing on the same machine.

| Variant | Peak RSS (VmHWM) | notes |
|---|---|---|
| baseline (pristine `HEAD`) | 716804.0 kB mean = **700.00MB** (716488/717228/717772/716200/716244/716892, 6 runs, 0.22% spread) | all exit 0 |
| bundle of 8 patches | 591504.7 kB mean = **577.64MB** (590824/591868/591480/592108/591404/591344, 6 runs, 0.22% spread) | all exit 0 |

Distributions don't overlap (baseline min 716200 > bundle max 592108).

Toolchain: SCons build (runtime-behavior measurement, not a compiled-artifact/disassembly claim -
no compiler version recorded, per the template's rule).

## Result

**Dimension measured: peak RSS only.**

**The 8 patches together reduce peak RSS by 122.36MB (-17.48%)**, from 700.00MB to 577.64MB.

The naive sum of the same 8 candidates' individually-measured deltas is **142.81MB**, so the stack
is **85.7% additive** - a 20.45MB (14.3%) shortfall from double-counting:

| Candidate | Individual delta | What it defers |
|---|---|---|
| 007 | -7.85MB | `ThemeEditor` preview gallery |
| 013 | -15.52MB | 4 `AnimationTreeEditor` sub-editors |
| 017 | -13.81MB | `EditorAbout` |
| 023 | -46.39MB | `FileDialog`'s 5 nested sub-dialogs (x64 instances) |
| 025 | -11.21MB | `LocalizationEditor`'s 5 `EditorFileDialog`s |
| 026 | -6.70MB | `ProjectExportDialog`'s 3 `EditorFileDialog`s |
| 031 | -3.85MB | `EditorAutoloadSettings`' 2 `EditorFileDialog`s |
| 032 | -37.48MB | 4 `ScriptCreateDialog`s + 4 `ShaderCreateDialog`s |
| **sum** | **-142.81MB** | |
| **measured together** | **-122.36MB** | **85.7% of the sum** |

**Where the 20.45MB of overlap comes from is directly observable**, not inferred: the startup
`EditorFileDialog` count under the bundle is **42, down from 64**
(`bench/verification_bundle_startup_counts.txt`), because 013 removes 4, 025 removes 5, 026 removes
3, 031 removes 2 and 032 removes 8. Candidate 023's saving is *per surviving instance*, so with 22
of the 64 instances gone it can only apply to 42 of them - and 46.39MB x (42/64) = 30.4MB, i.e.
~16MB less than 023 measured alone. That single interaction accounts for about three quarters of the
total shortfall; the remainder is the same effect on the smaller per-instance savings.

## Does it change behavior/output?

- **Bit-exact?** N/A (UI/interactive-behavior changes) - the 8 candidates' own RESULT.md files each
  carry their own verification with a baseline control. What this candidate adds is the
  **integration** check that stacking them does not break anything the individual runs would miss:
  - Project test suite on the bundle build: 1427 cases, 1426 passed, 1 failed (known pre-existing
    `test_ip.cpp` IPv6 failure, no IPv6 in this container), 3 skipped -
    identical to pristine `HEAD` and to each patch measured alone (`bench/test_suite_bundle.txt`).
  - Interactive re-run of candidate 032's verification plugin against the **bundle** build under
    `Xvfb`: all three real entry points still work (`ScriptEditor.open_script_create_dialog()`,
    the `ClassDB`-bound `EditorPlugin.get_script_create_dialog()`, and the File -> "New Shader..."
    menu signal), each creating exactly one dialog on demand with the same titles as pristine
    `HEAD`, on top of a tree where 22 file dialogs and both create-dialog families are now absent
    at startup.
- **Is this a breaking change?** This candidate introduces no new source change of its own.
- **patch.diff: N/A** - no new fix is proposed here. `bench/bundle_of_8_patches.diff` is a measured
  artifact (the concatenation of 8 already-recorded `patch.diff` files), not a new candidate patch,
  which is why this row is `N/A` in `SUMMARY.md` and absent from `SUMMARY-SHORT.md`: **its 122.36MB
  must not be added to any other row, and it must not be counted as a 19th win.**

## Why the result looks the way it does

Every patch in the bundle removes constructor-time allocations from the same one-shot startup path,
and none of them is on a hot loop, so there is no reason for them to interact except where they
target the same objects - which is exactly what the 14.3% shortfall consists of. The 85.7% figure
is high because most of the bundle attacks *disjoint* owners (a theme editor's preview gallery, an
about dialog, animation sub-editors, three different settings tabs' file dialogs); it is not 100%
because candidate 023 deliberately attacks the shared `FileDialog` class that the others delete
instances of.

Read the other direction, this is the strongest argument for candidate 023's approach: even after
five separate call-site candidates deleted 22 of the 64 dialogs, the shared per-instance fix still
contributes ~30MB, because 42 dialogs remain and no realistic amount of call-site grinding removes
them all.

## What's not done / open questions

- **The five excluded candidates (018, 019, 024, 029, 030) were not measured in a stack.** Adding
  them would need the `EditorNode` and `file_dialog.h` patches hand-merged. Their naive sum is a
  further 36.21MB, but 029 and 030 are per-instance `FileDialog` fixes and so would be discounted
  by the same 42/64 factor as 023 (predicting ~+10.3MB rather than +15.64MB from those two), while
  018/019/024 target disjoint owners and should be close to fully additive (~+20.6MB). A
  fully-merged 13-patch bundle is therefore predicted at roughly **-153MB (~22% of startup peak
  RSS)** - a prediction from this candidate's own model, not a measurement.
- Only peak RSS was measured. Startup *time* was not: every patch in the bundle removes work from
  startup, so the direction is presumably favourable, but this workspace never established a
  reliable startup-time harness (the `--quit-after 1` shape measures process lifetime dominated by
  the editor's own first-frame work, and the run-to-run spread swamped the effect in an early
  attempt).
- The bundle was not run against a *real* project (only the empty-project harness), so the figures
  describe the fixed startup cost, not a session with scenes/scripts open.
