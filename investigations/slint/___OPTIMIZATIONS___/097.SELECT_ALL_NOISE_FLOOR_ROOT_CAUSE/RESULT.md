# 97 - Root-cause the `--select-all` + `--type-text` rendering noise-floor jump

This is an **investigation candidate, not a performance candidate**: no new timing was measured.
It follows up on the top-of-backlog `IDEAS.md` entry added while building candidate 096, which
found that the `--select-all` scenario's own pre-093 baseline (binary "A" = 091+092) had a
run-to-run pixel noise floor that jumped from a few hundred px to 3600-3700px "starting around 150
edited frames", spanning rows that include the scrollbar-thumb row - large enough that it could
mask a real, smaller divergence the way candidate 093's own regression was invisible at
`tools/pixelcheck.sh`'s default `--frames 6`. The leading suspect going in was a wall-clock-driven
cursor blink or selection-highlight animation that `--bench` mode doesn't freeze.

## What was tried

1. **Ruled out the cursor-blink hypothesis with direct instrumentation**, not inference. Added a
   temporary `eprintln!` (behind `SLINT_DEBUG_BLINK`, see `blink_trace_diagnostic.diff`) to
   `TextCursorBlinker::start`'s `toggle_cursor` closure in `internal/core/input.rs` (the only place
   that ever flips the blink state), plus a per-frame wall-clock timestamp print in
   `perf_test/src/bench.rs`'s frame loop. Ran `--mode text --count 10000 --type-text --select-all
   --scroll-step 0 --frames 150` and the same without `--select-all`, both on pristine `master`.
   **Zero blink toggles fired in either run.** Reading `TextInput::key_event`'s regular-character
   branch (`internal/core/items/text.rs:1147`) explains why: every typed character calls
   `show_cursor()`, which restarts the blink timer to fire 500ms from *now* - and since
   `--type-text` dispatches one keystroke every single frame, the timer never gets 500ms of
   uninterrupted real time to actually fire, in either scenario. This candidate's own
   `internal/core/input.rs` change was reverted before finishing (see "Reproduce"); it is not part
   of `patch.diff`, kept only as `blink_trace_diagnostic.diff` for reproducibility of this step.
2. **Found the real mechanism by direct pixel inspection at higher repeat counts.** Comparing 4-8
   repeated `--bench` snapshots of the exact same binary/flags (candidate 096's own sample size)
   reproduces only a few hundred differing px, matching what 096 reported for other scenarios. Raising
   to **20 repeats** reproduces the full 3685-3718px magnitude 096 found for `--select-all` - but
   critically, it reproduces at the **identical** magnitude and in the **identical** rows (839-851,
   plus an unrelated pair at 796-797) for the **plain `--type-text` scenario with no `--select-all`
   at all**, and at **`--frames 6`** (the harness's own default), not just at 150+ frames. See the
   table below.
3. **Identified the pixel content directly**: cropping rows 835-855 of two differing runs (see
   `status_label_run1.png` / `status_label_run2.png` in this folder) shows readable text -
   `"+10000 lines: generated in 1.28ms, property set in 936.10µs - buffer 10000 lines / 1.11 MB
   (layout and rendering happen on the next frame)"` in one run, `"...generated in 1.22ms, property
   set in 827.32µs..."` in the other. This is `perf_test/src/logic.rs`'s `on_add_lines` handler
   (`app.set_status(format!("+{count} lines: generated in {build:.2?}, property set in
   {apply:.2?}..."))`) - a status label that is **always present** in the `text` scene (set once
   during the initial content fill, before frame 0, regardless of any later flag), and that
   necessarily differs every process run because it prints real measured wall-clock durations. This
   is the *same* status-label noise `tools/pixelcheck.sh`'s own header comment and this workspace's
   `README.md` already document ("the perf_test scenes contain a status label that prints a
   measured duration... a 15x10 px block... differs between two runs of the same binary") - just
   undersampled at RUNS=4-8, and never swept to the repeat count needed to see its full footprint.

| Scenario | Frames | Repeats | Differing px (self-noise) | Rows |
|---|---|---|---|---|
| `--select-all` (pristine master, no candidate patches) | 150 | 4 | 364 | 840-849 |
| `--select-all` (091+092, candidate 096's own "A" tree) | 150 | 8 | 444 | 840-849 |
| `--select-all` (091+092) | 150 | **20** | **3706** | 839-851 |
| `--select-all` (091+092) | 300 | 6 | 376 | 796-797, 840-849 |
| plain `--type-text`, no `--select-all` (091+092) | 150 | **20** | **3710** | 796-797, 839-851 |
| plain `--type-text`, no `--select-all` (091+092) | **6** (harness default) | **20** | **3715** | 839-851 |

The magnitude and the affected rows are identical whether or not `--select-all` is present and
regardless of `--frames` (6 through 300) - it scales with how many distinct timing strings get
sampled (repeat count), not with the scenario. **`--select-all` was never actually the cause; it
was a sampling-variance coincidence in candidate 096's own 4-6-repeat measurements** that this
label's noise happened to show up larger there than in the `--resize-step` scenario checked
alongside it.

4. **Implemented and verified a harness-only fix**: blank the `status` property immediately after
   the fill phase completes, before the first frame renders (`perf_test/src/bench.rs`, see
   `bench_status_blank.diff` - `app.set_status(Default::default())` right after
   `let fill = fill_start.elapsed();`). This is a `perf_test`-only change (the harness, not the
   product itself, kept applied per this workspace's "Documenting the harness" convention rather
   than reverted, and recorded in `README.md`'s instrumentation table).

| Build | Scenario | Frames | Repeats | Differing px after the fix |
|---|---|---|---|---|
| pristine master + fix | `--select-all` | 30 | 15 | **0** |
| 091+092 + fix | `--select-all` | 150 | 20 | **0** |
| 091+092 + fix | plain `--type-text` | 150 | 20 | **8** (rows 796-797, see below) |

## Result

The reported "select-all makes the noise floor jump" effect does not exist as a distinct
phenomenon: it is the harness's own always-present, timing-embedding status label, whose full
noise footprint (~3700px) only becomes visible once enough repeated runs are sampled (empirically
somewhere between 8 and 20 with this label's actual text-width variance) - present identically in
every `text`-mode scene tried, `--select-all` or not, `--frames 6` or `--frames 300`. Blanking that
label right after the fill phase in `--bench` mode removes it entirely: 0px differing across 15-20
repeats in both the pristine-master and 091+092 trees, for both `--select-all` and plain
`--type-text`.

A separate, much smaller (8px across 20 repeats, rows 796-797) residual remains in the plain
`--type-text` scenario after the fix - see "What's not done" below.

## Does it change behavior/output?

- **Bit-exact?** N/A to the product itself - no `internal/core` (or any other tracked source) file
  was changed by this candidate; `patch.diff` is empty. The only change is to `perf_test`'s own
  benchmark harness (`perf_test/src/bench.rs`, untracked, not part of the Slint product), which now
  blanks the `status` label in `--bench` mode after the fill phase. This makes `--bench` snapshots
  more deterministic than before, it does not change what the product renders for any real
  application - the `status` label and its real-timing text are `perf_test`'s own UI, not
  Slint's.
- **Is this a breaking change?** No - no public API, and no product behavior changed.
- Project test suite (`i-slint-core`'s own unit tests; not affected since no tracked source
  changed): `cargo test -p i-slint-core --all-features --lib`: **319 passed, 0 failed**, run once
  against the pristine (unmodified) tree - both "before" and "after" for this candidate, since it
  makes no product-source change to compare against. The temporary `blink_trace_diagnostic.diff`
  instrumentation used in step 1 above was reverted before this run and is not part of the tree
  state being reported here.

## Why the result looks the way it does

`perf_test`'s `text` scene generates its initial 10000 lines through `Logic.on_add_lines`, which
measures its own generation time (`Instant::now()`/`elapsed()`) and writes both numbers into a
user-visible `status` string, purely for interactive/manual use of the windowed app (so a human
running `perf_test` can see how long filling the document took). `--bench` mode reuses this exact
same fill code path ("Shared by the windowed app and the headless benchmark so both measure exactly
the same code" per `README.md`), so the label - and its real-time-dependent text - ends up baked
into every headless snapshot too, at a fixed row near the bottom of the window regardless of scene
or mode. `tools/pixelcheck.sh`'s own header comment already names this exact label as a known,
accepted noise source and computes its bounding box automatically at run time (rather than a fixed
row cutoff) specifically so a real difference elsewhere isn't hidden by it - but that automatic
noise-floor sampling is only as complete as the number of repeats (`RUNS`) actually run, and this
particular label's text width varies enough between runs (single vs. double-digit millisecond/
microsecond values shift where the string's trailing characters land) that a handful of repeats
consistently undersamples its true footprint by an order of magnitude. Candidate 096's own
`--select-all` measurements (4-6 repeats) happened to land on the higher end of that undersampled
range while its `--resize-step` measurements (also 4-6 repeats) happened to land lower - both
artifacts of the same underestimation, not a real difference between the two scenarios.

## What's not done / open questions

- **The residual 8px (rows 796-797) in the plain `--type-text` scenario after the fix was not
  root-caused.** Direct pixel inspection (see the crop taken during this investigation) shows it
  sits on a thin horizontal line consistent with the `ScrollView`'s horizontal scrollbar
  track/thumb boundary - the same general area candidates 093/094/095/096 already traced a real
  bug to (in a *different*, higher-frame-count, patched-tree scenario). Whether this is the same
  family of floating-point/property-evaluation-order non-determinism those candidates found, or an
  unrelated, much smaller effect, was not determined. Notably, this rules out one specific claim
  made in candidate 091's own `RESULT.md` ("matches baseline's own self-noise floor... a
  pre-existing caret-blink-timing non-determinism") - this candidate's direct instrumentation (step
  1 above) shows the blink timer never fires at all in this scenario, so that residual noise's real
  cause is still open, not caret-blink as previously assumed.
- **The harness fix (`bench_status_blank.diff`) was only applied to the `text`-mode fill path's
  entry point in `bench.rs`, once, right after the fill phase.** It has not been checked against
  every other mode/flag combination that also calls into `on_add_lines`/`on_add_records` (e.g. the
  `list`/`scroll` modes' own "+N records: model filled in..." status message, `logic.rs:88`, which
  embeds the same kind of real-timing text) - those likely have the identical noise source and
  would benefit from the same fix, but this was not verified.
- **This candidate's fix lives only in `perf_test` (the harness), not in any tracked/product
  source** - `patch.diff` against the real Slint source is empty. `bench_status_blank.diff` (the
  actual fix) and its README.md entry are the deliverable; there is nothing to apply to a real
  Slint application from this candidate.

## Reproduce

```sh
# Ruling out cursor blink (step 1): from a pristine checkout at base_commit,
git apply ___OPTIMIZATIONS___/097.SELECT_ALL_NOISE_FLOOR_ROOT_CAUSE/blink_trace_diagnostic.diff
cd perf_test && cargo build --bin perf_test --profile rdebug --features software
SLINT_DEBUG_BLINK=1 ./../target/rdebug/perf_test --bench --mode text --count 10000 --type-text \
    --select-all --scroll-step 0 --frames 150 --snapshot /tmp/sa.ppm 2>/tmp/blink.log
grep -c BLINK /tmp/blink.log   # 0 in every run tried, with or without --select-all
cd .. && git checkout -- internal/core/input.rs

# Reproducing the noise floor at high repeat counts (step 2/3): unpatched perf_test,
cd perf_test && cargo build --bin perf_test --profile rdebug --features software
for r in $(seq 1 20); do
  ./../target/rdebug/perf_test --bench --mode text --count 10000 --type-text --scroll-step 0 \
      --frames 6 --snapshot /tmp/r$r.ppm >/dev/null
done
# then compare all 20 .ppm files pairwise for pixels that differ between any two - converges on
# ~3700px in rows 839-851 regardless of --select-all/--frames, see this candidate's own analysis
# script output quoted in "What was tried" above.

# Verifying the fix (step 4): apply bench_status_blank.diff to perf_test/src/bench.rs (untracked,
# so this is a plain patch to a file in perf_test/, not a `git apply` against the main repo),
# rebuild, and repeat the 20-run comparison above - 0 differing px.
```
