# What is under test, and how it is measured

Everything here is measured with the `perf_test` project one directory up. Read this first
if you are picking the work up cold - it explains the harness, then `SUMMARY.md` lists the
candidates.

## Workload

`perf_test` has three modes over the same generated data (see `../README.md`):

- `text` - one `TextInput` inside a `ScrollView` (a hand-rolled `TextEdit`), N lines of
  ~110 unique lorem characters.
- `list` - `ListView` (virtualized) over N records, row = `Rectangle` + `HorizontalLayout`
  + 3 `Text`.
- `scroll` - the same rows in a plain `for` inside a `ScrollView` (no virtualization).
- `rects` - a grid of rounded, bordered rectangles, all visible, moving every frame. Added for
  candidate 06: dominated by per-primitive renderer cost rather than by the item tree.

Modifiers added for candidates 13 and 14, each reaching a path no static scene does:
`--mutate <n> --mutate-mode changed|changed-offscreen|push|insert-front|rotate` (live model data,
which is what real list UIs do) and `--animate [--animate-count <n>]` (a per-item binding on the
animation clock). **`--animate` is time-driven and therefore not pixel-comparable** - the harness
says so on its own output line.

`tools/matrix.sh "label|args" ...` runs a set of configurations N times each and prints medians
with the spread, which is the shape most of these questions want.

Two modifiers cut across the `text` scene, both added for candidate 03 because the code paths they
reach behave differently and neither was reachable before: `--word-wrap` (what Slint's own
`TextEdit` defaults to) and `--resize-step <px>`, which re-lays the scene out at a new width every
frame instead of scrolling. A scene that only scrolls cannot show what a resize costs.

Content is generated to defeat caching: every line/record differs by an index prefix, a
rotated word window and a hash suffix.

## Measurement harness

Two entry points, both fill the mode with content, render N frames while scrolling
200 px/frame, print a per-frame table and exit:

| command | renderer | needs a display |
| --- | --- | --- |
| `perf_test --bench` | software (own `MinimalSoftwareWindow` platform) | no |
| `perf_test --frames N` | the selected real renderer (FemtoVG default) | yes, or `xvfb-run` |

Per frame it reports: wall time, allocations, text shapes, paragraph shapes, line breaks. Plus RSS
after the last frame and the fill cost. The summary lines are **medians** with the spread printed
alongside, not means - see the trap list below.

- **allocations** - wrapping global allocator (`src/alloc_stats.rs`, feature `alloc-stats`),
  two relaxed atomics per allocation.
- **text shapes / line breaks** - `TextLayoutCache::cache_miss_count()` /
  `layout_miss_count()`, exposed by `i-slint-renderer-software`'s `testing` feature. These count
  cache *entries* that had to be rebuilt, not the work.
- **paragraph shapes** - `TextLayoutCache::paragraph_shape_count()`, the parley layouts actually
  built. Since candidate 03 a large document shapes its paragraphs on demand, so one entry can
  account for a hundred thousand of these or for none; this column, not `text shapes`, is the
  ground truth for "how much shaping happened".
- **allocation attribution** - `--alloc-profile <n>` (plus `--profile-phase fill|show|frames`)
  samples 1 in n allocations of the chosen phase, prints a ranking of allocation *sites* and the
  heaviest stacks. It stores unresolved backtraces in a fixed static table and resolves symbols only in
  the report (`src/alloc_profile.rs`). Needed because `perf`, `valgrind` and `heaptrack` are
  all unavailable in this environment (`perf_event_open` is blocked even with
  `perf_event_paranoid=-1`).
- **pixel comparison** - `--snapshot <file.ppm>` writes the last frame, of a `--frames` run or of a
  headless `--bench` run. Prefer `--bench`: it needs neither a display nor GL nor
  `libxkbcommon-x11`, and its scroll offset is derived from the frame count, so two runs are
  byte-identical. Two traps, both hit during this session:
  1. The **status line** prints per-run timings, so its pixels always differ. Compare only the
     area above it (rows 0..820 at 1280x900) or use a scene without it.
  2. The scroll offset used to follow the 1 ms timer's tick count, so a faster build scrolled a
     different distance per frame and two binaries were never comparable (1.2 MB of 3.1 MB
     differing, for a change that turned out to be pixel-identical). `src/frames.rs` now derives
     the offset from the number of frames already rendered; two runs of the same binary are
     byte-identical above the status line, and so are two different binaries when the change is
     visually neutral.
  Always run the same-binary-twice control before believing a pixel difference. **Use
  `tools/pixelcheck.sh <binary A> <binary B> -- <scene args>`**, which runs each side N times, takes
  the pixels that vary *within* a binary as the noise floor, and reports only differences outside
  it. That rule was already written here in prose and was skipped anyway: candidate 33 credited an
  ablation with a 26x10 px "stale label" that was nothing but this status line, and the retraction
  is in `33_.../RESULT.md`. A rule that needs remembering is worth less than one a script applies -
  the tool also prints the noise floor, so a suspiciously large one is visible instead of silently
  masking a real change.
- **paired comparison** - `tools/paired.sh <A> <B> -- <scene args>` alternates the two binaries and
  reports the *pair count* alongside the medians. Use it for every before/after: this machine has a
  per-process performance mode (~18.3 or ~24.5 ms, never between, surviving `taskset`), so unpaired
  medians compare two lotteries. It also refuses to run when the two binaries are byte-identical,
  which is what a patch that failed to apply looks like.
- **/tmp is not durable** - it was wiped twice mid-session here, taking 400 MB comparison binaries
  *and* the saved copies of the sources being ablated with it. Keep both under `target/`, which is
  gitignored and lives as long as the checkout: `target/bins/` for binaries, `target/isolate_backup/`
  for `isolate.py`'s originals. Save an ablation as a `.patch` next to its write-up rather than as a
  copy in a scratch directory - it is then documentation and backup at once, and `git apply` puts it
  back.
- **swapping sources under cargo** - a before/after needs two binaries from two trees, and the
  obvious way to get them (save the tree, `git checkout --`, build, restore, build) silently fails:
  `cp -a` restores the original *mtimes*, which are older than the fingerprint cargo just recorded,
  so the second build prints `Finished` without compiling and leaves the **first** binary in place.
  Every measurement after that is of the wrong tree, and it looks like the change did nothing.
  `tools/gpu_before_after.sh` touches the sources after restoring; anything else that swaps trees
  has to as well. Symptom to watch for: a `cargo build` that finishes in ~0.2 s after you changed
  files.
- **the two harnesses measure different windows** - `--bench` charges a frame from before the
  property writes, so its per-frame numbers are layout + render; `--frames` measures
  `BeforeRendering` to `AfterRendering`, i.e. render only, because the gap between two renders is
  mostly vsync idle. A scene that shapes text during layout therefore reports 0 paragraph shapes
  per frame in the windowed report and 18 in the headless one (`list`), and that is not a bug in
  either. Compare like with like, and don't read a windowed `0` as "no work".
- **one frame can carry a whole row** - the reports used to average frames 1..N. On a real
  compositor a single frame renders before the window is fully mapped (observed as *exactly* one
  allocation per item missing: 33 164 against 36 164 for 3 000 rectangles) or stalls on it (one
  60 ms frame of nine turned a 2.42 ms row into 8.80 ms). Both reports now print the **median** and
  a `spread <min>..<max>` line; read the spread before believing the headline, and treat a row
  whose spread is wider than the effect you are measuring as unusable.
- **measure one fix at a time, and build the off-switch when you write the fix** - several fixes
  in one file (or one directory) means reverting the file measures a bundle. `tools/isolate.py`
  disables exactly one, by restoring the loop or call site it replaced. Reconstructing that switch
  afterwards means re-deriving the original code from a diff, which is how you end up comparing the
  wrong pair of binaries. A combined before/after is worth having *as well*, but it attributes
  nothing: this session's combined table showed a `list` "+21% regression" that no individual fix
  caused and that turned out to be noise.
- **a median is wrong for a bimodal workload** - the summary lines are medians because one
  outlying frame contaminates a mean, but a workload that alternates (a subtree created on even
  frames and destroyed on odd ones) has *two* modes and the median reports one of them. A scene
  allocating 19 491 on half its frames and 25 on the rest read as "25" and looked unremarkable.
  The `spread <min>..<max>` line is what catches it; read the per-frame table whenever the work is
  not the same every frame.
- **allocation counts do not survive across runs** - they are exactly reproducible *within* one
  invocation of a benchmark script (a control row of the same binary matches to the allocation)
  and not between invocations: the same binary gave 29 972 and 36 171 for `rects 3000` on two
  runs. A before/after therefore has to build and run both binaries in one invocation, with a
  repeated row of the same binary to show the floor. Numbers recorded in an earlier session are
  history, not a baseline.
- **the baseline binary has to build against the pristine tree** - a before/after script reverts
  `internal/core` and rebuilds, so every counter or API the harness calls has to be stubbed out for
  that build. Adding instrumentation to core and using it in `perf_test` breaks the pristine build
  until a rule is added to `tools/gpu_before_after.sh`. It fails loudly, which is the good case;
  the bad case is stubbing it to something that silently changes what is measured.
- **verifying the two binaries differ** - before trusting a "no difference" result, print one
  number that is *known* to differ (here: resident memory on `text` 100 000, 543 MB vs 38 MB).
  A comparison of a binary against itself passes every check.
- **allocator cost** - `just allocbench` (`tools/allocbench.rs`): 34 ns per
  malloc+free pair of a 32-byte box, glibc, for turning allocation counts into milliseconds.

## Environment

- FemtoVG under Xvfb needed `libxkbcommon-x11-0` and `libxcb-xkb1`, which are **not installed
  here**. The measurements were taken with those two libraries borrowed from an opencv-python
  wheel:
  `LD_LIBRARY_PATH=/tmp/xkbstub:<venv>/opencv_python.libs` with `/tmp/xkbstub` holding
  `libxkbcommon.so.0`, `libxkbcommon-x11.so.0` and `libxcb-xkb.so.1` symlinked to the wheel's
  hashed copies. After installing the two packages, `just frames-xvfb` works without any of
  that.
- Machine: this one. Linux 7.0.0-28-generic, glibc malloc, `rdebug` profile
  (release + `debug = "full"`, no debug assertions).
- FemtoVG runs under `xvfb-run` with Mesa llvmpipe (`just frames-xvfb`), so its
  rasterization is on the CPU; traversal/layout/shaping/allocation numbers are unaffected.
  Needs `libxkbcommon-x11-0` and `libxcb-xkb1` installed.
- Slint: this working tree (1.18.0), femtovg from `../femtovg`.

## Ground rules used here

1. No claim without a command that reproduces it. Every candidate folder has the numbers
   and the exact command.
2. Every candidate that touches Slint gets a real `patch.diff` against this tree, applied
   and built for real, plus the repo's own tests where they cover the code.
3. Null results are written down with the same care as wins - a null result with no file is
   indistinguishable from "never checked".
