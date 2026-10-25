# Known open I-miscompile issues — reference / pattern-mining input only

**Not a candidate backlog.** Per the user's explicit instruction for this session: these are
already filed, already known. The recon pass over this list exists only to (a) check whether
each still reproduces on current nightly, as a cheap sanity data point, and (b) extract the
*pattern* each one exercises (which MIR/LLVM pass, which language feature) to seed the
differential-fuzzer's generator templates (see `../README.md` and `../miscompile-fuzzer/`). None
of these get a candidate folder or a `SUMMARY.md` row just for still reproducing — only a
genuinely new miscompilation the fuzzer finds does.

Host: `x86_64-unknown-linux-gnu`, AMD Zen4. 15 issues requiring a different target
(PowerPC/AVR/ARM/wasm32/nvptx-cuda/32-bit x86/Windows-MSVC-specific) were excluded as out of
scope — see the list at the end of this file.

Fill in the **Still reproduces?** column during the recon pass: `Yes` / `No` / `Not checked` /
`Needs nightly feature X` (if it couldn't be tried without extra setup not worth the time).

| Issue | Priority | Filed | Title | Pattern (for generator) | Still reproduces? |
|---|---|---|---|---|---|
| #61275 | P-high | 2019-05-28 | Varargs are completely unchecked if passed as generics | FFI varargs + generics: missing C default-argument-promotion cast, defeated by monomorphization-time codegen | Yes |
| #100914 | P-high | 2022-08-23 | LLVM miscompiles large stack allocations | LLVM large-stack-alloca + deep recursion lowering (stack-probe/red-zone interaction) | No |
| #107975 | P-high | 2023-02-13 | Miscompilation: Equal pointers comparing as unequal | pointer identity/equality vs provenance, stack-slot reuse + EarlyCSE/GVN | Yes |
| #118223 | P-high | 2023-11-24 | x86-64 assembler silently truncates 64-bit address | x86 asm!() operand encoding / addressing-mode selection (LLVM integrated assembler) | No |
| #153222 | P-high | 2026-02-28 | LLVM loop miscompilation | LLVM scalar-evolution/LICM miscompile around NonZero niche value (llvm/llvm-project#183906) | Yes |
| #159618 | P-high | 2026-07-20 | EarlyOtherwiseBranch introduces access to a dead local variable | EarlyOtherwiseBranch pass hoists a condition read past StorageDead on a match-tree local | Yes |
| #160004 | P-high | 2026-07-27 | GVN pass introduces UB by reborrowing from disabled reference | GVN + raw-pointer-derived &mut aliasing a place still reachable through another reference | Yes |
| #161898 | P-high | 2026-08-28 | JumpThreading miscompiles union field reads after sibling writes | JumpThreading + DataflowConstProp fail to flood aliasing across union-field sibling writes (value_analysis::for_each_aliasing_place gap) | Yes |
| #163777 | P-high | 2026-10-05 | ReferencePropagation looks through field projections but not when replacing | ReferencePropagation collapses a reborrow-through-projection chain without matching invalidation semantics on the write side | Yes |
| #163782 | P-high | 2026-10-05 | With GVN, Miri flags loads of a const packed struct field as insufficiently aligned | GVN folds a #[repr(packed)] field projection on a const into a literal constant with mismatched alignment | Yes |
| #163856 | P-high | 2026-10-06 | Wrong parameter deduction caused by default trait impl | DeduceParamAttrs misresolves default-vs-override trait method through dyn Trait fn-pointer, yielding false noalias/readonly | Yes |
| #149294 | P-medium | 2025-11-24 | Miscompile at -Copt-level=1 | LLVM SROA/isel + overlapping tuple-field store-to-load forwarding at -O1 | No |
| #155952 | P-medium | 2026-04-29 | LLVM miscompile with -Copt-level=2 and -Ctarget-cpu=znver2 | custom_mir tuple/array aggregate reconstruction + LLVM codegen on znver2 | No |
| #161973 | P-medium | 2026-08-29 | Miscompilation from mir-opt/Codegen/LLVM interaction on merged statics | static merging across mir-opt+codegen+LLVM | Not checked |
| #162078 | P-medium | 2026-08-31 | Miscompile from LLVM assuming that globals don't get merged | GVN dominating-equality constant/provenance replacement across distinct globals that may alias at link time | Yes |
| #163311 | P-medium | 2026-09-25 | Miscompilation: program prints 3 instead of 2 at opt-level=2 and 3 | nested loop/labeled-block control flow with mutable-static reads vs boolean state - likely SimplifyCfg/early-otherwise-branch or jump-threading | Yes |
| #98132 | P-none | 2022-06-15 | -Zvirtual-function-elimination miscompiles hello world with other opts on | -Zvirtual-function-elimination (vtable VFE/type.checked.load) + LTO - std's lang_start_internal call lacks checked-load instrumentation | Yes |
| #127336 | P-none | 2024-07-04 | Miscompilation of a program projecting field of an extern type | extern-type sized-field-access panic not understood by a MIR transform (likely GVN/SimplifyLocals) - diverges between opt-level=0 and 1 | Yes |
| #132898 | P-none | 2024-11-11 | ReferencePropagation introduces UB into code accepted by Stacked Borrows | ReferencePropagation + two-phase borrows / raw-pointer aliasing of a &mut self receiver | Yes |
| #144371 | P-none | 2025-07-23 | Unwinding within foreign ABIs can corrupt registers | cross-ABI unwinding - LLVM trusts win64 callee-saved convention for a landing pad on a non-win64 (SysV) unwind path | Yes |
| #147538 | P-none | 2025-10-09 | `i2p-p2i-opt` miscompiles rust code | exposed-provenance round-trip through a #[cold]/opaque function boundary (llvm/llvm-project#33896); #147541 mitigation masks this shape | No |
| #149429 | P-none | 2025-11-28 | LLVM miscompilation in instcombine | custom_mir isize::MIN shift/bitop sequence + raw-pointer write laundered through sroa+instcombine | No |
| #156008 | P-none | 2026-04-30 | AsyncDrop broken normal Drop | async-drop state-machine glue skips a still-live local's normal Drop when a sibling/nested task's async destructor runs first | Yes |
| #159268 | P-none | 2026-07-14 | autodiff: silent gradient corruption differentiating through coroutines | Enzyme/autodiff activity analysis loses gradients through compiler-generated coroutine state structs (missing type-tree/activity metadata) | Yes |
| #160267 | P-none | 2026-07-31 | Miscompile: LLVM lacks fn-pointer provenance + linker ICF merging identical functions | fn_addr_eq broken by lld --icf=safe merging bitwise-identical-but-provenance-distinct functions | Not checked |
| #160722 | P-none | 2026-08-07 | LLVM doesn't expose provenance if expose_provenance's return value is dropped | expose_provenance round-trip via with_exposed_provenance_mut - LLVM's int2ptr/ptr2int noalias model elides a bounds check | Yes |
| #163734 | P-none | 2026-10-04 | Safe code indexes with negative i32 at opt-level=3 on x86_64-pc-windows-msvc | LLVM CodeGenPrepare::optimizeExt hoists zext nneg above its sign-check guard, producing a branch-on-poison the backend folds away (Windows-MSVC only, same IR is fine on Linux) | Not checked |
| #163737 | P-none | 2026-10-04 | MergeFunctions + -Cpasses=sancov-module leaves coverage counters without PC table | MergeFunctions (default at -O2+) folds identical weak/no_mangle fns into aliases, desyncing SanitizerCoverage's per-function PC-table and counter array | Yes |
| #163825 | P-none | 2026-10-05 | GVN versus unions with reference fields | GVN merges two reads of a &'static u8 union field across a write through a sibling *mut u8 field (same family as #161898, not fixed by PR #163425) | Yes |
| #163911 | P-none | 2026-10-07 | Miscompilation with FFI bool return type on x86_64 | x86_64 SysV ABI under-zero-extends bool-returning extern "C" fns since LLVM IR moved from zeroext i1 to plain noundef i1 | Yes |
| #130388 | P-low | 2024-09-15 | Miscompilation: comparing a pointer against all possible addresses returns false | LLVM SCEV/loop-folding assumes no-wrap over an exhaustive full-usize-range loop of strict-provenance pointer comparisons (llvm/llvm-project#34450) | Yes |
| #162214 | P-low | 2026-09-02 | LLVM assumes unused allocations always succeed | LLVM dead/unused-allocation elimination assuming allocator calls can't fail + pointer-disjointness reasoning on mutable refs; needs 32-bit target or huge allocations to trigger here | Not checked |
| #163541 | P-low | 2026-09-30 | LLVM replaces pointer with null without respecting provenance | GVN/SCCP null-pointer replacement via dominating equality + wrapping-pointer round-trip (llvm/llvm-project#220206 case 2, sibling of #162078) | Yes |

## Out of scope this session (different target architecture/OS)

#100923 (PowerPC), #109000 (AVR), #109114 (windows-msvc dylib-lto), #114479 (32-bit x86 no
SSE2), #115567 (32-bit x86 NaN payload), #125033 (Cortex-A72 ARM), #127318 (wasm32), #128448
(Cortex-R5 ARM), #129880 (32-bit ARM NEON), #129886 (PowerPC altivec), #132287 (Windows ARM
asm), #146686 (nvptx64-nvidia-cuda), #157209 (wasm32), #157239 (ARM Big Endian), #157456 (wasm32
SIMD128).
