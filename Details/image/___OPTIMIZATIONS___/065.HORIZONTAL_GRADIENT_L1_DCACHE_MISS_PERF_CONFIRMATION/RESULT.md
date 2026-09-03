# Confirming candidate 064's cache-miss hypothesis for `horizontal_gradient` via `perf stat -e L1-dcache-load-misses`

Candidate 064 measured that the `DynamicImage::get_pixel`/`put_pixel` `#[inline]` fix (candidate
060) gives `vertical_gradient` a clear, outside-noise win (-23.4%/-21.9%) but leaves
`horizontal_gradient`'s delta (-6.5%/-3.2%) indistinguishable from that bench's own noise floor,
and proposed - without measurement, since `perf` was believed unavailable at the time - that
`horizontal_gradient`'s column-major write stride against the row-major-stored buffer makes it
cache-miss-bound rather than call-dispatch-bound. This candidate re-checked `perf` availability
(it **is** available on this session's host, `perf stat` runs cleanly, `perf_event_paranoid` is
`-1`, no privilege escalation needed - see `README.md`'s updated "Environment" section) and used
`perf stat -e L1-dcache-load-misses` (plus `L1-dcache-loads`, `instructions`, `cycles`,
`task-clock` for context) directly on both functions, baseline vs. candidate 060/064's own
already-recorded `#[inline]` patch, to confirm or refute the hypothesis. **No new source patch was
written for this candidate** - it reuses candidate 060/064's exact, unmodified fix purely as the
"before/after" toggle for this measurement (`patch.diff` in this folder is byte-identical to
candidate 064's, same convention candidate 037 used for its own measurement-only re-confirmation
of an already-known fix).

## What was tried

Base commit: `44ce9226c541dd2b11897b9ef07156a28871cdb7` (same as candidate 064). A temporary
standalone example, `examples/_perf_check_065.rs` (not part of `patch.diff` - measurement tooling,
same convention as candidates 025/037's own temporary drivers), takes a mode (`vertical`/
`horizontal`) and an iteration count, builds one 1920x1080 `DynamicImage::ImageRgba8` once, and
calls `imageops::vertical_gradient`/`horizontal_gradient` on it in a tight loop (200 iterations),
wrapping the mutable reference in `black_box` each call - deliberately not a criterion bench, so
`perf stat` can wrap just this process and attribute every counted event to exactly one target
function with no criterion/statistics-harness overhead mixed in.

Built two `--release` binaries from this example: `baseline` (unpatched source) and `patched`
(candidate 060/064's `#[inline(always)]`/`#[inline]` on `DynamicImage::get_pixel`/`put_pixel`
applied). For each of the 4 combinations (baseline/patched x vertical/horizontal), ran
`perf stat -e L1-dcache-loads,L1-dcache-load-misses,instructions,cycles,task-clock -- <bin> <mode>
200`, 5 repeats each under `taskset -c 3` (single-core, for a clean attribution), plus a further
3 repeats per combination unpinned (multi-core-equivalent check - both functions are
single-threaded, so "multi-core" here means only "not `perf`-pinned", to check the pinning itself
wasn't producing an artifact) using `L1-dcache-load-misses` alone. `uptime` load average was
0.27-2.6 throughout (well under the 24-core count).

| Variant | L1-dcache-load-misses (median of 5, `taskset -c 3`) | L1-dcache-loads (median of 5) | instructions (median of 5) | notes |
|---|---|---|---|---|
| baseline `vertical_gradient` | 26,843,593 | 4,163,910,614 | 17,904,563,552 | row-major write order |
| baseline `horizontal_gradient` | **422,379,611 (15.7x baseline vertical)** | 4,169,378,373 | 17,914,371,448 | column-major write order |
| patched `vertical_gradient` | 26,916,289 (+0.27% vs. baseline vertical) | 3,747,610,861 (-10.0%) | 20,808,565,331 (+16.2%) | |
| patched `horizontal_gradient` | 420,652,422 (-0.41% vs. baseline horizontal, 15.6x patched vertical) | 3,755,352,169 (-9.9%) | 20,815,416,579 (+16.2%) | |

Unpinned (no `taskset`) spot-check, `L1-dcache-load-misses` only, 3 repeats each: baseline
vertical 26.90-27.08M, baseline horizontal 421.3-424.2M, patched vertical 26.93M (all 3 runs),
patched horizontal 420.1-420.4M - same ~15-16x gap and same near-zero patch effect as the pinned
runs, confirming the result isn't a `taskset`-pinning artifact.

`task-clock`/`cycles` were also collected but turned out too noisy to use here: this host's CPU
governor is `powersave` (confirmed via `/sys/devices/system/cpu/cpu3/cpufreq/scaling_governor`),
and individual runs landed bimodally around either ~735ms or ~1035ms task-clock for the *same*
binary/mode/iteration-count, evidently a frequency-scaling effect unrelated to the code - this is
expected and not a concern for this candidate's actual question, since `L1-dcache-load-misses` (a
hardware event count, not a time) was stable to <1% across every repeat regardless of which
task-clock mode a given run happened to land in. Wall-clock comparison for this same patch was
already properly established with paired-run methodology in candidate 064; this candidate adds
the cache-counter evidence, not a re-measurement of wall-clock.

## Result

**Confirms the mechanism, does not confirm the specific "removing the fix's call overhead has no
effect because cache misses dominate" framing as literally as phrased.** Two separate, clean
findings:

1. **`horizontal_gradient` incurs ~15.6-15.7x more `L1-dcache-load-misses` than
   `vertical_gradient` on the same 1920x1080 image, in both the baseline and the patched build,
   pinned and unpinned** (422-424M vs. 26.8-27.1M). This is a large, stable (<1% run-to-run
   variance), directly-measured hardware-counter confirmation that `horizontal_gradient`'s
   column-major write stride against the row-major-stored `ImageBuffer` really does cause far more
   L1 data-cache misses than `vertical_gradient`'s row-major write order - the structural
   explanation candidate 064 proposed from code-reading alone is real, not just plausible.
2. **The `#[inline]` patch itself changes `L1-dcache-load-misses` by less than 1% for either
   function** (vertical: +0.27%; horizontal: -0.41%, both within this measurement's own run-to-run
   noise) - the patch is a pure call-dispatch/register-allocation change, not a memory-access-
   pattern change, so it was never going to move the miss count either way, and it doesn't. What
   it *does* measurably change is `L1-dcache-loads` (total load-instruction count, down ~10% for
   both functions, consistent with removing a real function call's argument/stack-frame loads) and
   `instructions` (up ~16.2% for **both** functions equally, not explained by this candidate - see
   "What's not done" below).

Framed against candidate 064's original question: the huge, patch-independent
vertical-vs-horizontal miss-count gap is exactly the kind of memory-bound cost that a pure
call-dispatch fix cannot address, which is consistent with (and now directly evidenced for) why
`horizontal_gradient`'s wall-clock delta from the same patch was much smaller and noise-floor-
indistinguishable while `vertical_gradient`'s was a clear win - `vertical_gradient`'s baseline cost
is not fighting a large cache-miss tax to begin with, so removing dispatch overhead is a
comparatively larger share of what's left; `horizontal_gradient`'s cost has a large cache-miss
component the patch structurally cannot touch, on top of the same dispatch overhead the patch does
remove.

## Does it change behavior/output?

**N/A - measurement/investigation only, no new patch applied to real source.** This candidate
reuses candidate 060/064's already bit-exact-verified `#[inline(always)]`/`#[inline]` fix
unchanged, purely as the toggle for this measurement; `patch.diff` in this folder is a copy of
candidate 064's, kept for reproducibility (same convention as candidate 037), not a new change.
`examples/_perf_check_065.rs` was temporary measurement tooling, removed before the working tree
was reverted, and was never part of `patch.diff`.

- **Bit-exact?** N/A (no new patch); candidate 064 already confirmed bit-exact output for this
  exact fix via an FNV-1a hash of `vertical_gradient`/`horizontal_gradient` output.
- **Is this a breaking change?** No - same as candidates 060/064, only `#[inline]` attributes,
  no signature/behavior change.
- Project test suite (`cargo test --release`), re-run directly in this candidate's own session as
  required: **550 passed / 1 failed, identically before and after** applying/reverting the same
  patch. The one failure is the same pre-existing, unrelated `tests/regression.rs::
  resizing_with_catmul` every prior candidate in this workspace touching this area has already
  documented.

## Why the result looks the way it does

See "Result" above - this section's job in the template is normally to explain a surprising
timing number, but this candidate didn't re-measure timing; the explanation for *why*
`horizontal_gradient`'s wall-clock result looked the way it did in candidate 064 is now backed by
the `L1-dcache-load-misses` numbers above rather than being restated here.

One genuinely surprising, unexplained side-observation from this candidate's own data: the patch
increases total dynamic `instructions` by ~16.2% for **both** functions equally (17.90B -> 20.81B
vertical; 17.91B -> 20.82B horizontal, for the same 200 iterations / same total pixel count),
despite `L1-dcache-loads` *decreasing* ~10% and static binary `.text` size being essentially
unchanged (`size`: baseline 335,842 bytes vs. patched 335,586 bytes; `objdump -d | wc -l`: 70,165
vs. 70,259 lines - both roughly flat). Inlining `DynamicImage::put_pixel`'s 8-arm `match` directly
into the hot loop, rather than leaving it a real call, plausibly changes what LLVM's loop
vectorizer/scheduler can do with the surrounding code (more instructions issued per pixel, but
fewer loads and apparently no worse - `cycles` counts were too noisy on this `powersave`-governor
host to compare directly) rather than a straightforward "fewer calls = fewer instructions" story -
this was not investigated further within this candidate's scope (see "What's not done").

## What's not done / open questions

- The ~16.2% dynamic-instruction increase from the patch (identical percentage for both
  functions) was observed but not root-caused - a side-by-side `objdump`/`perf annotate` disassembly
  comparison of the inlined vs. non-inlined `put_pixel` call site inside each function's hot loop
  (same technique candidate 055 used for its rematerialization finding) would show whether this is
  a vectorization-shape change, a different unroll factor, or something else. Added to `IDEAS.md`.
- `cycles`/`task-clock` were not usable for a wall-clock comparison in this candidate's own runs
  because of this host's `powersave` CPU governor causing bimodal per-run frequency scaling
  unrelated to the code; candidate 064's own criterion-based paired-run wall-clock numbers (which
  used many more repeats and a majority-of-pairs discipline, and are not questioned by anything
  found here) remain the reference wall-clock result for this fix.
- L2/LLC miss counters (`L1-dcache-load-misses` only measures L1; a miss there can still hit L2
  cheaply) were not measured - the horizontal/vertical gap measured here is specifically an L1
  effect, consistent with the stride argument, but this candidate doesn't rule out or confirm
  whether `horizontal_gradient`'s misses are mostly absorbed by L2 (cheap) or spill further
  (expensive); `LLC-load-misses`/`LLC-loads` on this same harness would answer that directly if a
  future candidate wants to quantify the actual cycle cost of `horizontal_gradient`'s access
  pattern rather than just confirming it exists.
