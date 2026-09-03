# 020.sse_comi_ucomi_unimplemented - all 24 SSE/SSE2 scalar compare-and-set-flags intrinsics are missing

`src/intrinsics/llvm_x86.rs` has no arm for `llvm.x86.sse.{,u}comi{eq,lt,le,gt,ge,neq}.ss` or
`llvm.x86.sse2.{,u}comi{eq,lt,le,gt,ge,neq}.sd`, so all 24 fall into the file's terminal `_ =>`
catch-all: a compile-time *warning* plus `codegen_panic_nounwind`. The crate builds successfully
and the produced program aborts the first time it executes one. That covers the 24 public,
stable-since-1.27 functions `_mm_comieq_ss` / `_mm_comilt_sd` / `_mm_ucomige_ss` / ... which are
SSE and SSE2 baseline on x86_64 - callable from safe-ish stable code on every x86-64 CPU with no
`target_feature` gating and no `is_x86_feature_detected!` guard. Lead: candidate 017's approach-2
sweep used these 24 as controls while fixing the sibling `cmp.ss`/`cmp.sd` gap, found they all
abort too, and left them as a separate suspect ("What's not done" in 017's RESULT.md).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| 1. Differential test vs LLVM on one hand-written case - `___BUGHUNT___/tools/diffcheck.sh` | single-file `_mm_comieq_ss(set_ps(4,3,2,1), set_ps(4,0,5,1))`, opt-level 0, main tree `dist/` @2bc05d45 | Reproduced immediately. LLVM prints `comieq_ss = 1`, exit 0. cg_clif warns `unsupported x86 llvm intrinsic llvm.x86.sse.comieq.ss; replacing with trap`, the binary exits 134 (SIGABRT) with `llvm.x86.sse.comieq.ss is not yet supported`. |
| 2. Generated sweep over the whole family with boundary values - new script `___BUGHUNT___/tools/comi_ucomi_sweep.py` | all 24 intrinsics, **one single-file program per intrinsic** (an abort kills the process, so one program per case is mandatory), 15 lane-0 boundary pairs each (equal / less / greater / QNaN in a / QNaN in b / QNaN in both / `-0.0` vs `0.0` both ways / `inf` vs `inf` / `-inf` vs `inf` / `inf` vs `MAX` / `MIN_POSITIVE` vs `0.0` / two negatives / smallest subnormal / signalling NaN), upper lanes deliberately made *different* between the two operands, opt-level 0 | `identical=0 abort=24 wrong=0 nobuild=0`. Every one of the 24 aborts; the compile log names exactly the 24 distinct missing intrinsics. |
| 3. `-O` and `-Ctarget-cpu=native` paths | `OPT=1` / `RUSTFLAGS_EXTRA=-Ctarget-cpu=native` on 2 representative programs (4 combinations) | Reproduces at `-O` too - the trap is emitted at codegen time and then executed, so MIR GVN const-folding does not mask it (unlike candidate 005's ICE). All 4 differ from LLVM. |
| 4. Standalone `repro/` crate through the real `dist/cargo-clif` | own `Cargo.toml`, bare `[workspace]`, `[profile.*] opt-level = 0`; part 1 = 12 f32 forms x 13 boundary pairs, part 2 = 12 f64 forms x 13 boundary pairs, part 3 = two realistic callers (a NaN-skipping window count + minimum search built on `_mm_ucomige_ss`/`_mm_ucomile_ss`/`_mm_ucomilt_ss`, and a `_mm_comineq_sd` adjacent-difference count) | 5/5 runs of the unpatched binary abort with exit 134, before printing a single result line. LLVM: 317 lines, exit 0. After the patch: 5/5 runs exit 0 and the output is byte-identical to LLVM's. |
| 5. Randomized differential test after the fix (to check the *values*, not just that it stops aborting) | one program calling all 24 intrinsics, 4000 iterations x (12 f32 + 12 f64) = 96000 evaluations; operands from a fixed LCG with the exponent field biased towards 0 / all-ones / 0x7f so NaN, infinity, subnormals and zeros dominate; results folded into a printed hash, opt-level 0 plus `-O` plus `-Ctarget-cpu=native` | Identical to LLVM in all three configurations (`acc32=0x178fd02575170f92 acc64=0x4f7b37b84df75514` on both backends). |

Not used, deliberately: `miri` (interprets MIR, never reaches codegen) and ASAN/UBSAN (they
instrument the compiler binary, not the code it emits) - neither can observe a backend lowering
gap.

## Status

**Confirmed**

- Unpatched cg_clif: 5/5 runs of the same `repro/` binary abort, exit 134, always at the very
  first `_mm_comieq_ss` call. Deterministic - the abort is an unconditional trap emitted at
  compile time in place of the call, so there is nothing input- or layout-dependent about it.
- 24/24 sweep programs differ from LLVM before the patch; 0/24 after it.
- The 24 compile-time warnings appear on every build and the crate still builds successfully
  (exit 0), so the failure only surfaces at run time.

**Fully reproducible**: **Yes** - `repro/` is a normal Cargo crate built by the real
`dist/cargo-clif`, and the real produced binary is executed; the only thing that differs between
the before and after runs is which cg_clif built it. `output.txt` is that binary's genuine
captured output (LLVM's 317 lines, the unpatched abort, and the patched run) plus the real sweep
tallies.

## Class

crash

## Root cause

`codegen_x86_llvm_intrinsic_call` in `src/intrinsics/llvm_x86.rs` simply never mentions these 24
names. Execution therefore reaches the terminal `_ =>` arm (llvm_x86.rs:1349-1360 at 2bc05d45),
which emits `fx.tcx.dcx().warn("unsupported x86 llvm intrinsic ...; replacing with trap")` and
then `crate::base::codegen_panic_nounwind`. Because that is a warning and not an error, the crate
compiles green and the missing lowering only shows up as a `SIGABRT` when the call is reached at
run time.

Nothing about the lowering is hard, which is why the gap is a plain omission rather than a design
problem. `COMISS`/`COMISD`/`UCOMISS`/`UCOMISD` compare lane 0 of the two operands, ignore the
upper lanes, and the intrinsic returns the corresponding x86 flag test as a plain 0/1 `i32`. The
six predicates map exactly onto six Cranelift `FloatCC` values:

- `comieq`/`ucomieq` -> `FloatCC::Equal` (EQ)
- `comilt`/`ucomilt` -> `FloatCC::LessThan` (LT)
- `comile`/`ucomile` -> `FloatCC::LessThanOrEqual` (LT | EQ)
- `comigt`/`ucomigt` -> `FloatCC::GreaterThan` (GT)
- `comige`/`ucomige` -> `FloatCC::GreaterThanOrEqual` (GT | EQ)
- `comineq`/`ucomineq` -> `FloatCC::NotEqual` (UN | LT | GT)

Only the last one is true for unordered operands. That was checked against LLVM rather than
assumed: LLVM's own output for `_mm_comineq_ss(NaN, 1.0)` is 1 while all five other predicates
give 0 (see `output.txt` part 1), i.e. LLVM implements it as an unordered-or-not-equal test, which
is precisely what Cranelift spells `FloatCC::NotEqual`. `comi` and `ucomi` differ only in whether
a *quiet* NaN raises the invalid-operation exception; the returned value is defined identically
for both, and Cranelift has no FP-exception model at all, so both spellings lower the same way.

## Introduced by

Never implemented - none of the 24 names has ever appeared anywhere in this repository's history
(`git log -S comieq --all`, `git log -S ucomieq --all` and `git log -S comisd --all` all return no
commits). The commit that decides what happens *instead* is:

1a268911 "Replace llvm intrinsics with runtime trap" (2019-07-28) - it introduced the `_ =>`
warn + run-time-trap fallback, which is why an unimplemented x86 intrinsic aborts the produced
program at run time rather than failing the build. The arm was carried forward through
98a276b5 "Merge commit '2bb3996244cf1b89878da9e39841e9f6bf061602' into sync_cg_clif-2022-12-14"
(2022-12-14), which is what `git blame` points at for the `_ =>` line itself.

## The fix

`patch.diff` (+52/-0, one file, `src/intrinsics/llvm_x86.rs`), rustfmt-clean, applies cleanly to a
fresh clone at base_commit 2bc05d45 (verified with `git apply` against a real
`git clone --no-local` checkout).

One new match arm listing all 24 intrinsic names, placed right after the existing
`llvm.x86.sse.cmp.ps | llvm.x86.sse2.cmp.pd` arm. It:

- picks the `FloatCC` from the second-to-last dot-separated component of the intrinsic name
  (`intrinsic.rsplit('.').nth(1)`, i.e. `comieq`/`ucomieq`/...), so one arm covers the
  ss/sd x comi/ucomi cross-product without 24 duplicated bodies;
- asserts the return type is `i32` (the same shape check the neighbouring
  `llvm.x86.avx.ptestz.256` arm makes);
- reads lane 0 of both operands with `value_lane(fx, 0).load_scalar(fx)`, emits one `fcmp`, and
  `uextend`s the `i8` result to `I32` - again the same `icmp -> uextend -> CValue::by_val ->
  ret.write_cvalue` shape `ptestz.256` already uses.

No existing code path is touched (the diff is pure addition), so the packed compares and every
other intrinsic are unaffected by construction.

Checked well beyond the one triggering input (CONFIRMATION.md section 7):

| input class | unpatched | patched |
|---|---|---|
| all 24 intrinsics x 15 lane-0 boundary pairs, one program each (`comi_ucomi_sweep.py`) | 24/24 abort | `identical=24 abort=0 wrong=0` |
| both lane counts / element widths (`f32x4` via `.ss`, `f64x2` via `.sd`) | abort | identical |
| QNaN in either operand, in both, signalling NaN, `-0.0` vs `0.0` both directions, `±inf`, `inf` vs `MAX`, subnormal vs `0.0`, smallest subnormal | abort | identical (including the `neq`-is-true-when-unordered asymmetry) |
| upper lanes made different between the two operands (catches a lowering that reads the wrong lane) | abort | identical |
| 96000 randomized evaluations over NaN/inf/subnormal-biased bit patterns, all 24 intrinsics | abort | identical hash on both backends |
| `-O`, and `-Ctarget-cpu=native` | abort | identical (checked on 4 sweep programs and on the randomized program) |
| `repro/`, real `dist/cargo-clif` binary, 317 output lines | exit 134, 5/5 | exit 0 and byte-identical to LLVM, 5/5 |

- Project test suite: `./y.sh test` in the worktree (full run, including the `download/` external
  repos) - **before the patch: pass, exit 0**; **after the patch: pass, exit 0**. Both runs: 35
  items (13 `[AOT]`, 6 `[BUILD]`, 6 `[COPY]`, 2 `[JIT]`, 3 `[PATCH]`, 5 `[TEST]`), 61
  `test result: ok` blocks, 0 `FAILED`; the two runs' item lines were diffed and are identical.
  The before run was taken on a `git status --short`-clean tree (the patch was stashed for it).
  `./y.sh test` aborts on the first failure, so exit 0 means every item ran.
  - Note on how that was measured, because it bit once: the first post-patch run was invalid. The
    `git stash pop` that restored the patch and the following `./y.sh build` happened inside the
    same wall-clock second, so cargo treated `rustc_codegen_cranelift` as fresh and did not rebuild
    it - `dist/lib/librustc_codegen_cranelift.so` kept its pre-stash-pop mtime and the "after" run
    silently used the *unpatched* backend. Caught by re-running the repro (which aborted again),
    fixed by re-invoking `./y.sh build` (which then really did recompile the backend), and the
    post-patch `./y.sh test` above is the re-run against the genuinely patched `.so`, with its
    timestamp checked against the source file's before and after.

## What's not done / open questions

- **Signalling vs quiet semantics are not modelled and cannot be.** `comi` should raise the
  invalid-operation exception on a quiet NaN and `ucomi` should not; Cranelift has no FP-exception
  model, so the patch lowers both identically. That matches the intrinsics' *return values*
  exactly (verified over 96000 random cases), but a program that reads `MXCSR` afterwards, or one
  compiled with a strict-FP mode that ever reaches cg_clif, would not see the exception flag. The
  same limitation already applies to the `_OS`/`_US` vs `_OQ`/`_UQ` predicates of the packed
  compares and to candidate 017's scalar `cmp.ss`/`cmp.sd` patch.
- The sibling scalar compares `llvm.x86.sse.cmp.ss` / `llvm.x86.sse2.cmp.sd` are still missing on
  this branch - that is candidate 017, whose patch is independent of this one (this patch adds a
  new arm and does not touch the `cmp.ps`/`cmp.pd` arm that 017 modifies, so the two apply
  together without a merge conflict). A maintainer taking both may want a single follow-up that
  covers the whole SSE/SSE2 scalar-compare surface.
- Only these 24 names were implemented. The AVX-512 `vcomi{sd,ss}` / `vcomi{sh}` and the FP16
  `llvm.x86.avx512fp16.vcomi.sh` relatives, and the `_mm_comieq_*` equivalents that go through
  `llvm.x86.avx512.*` mask registers, were not looked at; the hodow audit's 701-strong
  unsupported-intrinsic inventory is the place to enumerate them.
- The `_ =>` warn-and-trap fallback itself remains the real hazard: a crate using any of 701
  unimplemented `llvm.x86.*` intrinsics builds green and dies at run time. That design question
  (warn+trap vs hard error) is out of scope here and is worth its own upstream discussion.
- The candidate `justfile`'s `clone` / `build-clif` recipes could not be executed in this sandbox
  (`git clone git@github.com:...` -> no outbound ssh). Their equivalent was validated for real: a
  local `git clone --no-local` checked out at `base_commit` accepts `patch.diff` with `git apply`
  cleanly (+52/-0), and `just llvm`, `just clif` and `just compare` were all run for real with
  `CLIF_ROOT` pointed first at an unpatched cg_clif (abort, `compare` fails with all 317 lines
  missing) and then at the patched worktree (`IDENTICAL`).
