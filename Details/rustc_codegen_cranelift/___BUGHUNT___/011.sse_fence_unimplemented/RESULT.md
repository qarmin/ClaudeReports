# 011. sse_fence_unimplemented

`core::arch::x86_64::{_mm_sfence, _mm_lfence, _mm_mfence}` (stable public API since 1.27) lower to
the LLVM intrinsics `llvm.x86.sse.sfence`, `llvm.x86.sse2.lfence` and `llvm.x86.sse2.mfence`.
`src/intrinsics/llvm_x86.rs` has no arm for any of them, so all three fall into that file's `_ =>`
catch-all, which warns at compile time and emits `codegen_panic_nounwind` - the built program
aborts the first time it executes a fence. Suspected from candidate 002 (`_mm*_stream_*` ICE):
stdarch documents that a `_mm_sfence` must be executed by the storing thread after every
non-temporal store, so even after the `_mm*_stream_*` ICE is fixed, the *documented correct* way
to use non-temporal stores still dies at run time.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read `src/intrinsics/llvm_x86.rs` + `grep -rn 'fence' src/` | n/a | Only `atomic_fence`/`atomic_singlethreadfence` (`src/intrinsics/mod.rs:855-864`) and `llvm.aarch64.isb` map to Cranelift's `fence`. Nothing for the three x86 fences; `llvm.x86.sse2.pause` *is* handled (llvm_x86.rs:19), so the gap is specifically the fences. |
| Differential test vs LLVM, `___BUGHUNT___/tools/diffcheck.sh` | single file, opt-level 0, default target-cpu | Reproduced on the first try: LLVM exit 0 / cg_clif exit 134. |
| Boundary-value/family sweep around the suspect (`_mm_sfence`, `_mm_lfence`, `_mm_mfence`, `_mm_pause`, `_mm_clflush`) via diffcheck.sh, each in its own program | opt-level 0 | 4 of 5 differ: sfence, lfence, mfence, clflush all abort; pause is fine. |
| Standalone `repro/` crate through `dist/cargo-clif` (real binary, 4 sections incl. a real 2-thread producer/consumer and a non-temporal-store + sfence sequence) | debug, `[profile.*] opt-level = 0` | 5/5 runs abort unpatched; 5/5 runs byte-identical to LLVM patched. |
| `objdump -d` on the patched binary | opt-level 0 | The patched lowering emits a real `mfence`, not a silently dropped no-op (output.txt appendix A). |

Not used, deliberately: `miri` (never reaches codegen) and ASAN/UBSAN (they instrument the
compiler, not the code it emits) - neither can see a backend codegen gap.

## Status

**Confirmed**

- Unpatched cg_clif: 5/5 runs of the same `repro/` binary abort with
  `llvm.x86.sse.sfence is not yet supported`, exit 134 (SIGABRT). Deterministic - the abort is an
  unconditional trap emitted at compile time, so it happens on the first fence executed.
- The compile-time warnings (`unsupported x86 llvm intrinsic llvm.x86.sse.sfence; replacing with
  trap`, likewise lfence/mfence) appear on every build; the crate still builds "successfully", so
  the failure only shows up at run time.
- LLVM (the oracle), same source: exit 0, prints all four sections.

**Fully reproducible**: **Yes** - `repro/` is a normal Cargo crate built by the real
`dist/cargo-clif` and the resulting real binary is executed; the only thing that changes between
the two runs is which cg_clif built it.

## Class

crash

## Root cause

`src/intrinsics/llvm_x86.rs::codegen_x86_llvm_intrinsic_call` matches on the intrinsic name.
`llvm.x86.sse2.pause` and `llvm.x86.avx.vzeroupper` have arms (both no-ops - genuine perf hints),
but the three memory fences have none, so they reach the terminal `_ =>` arm at
`src/intrinsics/llvm_x86.rs:1349`:

    fx.tcx.dcx().warn(format!("unsupported x86 llvm intrinsic {}; replacing with trap", intrinsic));
    ...
    crate::base::codegen_panic_nounwind(fx, &msg, span);

i.e. the call site is replaced by an unconditional non-unwinding panic. The backend already has
everything needed to lower them: `fx.bcx.ins().fence()` is what `atomic_fence` uses
(`src/intrinsics/mod.rs:858`) and it assembles to `mfence` on x86-64.

This is not "a rarely used intrinsic is missing". `_mm_sfence` is the fence stdarch's own docs
require after `_mm_stream_*` ("a call to `_mm_sfence` must be performed by the thread that used
the intrinsic"), and `_mm_lfence`/`_mm_mfence` are the standard hand-rolled barriers in
lock-free/`asm`-adjacent code. `repro/`'s `part2` shows the concrete consequence: the scalar
`_mm_stream_si32` stores themselves already work under cg_clif, and it is only the mandatory
trailing `_mm_sfence` that kills the program.

## Introduced by

Never implemented - the three fences have no arm in cg_clif at any point in its history
(`git log -S sfence` over the whole repo returns no commits). The behaviour of *aborting* rather
than erroring at compile time dates to 1a268911 "Replace llvm intrinsics with runtime trap"
(2019-07-28), which introduced the `_ =>` warn + `codegen_panic_nounwind` fallback that these
three intrinsics land in.

## The fix

`patch.diff` (+6/-0, `src/intrinsics/llvm_x86.rs`) adds one arm next to the existing
`llvm.x86.sse2.pause` arm:

    "llvm.x86.sse.sfence" | "llvm.x86.sse2.lfence" | "llvm.x86.sse2.mfence" => {
        fx.bcx.ins().fence();
    }

Cranelift exposes only one fence instruction, a full barrier. That is *stronger* than both the
store-only (`sfence`) and load-only (`lfence`) variants, so using it for all three is a correct
(if slightly pessimistic) lowering; it is the same instruction cg_clif already emits for
`core::intrinsics::atomic_fence`. `objdump` on the patched binary confirms a real `mfence` reaches
the object code (output.txt appendix A), so the fences are not being silently dropped.

Checked against more than the one triggering input:

| input | unpatched cg_clif | patched cg_clif |
|---|---|---|
| `_mm_sfence()` alone | abort | matches LLVM |
| `_mm_lfence()` alone | abort | matches LLVM |
| `_mm_mfence()` alone | abort | matches LLVM |
| `_mm_pause()` alone (control, already implemented) | matches LLVM | matches LLVM |
| `_mm_clflush(p)` (same family, out of scope - see below) | abort | abort |
| `_mm_stream_si32` x8 then `_mm_sfence` (stdarch's documented pattern) | abort | matches LLVM |
| 2 threads, producer `sfence` / consumer `pause`+`lfence`, 64 values checked | abort | matches LLVM |
| all three fences via `#[inline(never)]` fns, 1000 loop iterations | abort | matches LLVM |

- Project test suite (`./y.sh test` in the worktree, full run including the `download/` external
  repos): **before the patch: pass, exit 0**; **after the patch: pass, exit 0**. Both runs executed
  the identical list of 35 items (13 `[AOT]`, 6 `[BUILD]`, 6 `[COPY]`, 2 `[JIT]`, 3 `[PATCH]`,
  5 `[TEST]`) - the two logs' item lines were diffed and are identical - including `[TEST] sysroot`
  and the external repos `rust-random/rand`, `regex`, `graviola`, `portable-simd`. No new failures.
  (`./y.sh test` aborts on the first failure, so exit 0 means every item ran.)

## What's not done / open questions

- **`_mm_clflush` (`llvm.x86.sse2.clflush`) is deliberately not fixed here.** It is in the same
  "memory ordering / cache control" family and aborts the same way, but Cranelift has no
  cache-line-flush instruction, so lowering it needs a maintainer decision between inline asm
  (`clflush` is baseline on every x86-64 CPU that has SSE2) and a documented no-op. A no-op is
  *observationally* fine for ordinary volatile RAM but silently wrong for persistent-memory code,
  which is exactly what calls `_mm_clflush`. Left as a separate suspect rather than bundled in.
  The same question applies to the never-checked `_mm_clflushopt` / `_mm_clwb` / `_mm_sfence`
  relatives in the CLFLUSHOPT/CLWB extensions.
- The fix uses a full barrier for `sfence`/`lfence`. Correct but not optimal; if Cranelift ever
  grows weaker fence variants, this arm should be revisited.
- `_mm256_stream_*` / `_mm512_stream_*` (wide non-temporal stores) still ICE - that is candidate
  002, unaffected by this patch. Only the scalar `_mm_stream_si32`/`_mm_stream_si64` path is
  exercised here.
- The candidate `justfile`'s `clone`/`build-clif` recipes could not be executed in this sandbox
  (`git ls-remote git@github.com:...` fails with "Host key verification failed" - no outbound ssh).
  The equivalent cycle was validated for real against a full local clone at `base_commit`
  (`git clone --no-local`, `git checkout 2bc05d45`, `git apply patch.diff` -> applies cleanly,
  6/0 numstat); `just llvm`, `just clif` and `just family` were all run for real with
  `CLIF_ROOT` pointed at both an unpatched and a patched checkout, and produced the output in
  output.txt.
