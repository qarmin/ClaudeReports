<!--
Copy into NN.CANDIDATE_NAME/RESULT.md. No "verdict" line (applied/deferred/etc) - just facts.
-->

# `fearless_simd` `f32x8` non-inlining (candidate 013's suspect #4): upstream known-issue check, plus a decisive root-cause re-characterization

Follow-up to candidate `013`, which found that candidate `002`'s never-applied deinterleave-based
SIMD rewrite of the horizontal IDWT filter (`irreversible_filter_97i_horizontal_simd`,
`hayro-jpeg2000/src/j2c/idwt.rs`) compiles its `f32x8` arithmetic to real out-of-line
`__fearless_simd_kernel` calls under `Avx512`, instead of inlining the way the existing, fast
`filter_vertical` path does - and hypothesized, without confirming, that this was an LLVM
inliner size/complexity threshold being crossed. This candidate's assigned scope was to check
whether that class of problem is already known/fixed upstream in `fearless_simd`, and file it if
not. `rustc 1.98.0 (88d9e12ae 2026-08-18)`, `cargo 1.98.0`.

## What was tried

1. **Upstream research** (`gh api`/`gh issue list` against `linebender/fearless_simd`, crates.io
   API for release history):
   - hayro pins `fearless_simd = "0.6.0"` (`Cargo.toml`); newest published release is **0.7.0**
     (2026-08-11), with an unreleased `[Unreleased]` section on top.
   - Reading `0.7.0`'s and `[Unreleased]`'s `CHANGELOG.md` entries: nothing addresses inlining
     reliability of `f32x8`/kernel ops.
   - [linebender/fearless_simd#338](https://github.com/linebender/fearless_simd/issues/338)
     ("vectorize() + #[inline(always)] is error-prone and hard to teach", **open**) is the
     matching upstream tracking issue: the maintainer states plainly that a domain expert
     "entirely fail[ed] to get it to work" and that the crate needs "some sort of annotation that
     you just put on every function generic over `Simd` and do not think about inlining at all,
     ever" - i.e. the exact "silently degrades, no compiler warning" symptom candidate 013
     described is the acknowledged, general failure mode this issue tracks.
   - [linebender/fearless_simd#347](https://github.com/linebender/fearless_simd/pull/347) ("Add
     `#[simd]` macro", **open**, unmerged as of this session) is an in-progress fix for #338: a
     proc macro that would auto-wrap a `fn f<S: Simd>(...)` body in `vectorize()` so the inlining
     annotation can no longer be forgotten. Per the PR author's own comment, it's expected to
     ship "in lockstep with v1.0", i.e. not in any version released yet.
   - [linebender/fearless_simd#194](https://github.com/linebender/fearless_simd/issues/194)
     (`dispatch!` is expensive, open) is a related but distinct overhead report (dispatch-site
     branching cost, not per-op kernel-call cost) - not the same mechanism, kept separate.
   - No exact duplicate of candidate 013's specific "no warning, silent" framing existed as a
     *new* issue - but the general problem is already known and actively being worked, so no new
     upstream issue was filed (see "Result" below for why).

2. **Empirical re-check on the current release** (in an isolated worktree, `patch.diff` =
   candidate 013's own patch reapplied: candidate 002's rewrite + the diagnostic module, both
   reverted afterward): built the same real, private `irreversible_filter_97i_horizontal_simd`
   function against **`fearless_simd` 0.6.0** (current pin) and then **0.7.0** (`cargo update -p
   fearless_simd@0.6.0 --precise 0.7.0`), reused candidate 002's own `bench/idwt` harness (real
   `sample.jp2` JPEG2000 decode, output-hash-checked every iteration), and counted real
   `__fearless_simd_kernel` call sites in the compiled `<Avx512>` monomorphization via
   `nm`/`objdump`.

3. **Root-cause re-check, prompted by reading the real, working `filter_vertical_impl` next to
   the broken `irreversible_filter_97i_horizontal_simd` side by side**: every one of
   `filter_vertical`'s real, currently-shipping SIMD-generic helper functions
   (`filter_vertical_impl`, `reversible_filter_53r_simd`, `irreversible_filter_97i_simd`,
   `interleave_samples_inner`) carries `#[inline(always)]` on the function itself. Candidate 002's
   patch (reapplied here) does **not** put `#[inline(always)]` on
   `irreversible_filter_97i_horizontal_simd<S: Simd>` - only on its two small
   deinterleave/interleave helpers, not the function containing the `f32x8` arithmetic. Tested
   directly: added the one missing `#[inline(always)]` and rebuilt.

| Variant | `__fearless_simd_kernel` calls in `irreversible_filter_97i_horizontal_simd::<Avx512>` | single-core p50 (sample.jp2 full decode, `taskset -c 2`, 200 iter) | output hash |
|---|---|---|---|
| `fearless_simd` 0.6.0 (current pin), no `#[inline(always)]` on the outer fn (candidate 002/013 as originally patched) | 28 | 22.24ms / 22.40ms (2 runs) | `339b9db71ea46bfc` |
| `fearless_simd` 0.7.0 (newest release), same source, no `#[inline(always)]` | 28 (identical call sites) | 22.17ms / 21.14ms (2 runs); 21.93ms unpinned | `339b9db71ea46bfc` |
| `fearless_simd` 0.6.0, **with** the one missing `#[inline(always)]` added | **0** - function fully inlines into `filter_horizontal`'s `vectorize_avx512` closure body | **10.27ms / 10.25ms** (2 runs) | `339b9db71ea46bfc` |

(`evidence/call_count_summary.txt` + the three full `.asm` dumps back these counts;
`nm --demangle`/`objdump -d --demangle` on the real `bench_idwt` binary in each configuration.)

## Result

**Upstream status: already known, not yet fixed in any released version.** `fearless_simd`
0.7.0 (the newest release) reproduces the exact same 28 real out-of-line kernel calls as 0.6.0 -
bumping the dependency alone does not fix it. The general class of bug - a `fn f<S: Simd>(...)`
that silently stops inlining its SIMD ops with no compiler warning when the required
`#[inline(always)]` is missing or a related complexity threshold is crossed - is tracked upstream
by [#338](https://github.com/linebender/fearless_simd/issues/338) (open) with an in-progress,
unreleased fix ([#347](https://github.com/linebender/fearless_simd/pull/347), an opt-in `#[simd]`
proc macro that would make this mistake structurally impossible). Since it's already known and
actively being worked, **no new upstream issue was filed** - filing a duplicate would be noise,
per the task's own "if not [known], file it" condition not being met.

**However, this session's own empirical check (item 3 above) found the *actual* mechanism behind
candidate 013's specific observation, and it is simpler and more mundane than 013's
"LLVM inliner size/complexity threshold" hypothesis: `irreversible_filter_97i_horizontal_simd`
is simply missing the `#[inline(always)]` attribute that every other real, working
`<S: Simd>`-generic function in the same file has.** Adding that single attribute:
- drops the real call count from 28 to **0** (full inlining, matching `filter_vertical`'s shape),
- drops the full `sample.jp2` decode single-core p50 from ~22.2ms to ~10.25ms (**~2.17x**, on the
  *whole* decode, not just the horizontal filter in isolation - note this benchmark's total time
  includes `filter_vertical` and other unrelated work too, so the horizontal filter's own
  contribution improved by more than 2.17x),
- produces a bit-exact identical output hash (`339b9db71ea46bfc`) before and after.

This is exactly the failure mode #338 describes ("error-prone and hard to teach", no compiler
warning) - not a new, different bug. It directly explains candidate 013's finding without needing
013's own unconfirmed inliner-threshold hypothesis, and substantially undercuts candidate 002's
original "regressed 8.6x" conclusion, since that measurement was against code missing this
attribute. Flagged as `NewIdeasNoticed` in this session's final report rather than pursued further
here, since re-validating candidate 002's rewrite end-to-end (not just this one function) is a
distinct candidate's worth of work, out of this candidate's own assigned scope.

## Does it change behavior/output?

N/A - measurement/investigation only, no patch applied to real source. `patch.diff` reapplies
candidate 013's own patch (candidate 002's rewrite + 013's diagnostic test module) so the function
under investigation exists to inspect; it and the one-line `#[inline(always)]` experiment were
both reverted from the worktree before this folder was written (`git status` clean in
`/tmp/hayro_wt_036`, the isolated worktree used).

- **Bit-exact?** Yes, across all three configurations measured (same `sample.jp2` decode output
  hash `339b9db71ea46bfc`).
- **Is this a breaking change?** N/A, nothing applied to real source.
- Project test suite: `cargo test -p hayro-jpeg2000 -p hayro-jbig2 --release --lib` (unit tests
  only, unmodified real source) - **`fearless_simd` 0.6.0 (baseline)**: `hayro-jpeg2000` 1
  passed/0 failed, `hayro-jbig2` 2 passed/0 failed. **`fearless_simd` 0.7.0 (bumped, unmodified
  hayro source otherwise)**: identical, 1 passed/0 failed and 2 passed/0 failed - no new failures
  from the dependency bump alone. Both configurations' `--test asset_suite` integration tests fail
  the same way in this environment before *and* after (`missing test input
  serenity/buggie-gray.jpf` / jbig2's own asset dir) - a pre-existing local environment gap (no
  `python sync.py` run to fetch external test assets), unrelated to `fearless_simd`'s version,
  same under both versions.
- One incidental, harmless build-time warning surfaced under 0.7.0 only: `hayro-jbig2/src/simd.rs`
  imports `SimdInt` which becomes unused after 0.7.0's breaking API move of shared ops onto
  `SimdBase` (changelog: "Breaking change: Operations shared by integer and floating-point
  vectors have moved from `SimdInt`/`SimdFloat` to `SimdBase`..."). Not a functional bug, just
  something a real version-bump PR would need to clean up (drop the unused import) - noted here,
  not worth its own bughunt entry.

## Why the result looks the way it does

`fearless_simd`'s v1.0 stabilization work is still in progress (0.7.0 is pre-1.0, breaking
changes every release per its changelog); the ergonomic-inlining problem #338 describes is a
known, acknowledged, unsolved design gap, not an overlooked bug - the maintainers' own proposed
fix (`#[simd]` macro, #347) is real work in flight, just not released yet. Separately, candidate
002's original patch simply missed applying the project's own established convention (every other
`<S: Simd>`-generic helper in `idwt.rs` is `#[inline(always)]`) to the one function it added that
needed it - an easy mistake to make precisely because, per #338, the compiler gives no signal when
it's missing.

## What's not done / open questions

- No upstream issue/PR filed (see "Result" - the conditions for filing weren't met: already known,
  fix in progress).
- The `#[inline(always)]` fix was validated only for `irreversible_filter_97i_horizontal_simd`
  (the `Irreversible97` transform path) with the full end-to-end `sample.jp2` decode as the
  correctness/timing proxy - `reversible_filter_53r_horizontal_simd` (the `Reversible53`/lossless
  path, also introduced by candidate 002 without `#[inline(always)]`) was not separately checked
  for the same missing attribute, though the same fix likely applies there too by the same
  reasoning.
- This candidate's own scope (upstream check) is complete; whether candidate 002's rewrite,
  fixed this way, is actually worth applying for real (accounting for the `evens`/`odds`
  deinterleave buffers' own allocation/copy overhead, which this candidate did not re-isolate) is
  a distinct, not-yet-run candidate - see `NewIdeasNoticed` in this session's final report.
