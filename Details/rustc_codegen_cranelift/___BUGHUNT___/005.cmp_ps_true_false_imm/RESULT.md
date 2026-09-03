# 005.cmp_ps_true_false_imm - `todo!()` ICE for the `_CMP_FALSE_*` / `_CMP_TRUE_*` compare immediates

`src/intrinsics/llvm_x86.rs` lowers `llvm.x86.sse.cmp.ps` / `llvm.x86.sse2.cmp.pd` (i.e.
`_mm_cmp_ps` / `_mm_cmp_pd`, and every `_mm_cmpXX_ps`/`_pd` that funnels into them) by mapping the
5-bit predicate immediate onto a Cranelift `FloatCC`. Two of the sixteen mapping arms were literal
`todo!()`, at lines 232 and 236. A `todo!()` in a codegen backend is an ICE, not a diagnostic. The
spot was found by code reading during the 2026-09-02 hodow audit; hodow never generates those four
immediates, so the 19k-test audit never hit it.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| 1. Differential test vs LLVM, hand-written case | `___BUGHUNT___/tools/diffcheck.sh` on a single file calling `_mm_cmp_ps::<_CMP_TRUE_UQ>` and `::<_CMP_FALSE_OQ>`, opt-level 0, main tree `dist/` @2bc05d45 | Reproduced immediately. LLVM prints all-ones / all-zero lanes; cg_clif ICEs: `panicked at src/intrinsics/llvm_x86.rs:232:50: not yet implemented`, backtrace through `codegen_x86_llvm_intrinsic_call`. |
| 2. Boundary-value construction around the suspect operation | NaN / `-0.0` vs `0.0` / `±inf` operand sets, so every predicate produces a distinct lane mask instead of accidentally agreeing | The four immediates 0x0b, 0x0f, 0x1b, 0x1f ICE regardless of operands; the other 28 give bit-identical results to LLVM on all of those operand sets. |
| 3. Generated sweep of the whole immediate space | new script `___BUGHUNT___/tools/cmp_imm_sweep.py` - all 32 immediates x `_mm_cmp_ps`/`_mm_cmp_pd`, one program per immediate (an ICE aborts the crate, so a single 32-way program would only ever show the first failure), each diffed against LLVM | Pristine worktree build: **identical=56, ice=8, wrong=0** - exactly `{0x0b, 0x0f, 0x1b, 0x1f}` x `{ps, pd}`. No additional wrong-value mapping among the 28 implemented predicates. |
| 3b. Same sweep extended to `_mm_cmp_ss` / `_mm_cmp_sd` | same script, `--only` omitted | A *different*, pre-existing gap, not this bug: `llvm.x86.sse.cmp.ss` / `llvm.x86.sse2.cmp.sd` are not implemented at all, so all 64 of those cases compile with a warning and then abort at run time via `codegen_panic_nounwind`. Recorded under "What's not done" rather than folded into this candidate. |
| 4. `-O` and `-Ctarget-cpu=native` paths after the fix | `OPT=1` and `RUSTFLAGS_EXTRA=-Ctarget-cpu=native` on the eight previously-ICEing programs | All 8 identical on both paths as well. (At `-O` the ICE is *hidden* on the pristine build - MIR GVN const-folds the intrinsic through the const-eval interpreter, so the backend is never asked to lower it. Only opt-level 0 exposes it.) |

## Status

**Confirmed** - 3/3 identical runs of the pristine build (the two sweep runs, on the main tree
`dist/` and on the freshly built pristine worktree `dist/`, plus the standalone `repro/` run), all
opt-level 0, cg_clif @2bc05d45 with rustc nightly-2026-08-24. Deterministic: it is a `todo!()` on a
compile-time-constant immediate, so there is nothing for it to be flaky about. It does **not**
reproduce at `-O` for the reason in row 4 above.

**Fully reproducible**: **Yes**. `repro/` is a standalone crate (bare `[workspace]`, `opt-level =
0`) whose real binary is built by `dist/cargo-clif`; the captured `output.txt` is the genuine
compiler output - the real backend really ICEs while codegening it, and after the patch the real
generated binary really runs and prints exactly LLVM's lane masks.

## Class

crash (backend ICE on valid Rust)

## Root cause

The predicate-immediate mapping tries to express all 32 x86 compare predicates as a Cranelift
`FloatCC` and then emit a single `fcmp`. Four of them cannot be expressed that way: `_CMP_FALSE_OQ`
(0x0b), `_CMP_TRUE_UQ` (0x0f), `_CMP_FALSE_OS` (0x1b) and `_CMP_TRUE_US` (0x1f) are the constant
predicates - always-false and always-true irrespective of the operands, NaN included - and
Cranelift's `FloatCC` enum has no always-false/always-true variant to map them onto. Rather than
bypassing `fcmp` for those, the two arms were left as `todo!()`. Because the arm's value is needed
before any code is emitted, reaching either one panics inside
`codegen_x86_llvm_intrinsic_call`, which surfaces as "the compiler unexpectedly panicked" and
aborts the whole crate.

## Introduced by

e8168ce8 "Merge commit '1eded3619d0e55d57521a259bf27a03906fdfad0' into sync_cg_clif-2023-07-22"
(2023-07-22) - `git log -S` and `git blame -L 232,232` both point at that commit, which is the
subtree sync that first brought the `llvm.x86.sse.cmp.ps` / `sse2.cmp.pd` implementation (including
both `todo!()`s) into this repository. The original authoring commit lives in the rust-lang/rust
monorepo, not in this repo's history, so a finer attribution is not available from this checkout.

## The fix

`patch.diff` replaces the bare `FloatCC` with a two-variant local `Predicate { Cc(FloatCC),
Const(bool) }`: the 28 expressible predicates keep mapping to a `FloatCC` and still emit `fcmp`,
while the four constant predicates map to `Const(false)` / `Const(true)` and the per-lane closure
emits `iconst.i8 0` / `iconst.i8 1` instead of an `fcmp`. That constant then flows through the
unchanged `bool_to_zero_or_max_uint`, so the lane mask (all-zero / all-ones, correctly widened and
bitcast to the float lane type) is produced by exactly the same code as before. 29 added, 18
removed, one file.

Verified beyond the single triggering input, per CONFIRMATION.md section 7: the full 64-case sweep
(all 32 immediates x `_mm_cmp_ps` and `_mm_cmp_pd`, each over the NaN / signed-zero / infinity
operand sets) goes from **identical=56 ice=8** on the pristine build to **identical=64 ice=0
wrong=0** on the patched build - i.e. both lane counts, both element widths, all four affected
immediates, and no regression in the 28 that already worked. The eight previously-ICEing programs
are also identical to LLVM under `-Ctarget-cpu=native` and under `-O`.

`repro/` before -> after (full text in `output.txt`): pristine cg_clif never produces a binary
(`panicked at src/intrinsics/llvm_x86.rs:232:50: not yet implemented`, rustc exit 101); patched
cg_clif produces a binary that prints `[00000000, ...]` for the two FALSE predicates and
`[ffffffff, ...]` for the two TRUE predicates, byte-for-byte equal to the LLVM oracle, on 2/2
consecutive runs.

- Project test suite: `./y.sh test` in the worktree - **passing before -> passing after** (exit 0
  both times, 61 `test result: ok` blocks, 0 failures either side). The before run was deliberately
  redone on a `git checkout -- src/`-clean tree after an earlier baseline run had raced with the
  edits, so the recorded baseline is genuinely pristine.

## What's not done / open questions

- `llvm.x86.sse.cmp.ss` and `llvm.x86.sse2.cmp.sd` (`_mm_cmp_ss` / `_mm_cmp_sd`) are not
  implemented at all - all 32 immediates each compile with a warning and abort at run time. That is
  the known "unimplemented `llvm.x86.*`" family, a separate finding; this patch does not touch it.
  The sweep script covers them (`--only=ss,sd`) if someone implements them.
- The 256-bit forms (`llvm.x86.avx.cmp.ps.256` / `cmp.pd.256`, `_mm256_cmp_ps`/`_pd`) were not
  examined here; if they are ever implemented they will need the same constant-predicate handling,
  since the `FloatCC` limitation is identical.
- The local `Predicate` enum is declared inside the match arm to keep the diff minimal. A
  maintainer may prefer it hoisted to module scope, or prefer emitting the lane mask directly
  (`iconst` of 0 / -1 into the result lane type) instead of routing a constant bool through
  `bool_to_zero_or_max_uint`. Both are cosmetic - the observable output is already bit-exact.
- Not audited: whether any of the 28 `FloatCC` mappings differ from x86 semantics in the
  *signaling* dimension (`_OS`/`_US` vs `_OQ`/`_UQ`). Cranelift has no FP-exception model, so the
  sweep cannot see a difference there; it only proves the lane masks agree.
