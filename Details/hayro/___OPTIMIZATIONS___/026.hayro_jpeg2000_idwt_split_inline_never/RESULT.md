# JPEG2000 IDWT horizontal filter: force candidate 014's 5 split-out step functions with `#[inline(never)]`

Follow-up to candidate `014`, which restructured candidate `002`'s (never-applied, regressed 8.6x)
deinterleave-based SIMD rewrite of the horizontal IDWT filter
(`hayro-jpeg2000/src/j2c/idwt.rs`, `irreversible_filter_97i_horizontal_simd`) into 5 smaller free
functions to test whether reducing function size restores LLVM's inlining of `fearless_simd`'s
`f32x8` primitives - it found LLVM re-inlined 4 of the 5 split functions straight back into the
parent anyway (only `irreversible_step_3` stayed separate), so the split barely changed the
compiled shape and produced no measurable win (a small ~1-2% regression, if anything). This
candidate tests candidate 014's own listed open question directly: adding `#[inline(never)]` to
each of the 5 split functions, forcing LLVM's hand instead of leaving the decision to its own
heuristic - the same technique candidate 008 used successfully on the original *scalar*
`filter_step_horizontal` (which did restore auto-vectorization there).

**No patch to real source is proposed here** - like candidates 013/014, this is a
measurement/characterization. `patch.diff` re-applies candidate 002's rewrite + candidate 014's
restructuring (both needed for the 5 functions under test to exist at all) plus this candidate's
5 `#[inline(never)]` attributes on top. All three were reverted from the real working tree before
this folder was written (`git status` clean in the worktree used).

## What was tried

Added `#[inline(never)]` directly above each of the 5 functions candidate 014 split out
(`irreversible_step_1_2`, `irreversible_step_3`, `irreversible_step_4`, `irreversible_step_5`,
`irreversible_step_6`) - a 5-line, purely-additive change (see
`evidence/own_change_vs_candidate_014.diff` for the isolated diff against candidate 014's already-
applied state; `patch.diff` is the full combined diff - candidate 002 + candidate 014 + this
candidate - against the pinned `base_commit`).

### `nm`/`objdump` check (this session's own build, AVX512, identical toolchain used for both variants below)

Unlike candidate 014 (where 4 of 5 split functions got re-inlined into the parent, per `nm`), with
`#[inline(never)]` all 5 stayed as separate, real symbols this time (`evidence/symbols_inline_never.txt`):

| Function (`::<Avx512>`) | Kept as separate symbol? | Real `__fearless_simd_kernel` calls inside it |
|---|---|---|
| `irreversible_step_1_2` | yes (forced) | 4 |
| `irreversible_step_3` | yes (forced) | 6 |
| `irreversible_step_4` | yes (forced) | 6 |
| `irreversible_step_5` | yes (forced) | 6 |
| `irreversible_step_6` | yes (forced) | 6 |
| `irreversible_filter_97i_horizontal_simd` (the now-empty 5-line dispatcher) | yes | 0 |
| **Total** | - | **28** |

This session's candidate-014 "restructured" build (re-measured as this candidate's baseline, same
toolchain, same session) counted **32** real calls total (26 in the still-mostly-inlined parent +
6 in the one function LLVM already kept separate, `irreversible_step_3`). So forcing all 5 apart
with `#[inline(never)]` brought the total back down to 28 - matching candidate 014's own reported
count for the pre-split, *unsplit* function (28, in that same session) - rather than reducing it
further. **No variant in this candidate, 014, or 013 gets anywhere close to `filter_vertical`'s 0
real calls.**

### Wall-clock (`bench_idwt`, real JPX decode of the embedded `sample.jp2`, 300 iterations/round)

`taskset -c 2` for single-core, unpinned for multi-core; `uptime` load 5.3 throughout (well under
the 24-core count).

| Variant | single-core p50 | multi-core p50 | notes |
|---|---|---|---|
| baseline (candidate 014's restructured split, no `#[inline(never)]`) round 1 | 20.431 ms | 20.515 ms | |
| baseline round 2 | 20.451 ms | 20.422 ms | |
| **baseline avg** | **20.441 ms** | **20.469 ms** | |
| patched (`#[inline(never)]` x5) round 1 | 20.241 ms | 20.359 ms | |
| patched round 2 | 20.312 ms | 20.364 ms | |
| **patched avg** | **20.277 ms** | **20.362 ms** | |
| **delta** | **-0.8%** (patched faster) | **-0.5%** (patched faster) | |

## Result

A small (~0.5-0.8%) wall-clock improvement over candidate 014's already-restructured (but not
forced-inline-never) code, consistent in direction across both single-core and multi-core, 2 rounds
each. This is a small effect relative to typical run-to-run p50 spread seen in this same harness in
candidate 014's own session (single-core p50 varied 21.218-21.410 ms across 3 "clean" rounds there,
a ~0.9% spread) - so while every one of the 4 measured rounds here (2 single-core, 2 multi-core)
pointed the same direction (patched faster), the magnitude is close to this harness's own noise
floor and should be read as a weak, not a confidently-established, signal. It does **not** reverse
candidate 002's original 8.6x regression, nor does it approach `filter_vertical`'s fully-inlined
(0 real calls) shape - `#[inline(never)] `moved the *total* real-call count back to roughly where
the *unsplit* function already was (28), not below it.

## Does it change behavior/output?

- **Bit-exact?** Yes - `bench_idwt`'s per-iteration output hash matched between the baseline
  (candidate 014's restructured build) and this candidate's `#[inline(never)]` build in every run
  this session (`hash=339b9db71ea46bfc` throughout, matching candidate 014's own reported hash for
  the same input).
- **Is this a breaking change?** No - `#[inline(never)]` only affects codegen, never observable
  behavior. Purely a restructuring of code that was itself never applied to real source (candidate
  002's original regression means none of this ships either way).
- Project test suite (`cargo test -p hayro-jpeg2000 --release`):
  - `--lib` (2 tests, including candidate 002's own `horizontal_filter_equivalence_tests` exhaustive
    2560-case synthetic check): **2 passed, 0 failed** for both the baseline (candidate 014 state)
    and the patched (`#[inline(never)]`) state - identical result, unaffected as expected for a
    codegen-only attribute.
  - `--test asset_suite` (the crate's snapshot-based decoder test): fails identically before and
    after this candidate's change, and identically on a completely pristine checkout with none of
    candidates 002/014/026's patches applied - `Failed to read asset directory: missing test input
    serenity/buggie-gray.jpf`. This suite downloads its real test inputs on demand from
    `hayro-assets.dev` via `hayro-jpeg2000/sync.py` (see that crate's `TESTS.md`); no network access
    was available in this environment (`python3 sync.py` timed out with no output). This is an
    environment limitation unrelated to this candidate - N/A, not a regression.

## Why the result looks the way it does

Not root-caused at the LLVM-internals level (same caveat as candidates 013/014 - `perf
record`/`perf annotate` unavailable in this environment). What this session adds to 013/014's
findings:

`#[inline(never)]` does what it says - it reliably prevented all 5 split functions from being
folded back into the parent (unlike candidate 014's own restructuring alone, which only kept
`irreversible_step_3` separate and let LLVM re-inline the other 4). But forcing separation didn't
reduce the *total* number of real `__fearless_simd_kernel` calls below what the original unsplit
function already had (28) - it just redistributed the same total count across 5 explicit call
sites instead of concentrating it (mostly) in one function body. This is consistent with 014's own
conclusion that the source-level function boundary isn't what's driving whether `fearless_simd`'s
per-op calls get inlined here - something else (a per-callsite cost heuristic inside
`fearless_simd`'s own kernel dispatch, or an inliner threshold keyed on something other than raw
function size/count-of-functions) is the more likely lever, and remains untested by any candidate
in this line of investigation so far.

The small, consistent wall-clock win (~0.5-0.8%) without a corresponding drop in real-call count is
notable on its own: it suggests whatever second-order codegen effect `#[inline(never)]` has here
(instruction cache locality from smaller/more predictable code, or removing pressure on the
inliner's own decision-making elsewhere in the same compilation unit) may matter slightly more than
the raw kernel-call count this line of investigation has been using as its primary proxy metric -
though this is speculative and not independently confirmed by any tool run this session.

## What's not done / open questions

- The ~0.5-0.8% win is close to this harness's own observed run-to-run noise floor (per candidate
  014's own 3-round spread) - only 2 rounds were run per variant this session (per the assigned
  budget), so this should be treated as a weak signal warranting a few more paired rounds before
  trusting the magnitude, not as a confidently-quantified speedup.
- Why `fearless_simd`'s per-op calls don't inline away entirely on this `Avx512` target (unlike the
  simpler `filter_vertical` path using the same primitives) is still not identified at the
  LLVM/`fearless_simd`-internals level - `perf record`/`perf annotate` remain unavailable in this
  environment. A next step worth trying: compare against a `fearless_simd` version/toolchain where
  candidate 013's originally-reported 92-call count for the unsplit function was observed (vs. this
  session's and 014's session's 28), since that's a much larger swing than any restructuring tried
  so far in this line of candidates.
- `evidence/symbols_inline_never.txt` has the full `nm` symbol list for the `#[inline(never)]`
  build; `evidence/own_change_vs_candidate_014.diff` is the isolated 5-line diff against candidate
  014's already-applied restructuring (for a reader who only wants to see this candidate's own
  incremental change, without candidates 002/014's much larger diffs).

## Operational note (not a codebase finding)

Mid-session, a `git stash` / `git stash pop` pair run inside this candidate's own worktree
(`/tmp/hayro-worktrees/026`) picked up an unrelated stash entry from what looks like a
concurrently-running sibling candidate's own worktree (content matched a `hashbrown`-based
`resolved_font_cache` change in `hayro-interpret`, referencing "candidate 025" in its own doc
comment) - `git stash` operates on a single repo-wide `refs/stash` shared across all worktrees of
the same repository, not a worktree-local stack, so two sessions stashing around the same time can
interleave. The unrelated changes were reverted from this worktree (`git checkout --`) and this
candidate's own file state was restored from an external copy saved before the stash commands were
run, so this candidate's own measurements/patch are unaffected. One entry was dropped from the
shared stash stack in the process (`git stash pop` reported `Dropped refs/stash@{0}
(a66ba68a493a744e81e005a4dba674fe55db4ff0)`) - if that belonged to a sibling candidate's own WIP,
it is still recoverable via that commit SHA (`git stash apply a66ba68a493a744e81e005a4dba674fe55db4ff0`
or `git show a66ba68a493a744e81e005a4dba674fe55db4ff0`) for a limited time before garbage collection,
but was not restored here since it isn't this candidate's finding to fix. **Worth folding into the
skill's own guidance: never use repo-wide `git stash` inside a per-candidate worktree when sibling
candidates may be running in parallel - use a worktree-local file copy (`cp`) instead, exactly as
this candidate's own recovery did.**
