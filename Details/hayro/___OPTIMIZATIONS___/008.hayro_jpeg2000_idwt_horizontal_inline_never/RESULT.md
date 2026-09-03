# JPEG2000 IDWT horizontal filter: `#[inline(never)]` on `filter_step_horizontal`

Follow-up to candidate `002`, whose deinterleave-based SIMD rewrite regressed 8.6x for reasons
never root-caused (`perf` unavailable that session). This candidate's job was to actually get a
profiler on the lifting-step loop before trying another rewrite. `perf` is still unavailable in
this environment (not installed, no root, no passwordless `sudo` - a "no new privileges" sandbox
flag blocks it outright). Substituted `valgrind --tool=callgrind` (instruction-retirement +
call-graph profiling) plus `objdump` disassembly reading and a standalone isolated microbenchmark,
which together did produce a concrete, testable, low-risk hypothesis - see below for why it still
didn't turn into a measurable win.

## Root-cause investigation (this session's actual "get perf on it" step)

1. **Callgrind on the real `bench_idwt` harness** (5 iterations, `sample.jp2` = a real
   1241x1752 JPX codestream from `issue391.pdf`, Irreversible97/9-7 transform) pinpointed one
   single source line as 9.05% of the *entire program's* retired instructions:
   `scanline[i] = f(scanline[i], scanline[i - 1], scanline[i + 1]);` inside
   `filter_step_horizontal`'s middle loop (`hayro-jpeg2000/src/j2c/idwt.rs`). The whole function
   chain (`filter_horizontal` -> `filter_row` -> `reversible_filter_53r`/`irreversible_filter_97i`
   -> `filter_step_horizontal` -> the `f` closure) had all been inlined into one giant function,
   `filter_2d` (11.2% + a further 5.26% attributed to inlined `math.rs` code + 1.80% to inlined
   `step_by` iterator code = ~18% of the whole program's instructions for the horizontal filter
   alone, before counting `filter_vertical`).
2. **Disassembling `filter_2d`** (via `objdump -d --demangle`) confirmed no `ymm`/`xmm` packed
   arithmetic anywhere near this line - it's fully scalar in that context, plus 30 distinct
   `panic_bounds_check` call sites scattered through the (very large, ~8700-line disassembly)
   function.
3. **Built a standalone microbenchmark** (`/tmp` during the session, not part of this folder's
   deliverable - the loop body is small enough to fully reproduce inline in a throwaway file)
   with 3 variants of the same loop as an `#[inline(never)]` function: the original `step_by(2)`
   version, a manual `while i += 2` rewrite, and a `chunks`/slice-based rewrite. Disassembling the
   `#[inline(never)]` `step_by` version showed LLVM **does** auto-vectorize it with SSE
   (`mulps`/`addps`/`shufps`, processing 4 output elements per iteration via a shuffle-based
   degather of the 3 overlapping strided loads) - just not when it's forced fully into `filter_2d`.
   The manual/`chunks` rewrites were both ~1.6-2x *slower* in this isolated microbenchmark (no
   auto-vectorization triggered for those shapes) - ruled out immediately, never touched real
   source.
4. Based on (1)-(3): the real fix tried is not a rewrite, but removing the `#[inline(always)]`
   annotation on `filter_step_horizontal` (changing it to `#[inline(never)]`) so this loop stays
   a separate, small, autovectorizable function instead of being flattened into `filter_2d`.
   Confirmed via `objdump` on the real patched build: `filter_step_horizontal::<irreversible_filter_97i::{closure#0}>`
   (and the other 3 closures) now compile with `mulps`/`addps`/`shufps` SSE code, matching the
   microbenchmark.

## What was tried

Single fix: `#[inline(always)]` -> `#[inline(never)]` on `filter_step_horizontal` in
`hayro-jpeg2000/src/j2c/idwt.rs` (see `patch.diff`). Bench: `bench_idwt` (candidate 002's harness,
reused as-is), real JPX decode of the embedded `sample.jp2`, `taskset -c 2`, 300 iterations,
hash-checked every iteration.

| Variant | single-core p50 (5 alternating rounds) | multi-core p50 (3 alternating rounds) | notes |
|---|---|---|---|
| baseline (`#[inline(always)]`) | 8.883 / 8.945 / 8.940 / 8.951 / 8.946 ms | 8.995 / 9.074 / 9.076 ms | |
| patched (`#[inline(never)]`) | 8.978 / 8.940 / 8.982 / 8.976 / 8.954 ms | 9.113 / 9.149 / 9.065 ms | |

`min` across the same rounds: baseline 8.663-8.855ms (avg ~8.79ms single-core, ~8.81ms
multi-core), patched 8.827-8.878ms (avg ~8.85ms single-core, ~8.88ms multi-core) - patched runs
marginally higher on average by ~0.6-1%, but the spread between rounds of the *same* binary is
already ~1-2%, so this is not distinguishable from noise in either direction.

Additionally instrumented `filter_horizontal` itself with an `Instant`-based nanosecond counter
(temporary - not part of `patch.diff`, reverted; see below) to measure just the horizontal filter
in isolation, independent of the rest of decode: baseline ~1.32-1.52ms/iter across 9 alternating
runs (avg ~1.39ms), patched ~1.35-1.44ms/iter across the same 9 (avg ~1.38ms) - again no
distinguishable difference.

## Result

No measurable performance change, in either direction, at real-hardware wall-clock granularity -
neither for the whole decode nor for the horizontal filter measured in isolation via a temporary
timer. This holds despite `callgrind`'s **instruction-retirement count** for the horizontal
filter's own code dropping from ~201M to ~185M instructions over 5 iterations (~8% fewer
instructions, matching the SSE-vectorization confirmed via disassembly) - retired-instruction
count is not a wall-clock proxy on real hardware for vectorized vs. scalar code (different
instruction latencies/throughput, branch counts, and pipeline behavior aren't visible to
`callgrind`'s `Ir` metric), which is exactly the trap this candidate's own methodology fell into
before doing the direct wall-clock check.

## Does it change behavior/output?

- **Bit-exact?** yes. `bench_idwt`'s per-iteration FNV-1a hash of the decoded output
  (`0a437d805a8c66c2`) is identical for baseline and patched across every run in this session.
- **Is this a breaking change?** no - it's an inlining-attribute change only, no visible API or
  behavior change.
- Project test suite: `cargo test -p hayro-jpeg2000 -p hayro-tests --release --test tests --
  stream_jpx` - **1 passed, 3 failed both before and after this patch, with identical pixel-diff
  counts** (`stream_jpx_3`: 65974, `stream_jpx_5`: 91173, `stream_jpx_6`: 29580, unchanged by the
  patch). Confirmed via `git stash`/`git stash pop` around the same test run: these 3 failures are
  pre-existing on a clean checkout in this environment, unrelated to this candidate (matches
  candidate 002's RESULT.md note about FMA-vs-non-FMA bulk/remainder precision splits already
  existing in `filter_vertical` on this kind of hardware - likely the same AVX-512-capable-CPU
  cause, not investigated further here since it's out of this candidate's scope).

## Why the result looks the way it does

The profiling data was real and the mechanism (auto-vectorization recovered by de-inlining) is
directly confirmed in the disassembly - but the absolute time this loop takes is apparently
dominated by something callgrind's `Ir` metric doesn't capture: most likely memory-latency-bound
strided access (the `scanline[i-1]`/`scanline[i+1]` reads are only 4 bytes apart in a small
per-row buffer that mostly fits in L1, so this isn't obviously a cache-miss story either) or the
fact that 4-wide SSE with a shuffle-heavy degather is simply not that much cheaper per real cycle
than 4 independent scalar mul-add chains, which a modern out-of-order CPU can already execute with
significant instruction-level parallelism on its own. Without real `perf` (cycle counts, branch
mispredictions, actual port utilization), this can't be narrowed further this session - see "open
questions" below. Candidate 002's suspect #1 ("poor inlining across the closure chain") is now
**ruled out** as the dominant cause: forcing the un-inlined, auto-vectorized shape didn't help.

## What's not done / open questions

- Candidate 002's suspect #2 (unaligned SIMD loads in a full deinterleave rewrite) and #3
  (problem-shape/amortization - many ~600-element buffers per row vs. `filter_vertical`'s
  many-rows-per-call SIMD) remain **unconfirmed**, not newly informed by this session beyond
  ruling out #1. A real `perf record`/`perf annotate` (cycle-level, not instruction-retirement)
  is still the natural next step if this environment ever gets one.
- The temporary `Instant`-based instrumentation used to measure `filter_horizontal` in isolation
  (a `pub(crate) static AtomicU64` in `idwt.rs`, a `pub(crate) mod idwt` visibility change, and a
  `pub fn __debug_horizontal_nanos()` in `lib.rs`, plus a matching print in the bench's
  `src/main.rs`) was reverted from real source before this folder was written - not included in
  `patch.diff` since it's measurement tooling, not the fix. The exact diff is reproducible from
  this RESULT.md's description if a future session wants to reuse it (same shape as candidate
  002's `_reference/` instrumentation diffs, just not saved as one here since it's small).
- `stream_jpx_3/5/6`'s pre-existing pixel-diff failures on this machine were confirmed unrelated
  to this candidate but not root-caused - worth a dedicated look (added to `IDEAS.md`).
