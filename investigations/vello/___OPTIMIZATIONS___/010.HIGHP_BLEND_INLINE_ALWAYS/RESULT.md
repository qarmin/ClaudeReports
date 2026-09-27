# highp blend path: is open PR #1579's remaining `#[inline(always)]` still worth anything?

Candidate 010 was assigned to check open PR
[linebender/vello#1579](https://github.com/linebender/vello/pull/1579) ("Add vectorisation to
some suspicious blend related missing parts"), which adds `#[inline(always)]` to
`highp::blend::mix`, `highp::compose::ComposeExt::compose`, and wraps `highp::mod::fill::blend`'s
loop body in `simd.vectorize` - all in `sparse_strips/vello_cpu/src/fine/highp/`. Toolchain:
`rustc 1.98.0 (88d9e12ae 2026-08-18)`, `cargo 1.98.0`.

## What was tried

**`gh pr diff 1579` does not apply to `base_commit`** (`git apply --check` fails on all three
touched files: `blend.rs`, `compose.rs`, `mod.rs`). Reading the current code explained why: PR
[#1688](https://github.com/linebender/vello/pull/1688) ("Better inlining of SIMD kernels for
improved x86 performance"), already merged into `base_commit`, independently rewrote all three
files far more thoroughly than #1579 proposed:

- `highp::blend::mix` now delegates to a separate `mix_inner` wrapped in `simd.vectorize`
  (`#[inline(always)]` on the closure, not on `mix` itself).
- `highp::compose::ComposeExt::compose` likewise delegates to `compose_inner` inside
  `simd.vectorize`.
- **Both** `fill::blend` and `alpha_fill::blend` in `mod.rs` already wrap their per-pixel loops in
  `simd.vectorize` (PR #1579 only proposed this for `fill::blend`; the current code already has it
  for `alpha_fill::blend` too, which #1579 never touched).

So the specific vectorization gap #1579 was written against no longer exists at `base_commit` -
it was closed by a different, later PR before #1579 could be reviewed. #1688's own description
explains a deliberate design choice that's directly relevant: outer wrapper functions (`mix`,
`compose`, `blend`) are **not** marked `#[inline(always)]` on purpose, because the `vectorize` call
already happens at the right place inside them - "we just call `fill` and any method inside of
that will have a corresponding vectorize call, so that should be enough. This way, we also don't
have to inline it." That is exactly the remaining gap between #1579's diff and the current code:
#1579 still wants `#[inline(always)]` on the four outer wrapper functions themselves
(`blend::mix`, `ComposeExt::compose`, `fill::blend`, `alpha_fill::blend`), which #1688's author
considered unnecessary.

This candidate tests that specific, narrower, still-open question for real: **does adding
`#[inline(always)]` to those four outer functions change anything**, given everything inside them
is already vectorized? `patch.diff` adds exactly those four attributes (verified with
`git apply --check` against `base_commit`, applies cleanly).

Benchmark: the project's own `sparse_strips/vello_bench` `fine_blend` criterion group already
benches every blend mode, but only for the `U8Kernel` (`#[vello_bench]`'s `f32`/`F32Kernel`
variants are commented out project-wide, same situation as candidate 005). `harness.diff` adds two
small, standalone bench functions in `vello_bench/src/fine/blend.rs`: `multiply_f32` (exercises
`highp::blend::mix`'s separable-`Mix` code path) and `src_atop_f32` (`Mix::Normal` + `Compose`,
exercises `highp::compose::ComposeExt::compose` while `mix` short-circuits), each run at every
available SIMD level on this x86_64 host (`sse4_2`, `avx2`). `harness.diff` was applied for both
the baseline and patched builds, so the only difference between the two is `patch.diff` itself.

**The baseline and patched release binaries are byte-identical** (`md5sum` on
`target/release/deps/main-*`, confirmed twice: once in the isolated worktree used for this
candidate's own measurement, and independently again from a completely fresh
`git clone` + `git checkout base_commit` in a separate scratch directory, both times the same
result). Adding `#[inline(always)]` to these four functions produced exactly zero change in
generated machine code - LLVM was already inlining all four, `#[inline(always)]` here is a no-op
hint confirming what the compiler had already decided.

Given identical binaries, single-core (`taskset -c 3`, `flock`-serialized against sibling
candidates' own builds/benchmarks in the same batch session) and multi-core (unpinned) numbers
below are two runs of **the same binary**, included only to show the noise floor this candidate
was measured against - they are not evidence of any real delta, since there is none to find.

| Variant (single-core, `taskset -c 3`) | baseline run 1 | baseline run 2 (repeat) | "patched" (same binary) |
|---|---|---|---|
| `multiply_f32_sse4_2` | 12.59 µs | 12.24 µs | 13.04 µs |
| `multiply_f32_avx2` | 11.83 µs | 11.72 µs | 11.90 µs |
| `src_atop_f32_sse4_2` | 4.96 µs | 5.19 µs | 4.90 µs |
| `src_atop_f32_avx2` | 5.19 µs | 5.20 µs | 5.23 µs |

| Variant (unpinned, shared machine under sibling-candidate load) | baseline | "patched" (same binary) |
|---|---|---|
| `multiply_f32_sse4_2` | 13.19 µs | 13.12 µs |
| `multiply_f32_avx2` | 12.19 µs | 12.01 µs |
| `src_atop_f32_sse4_2` | 6.85 µs | 5.00 µs |
| `src_atop_f32_avx2` | 5.54 µs | 5.25 µs |

(The `src_atop_f32` unpinned swing - baseline 6.85 µs vs. "patched" 5.00 µs, a ~27% apparent
"improvement" - is pure shared-machine noise from sibling candidates' own builds/benches running
concurrently outside the `flock`-serialized measurement window; it disappears entirely in the
single-core numbers above, and both numbers come from the identical binary, so it cannot reflect
a real code difference.)

## Result

Time only. **No change, by construction**: the baseline and patched binaries are byte-identical,
so there is no delta to measure - the numbers above are the same binary benchmarked twice, and
their spread (up to ~30% on `src_atop_f32` unpinned, ~5-8% single-core) is exactly the kind of
same-binary noise floor this project's other candidates in this batch also observed on this shared
24-core machine.

## Does it change behavior/output?

- **Bit-exact?** Yes - not just in output, in the compiled binary itself (byte-identical
  `target/release/deps/main-*`, confirmed via `md5sum` from two independent builds).
- **Is this a breaking change?** No - no public API changed, and no machine code changed either.
- **Project test suite** (`cargo test --release -p vello_cpu --features f32_pipeline --lib`):
  - Before (harness-only, no `patch.diff`): 81 passed, 0 failed.
  - After (`patch.diff` applied): 81 passed, 0 failed. Identical, as expected from an identical
    binary.
  - `vello_sparse_tests` (the crate with real snapshot tests exercising the highp/`OptimizeQuality`
    path) was not run: this sandbox has no `git-lfs` binary (same pre-existing, unrelated
    limitation candidate 005 already documented), so its reference PNGs check out as LFS pointer
    text, not real images, regardless of this patch.

## Why the result looks the way it does

The premise of the assigned candidate (open PR #1579) no longer matches current `HEAD`: the
specific missing-vectorization problem it was written against was already independently fixed,
more thoroughly, by PR #1688 before this session started - #1688's own PR description even reports
"85%-95% better performance for blending" from that work, which is presumably most of what #1579
was chasing. What's left of #1579's diff after accounting for what #1688 already did is four
`#[inline(always)]` attributes on outer wrapper functions whose bodies are already fully vectorized
- and #1688's author explicitly designed it that way on purpose (see the quoted PR description
above): the `simd.vectorize` call already happens at the right call site inside each wrapper, so
the wrapper itself doesn't need forcing. LLVM agrees emphatically: it already inlines all four
functions with no attribute present, so adding the attribute changes nothing in the compiled
output. This is a clean, decisive "no-op" finding, not a noisy one - byte-identical binaries are
about as strong a "no change" result as this kind of investigation can produce.

## What's not done / open questions

- PR #1579 itself is still open on GitHub as of this check; a maintainer closing it as superseded
  by #1688 (rather than merging it) would match what this investigation found.
- Only `sse4_2`/`avx2` were exercised (x86_64 host); `neon` and the portable fallback use the same
  already-vectorized code paths but were not separately benchmarked here.
- `vello_sparse_tests`'s real snapshot suite could not be run due to missing `git-lfs` in this
  sandbox - unrelated to this candidate specifically (see candidate 005's own RESULT.md for the
  same limitation), but still the one remaining correctness check a maintainer with `git-lfs`
  installed could run for extra confidence, even though a byte-identical binary makes a behavior
  change here essentially impossible.
