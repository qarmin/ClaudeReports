# 99 - Root-causing the residual 8px `--type-text` non-determinism left after candidate 097

This is an **investigation candidate, not a performance candidate**: no timing was measured, and
`patch.diff` against real Slint source is empty. It follows up on the top-of-backlog `IDEAS.md`
entry added while building candidate 097: after 097's harness fix removed the ~3700px
status-label noise, a much smaller (8px, rows 796-797) residual remained in the plain
`--type-text` scenario, in an area candidate 097 identified as "a horizontal-scrollbar-track-shaped
line" - and 097's own direct instrumentation had already disproved candidate 091's claim that a
similarly-sized residual elsewhere was "pre-existing caret-blink-timing non-determinism" (the
blink timer provably never fires while `--type-text` is active), leaving this residual's real cause
open.

## What was tried

1. **Reproduced candidate 097's exact residual** on the `091+092` tree (the same tree 097's own
   "verifying the fix" table used): `perf_test --bench --mode text --count 10000 --type-text
   --scroll-step 0 --frames 150`, 20 repeated runs of the identical binary/flags, `cmp`-style
   pairwise pixel comparison (script in this folder, `diffscan.py`... see `Reproduce` below - the
   working script lives in this session's `tmp/`, not copied into this folder since it's a
   one-off comparison script, not durable tooling; the existing `tools/pixelcheck.sh`/`pixeldiff.py`
   cover the same job for future sessions). Confirmed: a handful of pixels (4-14 across different
   20-run samples taken this session) differ at rows 796-797, columns clustered at ~353-357 and
   ~1143-1144, all grayscale antialiasing-intensity values (138/196/253) - consistent with
   sub-pixel jitter in the rounded end-caps of a horizontally-positioned element, not a text/glyph
   difference.
2. **Checked whether this is specific to candidates 091/092's caching changes, or pre-existing.**
   Rebuilt and ran the identical 20-repeat comparison on **pristine `master`** (no candidate patch
   at all) at the same `--frames 150`: **0 differing pixels.** This is the same scene, same typed
   sequence, same window size - the only variable is which tree is running.
3. **Localized the pixel cluster to the `ScrollView`'s horizontal `ScrollBar` thumb.**
   `internal/compiler/widgets/fluent/scrollview.slint`'s `ScrollBar` component's `thumb` Rectangle
   has `animate width, height { duration: 150ms; easing: ease-out; }` and computes its own
   `x`/`width` from `-root.value/root.maximum` - exactly the kind of rounded-rectangle geometry
   (`border-radius: (root.horizontal ? self.height : self.width) / 2`) whose end-caps would show
   this exact antialiasing-jitter signature if sampled mid-transition.
4. **Found the real mechanism by direct instrumentation**: `perf_test --bench`'s
   `update_timers_and_animations()` call (`bench.rs`'s frame loop) drives Slint's animation clock
   via `Platform::duration_since_start()`, whose **default implementation reads the real system
   clock** (`internal/core/platform.rs`: `time::Instant::now() - INITIAL_INSTANT`) - and
   `perf_test`'s own `BenchPlatform` does not override it. So every `animate` transition in the
   scene (including the ScrollBar thumb's) is driven by **real elapsed wall-clock time between
   frames**, not a value derived from the frame count the way scrolling/mutation/resize already
   are (per this workspace's `README.md`: "the scroll offset is derived from the frame count so
   two runs are byte-identical" - true for the scroll offset itself, not for `animate` bindings).
   Added a temporary diagnostic (`virtualclock_diagnostic.diff`, this folder, not part of
   `patch.diff`): a `thread_local!` `Cell<Duration>` that `BenchPlatform::duration_since_start`
   returns instead of the real clock, advanced by a fixed synthetic `16ms` per frame in the frame
   loop instead of tracking real time.

| Tree | Frames | Repeats | Real wall clock (unmodified) | Deterministic virtual clock (diagnostic) |
|---|---|---|---|---|
| pristine `master` | 150 | 20 | 0 differing px | 0 differing px (not expected to change; confirms the diagnostic itself is inert when there's nothing to fix) |
| `091+092` | 150 | 20 | **4-14 differing px**, rows 796-797 (magnitude varies slightly run-session to run-session, consistent with real-time jitter) | **0 differing px** |

## Result

**Root-caused**: the residual is real-wall-clock-driven `animate` interpolation of the `ScrollView`'s
horizontal scrollbar thumb geometry, sampled by `--bench` mode at a point that depends on exactly
how much real time elapsed between frames on the host machine - not a text-layout bug, not
property-evaluation order, and not specific to any of candidates 091-096's caching logic. Swapping
`BenchPlatform`'s clock for a deterministic one makes the `091+092` tree bit-exact across 20
repeats at the same scenario/frame count that showed 4-14px of noise with the real clock,
confirming the mechanism directly rather than by elimination alone.

**Why it only shows up on `091+092` (and, per candidate 095/097, on `093-096`'s further-caching
trees) and not on pristine `master`:** the thumb's `animate width, height` transition has a fixed
150ms duration. `--type-text` retypes one character per frame, so the thumb's *target* width/x
changes essentially every frame. On pristine `master`, one edited frame's full document reshape at
10000 lines takes tens to hundreds of milliseconds (see candidate 020/091's own numbers) - long
enough that the previous frame's 150ms transition has already fully settled by the time the next
`update_timers_and_animations()` call happens, so the sampled state is always exactly the
transition's *end* value: deterministic. On `091+092` (and later candidates), the same frame costs
low tens of milliseconds - faster than the 150ms transition duration - so consecutive frames
routinely land the sampled state *mid-transition*, at a point that depends on the accumulated real
elapsed wall-clock time since the process started, which varies by a few milliseconds run-to-run
from ordinary OS scheduling jitter. That the *bug* (this non-determinism) only became *visible*
once the code got fast enough is a plausible read of why 097's own IDEAS entry flagged this as
"possibly the same family" as 093-096's floating-point findings even though the actual mechanism
here is unrelated to floating-point summation order - both families only manifest once a keystroke
becomes cheap enough to run faster than something else in the scene that assumes real time passes
at a "normal" pace.

This also settles candidate 095's own unconfirmed "secondary finding" ("two identical runs of the
pre-093 binary never differ [at the scrollbar row]... it's plausible this is ordinary Slint
property-evaluation scheduling order... not confirmed") - 095 measured only 3 repeats at frame 80;
this candidate's 20 repeats at frame 150 shows the pre-093 (`091+092`) tree **does** have this
exact non-determinism, just below 095's own sampling threshold, in the same undersampling shape
097 already documented for the status-label noise. The mechanism is not "property-evaluation
order" in the sense 095 speculated (a call-order race between two `layout()` calls) - it is
real-clock-driven animation sampling, a layer below any of the text-layout code 091-096 touch.

## Does it change behavior/output?

- **Bit-exact?** N/A to the product itself - no tracked Slint source was changed by this candidate;
  `patch.diff` is empty. The diagnostic that confirmed the mechanism
  (`virtualclock_diagnostic.diff`) touches only `perf_test`'s own untracked benchmark harness
  (`bench.rs`), and was reverted before finishing this candidate - it is not part of the harness's
  permanent state (unlike 097's status-label fix, this diagnostic doesn't make the harness *more*
  correct for its own sake; it exists only to test a hypothesis and would need real design work -
  e.g. deciding whether `--bench` should freeze all animations outright - to be worth keeping, see
  "What's not done" below).
- **Is this a breaking change?** No - no public API, and no product behavior changed. This is not
  a Slint bug: an `animate` binding sampled against real wall-clock time behaving differently
  across runs whose real per-frame timing differs is expected, correct behavior for any real
  (non-headless) use of `animate` - it only reads as "non-determinism" here because `--bench`
  mode's own documented goal (byte-identical snapshots for a given flag/frame combination) doesn't
  hold for scenes with a live `animate` transition, a limitation of the harness, not of Slint.
- Project test suite (`i-slint-core`'s own unit tests; unaffected, since no tracked source
  changed): `cargo test -p i-slint-core --all-features --lib`: **319 passed, 0 failed**, run once
  against the pristine (unmodified) tree - both "before" and "after" for this candidate, matching
  candidate 097's own convention for a patch-less investigation.

## Why the result looks the way it does

`perf_test --bench`'s `MinimalSoftwareWindow`/`BenchPlatform` combination was built to make
scrolling and model mutation deterministic (both driven directly from the frame index, per this
workspace's `README.md`), but `Platform::duration_since_start` was never overridden, so anything
in a scene that depends on Slint's own animation clock (any `animate` binding, `--animate`'s own
explicit time-driven rectangles which `README.md` already documents as "not pixel-comparable") is
implicitly real-time-driven too. `std-widgets`' `ScrollView`/`ScrollBar` (Fluent style, this
harness's compiled style) has one such binding on its thumb (`animate width, height`) that nothing
in `perf_test`'s own scene authoring controls or disables. As long as a scene's per-frame cost
stayed slower than the animation's own transition duration, this was invisible (the state always
settles before it's sampled) - `091+092`'s own speedup (the thing candidates 091-096 exist to
measure) is what crossed that threshold and made the harness's own long-standing gap in
"`--bench` is byte-identical" visible, for a component (`ScrollBar`) unrelated to what 091-096
actually changed.

## What's not done / open questions

- **No permanent harness fix was applied.** `virtualclock_diagnostic.diff` is a proof-of-mechanism,
  not a proposed permanent change - the more faithful fix, freezing `BenchPlatform`'s clock (or
  every animation) altogether for `--bench` mode, has design implications this candidate did not
  work through: `--animate`'s own explicit time-driven scene already documents itself as "not
  pixel-comparable", so a future session should decide whether `--bench` should (a) freeze the
  clock globally except when `--animate` is passed, (b) freeze it always and make `--animate` warn
  it has no effect under `--bench`, or (c) leave it as-is and just document the `ScrollView`
  animation as a second, now-understood, accepted noise source alongside the status label -
  whichever is chosen affects every future `--bench` pixel comparison in this workspace, not just
  this scene, so it deserves its own deliberate candidate rather than being folded in here.
- **Not checked whether other `std-widgets` components used elsewhere in `perf_test`'s scenes
  carry their own `animate` bindings** that could show the same class of noise once a future
  candidate makes some other scene fast enough to expose it (e.g. `list`/`scroll` modes' own
  scrollbars use the exact same `ScrollBar` component).
- **The exact px count/columns varies slightly between repeat sessions of this same
  investigation** (4-14px differing, always rows 796-797, columns clustered near 353-357 and
  1143-1144) - expected for something driven by real scheduling jitter, but means a future
  session re-verifying this should not expect an exact byte-for-byte match to the numbers in this
  `RESULT.md`, only the same rows/mechanism.
- This candidate does not touch any of candidates 093-096's 2-slot/metrics-only mechanisms at all
  (tested only on `091+092`, the simplest tree that already reproduces it) - the "same family as
  093-096" possibility 097's `IDEAS.md` entry raised is now ruled out as a *shared root cause*
  (different mechanism entirely), though both families remain "things that only became visible
  once a keystroke got fast enough."

## Reproduce

```sh
# 1. Reproduce the residual on 091+092 with the real clock (from pristine master):
git apply ___OPTIMIZATIONS___/092.RETAINED_LINE_BREAKING_ACROSS_RESHAPE/patch.diff  # bundles 091+092
cd perf_test && cargo build --bin perf_test --profile rdebug --features software
for r in $(seq 1 20); do
  ./../target/rdebug/perf_test --bench --mode text --count 10000 --type-text --scroll-step 0 \
      --frames 150 --snapshot /tmp/r$r.ppm >/dev/null
done
# compare all 20 .ppm pairwise for differing pixels (e.g. cmp -l, or tools/pixeldiff.py) - a
# handful of px at rows 796-797, columns ~353-357 and ~1143-1144, every session this was tried.

# 2. Confirm pristine master alone never shows it (revert the patch first):
cd .. && git checkout -- internal/core/item_rendering.rs internal/core/textlayout/sharedparley.rs \
  internal/core/textlayout/sharedparley/cache.rs internal/core/textlayout/sharedparley/layout.rs \
  internal/core/textlayout/sharedparley/shaping.rs internal/core/textlayout/sharedparley/tests.rs
cd perf_test && cargo build --bin perf_test --profile rdebug --features software
# repeat the same 20-run loop above against this binary - 0 differing pixels.

# 3. Confirm the mechanism directly: apply this candidate's own perf_test-only diagnostic
# (virtualclock_diagnostic.diff, NOT part of patch.diff) on top of 091+092, rebuild, repeat the
# 20-run loop - 0 differing pixels, same tree that showed 4-14px with the real clock.
cd .. && git apply ___OPTIMIZATIONS___/092.RETAINED_LINE_BREAKING_ACROSS_RESHAPE/patch.diff
patch -p1 -d perf_test < ___OPTIMIZATIONS___/099.SCROLLBAR_THUMB_ANIMATION_REALTIME_JITTER/virtualclock_diagnostic.diff
cd perf_test && cargo build --bin perf_test --profile rdebug --features software
# repeat the 20-run loop - 0 differing pixels.

# Then revert everything (both the product patch and the perf_test diagnostic) before finishing,
# per this workspace's "always leave the tree clean" rule.
```

Visual evidence: `run1_left.png`/`run3_left.png` and `run1_right.png`/`run3_right.png` in this
folder are 8x-scaled crops (60x30px source region, rows 782-812, columns 325-385 and 1115-1175) of
two of the 20 real-clock `091+092` runs from step 1 above - the antialiasing intensity at the
thumb's rounded end-cap visibly differs between the two despite identical typed input.
