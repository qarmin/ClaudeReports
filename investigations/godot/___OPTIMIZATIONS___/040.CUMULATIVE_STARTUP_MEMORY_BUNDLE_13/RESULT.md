# 040. All 13 startup-memory candidates stacked: -150.4MB peak RSS (-21.4%), 84.0% additive

- **Project**: godotengine/godot
- **Base commit**: `9552dfb6859a1aaba1e570b8e0ef5c599b830f19`
- **Metric(s) measured**: peak RSS (`VmHWM`) and Godot's tracked peak heap
  (`get_mem_max_usage()`)
- **Workload**: empty-project editor startup,
  `--headless --editor --path harness/empty_project --quit-after 1`
- **No `patch.diff` of its own**: this candidate measures a *stack of other candidates' patches*,
  not a new fix. The stacked diff is `bench/bundle_of_13_patches.diff` (20 files, +636/-279); every
  hunk in it belongs to candidates 007, 013, 017, 018, 019, 023, 024, 025, 026, 029, 030, 031 and
  032, which own their individual measurements. Nothing was applied to real source beyond this
  measurement, and no `APPLIED_FIXES.*` was produced - consolidating winners for real application
  is opt-in per `SKILL.md` and was not requested.

## What was tried

Candidate 033 stacked 8 of the startup-memory candidates and measured -122.36MB (85.7% of the naive
sum), but had to **exclude 5** because patches generated separately against pristine `HEAD` collide
as patch text: 018, 019 and 024 all edit `EditorNode`'s constructor and member block, and 029 and
030 both edit `scene/gui/file_dialog.h` alongside 023. It predicted **~-153MB (~22%)** for the full
13 and left the hand-merge as the top backlog item.

This candidate did that merge and measured it. The merge is real work, not a formality: `git apply
-3` refuses without a matching index and `git apply --reject` leaves the conflicting hunks behind,
so each of the 5 had its rejected hunks hand-applied - in every case the *accessor definition*, the
*eager-construction removal* and the *header declaration*, which conflicted only because the bundle's
own 017/023 hunks had already changed the surrounding context lines. No semantic conflict existed
between any two candidates.

## Result

**Peak RSS: -150.36 MB (-21.41%)**, 702.32 -> 551.97 MB mean, 8 interleaved A/B pairs, bundle lower
in **8/8** pairs with completely non-overlapping distributions (baseline 718624-719840 kB, bundle
564292-565592 kB). All 16 runs exit 0.

**Tracked peak heap: -113.58 MB (-26.57%)**, 448282957 -> 329184798 B mean.

**Additivity: 84.0%.** The 13 candidates' individually-measured RSS deltas sum to 179.02 MB
naively; 150.36 MB is what they actually deliver together, a shortfall of 28.66 MB. Candidate 033's
8-patch figure was 85.7%, so adding the 5 remaining patches lowered additivity slightly, exactly as
its double-counting explanation predicts.

**Candidate 033's prediction was nearly exact**: it extrapolated ~-153MB (~22%) and the measured
result is -150.36MB (-21.41%).

**The RSS/tracked ratio is 1.32x**, in line with the 1.23-1.31x seen in candidates 035 and 036 - so
here the tracked-heap saving does reach the process and then some, unlike candidate 037 where the
two metrics moved in opposite directions.

## Does it change behavior/output?

**Full test suite, both arms identical**: 1427 cases, 1426 passed, 1 failed, 3 skipped; 424066
assertions, 423066 passed, 1000 failed. The single failure is the known `test_ip.cpp` IPv6 check.
This is the integration test the individual candidates cannot provide - 13 independently-developed
deferrals interacting in one build, including three that all rewrite `EditorNode`'s constructor and
three that all rewrite `FileDialog`.

**Xvfb startup node counts** (candidate 037's counting plugin, run against both arms):

| | baseline | bundle of 13 |
|---|---|---|
| `EditorFileDialog` | 64 | **35** |
| `ConfirmationDialog` | 400 | **165** |
| `EditorHelpSearch` | 1 | 1 |
| firing the real "New Text File..." item | opens `New Text File...` | opens `New Text File...` |

The `ConfirmationDialog` count more than halves because candidate 023 removes 5 nested sub-dialogs
from every surviving `FileDialog` instance while the per-call-site candidates remove 29 of the
instances themselves. `EditorHelpSearch` stays at 1 because candidate 037 is deliberately not in
this bundle (see below).

Each individual candidate's own behavior-change notes still apply unchanged and are not restated
here; the translation-timing and internal-child-count caveats from candidates 023 and 037 are the
notable ones.

## Why the result looks the way it does

The 28.66 MB shortfall is **observable, not arithmetic**, and it is the same mechanism candidate 033
identified: candidate 023 is a *per-instance* fix (it removes 5 sub-dialogs from every `FileDialog`)
while 024, 025, 026, 031 and 032 are *per-call-site* fixes (they remove `FileDialog` instances
outright). Measured alone against pristine `HEAD`, candidate 023 applies to all 64 startup instances;
inside this bundle only 35 instances remain, so its contribution scales to roughly 46.39 x 35/64 =
25.4 MB - about 21 MB less than its standalone figure, which is roughly three-quarters of the total
shortfall. Disjoint owners (007's `ThemeEditor`, 013's `AnimationTreeEditor`, 017/018/019's editor
dialogs) stack essentially perfectly.

**Candidate 037 was deliberately excluded** even though it is a startup-memory candidate of the same
family, because its own measurement showed a peak-RSS *regression* (+2.90MB) alongside its
tracked-heap win; folding it in would have mixed a known-contradictory result into the headline
number. It is the obvious next thing to add to this stack once its RSS behaviour is understood
(see `IDEAS.md`).

## What's not done / open questions

- **The bundle's tracked peak is no longer byte-deterministic** (1.94 MB spread over 3 runs), where
  every individual candidate's was exact. With this many dialogs deferred the tracked peak moves to
  a later, timing-sensitive phase of startup. Not investigated; it matters because candidate 035's
  "one run per arm is enough" recipe silently stops holding for heavily-patched trees.
- The `-150.36MB` figure is for an **empty** project. A real project loads assets, so both the
  absolute baseline and the achievable saving will differ.
- Startup **time** was not measured for the bundle. Deferring work should not slow startup and
  several candidates plausibly speed it up slightly, but that is an assumption here, not a result.
- 54 of the 66 `memnew(EditorFileDialog)` call sites remain eager (35 instances still built at
  startup), so this is not a ceiling - see `IDEAS.md`'s remaining per-call-site tail.
