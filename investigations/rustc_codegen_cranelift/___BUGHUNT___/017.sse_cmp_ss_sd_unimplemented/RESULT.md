# 017.sse_cmp_ss_sd_unimplemented - the SSE/SSE2 *scalar* float compares are not lowered at all

`src/intrinsics/llvm_x86.rs` implements the packed float compares (`llvm.x86.sse.cmp.ps` /
`llvm.x86.sse2.cmp.pd`) but has no arm for their scalar siblings `llvm.x86.sse.cmp.ss` /
`llvm.x86.sse2.cmp.sd`, so both fall into the file's terminal `_ =>` catch-all: a compile-time
warning plus `codegen_panic_nounwind`, i.e. the crate builds "successfully" and the produced
program aborts the first time it executes one. That covers `_mm_cmp_ss::<IMM>` / `_mm_cmp_sd::<IMM>`
(AVX) and, more importantly, all 24 of the SSE/SSE2-baseline `_mm_cmpXX_ss` / `_mm_cmpXX_sd`
functions (`_mm_cmpeq_ss`, `_mm_cmplt_sd`, ...), which stdarch implements on top of the same two
LLVM intrinsics and which are callable from stable code on every x86-64 CPU with no
`target_feature` gating at all. Lead: candidate 005's row 3b noticed the gap while sweeping the
packed forms and left it as a separate suspect.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| 1. Generated immediate sweep vs LLVM - reused candidate 005's `___BUGHUNT___/tools/cmp_imm_sweep.py --only=ss,sd` | 32 immediates x `_mm_cmp_ss`/`_mm_cmp_sd`, one single-file program per immediate (an abort kills the process, so one program per case is mandatory), 3 operand sets each (ordinary, NaN, signed-zero/infinity), opt-level 0, main tree `dist/` @2bc05d45 | Reproduced on every case: **identical=0 wrong=64**. LLVM prints the lane masks and exits 0; cg_clif warns `unsupported x86 llvm intrinsic llvm.x86.sse.cmp.ss; replacing with trap` and the binary exits 134 (SIGABRT) with `llvm.x86.sse.cmp.ss is not yet supported`. |
| 2. Boundary-value construction over the whole *public* scalar-compare family - new script `___BUGHUNT___/tools/scalar_cmp_family_sweep.py` | all 24 `_mm_cmpXX_ss`/`_mm_cmpXX_sd` + the 24 `_mm_comiXX_*`/`_mm_ucomiXX_*` relatives as controls, one program each, 6 operand sets each (equal / greater / NaN in a / NaN in b / `-0.0` vs `0.0` / `±inf`), opt-level 0 | All 48 differ from LLVM by aborting. 26 distinct intrinsics are involved: `llvm.x86.sse.cmp.ss`, `llvm.x86.sse2.cmp.sd`, plus 24 `{,u}comi{eq,lt,le,gt,ge,neq}.{ss,sd}` - the latter are a *separate* unimplemented set (see "What's not done"). |
| 3. Standalone `repro/` crate through the real `dist/cargo-clif` | own `Cargo.toml`, bare `[workspace]`, `[profile.*] opt-level = 0`; part 1 = all 24 public SSE/SSE2 scalar compares, part 2 = `_mm_cmp_ss`/`_mm_cmp_sd` over all 28 FloatCC-expressible immediates, part 3 = a branch-free scalar clamp built out of `_mm_cmplt_ss`/`_mm_cmpgt_ss` masks plus a `_mm_cmpunord_sd` NaN count | 5/5 runs of the unpatched binary abort with exit 134, before printing a single result line. LLVM: 486 lines, exit 0. After the patch: 5/5 runs exit 0 and the run output is byte-identical to LLVM's. |
| 4. `-O` and `-Ctarget-cpu=native` paths | `OPT=1` / `RUSTFLAGS_EXTRA=-Ctarget-cpu=native` on four representative programs | Reproduces at `-O` as well (unlike candidate 005's ICE, which `-O` hides): the trap is emitted at codegen time and executed at run time, so MIR GVN const-folding the intrinsic does not mask it. After the patch, all four are identical to LLVM at `-O` and under `-Ctarget-cpu=native` too. |

Not used, deliberately: `miri` (interprets MIR, never reaches codegen) and ASAN/UBSAN (they
instrument the compiler binary, not the code it emits) - neither can observe a backend lowering
gap.

## Status

**Confirmed**

- Unpatched cg_clif: 5/5 runs of the same `repro/` binary abort, exit 134, always at the very
  first `_mm_cmpeq_ss` call. Deterministic - the abort is an unconditional trap emitted at compile
  time in place of the call.
- 64/64 immediate-sweep cases and 24/24 public-API family cases differ from LLVM before the patch;
  0/64 and 0/24 after it.
- The compile-time warnings appear on every build and the crate still builds successfully, so the
  failure only surfaces at run time.

**Fully reproducible**: **Yes** - `repro/` is a normal Cargo crate built by the real
`dist/cargo-clif`, and the real produced binary is executed; the only thing that differs between
the before and after runs is which cg_clif built it. `output.txt` is that binary's genuine output.

## Class

crash

## Root cause

Two independent defects stack up in `src/intrinsics/llvm_x86.rs`:

1. **No arm for the scalar forms.** `codegen_x86_llvm_intrinsic_call` matches
   `"llvm.x86.sse.cmp.ps" | "llvm.x86.sse2.cmp.pd"` (line 202 at 2bc05d45) but never mentions
   `llvm.x86.sse.cmp.ss` / `llvm.x86.sse2.cmp.sd`, so they reach the terminal `_ =>` arm
   (llvm_x86.rs:1350-1352), which warns and then emits `crate::base::codegen_panic_nounwind`. Nothing
   about the lowering is hard: the scalar forms differ from the packed ones only in that lane 0
   holds the comparison mask and the remaining lanes are copied straight from the first operand.

2. **The predicate immediate is not an `Operand::Constant` for the scalar forms.** The existing
   arm reads the immediate with `kind.node.constant()` and `unreachable!("{kind:?}")` otherwise.
   That works for the packed forms only because stdarch writes them as
   `vcmpps(a, b, const { IMM5 as i8 })` - an inline `const` block, hence a MIR constant. The
   scalar forms are written `vcmpss(a, b, IMM5 as i8)` (stdarch `x86/avx.rs`, `_mm_cmp_ss` /
   `_mm_cmp_sd`), so the cast lands in a temporary and the argument arrives as `move _3`. Simply
   adding the two names to the existing match therefore trades the run-time abort for a
   compile-time ICE - measured: `internal error: entered unreachable code: Spanned { node: move
   _3, span: .../x86/avx.rs:888:27 }` for all 64 cases. cg_clif already has the helper for exactly
   this situation, `crate::constant::mir_operand_get_const_val` (whose own FIXME cites
   rust-lang/rust#85105, "Casts like `IMM8 as u32` result in the const being stored inside a
   temporary"); the cmp arm just wasn't using it.

## Introduced by

Never implemented - `llvm.x86.sse.cmp.ss` and `llvm.x86.sse2.cmp.sd` have never appeared anywhere
in this repository's history (`git log -S "sse.cmp.ss" --all` and `git log -S "cmp.sd" --all` both
return no commits). Two commits bound the behaviour:

- 1a268911 "Replace llvm intrinsics with runtime trap" (2019-07-28) introduced the `_ =>` warn +
  `codegen_panic_nounwind` fallback, which is why an unimplemented intrinsic aborts at run time
  instead of failing the build.
- e8168ce8 "Merge commit '1eded3619d0e55d57521a259bf27a03906fdfad0' into sync_cg_clif-2023-07-22"
  (2023-07-22) brought in the packed `llvm.x86.sse.cmp.ps` / `sse2.cmp.pd` arm (`git blame -L
  202,202`) without extending it to the scalar forms. The original authoring commit is in the
  rust-lang/rust monorepo, not in this checkout's history.

## The fix

`patch.diff` (+42/-19, one file, `src/intrinsics/llvm_x86.rs`), rustfmt-clean:

- adds `"llvm.x86.sse.cmp.ss"` and `"llvm.x86.sse2.cmp.sd"` to the existing cmp arm's pattern;
- replaces the `kind.node.constant()` / `unreachable!` immediate extraction with
  `crate::constant::mir_operand_get_const_val`, which handles both the packed forms' `const {}`
  block and the scalar forms' cast-into-a-temporary (this is the same helper the `sha1rnds4` and
  `pcmpestri` arms already use for their immediates), and simplifies the decode to
  `i32::from(kind.to_u8())` since that helper returns a `ScalarInt`;
- branches at the emit site: for the two scalar intrinsics it compares lane 0 only
  (`fcmp` -> `bool_to_zero_or_max_uint`, exactly the per-lane code the packed path uses) and copies
  lanes `1..lane_count` of the first operand into the result; the packed path keeps calling
  `simd_pair_for_each_lane` unchanged.

Checked well beyond the one triggering input:

| input class | unpatched | patched |
|---|---|---|
| 24 public `_mm_cmpXX_ss`/`_mm_cmpXX_sd`, 6 operand sets each (incl. the shuffle-based `gt`/`ge`/`ngt`/`nge` forms) | 24/24 abort | 24/24 byte-identical to LLVM |
| `_mm_cmp_ss`/`_mm_cmp_sd`, all 32 immediates x 3 operand sets | 64/64 abort | 56 identical, 8 ICE (the four constant predicates - candidate 005's bug, see below) |
| packed `_mm_cmp_ps`/`_mm_cmp_pd`, all 32 immediates (regression check) | 56 identical / 8 ICE | 56 identical / 8 ICE - unchanged |
| both lane counts (`f32x4` and `f64x2`), NaN in either operand, `-0.0` vs `0.0`, `±inf` | abort | identical |
| `-O`, and `-Ctarget-cpu=native` | abort | identical |
| `repro/`, real `dist/cargo-clif` binary, 486 output lines | exit 134, 5/5 | exit 0 and byte-identical to LLVM, 5/5 |

The 8 remaining ICEs are **not** this bug: they are the pre-existing `todo!()` at llvm_x86.rs:230
and :234 for the four constant predicates `_CMP_FALSE_OQ`/`_CMP_TRUE_UQ`/`_CMP_FALSE_OS`/
`_CMP_TRUE_US` (0x0b/0x0f/0x1b/0x1f), which breaks the packed forms identically and is workspace
candidate 005. Composing this patch with candidate 005's patch (the two are logically independent
but touch adjacent lines, so `git apply` of one onto the other needs a manual merge - done by hand
here) and re-running the sweep over **all 128** cases (ps, pd, ss, sd x 32 immediates) gives
**identical=128 ice=0 wrong=0**. `patch.diff` in this folder is this candidate's change only.

- Project test suite: `./y.sh test` in the worktree (full run, including the `download/` external
  repos) - **before the patch: pass, exit 0**; **after the patch: pass, exit 0**. Both runs: 35
  items (13 `[AOT]`, 6 `[BUILD]`, 6 `[COPY]`, 2 `[JIT]`, 3 `[PATCH]`, 5 `[TEST]`), 61
  `test result: ok` blocks, 0 `FAILED`. The two runs' item lines were diffed and are identical.
  The before run was taken on a `git status --short`-clean tree. (`./y.sh test` aborts on the first
  failure, so exit 0 means every item ran.)

## What's not done / open questions

- **`_mm_comiXX_ss` / `_mm_comiXX_sd` / `_mm_ucomiXX_ss` / `_mm_ucomiXX_sd` are deliberately not
  fixed here.** Approach 2 found that all 24 of them abort the same way, on 24 further
  unimplemented intrinsics
  (`llvm.x86.sse.{,u}comi{eq,lt,le,gt,ge,neq}.ss`, `llvm.x86.sse2.{,u}comi{eq,lt,le,gt,ge,neq}.sd`).
  They are a distinct set - different names, an `i32` 0/1 return instead of a vector, and a
  signaling-vs-quiet distinction that Cranelift has no model for - so they are left as a separate
  suspect rather than bundled into this patch. `scalar_cmp_family_sweep.py` already covers them.
- The four constant predicates (candidate 005) still ICE for the scalar forms with only this patch
  applied; both patches are needed for a complete `_mm_cmp_ss`/`_mm_cmp_sd`. Whoever takes these
  upstream should merge them into one arm.
- The 256-bit compares (`llvm.x86.avx.cmp.ps.256` / `cmp.pd.256`) were not examined; there are no
  256-bit scalar forms, so this specific gap does not extend there.
- Signaling semantics are not covered: Cranelift has no FP-exception model, so nothing here can
  distinguish `_OS`/`_US` from `_OQ`/`_UQ` or `comi` from `ucomi`. The sweeps only prove the lane
  masks and returned values agree with LLVM.
- The `unreachable!("{kind:?}")` fallback is kept (now only reachable if the immediate is genuinely
  non-constant, which the `const IMM5: i32` generic makes impossible from stdarch). A maintainer
  may prefer the `span_fatal` diagnostic that the `sha1rnds4` arm uses for the same situation -
  compare candidate 009, which is about exactly that kind of non-constant-immediate handling.
- The candidate `justfile`'s `clone`/`build-clif` recipes could not be executed in this sandbox
  (`git ls-remote git@github.com:...` -> "Host key verification failed", no outbound ssh). Their
  equivalent was validated for real: a full local clone (`git clone --no-local`) checked out at
  `base_commit` accepts `patch.diff` with `git apply` cleanly (+42/-19), and `just llvm`,
  `just clif` and `just compare` were all run for real with `CLIF_ROOT` pointed first at an
  unpatched cg_clif (abort, `compare` fails) and then at the patched worktree (`IDENTICAL`).
