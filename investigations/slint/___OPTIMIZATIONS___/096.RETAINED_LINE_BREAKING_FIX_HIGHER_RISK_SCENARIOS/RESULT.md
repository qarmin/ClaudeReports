# 96 - Verify candidate 095's `drop(other)` fix against candidate 093's own flagged higher-risk scenarios

This is a **correctness-verification candidate, not a performance candidate**: no new timing was
measured. It follows up on candidate 095's own "what's not done" item - its one-line
`experimental_partial_fix.diff` (`drop(other)` in `layout()`'s "full recompute, no `dirty_range`,
no slot match" branch, `internal/core/textlayout/sharedparley/layout.rs`) was only checked against
the exact scene candidate 093 originally found its scrollbar-thumb regression in
(`--type-text --scroll-step 0`). Candidate 093's own doc comment had already flagged two other,
higher-risk call patterns as untested: `--resize-step` (a third distinct width entering the 2-slot
cache) and `--select-all`/cursor-placement queries (which read `TextParagraph::layout.lines()`
directly via `selection.rs`, rather than only metrics). This candidate re-runs 093/095's own
pixel-comparison technique against both. Applied **on top of** candidate 093's own patch (091+092+093)
with candidate 095's `experimental_partial_fix.diff` on top - `patch.diff` bundles all of that
against pristine `master`, the same convention 092/093/094/095 used. No source code was changed by
this candidate itself; it only exercises the existing 093+095-fix tree against new scenarios.

## What was tried

Three binaries built from the exact same base commit, compared pairwise with a noise-floor-aware
pixel comparison. `tools/pixelcheck.sh` gained a `FRAMES` env var as part of this candidate (it was
hardcoded to `--frames 6`, too low to see 093's regression, which only appears past ~80 edited
frames - see "Instrumentation added" below); its own bbox-based verdict still reports
"inconclusive" here because the noise floor is scattered (a status label in one corner, the
scrollbar row elsewhere), so the row/column tables below were produced with the new
`tools/pixeldiff.py` (added this candidate too), which subtracts noise as an exact pixel-position
set instead of a bounding box - see that script's own header for why both tools are kept rather
than one replacing the other.

- **A** = 091+092 (pre-093, no 2-slot cache at all) - the "presumed correct" reference, since 092's
  own `RESULT.md` reported no rendering difference from further-upstream `master`.
- **B** = 091+092+093, unfixed - candidate 093's own patch, the tree the original regression was
  found in.
- **C** = 091+092+093 + candidate 095's `experimental_partial_fix.diff` - the tree under test here.

Both scenarios used `--mode text --count 10000 --type-text --scroll-step 0` (093/095's own base
scene) plus the one extra flag under test. Each comparison used **6 repeated runs per side** (4 at
one frame count where 6 wasn't collected first) to sample each binary's own run-to-run noise floor
before attributing anything to a real difference - necessary per 095's own finding that candidate
093's mechanism has real run-to-run non-determinism of its own, unlike the pre-093 tree.

### Scenario 1: `--resize-step 50` (third distinct width)

| Frames | A vs B (unfixed) noise floors | raw diff | real diff (outside both noise floors) | A vs C (fixed) noise floors | raw diff | real diff |
|---|---|---|---|---|---|---|
| 80 (6 reps/side) | A: 826px, B: 728px | 550px | **38px**, rows 764-766 | A: 826px, C: 738px | 522px | **0px** |
| 150 (4 reps/side) | A: 836px, B: 7354px | 638px | **18px**, rows 765-766 | A: 836px, C: 706px | 518px | **0px** |

The 38px/18px real differences in the unfixed (B) tree land in the same scrollbar-thumb region
093/095 originally found, confirmed by direct pixel inspection at row 765: at frame 150, binary
B's scrollbar thumb starts at column 222 (antialiasing beginning at 222, full color from 223) and
ends at column 985 (last non-background pixel), while both A and the fixed binary C start at
column ~230 and end at column 984 - B's thumb is measurably longer at *both* ends, not just one.
See `A_150_resizestep_thumbstart.png` / `B_150_resizestep_thumbstart_unfixed.png` /
`C_150_resizestep_thumbstart_fixed.png` in this folder (8x-scaled crops of the same 60x8 px region,
one per binary) - B's thumb visibly starts 7-8px earlier than A/C.

Also notable: **B's own run-to-run noise floor exploded at 150 frames** (7354px, vs. 836px for A
and 706px for C) - consistent with 095's own finding that the unfixed 093 mechanism has
non-deterministic call-order-dependent rendering that gets worse the longer the scene runs, and
that this non-determinism does not carry over to the fixed tree.

### Scenario 2: `--select-all` (reads `.lines()` directly via `selection.rs`)

| Frames | A vs B (unfixed) noise floors | raw diff | real diff | A vs C (fixed) noise floors | raw diff | real diff |
|---|---|---|---|---|---|---|
| 80 (6 reps/side) | A: 280px, B: 255px | 222px | **0px** | A: 280px, C: 238px | 184px | **0px** |
| 150 (4 reps/side) | A: 3685px, B: 249px | 257px | **0px** | A: 3685px, C: 271px | 165px | **0px** |
| 300 (6 reps/side) | A: 3718px, B: 3620px | 249px | **0px** | A: 3718px, C: 3680px | 3670px | **0px** |

No divergence outside the noise floor was found at any frame count tried (80/150/300), in either
the unfixed or fixed tree. **Caveat**: this scenario's own baseline (A, pre-093) noise floor is
unusually large starting at 150 frames (3685-3718px, roughly 0.3% of the frame, vs. under 1000px
for the resize-step scenario) - the noise band covers rows 796-851 (a status-label region plus a
band that includes the scrollbar-thumb row), which is wide enough that a divergence smaller than
that band could be masked rather than genuinely absent. This was not resolved by adding more
frames (300 has the same large noise floor as 150) or more repeats (6 vs 4 made no difference to
the band's extent).

## Result

**The fix generalizes to the `--resize-step` scenario**: the same scrollbar-thumb-length
divergence 093 found in its original scene reappears with `--resize-step` in the unfixed tree (a
real, reproducible, noise-floor-exceeding difference at both frame counts tried), and candidate
095's one-line `drop(other)` fix closes it to within noise floor at both frame counts tried (80 and
150), the same as it did in the original scene.

**The `--select-all` scenario shows no divergence in either tree at the frame counts tried
(80/150/300)** - but this is a weaker result than the `--resize-step` finding, because this
scenario's own baseline noise floor is large enough (up to 3718px) that it cannot rule out a
smaller divergence being masked by noise, the same way 093's original regression was invisible to
`tools/pixelcheck.sh`'s own default `--frames 6` until manually swept to 80+ frames.

## Does it change behavior/output?

This candidate does not add or change any behavior itself - `patch.diff` is exactly candidate
093's own patch with candidate 095's already-reported one-line fix on top, both previously
characterized. What's new here is evidence about *where that combination still does and doesn't
diverge from pre-093 rendering*:

- **Bit-exact?** No, by the same margin 093/095 already reported (small floating-point/rounding
  noise inherent to this rendering pipeline, well below the measured noise floor) - this candidate
  did not find any *additional* bit-exactness gap beyond what 093/095 already characterized.
- **What changes, for whom, under what conditions:** for a horizontally-scrollable,
  non-word-wrapped `TextEdit` that is also resized mid-session (`--resize-step`-like usage), the
  093-unfixed scrollbar-thumb-length bug is present with the same character (thumb ends up
  slightly too long at both ends) as in the original scene, and the 095 one-line fix closes it the
  same way. For a `--select-all`-style usage (cursor/selection queries reading `.lines()`
  directly), no divergence was found in either tree up to 300 edited frames, though the scenario's
  own high baseline noise limits how confidently that can be read as "safe" rather than "not yet
  observed."
- **Is this a breaking change?** No public API changed (same as 093/095).
- Project test suite (`i-slint-core`'s own unit tests; these don't exercise `ScrollView` extent
  computation or the pixel-level rendering checked above):
  - `cargo test -p i-slint-core --all-features --lib`, tree = 091+092+093 (093 alone, unfixed):
    **319 passed, 0 failed**.
  - Same command, tree = 091+092+093 + candidate 095's `experimental_partial_fix.diff` (fixed):
    **319 passed, 0 failed**. Identical pass/fail set before and after.
  - The broader `tests/` driver suites were not re-run, for the same reason 093/094/095 skipped
    them (a `ScrollView`-extent/scrollbar-pixel-only difference that a non-pixel-comparing suite
    would not catch).

## Why the result looks the way it does

095's root cause (the "full recompute, no `dirty_range`, no slot match" branch calling
`break_all_lines` on every paragraph without dropping the other retained slot, contaminating a
later caller's assumption that its own dirty range was the only thing that changed) is not
specific to the exact call pattern of the original scene - it is a property of *any* call sequence
that reaches that branch. `--resize-step` reaches it more often (a third width means more calls
find neither retained slot matching, i.e. more full-recompute-no-match calls), so it is unsurprising
the same bug reappears there and the same fix closes it. `--select-all`'s lack of an observed
divergence is most simply explained by it not changing which *widths* alternate through the cache
at all (selection is drawn using whatever width the ordinary draw call already established); it
adds a *reader* of the shared `parley::Layout` state, but 093/095's bug is about a *writer*
(`break_all_lines`) stepping on another slot's assumptions, and `--select-all` alone doesn't add a
new writer/width to the mix - consistent with, but not conclusive proof of, "no bug here."

## What's not done / open questions

- **`--select-all`'s own large baseline noise floor was not root-caused.** It starts appearing at
  150 frames (not at 80) and does not shrink with more repeats or more frames sampled - a source
  worth identifying (candidate suspicion: a wall-clock-driven cursor blink or selection-highlight
  animation that the harness's `--bench` mode does not otherwise disable) before trusting a "no
  divergence" result in this scenario as strongly as the `--resize-step` one.
- **`--select-all` combined with `--resize-step` together** (the actual worst case implied by
  093's doc comment - a `.lines()`-reading caller *and* a third width at once) was not tried this
  session.
- **Frame counts beyond 300 for `--select-all`** and **beyond 150 for `--resize-step`** were not
  swept; 093's own original bug grew from 45-60px at frame 80 to 803px at frame 150 in its
  original scene, so a slower-building divergence in either scenario tried here cannot be fully
  ruled out from these frame counts alone.
- As with 093/095, this candidate's `patch.diff` combination (093 + 095's experimental fix) should
  not be taken as a finished, ready-to-merge patch on the strength of this candidate alone - it is
  reported here, with what was and wasn't checked stated plainly, per this workspace's
  characterized-difference policy.

## Reproduce

```sh
cd perf_test
git apply ../___OPTIMIZATIONS___/093.RETAINED_LINE_BREAKING_2SLOT_CACHE/patch.diff  # (from repo root)
# build the unfixed binary first, copy it aside, then revert and reapply plus the fix:
just build  # or: cargo build --bin perf_test --profile rdebug --features software
cp target/rdebug/perf_test /tmp/B_093_unfixed
cd .. && git checkout -- .
git apply ___OPTIMIZATIONS___/093.RETAINED_LINE_BREAKING_2SLOT_CACHE/patch.diff
git apply ___OPTIMIZATIONS___/095.RETAINED_LINE_BREAKING_STALE_SLOT_TRACE/experimental_partial_fix.diff
cd perf_test && cargo build --bin perf_test --profile rdebug --features software
cp target/rdebug/perf_test /tmp/C_093_fixed
cd .. && git checkout -- .
# also build a pre-093 reference (091+092 only) the same way, from
# ___OPTIMIZATIONS___/092.RETAINED_LINE_BREAKING_ACROSS_RESHAPE/patch.diff, as /tmp/A_091_092_baseline

# resize-step scenario, 6 runs per side, frame 150:
for r in 1 2 3 4 5 6; do
  /tmp/A_091_092_baseline --bench --mode text --count 10000 --type-text --scroll-step 0 \
    --resize-step 50 --frames 150 --snapshot /tmp/rs150_A_$r.ppm
  /tmp/B_093_unfixed --bench --mode text --count 10000 --type-text --scroll-step 0 \
    --resize-step 50 --frames 150 --snapshot /tmp/rs150_B_$r.ppm
  /tmp/C_093_fixed --bench --mode text --count 10000 --type-text --scroll-step 0 \
    --resize-step 50 --frames 150 --snapshot /tmp/rs150_C_$r.ppm
done
# quick check with the harness's own tool (FRAMES env var added by this candidate):
RUNS=6 FRAMES=150 bash ___OPTIMIZATIONS___/tools/pixelcheck.sh \
  /tmp/A_091_092_baseline /tmp/B_093_unfixed -- \
  --mode text --count 10000 --type-text --scroll-step 0 --resize-step 50
# for the precise row/column breakdown (pixelcheck.sh's bbox verdict says "inconclusive" here
# because the noise floor is scattered - see "What was tried" above):
python3 ___OPTIMIZATIONS___/tools/pixeldiff.py \
  --a-repeats /tmp/rs150_A_1.ppm /tmp/rs150_A_2.ppm /tmp/rs150_A_3.ppm /tmp/rs150_A_4.ppm \
  --b-repeats /tmp/rs150_B_1.ppm /tmp/rs150_B_2.ppm /tmp/rs150_B_3.ppm /tmp/rs150_B_4.ppm

# select-all scenario: same recipe with `--select-all` instead of `--resize-step 50`.
```
