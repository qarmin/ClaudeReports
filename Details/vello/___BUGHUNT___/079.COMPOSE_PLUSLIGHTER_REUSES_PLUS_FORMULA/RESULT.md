# COMPOSE_PLUSLIGHTER_REUSES_PLUS_FORMULA

`sparse_strips/vello_cpu/src/fine/highp/compose.rs:73` and `lowp/compose.rs:58` handle
`Compose::PlusLighter` by literally falling back to the `Compose::Plus` formula, with a comment
admitting "Have not been able to find a formula for this" (flagged by `clippy::match_same_arms`).
Looked suspicious as a genuine, admitted approximation diverging from spec-correct
plus-lighter compositing.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Find a reference implementation elsewhere in the same repo | Searched all `.rs`/`.wgsl`/`.wesl` files for `PlusLighter`/`COMPOSE_PLUS_LIGHTER` | Found a dedicated, non-fallback formula already implemented for the GPU path in both `vello_shaders/shader/shared/blend.wgsl` and `sparse_strips/vello_sparse_shaders/shaders/helpers/blend.wesl` (the latter is the shader `vello_hybrid` actually runs): `case COMPOSE_PLUS_LIGHTER: { return min(vec4(1.0), vec4(as_ * cs + ab * cb, as_ + ab)); }` (separated, non-premultiplied `cs`/`cb`, alphas `as_`/`ab`). |
| Existing cross-backend snapshot test (`cargo test -p vello_sparse_tests mix_compose_combined_test_matrix`, debug build, in a fresh `git worktree` clone at HEAD `75168314`) | This test's compose-mode matrix includes `Compose::PlusLighter` and runs the *same* golden PNG (`sparse_strips/vello_sparse_tests/snapshots/mix_compose_combined_test_matrix.png`) against every backend: `cpu_f32_{scalar,sse42,avx2}`, `cpu_u8_{scalar,sse42,avx2}`, `cpu_multithreaded`, and `hybrid` (the real wgpu/`blend.wesl` GPU path) | All 8 runnable backends pass (`neon` variants ignored, non-ARM host) - see `verification/output_cross_backend_test.txt`. The CPU fallback-to-`Plus` output for `PlusLighter` already matches the GPU's dedicated formula's output pixel-for-pixel (within the test's tolerance of 1-3). |
| Algebraic derivation + standalone differential/boundary-value script (`verification/diff_check.py`, Python 3, no project deps) reimplementing both formulas verbatim from source | Boundary grid over `{0, 1, 0.5, 1e-6, 1-1e-6}` for both alphas and all color channels (625 cases) + 200,000 uniform-random `(cs, as_, cb, ab)` samples in `[0,1]`, seed 12345 | 0 mismatches (tolerance `1e-9`), max observed per-channel diff `0.0` across all 200,625 cases - see `verification/output_diff_check.txt`. Proven, not just observed: substituting `src_c = as_*cs` (premultiplied), `bg_c = ab*cb` into the CPU `Plus` formula (`fa=fb=1`, saturate `[0,1]` applied uniformly to R,G,B *and* the alpha lane) yields exactly `clamp(as_*cs + ab*cb, 0, 1)` for RGB and `clamp(as_+ab, 0, 1)` for alpha - which is textually the GPU's dedicated `PlusLighter` formula. |
| Manual check of the `lowp` (u8-quantized) path's arithmetic | Read `sparse_strips/vello_cpu/src/fine/lowp/compose.rs` and `sparse_strips/vello_cpu/src/util.rs`'s `normalized_mul`/`div_255` | For `Plus`/`PlusLighter`, `fa`/`fb` are the constant `u8x32::splat(255)`, and `div_255` is documented (and tested) to be exact for values divisible by 255, so `src_c.normalized_mul(255) == src_c` and `fb.normalized_mul(bg_c) == bg_c` exactly - the u8 path reduces to `clamp(src_c + bg_c, 0, 255)` per channel, the same formula scaled to the u8 domain, not something quantization could newly diverge on beyond the 8-bit rounding already common to every other compose mode (already covered by the passing `cpu_u8_*` variants of the same test above). |

## Status

**Not reproducible.** The reused `Plus` formula is not an approximation that diverges from the
spec-correct `plus-lighter` formula - it is, algebraically, the exact same formula already used
by this project's own GPU shader for `PlusLighter`, and this is corroborated empirically by the
existing `mix_compose_combined_test_matrix` snapshot test (which already exercises
`Compose::PlusLighter` cross-backend, CPU vs. the real `wgpu`/`blend.wesl` GPU path, against one
shared golden image) passing on all 8 runnable backends, and by a from-scratch 200,625-case
boundary/differential check finding zero discrepancy.

No further tool escalation was warranted (soft cap of 3 approaches reached with a consistent,
reproducible negative signal at every step, not a partial/flaky one).

## Class

N/A (not reproduced).

## Root cause

N/A - not a bug. The `// Have not been able to find a formula for this` comment is stale/misleading
(the author apparently didn't realize the already-shipped `Plus` formula, which saturates *all
four* lanes (R, G, B, and alpha) to `[0, 1]`, is precisely CSS's/WebKit's `plus-lighter` blend
formula: `Co = min(1, Cs_premul + Cd_premul)`, `Ao = min(1, As + Ad)`), but the code itself already
computes the correct result, matching the dedicated formula this same repo's GPU shader uses.

## Introduced by

N/A - not a bug, no fix needed. (For context: the `Compose::PlusLighter => Plus::compose(...)`
fallback and its comment were introduced in cce9d918 "Rewrite `vello_cpu` to be more SIMD-friendly
(#1078)"; not tracked further since there is nothing to fix.)

## The fix

None needed/applied. The `clippy::match_same_arms`-flagged duplication is real (the two `match`
arms are textually identical), but removing the duplicate arm (e.g. via a `|`-pattern
`Compose::Plus | Compose::PlusLighter => Plus::compose(...)`) would be a pure lint-satisfying
refactor with no behavior change - not attempted here since no bug was found to fix, and the
project's own comment on the line signals this was already a deliberate choice, just for the wrong
stated reason.

- Project test suite: N/A, no fix was applied. The existing `mix_compose_combined_test_matrix` was
  run before this investigation for confirmation purposes only (8 passed, 0 failed, 2 ignored -
  see above); nothing was changed, so there is no meaningful before/after comparison here.

## What's not done / open questions

- The apparent discrepancy between `Compose::Plus` and `Compose::PlusLighter` *in the GPU shaders
  themselves* was noticed but is out of scope for this candidate (which is specifically about the
  CPU's `PlusLighter` reusing `Plus`): in both `blend.wgsl` and `blend.wesl`, `COMPOSE_PLUS` falls
  through to the generic bottom-of-function path that only clamps the alpha channel to `[0, 1]`
  and leaves the premultiplied RGB channels *unclamped*, whereas the dedicated
  `COMPOSE_PLUS_LIGHTER` case clamps RGB *and* alpha. That means the GPU's own `Plus` and
  `PlusLighter` are *not* identical to each other (unlike the CPU, where they are identical by
  construction, since CPU's `Plus` already saturates all four channels). Whether the GPU's
  unclamped-RGB `Plus` is itself correct/intentional (deliberately allowing values above 1 to
  survive further blending, e.g. for HDR-ish accumulation) or is a separate, un-investigated bug
  was not assessed here - added to `IDEAS.md`'s backlog as a new suspect for a future candidate
  rather than being chased down inside this one.
- The comment on the `Compose::PlusLighter` line could be updated to state the actual reasoning
  (a stale-comment cleanup, not a bug fix) - not done here since it's a documentation nit, not a
  behavior change, and this session's scope was confirm-or-refute the bug, not general cleanup.
