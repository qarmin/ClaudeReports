# 024. sse2_clflush_unimplemented

`core::arch::x86_64::_mm_clflush` (stable public API since 1.27, SSE2 baseline so no runtime
feature check is even needed) lowers to `llvm.x86.sse2.clflush`, and the nightly
`_mm_clflushopt` lowers to `llvm.x86.clflushopt`. Neither has an arm in
`src/intrinsics/llvm_x86.rs`, so both fall into that file's `_ =>` catch-all: a compile-time
warning plus `codegen_panic_nounwind`, i.e. the built program aborts the first time it executes a
flush. Left over from candidate 011 (`_mm_sfence`/`_mm_lfence`/`_mm_mfence`), which fixed the
fences with `fx.bcx.ins().fence()` but deliberately excluded `_mm_clflush` because Cranelift has no
cache-line-flush instruction and the choice between inline asm and a documented no-op looked like a
maintainer decision.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read `src/intrinsics/llvm_x86.rs` + `grep -rn 'clflush\|clwb\|clflushopt' src/` | n/a | Zero hits anywhere in `src/`. The nearest arms are `llvm.x86.sse2.pause` and `llvm.x86.avx.vzeroupper`, both genuine no-ops. So the gap is real, not a differently-named arm. |
| Enumerate the whole reachable family out of stdarch: `grep -rhn 'link_name = "llvm' core_arch/src/{x86,x86_64}` filtered to cache/fence/prefetch/monitor mnemonics | nightly-2026-08-24 stdarch | Exactly 7 intrinsics are reachable from stable/nightly stdarch: `llvm.x86.sse2.clflush`, `llvm.x86.clflushopt`, `llvm.x86.prefetchrs`, `llvm.prefetch.p0`, `llvm.x86.sse2.pause`, `llvm.x86.sse.sfence`, `llvm.x86.sse2.{lfence,mfence}`. **There is no `_mm_clwb`/`llvm.x86.clwb` in this stdarch at all** (nor `_mm_cldemote`, `wbinvd`, `monitor`/`mwait`), so the candidate's "presumably `_mm_clwb`" half is currently unreachable from Rust and cannot be tested. |
| Differential test vs LLVM, `___BUGHUNT___/tools/diffcheck.sh` | single file, opt-level 0, default target-cpu | Reproduced first try for `_mm_clflush`: LLVM exit 0 / cg_clif exit 134, `llvm.x86.sse2.clflush is not yet supported`. |
| Boundary/family sweep over all 7 of the above, each in its own single-file program (`repro/sweep/`, driven by `just sweep`) | opt-level 0 | 4 of 6 programs differ from LLVM: `clflush`, `clflushopt`, `prefetchrs` and the `sfence`/`lfence`/`mfence` trio all abort; `pause` and all six `_mm_prefetch` hint values are fine. `prefetchrs` is a newly found sibling, not part of this candidate's patch. |
| Standalone `repro/` crate through the real `dist/cargo-clif` (5 sections: plain flushes, `clflushopt` behind `#[target_feature]`, a 512-iteration loop with 6 other values live across the flush, flush/write/read-back on a heap `Box<[u32]>`, and a mixed unaligned/stack-local/4 KiB pass) | debug, `[profile.*] opt-level = 0` | 5/5 runs abort unpatched (exit 134); 5/5 runs byte-identical to LLVM after the patch. |
| `objdump -d` on the patched binary | opt-level 0 | Real `clflush (%rax)` and `clflushopt (%rax)` reach the object code - the lowering is not a silently-dropped no-op (output.txt section E). |
| Variation matrix on the patched backend: opt-level 0 / `-O` / `-Ctarget-cpu=native` / `-O` + native | 8 combinations | All 8 identical to LLVM (output.txt section G). |

Not used, deliberately: `miri` (never reaches codegen) and ASAN/UBSAN (they instrument the
compiler, not the code it emits) - neither can see a backend codegen gap.

## Status

**Confirmed**

- Unpatched cg_clif: 5/5 runs of the same `repro/` binary abort with
  `llvm.x86.sse2.clflush is not yet supported`, exit 134 (SIGABRT). Deterministic - the abort is
  an unconditional trap emitted at compile time, so it fires on the first flush executed.
- The compile-time warnings (`unsupported x86 llvm intrinsic llvm.x86.sse2.clflush; replacing with
  trap` and the same for `llvm.x86.clflushopt`) appear on every build, but the crate still builds
  "successfully" - the failure only shows up at run time.
- LLVM (the oracle), same source: exit 0, prints all five sections.
- After the patch: 5/5 runs exit 0 with output md5 `ffee85099d60dacb33e26fda647b7c41`, identical
  to the LLVM binary's md5 on the same source.

**Fully reproducible**: **Yes** - `repro/` is a normal Cargo crate built by the real
`dist/cargo-clif` and the resulting real binary is executed; the only thing that changes between
the two runs is which cg_clif built it.

## Class

crash

## Root cause

`src/intrinsics/llvm_x86.rs::codegen_x86_llvm_intrinsic_call` dispatches on the intrinsic name
string. There is no arm for `llvm.x86.sse2.clflush` or `llvm.x86.clflushopt`, so both reach the
terminal `_ =>` arm (`src/intrinsics/llvm_x86.rs:1349` before the patch):

    fx.tcx.dcx().warn(format!("unsupported x86 llvm intrinsic {}; replacing with trap", intrinsic));
    ...
    crate::base::codegen_panic_nounwind(fx, &msg, span);

so the call site is replaced by an unconditional non-unwinding panic and the program aborts there.

Why these two were never implemented, unlike the neighbouring `pause`/`vzeroupper` no-ops and
unlike the fences: Cranelift's IR has no cache-line-flush instruction at all. `pause` and
`vzeroupper` can be legitimately dropped (pure perf hints), and the fences map onto Cranelift's
own `fence`, but a flush has no IR equivalent, so the arm would have to reach for inline asm - and
dropping it instead would be *observationally* fine for ordinary volatile RAM yet silently wrong
for the persistent-memory code that is the main reason to call `_mm_clflush` in the first place.

## Introduced by

Never implemented - `git log --oneline -S clflush -- src/` returns no commits over the whole
history, so no arm for either intrinsic ever existed. The behaviour of *aborting at run time*
rather than erroring at compile time dates to 1a268911 "Replace llvm intrinsics with runtime trap"
(2019-07-28), which introduced the `_ =>` warn + `codegen_panic_nounwind` fallback these two land
in. (`llvm.x86.clflushopt` only became reachable from Rust when stdarch added `_mm_clflushopt`
under `simd_x86_clflushopt`, rust-lang/rust#157096 - that is on the rustc side, not a cg_clif
commit.)

## The fix

`patch.diff` (+25/-0, `src/intrinsics/llvm_x86.rs`) adds one arm next to the existing
`llvm.x86.avx.vzeroupper` arm, emitting the real instruction through the backend's existing
inline-asm path (`codegen_inline_asm_inner`, already used for `crc32`, `pclmulqdq`, the AES and
SHA intrinsics, `rdtsc`, `vcvtps2ph`, ...):

    "llvm.x86.sse2.clflush" | "llvm.x86.clflushopt" => {
        intrinsic_args!(fx, args => (ptr); intrinsic);
        let ptr = ptr.load_scalar(fx);
        let asm = match intrinsic {
            "llvm.x86.sse2.clflush" => "clflush [rax]",
            "llvm.x86.clflushopt" => "clflushopt [rax]",
            _ => unreachable!(),
        };
        codegen_inline_asm_inner(fx, &[...asm...],
            &[CInlineAsmOperand::In { reg: ...X86InlineAsmReg::ax, value: ptr }],
            InlineAsmOptions::NOSTACK);
    }

Design notes:

- Inline asm, not a no-op. `clflush` is baseline on every x86-64 CPU that has SSE2, so no runtime
  feature check is needed for it; `clflushopt` is only reachable from stdarch behind
  `#[target_feature(enable = "clflushopt")]`, so the caller already owns the feature check exactly
  as it does under LLVM.
- Deliberately **not** `PURE`/`NOMEM`. A flush is ordered against stores to the flushed line, so
  the asm block must stay a memory barrier from the optimiser's point of view; only `NOSTACK` is
  set. `objdump` confirms a real `clflush`/`clflushopt` in the object code (output.txt section E).
- Intel syntax with no register prefix, matching what `src/inline_asm.rs` emits
  (`.intel_syntax noprefix`) and what every existing asm-based arm in the file uses.

Checked against more than the one triggering input:

| input | unpatched cg_clif | patched cg_clif |
|---|---|---|
| `_mm_clflush(p)` alone | abort | matches LLVM |
| `_mm_clflushopt(p)` behind `#[target_feature]` | abort | matches LLVM |
| 4 lines flushed from a stack `[u8; 256]` | abort | matches LLVM |
| 512-iteration loop, 6 unrelated `u64`s live across each flush (the arm pins the pointer into `rax`) | abort | matches LLVM, all 6 values bit-identical |
| flush -> write -> flush -> read back on a heap `Box<[u32]>` | abort | matches LLVM |
| unaligned pointer (`+1`), stack-local pointer, last byte of a 4 KiB `Vec`, 64 interleaved `clflushopt`s | abort | matches LLVM |
| opt-level 0 / `-O` / `-Ctarget-cpu=native` / `-O` + native, both intrinsics (8 combinations) | abort | matches LLVM in all 8 |
| `_mm_pause`, all six `_mm_prefetch` hints (controls, already working) | matches LLVM | matches LLVM |
| `_mm_sfence`/`_mm_lfence`/`_mm_mfence` (candidate 011, out of scope here) | abort | abort |
| `_m_prefetchrs` (`llvm.x86.prefetchrs`, out of scope - see below) | abort | abort |

- Project test suite (`./y.sh test` in the worktree, full run including the `download/` external
  repos): **before the patch: pass, exit 0**; **after the patch: pass, exit 0**. Both runs executed
  the identical list of 35 items (`[AOT]`/`[BUILD]`/`[COPY]`/`[JIT]`/`[PATCH]`/`[TEST]` lines
  diffed, identical), including `[TEST] sysroot` and the external repos. No new failures.
  (`./y.sh test` aborts on the first failure, so exit 0 means every item ran.)

## What's not done / open questions

- **`llvm.x86.prefetchrs` (`_m_prefetchrs`, the MOVRS-extension prefetch) aborts identically and is
  NOT covered by this patch.** Found by this candidate's family sweep; it is a different extension
  and a different instruction (`prefetchrst2`), so it is left as a separate suspect rather than
  bundled in. The fix would be the same shape (one more alternative in this arm's match, asm text
  `prefetchrst2 [rax]`) but it needs its own verification: unlike `clflush`, `prefetchrst2` is a
  brand-new encoding, so whether the system assembler accepts it is an open question, and a
  prefetch is a pure hint so the no-op option is genuinely defensible there.
- **`_mm_clwb` / `llvm.x86.clwb` could not be tested**: it does not exist in nightly-2026-08-24's
  stdarch, so nothing in Rust can reach it. If stdarch adds it, the same arm extends to it with
  asm text `clwb [rax]` - but note `clwb` is *not* baseline (it needs the CLWB CPU feature), same
  situation as `clflushopt`.
- Whether the maintainers actually want inline asm here at all is the open decision this candidate
  exists to surface. The alternative - a documented no-op with a comment - keeps the backend free
  of a new asm dependency and is fine for every use of `_mm_clflush` that is merely a cache-timing
  or benchmark trick, but it silently breaks persistent-memory durability code. This patch takes
  the conservative side (emit the real instruction).
- The `NOSTACK`-only option set means cg_clif treats the asm as clobbering memory, which is what
  makes the flush ordered. It has not been checked whether Cranelift's own optimiser could still
  reorder a *neighbouring* non-volatile store across the asm block in some other configuration;
  the four opt-level/target-cpu combinations tested all matched LLVM.
- The candidate `justfile`'s `clone`/`build-clif` recipes could not be executed in this sandbox
  (`git ls-remote git@github.com:...` fails with "Host key verification failed" - no outbound ssh).
  `just llvm`, `just clif` and `just sweep` were all run for real with `CLIF_ROOT` pointed at both
  an unpatched checkout (the main tree) and a patched one (the worktree), and produced the output
  in output.txt; `patch.diff` was captured with `git -C "$WT" diff` against base_commit 2bc05d45.
