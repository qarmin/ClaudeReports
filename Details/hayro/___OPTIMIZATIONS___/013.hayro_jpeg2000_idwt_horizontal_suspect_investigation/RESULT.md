# JPEG2000 IDWT horizontal filter: testing candidate 002's suspects #2/#3 (unaligned SIMD loads, problem-shape/amortization)

Follow-up to candidates `002` (deinterleave-based SIMD rewrite of `filter_horizontal`, regressed
~8.6x, three suspects listed but not root-caused) and `008` (used `valgrind --tool=callgrind` to
rule out suspect #1, poor inlining of the *original scalar* code). `perf` is still unavailable in
this environment (`perf: command not found`, confirmed again this session) - substituted targeted
isolated microbenchmarks (calling the real, private `filter_step_horizontal_simd` directly from a
temporary `#[cfg(test)]` module in `hayro-jpeg2000/src/j2c/idwt.rs`) plus `nm`/`objdump` reading of
the actual compiled binary, which together produced a decisive answer.

**No patch to real source is proposed here** - this candidate is a measurement/characterization
that narrows down 002/008's open suspects, per `SUMMARY.md`'s "N/A" impact category. `patch.diff`
reapplies candidate 002's rewrite (needed for the functions under test to exist) plus the
temporary diagnostic test module; both were reverted from the real working tree before this
folder was written.

## What was tried

Four isolated `#[ignore]`d tests added to `hayro-jpeg2000/src/j2c/idwt.rs` (see `patch.diff`),
each calling the real `filter_step_horizontal_simd` (or a byte-for-byte copy of its scalar
formula) directly, run via `taskset -c 2 cargo test -p hayro-jpeg2000 --release --lib -- --ignored
--nocapture --test-threads=1`, `uptime` load 1.2 (well under the 24-core count), 5 paired
alternating rounds per test:

| Test | What it isolates | Result |
|---|---|---|
| `print_detected_level` | Which `fearless_simd::Level` this CPU (AMD Ryzen 9 7900, Zen4, full `avx512f/bw/vl/cd/dq/ifma/vbmi/vbmi2/vnni/bitalg/vpopcntdq` + `vaes`/`gfni`) actually dispatches to | `Level::Avx512` - the fast, best-available path is being used, not a scalar/`Fallback` degradation |
| `suspect_2_alignment` | Suspect #2: does a plain +4-byte (one `f32`) base-address shift of the exact same computation (same `first=0`, same total width, same call count - only the slice's start address differs) cost anything extra? | offset+0 median 3.669ms vs offset+1elem median 3.667ms over 4,000,000 elements - **ratio 0.999x, no measurable difference** |
| `suspect_3_amortization` | Suspect #3: many small (~620-element, matching a real per-row evens/odds buffer) calls vs one big call doing the identical total amount of arithmetic (7,008 calls x 620 width vs 1 call x 4,344,960 width - 7,008 ≈ 1,752 rows x 4 `Irreversible97` lifting steps, one real image-sized level) | many-small-calls median 4.126ms vs one-big-call median 3.984ms - **ratio 1.04x, within noise, not remotely close to the observed 8.6x real-world regression** |
| `suspect_4_simd_kernel_vs_naive_scalar` (added after #2/#3 both came back negative) | Whether the SIMD kernel itself, at a fixed size that's amortization- and alignment-neutral (one call, offset 0 for both variants), simply costs more per element than the equivalent plain scalar Rust loop doing the identical FMA arithmetic | simd median 3.709ms vs scalar median 0.281ms over 4,000,000 elements - **ratio 13.21x slower**, by far the largest effect of anything measured this session |

`suspect_4`'s magnitude sanity-checks against candidate 002's own numbers: 7,008 calls x 620
elements (one image-sized decomposition level's worth of `Irreversible97` lifting-step work) took
~4.1ms in this isolated microbenchmark; candidate 002 measured the real per-decode "lifting steps"
phase at 15.35ms/decode (1,535ms / 100 iterations) for the same `sample.jp2`. Multiplying the
microbenchmark's single-level number by roughly 3-4x (a handful of dyadic decomposition levels
summing to ~4/3 the top level's row count, times likely 3 colour components) lands in the same
ballpark - i.e. this isolated microbenchmark is not a toy disconnected from the real regression,
it reproduces the real per-element cost closely enough to trust its *relative* comparisons.

### Root-causing suspect #4: `nm`/`objdump` on the real compiled code

Built candidate 002's own `bench/idwt/` bench (path-dependency pointed at the live
`hayro-jpeg2000` checkout with candidate 002's patch applied, release profile) and inspected the
actual compiled binary directly, independent of the synthetic microbenchmark above:

```
nm --demangle bench_idwt | grep -i "idwt::" | grep -vi "kernel|closure"
```

`irreversible_filter_97i_horizontal_simd::<fearless_simd::generated::avx512::Avx512>` (the real,
patched horizontal-filter function, monomorphized for the level this CPU actually dispatches to)
disassembles to **92 real `call` instructions to
`<Avx512 as Simd>::{add,sub,mul,mul_add,splat,floor}_f32x8::kernel::__fearless_simd_kernel`**
(`evidence/irreversible_filter_97i_horizontal_simd_avx512.asm`, `grep -c "call.*__fearless_simd_kernel"`
= 92). Each call is preceded by `movaps`-ing the operands into stack scratch space and passing
pointers (`lea rdi, [rsp+...]; lea rsi, [rsp+...]; call ...`) - real function-call overhead (stack
frame, register spills, no cross-call vectorization) for what should be a single machine
instruction (`vaddps`/`vfmadd.../vmulps` on a `zmm`/`ymm` register) per `f32x8` operation.

By contrast, `filter_2d` (the top-level function that also inlines `filter_vertical`'s equivalent
`f32x8` arithmetic, the existing, proven-fast SIMD code) has **zero** such calls anywhere in its
~15,700-line disassembly (`grep -c "call.*__fearless_simd_kernel"` = 0) - `filter_vertical`'s SIMD
ops are fully inlined into straight-line code. (One caveat about this half of the comparison: the
inlined instructions found there are 128-bit `addps`/`mulps`/`subps`, not 256-bit `vaddps`/512-bit
equivalents - this project's own `objdump`-based tooling wasn't pushed far enough this session to
confirm which of `dispatch!`'s inlined per-level branches those specific instructions belong to,
since `filter_2d` inlines all four `Level` branches guarded by runtime checks. The **zero-real-calls**
finding for `filter_2d` overall is unambiguous regardless of that caveat; the *why is filter_vertical's
version smaller/narrower* question is not.)

## Result

Suspects #2 (unaligned SIMD loads) and #3 (problem-shape/amortization) are **ruled out** -
neither produces a measurable effect (0.999x and 1.04x respectively) anywhere near the 8.6x
real-world regression. The dominant, measured cause is a **new suspect #4**: candidate 002's
horizontal rewrite's `f32x8` arithmetic compiles to genuine out-of-line function calls
(`__fearless_simd_kernel`, one per `add`/`sub`/`mul`/`mul_add`/`splat`/`floor` op, 92 in one
function alone) under the `Avx512` level actually selected on this hardware, instead of inlining
into straight-line vector instructions the way the existing `filter_vertical` path does. In
isolation (same total arithmetic, same buffer size, amortization- and alignment-neutral), this
costs **13.21x** versus an equivalent plain scalar loop - which by itself accounts for essentially
all of candidate 002's originally observed regression once real per-decode call counts (multiple
decomposition levels, multiple colour components) are factored back in.

## Does it change behavior/output?

No patch is proposed. The temporary diagnostic test module added this session used its own
synthetic buffers (not real image data) and made no attempt to reproduce/verify decode output; it
was reverted from `hayro-jpeg2000/src/j2c/idwt.rs` before this folder was written, together with
candidate 002's rewrite it depended on. `git status` on the main project is clean.

## Why the result looks the way it does

Not fully root-caused at the LLVM-internals level (would need `perf record`/`perf annotate` -
still unavailable - or a closer read of `fearless_simd`'s codegen macros/target-feature attribute
handling to say *why* LLVM inlines `filter_vertical`'s per-op calls but not
`irreversible_filter_97i_horizontal_simd`'s). Plausible, unconfirmed mechanism: `filter_vertical`'s
call chain (`filter_vertical` -> `dispatch!` -> `filter_vertical_impl` -> `filter_step_vertical`,
called a handful of times total, once per equation-step, with a large iteration count inside each
call) is small and simple enough for LLVM's inliner to collapse the whole thing (including
`fearless_simd`'s own per-op `__fearless_simd_kernel` calls) into one straight-line body within its
size/complexity threshold. `irreversible_filter_97i_horizontal_simd` is substantially larger and
more complex (deinterleave loop, 4 separate lifting-step calls each carrying two `impl Fn` closure
parameters, an even/odd dispatch branch inside `filter_step_horizontal_dispatch`) - plausibly
enough to push LLVM's inliner past whatever threshold makes it decide the *outer* function isn't
worth fully inlining, at which point `fearless_simd`'s low-level per-op functions (each presumably
`#[target_feature(enable="avx512f,...")]`-gated, since that's how `Level`-specific dispatch usually
requires per-function attribution in Rust) can no longer be inlined into a caller that itself isn't
marked with the matching `target_feature` at that specific call site - producing a real call
instead, with the pointer-based/stack-marshaled ABI those `unsafe fn` boundaries force. This is a
hypothesis, not confirmed; it would explain both observations (large horizontal function -> real
calls, small vertical function -> full inlining) but wasn't independently verified this session
(e.g. by shrinking `irreversible_filter_97i_horizontal_simd` and checking whether inlining is
restored).

## What's not done / open questions

- The exact LLVM/`fearless_simd` mechanism causing the non-inlining is not confirmed, only
  hypothesized (see above) - added to `IDEAS.md` as a concrete next step: try restructuring the
  horizontal rewrite to reduce `irreversible_filter_97i_horizontal_simd`'s size/closure count (e.g.
  by moving the closures to free functions, or splitting the 4 lifting steps into 4 smaller
  functions) and check via `nm`/`objdump` whether that alone restores inlining and, separately,
  whether it changes wall-clock time.
- Whether `filter_vertical`'s inlined code is genuinely using the `Avx512`-selected width
  (`zmm`) or a narrower one is unresolved (128-bit `addps`/`mulps` was observed in `filter_2d`'s
  disassembly, but that function inlines all four `dispatch!` branches guarded by runtime checks,
  and this session didn't isolate which branch those specific instructions belong to).
- This candidate does not attempt a fix - it narrows the cause. A future candidate could try the
  restructuring above, or report this as an upstream `fearless_simd` issue (a generic function
  whose body doesn't get its `#[target_feature]`-gated primitive calls inlined once the body grows
  past some threshold is a footgun other callers of that crate could hit too) - not decided here,
  per this skill's "no verdict" rule.
- `evidence/` in this folder has the `nm` symbol list and the full `objdump` disassembly of
  `irreversible_filter_97i_horizontal_simd::<Avx512>` used for the call-count claim above.
