# STROKE_BUFFER_GROWTH_MEMMOVE

`StrokeCtx` (`kurbo/src/stroke.rs:204-220`) holds four `BezPath`s and `stroke()` builds a fresh
`StrokeCtx::default()` per call, so all four `Vec<PathEl>` grow from zero capacity every time. The
`stroke` scene is by far the highest-allocation-volume scene in the harness (~25 allocations and
~1.36 MB requested per iteration, two orders of magnitude above every other scene), and
`IDEAS.md`'s profile table attributed **4.7% of the scene's self time to `__memmove_avx512`**. The
candidate was to attribute that memmove and those allocations to a specific buffer and then reserve
capacity up front.

Two of the three premises did not survive measurement (see "Why the result looks the way it does"),
so this folder is as much an attribution record as a patch record.

Toolchain for every compiled-code number below: `rustc 1.98.0 (88d9e12ae 2026-08-18)` /
`cargo 1.98.0 (797e8a9bc 2026-08-05)`. Base commit `ca273499e3e48bd2de6f02aa8e99a148984e45f3`.

## What was tried

All work done in a dedicated `git worktree` at the pinned base commit, with the `kurbo_bench`
harness copied in and repointed at that worktree's own `kurbo` (binary sha256 differs between the
two builds - `d9a11e0a…` baseline vs `bb81e5f0…` patched - and "Compiling kurbo" appeared in the
patched build, so the wiring was verified before any number was trusted). Every build, benchmark
and `cargo test` ran under the shared `flock` so 5 sibling candidates could not corrupt the
measurement.

### 1. Attribution: which buffer owns the allocations (`bench/src/allocprobe.rs`)

A `GlobalAlloc` that logs every `(old_size, new_size)` pair for exactly one `stroke()` call, on the
same input the `stroke` scene uses (`gen_path(100, 8, 2)`, 900 elements in, 17 834 elements out;
`size_of::<PathEl>() == 56`):

| buffer | allocations | new bytes requested | share of bytes |
|---|---|---|---|
| `output` | 9 (`98 → 196 → 392 → 784 → 1568 → 3136 → 6272 → 12544 → 25088` elements) | 1 404 928 | **98.6%** |
| `forward_path` | 6 (`4 → 9 → 18 → 36 → 72 → 144`) | 8 064 | 0.6% |
| `backward_path` | 6 (same ladder) | 8 064 | 0.6% |
| `result_path` | 5 (`4 → 8 → 16 → 32 → 64`) | 3 584 | 0.3% |
| **total** | **26** | **1 424 640** | |

So it is essentially all `output`. `forward_path`/`backward_path`/`result_path` together are
19 712 bytes - 1.4% - because they are reused across subpaths/segments and only ever hold one
subpath's worth of elements.

`result_path` needed no fix: `offset_cubic` opens with `result.truncate(0)`
(`kurbo/src/offset.rs:109`), which retains capacity, so it is already reused across all 800 cubics
within a call. `StrokeCtx::reset` (`stroke.rs:230-241`) deliberately omits `result_path` for the
same reason - that half of the candidate's premise was already done.

### 2. Attribution: where the memmove actually comes from

`perf record -F 3999 -g` on `taskset -c 2 ./bin_baseline stroke 40`:

| symbol | self % |
|---|---|
| `CubicOffset::eval_err` | 61.69 |
| `CubicOffset::offset_rec` | 19.92 |
| `CubicOffset::apply` | 7.41 |
| `__ieee754_atan2_fma` | 1.99 |
| `__sincos_fma` | 1.84 |
| `solve_itp` | 1.55 |
| `Hash::path` (harness checksum) | 1.52 |
| **`__memmove_avx512_unaligned_erms`** | **1.08** |

Not 4.7%. Callgrind (`callgrind_annotate --tree=caller`, `stroke 3`) puts
`__memcpy_avx_unaligned_erms` at 11 780 168 of 742 515 059 `Ir` = **1.59%**, and splits its
callers:

| caller | Ir | calls | share of the memcpy |
|---|---|---|---|
| `offset.rs:CubicOffset::offset_rec'2` (the recursive instantiation) | 10 264 510 | 193 670 | **87%** |
| `offset.rs:CubicOffset::offset_rec` | 664 090 | 12 530 | 5.6% |
| `???:0x4a666a0` (the `realloc`/`Vec` growth path) | 850 124 | 116 | **7.2%** |
| everything else (stdio, `flt2dec`, startup) | ~1 400 | ~80 | ~0.0% |

The memmove is not the `BezPath` buffers. It is `offset.rs`'s **264-byte `ErrEval` struct**
(`err_squared: f64` + `unorms: [Vec2; 8]` + `err_vecs: [Vec2; 8]`, `offset.rs:84-91`) being
returned by value from `eval_err` and copied around inside `offset_rec` - 193 670 copies per 3
iterations, ~65 000 per iteration. The whole `BezPath`-growth contribution is 116 calls / 850 124
`Ir` = **0.11% of the scene**.

### 3. Bound: what removing *all* the allocations buys (`bench/src/bound.rs`)

kurbo already ships the caller-side reuse API, so the ceiling for this entire candidate can be
measured with no patch at all: alternate `stroke()` (fresh ctx, 26 allocations, 1.42 MB) against
`stroke_with()` on a `StrokeCtx` already warmed to final capacity (0 allocations, 12 bytes), in one
process, same build:

| run | `stroke()` p50 | `stroke_with()` p50, 0 allocations | delta | output identical? |
|---|---|---|---|---|
| 1 | 10.0522 ms | 10.0356 ms | **-0.17%** | yes (`0x34c0649eed624132`) |
| 2 | 9.9207 ms | 9.9911 ms | **+0.71%** | yes |
| 3 | 10.1340 ms | 10.1967 ms | **+0.62%** | yes |

Removing every allocation and all 1.42 MB of allocator traffic from a `stroke()` call is worth
**nothing measurable** (-0.17% / +0.71% / +0.62%, straddling zero). No reserve-capacity patch can do
better than that, because it is a strict subset.

### 4. The patch (`patch.diff`, +20/-0)

`stroke_undashed` takes the input iterator's `size_hint().0` and `reserve`s that much on `output`,
`forward_path` and `backward_path` (each genuinely needs at least one element per input element).
`BezPath` had no `reserve`, so one was added next to the existing `with_capacity`. Dashed strokes are
untouched by construction: `dash_iter`'s `size_hint` lower bound is 0, so all three calls are
no-ops there.

Wall clock, single-core `taskset -c 2`, 40 iterations, and multi-core unpinned, via
`tools/paired.sh` (host load average 3.2-4.8, 5 sibling candidates active):

| Variant | single-core p50 | multi-core p50 | allocs/iter | bytes/iter | notes |
|---|---|---|---|---|---|
| baseline, `stroke` (run 1) | 11.9251 ms | 12.2744 ms | 25.0 | 1 361 928 | |
| baseline, `stroke` (run 2) | 11.8620 ms | - | 25.0 | 1 361 928 | noise floor 0.53% |
| patched, `stroke` | 11.9411 ms | - | **13.0** | 1 907 592 | vs 11.9476 same-block baseline |
| baseline, `stroke-miter` | 11.8965 ms | 11.9841 ms | 25.0 | 1 390 600 | |
| patched, `stroke-miter` | 11.9745 ms | - | **13.0** | 1 907 592 | |
| baseline, `dash` | 7.5626 ms | 7.6042 ms | 25.0 | 1 839 608 | |
| patched, `dash` | 7.5497 ms | - | 25.0 | 1 839 608 | unchanged by design |

Paired runs (the pair count is what carries the conclusion, not the medians):

| comparison | median | pairs won | verdict |
|---|---|---|---|
| `stroke`, single-core, RUNS=9 | 11.893 → 11.876 (-0.1%) | 5 of 9 | NOT consistent |
| `stroke`, multi-core, RUNS=9 | 11.933 → 11.908 (-0.2%) | 6 of 9 | NOT consistent |
| `stroke`, single-core, RUNS=15 | 11.862 → 11.861 (-0.0%) | 7 of 15 | NOT consistent |
| `stroke-miter`, single-core, RUNS=9 | 11.852 → 11.902 (+0.4%) | 2 of 9 | NOT consistent |
| `dash`, single-core, RUNS=9 | 7.546 → 7.537 (-0.1%) | 5 of 9 | NOT consistent |

Callgrind `Ir` (deterministic, immune to sibling load), `<scene> 3`:

| scene | baseline `Ir` | patched `Ir` | delta |
|---|---|---|---|
| `stroke` | 742 514 904 | 742 137 797 | **-0.051%** |
| `stroke-miter` | 741 836 675 | 741 441 718 | **-0.053%** |
| `dash` | 462 407 380 | 462 364 402 | -0.009% |

Memory dimensions, `stroke` scene:

| metric | baseline | patched | delta |
|---|---|---|---|
| allocations / iteration | 25.0 | 13.0 | **-48%** |
| allocations / `stroke()` call (allocprobe) | 26 | 13 | **-50%** |
| new bytes requested / iteration | 1 361 928 | 1 907 592 | **+40%** |
| new bytes requested / call (allocprobe) | 1 424 640 | 1 717 184 | **+21%** |
| realloc move volume / call (allocprobe) | 1 418 368 | 1 565 760 | **+10%** |
| final `output` capacity | 25 088 elements (1 404 928 B) | 28 800 elements (1 612 800 B) | **+15%** |
| peak RSS, 6 runs each (kB) | 4776 4612 4784 4720 4776 4832 (p50 4776) | 4660 4484 4492 4704 4728 4492 (p50 4576) | -200 kB, ranges overlap |
| peak RSS from a fresh clone, 6 runs each (kB) | 4848 4780 4780 4816 4720 4620 (p50 4780) | 4732 4728 4712 4784 4652 4452 (p50 4720) | -60 kB, ranges overlap |

## Result

**Time: no change.** -0.1%/-0.2%/-0.0%/+0.4% across four paired comparisons, every one of them
`NOT consistent` (5, 6, 7/15 and 2 of 9 pairs) against a measured 0.53% run-to-run noise floor, and
-0.05% `Ir` on the two affected scenes. This is not "too noisy to tell": experiment 3 above shows
that deleting *every* allocation in a `stroke()` call, using kurbo's own existing API, is also worth
nothing, so there is no win here to resolve.

**Allocation count: a real, deterministic -48%** on `stroke` and `stroke-miter` (25.0 → 13.0 per
iteration; 26 → 13 per `stroke()` call). This is a hardware-independent count, not an estimate.

**Allocated byte volume: a real regression, +40%** per iteration (1 361 928 → 1 907 592) and +21%
per call in the allocprobe's stricter accounting, with the peak `output` capacity growing 15%
(25 088 → 28 800 elements). Not a measurement artifact; see below.

**Peak RSS: no resolvable change** (p50 -200 kB in the worktree, -60 kB from a fresh clone, both
with fully overlapping 6-run ranges against a ~200 kB run-to-run band).

**`dash`: untouched in every dimension**, as designed - `dash_iter`'s `size_hint` lower bound is 0.
(Worth stating explicitly, since candidates 003, 005 and 012 all improved `offset`/`stroke` while
regressing `dash`.)

## Does it change behavior/output?

- **Bit-exact?** **Yes.** All 25 harness scenes produce byte-identical `CHECK` checksums between
  the two builds (`diff` of the full 25-line list is empty). Capacity changes cannot alter float
  math, as expected.
- **Is this a breaking change?** No. It adds one public method, `BezPath::reserve`, which is
  additive; `stroke`/`stroke_with`/`StrokeCtx` signatures and semantics are unchanged. A caller
  passing a lazy iterator with a `size_hint` lower bound of 0 gets exactly the previous behavior.
- Project test suite (`cargo test --workspace --all-features`, run under the lock both times):
  - **before: 252 passed, 0 failed** (187 + 10 + 55 + 0, 4 `test result` lines, 1 ignored)
  - **after: 252 passed, 0 failed** (identical split)
  - `grep -E '^error'` empty in both runs, so neither suite failed to build.
- `cargo build -p kurbo --no-default-features --features libm` (the `no_std` path): builds clean.
- `cargo clippy --workspace --all-features --all-targets`: one warning, `needless_late_init` at
  `kurbo/src/common.rs:389`, which is pre-existing and in a file this patch does not touch. No new
  lint.
- This candidate **is** a change to kurbo's own source, so `patch.diff` exists (+20/-0 across
  `kurbo/src/bezpath.rs` and `kurbo/src/stroke.rs`), verified with `git apply --check` against the
  pinned base commit.

## Why the result looks the way it does

**The `IDEAS.md` 4.7% memmove figure does not reproduce, and even at its real size it is not the
`BezPath` buffers.** `perf` now puts `__memmove_avx512_unaligned_erms` at 1.08% of the `stroke`
scene and callgrind puts `__memcpy_avx_unaligned_erms` at 1.59% of `Ir`, of which 92.6% is called
from `offset_rec` (193 670 + 12 530 calls per 3 iterations) and only 7.2% (116 calls) from the
`Vec` growth path. The `offset_rec` copies are `ErrEval` - 264 bytes of `f64` and `Vec2` arrays
returned by value from `eval_err`. So the candidate's ceiling was 0.11% of the scene, not 4.7%,
before a single line was written.

**Reserving capacity cannot reduce a doubling `Vec`'s byte volume, only its allocation count.**
With growth factor 2, the total bytes a `Vec` ever requests while growing to final capacity `F`
from initial capacity `a` is `≈ 2F - a`, and the total bytes `realloc` ever has to move is
`≈ F - a`. Both are dominated by the *last* doubling, so seeding `a` larger shortens only the cheap
prefix of the ladder - and it changes which power of two `F` lands on, which can make things worse.
That is exactly what happened here: baseline `output` starts at 98 elements (the exact-size reserve
of the first `extend(&forward_path)`) and ends at `98 × 2^8 = 25 088`; patched it starts at 900 and
ends at `900 × 2^5 = 28 800`. 17 834 elements are actually needed, so the patch overshoots by 61%
instead of 41%. **Any** reserve of the form `k × n_input` lands on the same 28 800 for this input
(`900 × 2^k` for every `k`), so no choice of multiplier fixes this - only an estimate of the *output*
size would, and the expansion factor here (19.8x) is a function of `tolerance`, `width` and join
style, not of the input element count. The allocation *count* does drop, because that is
`log2(F/a)`, which the seed genuinely shortens.

**And allocation cost is not on the critical path at all.** Experiment 3 is the load-bearing
measurement: `stroke_with` on a warmed context does zero allocations and zero allocator traffic, is
bit-exact, and is not faster. At 11.9 ms per iteration for a 900-element path, ~65 000 `eval_err`
calls dominate so completely (`eval_err` + `offset_rec` + `apply` = 89% of self time) that 26
`malloc`s and ~1.4 MB of `mremap`-backed growth vanish into the noise. glibc serves the four large
`output` reallocs (>128 kB `MMAP_THRESHOLD`) with `mremap`, which remaps pages rather than copying,
so even the "1.4 MB memmoved" upper bound in the allocprobe log is a worst case that is not actually
paid.

**`result_path` reuse was already correct**, so the second half of the candidate's proposed fix had
nothing to do: `offset_cubic` starts with `truncate(0)` (retains capacity) rather than replacing the
path, and `StrokeCtx::reset` intentionally leaves `result_path` alone.

## What's not done / open questions

- **The real memcpy target was found but not fixed.** `ErrEval` (264 bytes) is returned by value
  from `eval_err` and copied inside `offset_rec`; `offset.rs:227-228` even does
  `let err_init = self.eval_err(...); let mut err = err_init;` where `err_init` has no other use, an
  outright redundant 264-byte copy. Worth ~1.5% of the `stroke`/`offset` scenes at most, and the
  candidate-006-vs-007 lesson applies (a struct returned through memory is exactly the shape that
  wins there). Not attempted here - different file, different fix, would have made this a bundle.
- **The maintainers' own noted optimization is still open**, and was deliberately not attempted:
  `stroke.rs:205-207` says *"we might not need separate storage for forward and backward paths, we
  can add forward to the output in-place"*. Sized while working this candidate: it would remove 6
  allocations, ~8 kB, and the ~493 kB/iteration `output.extend(&forward_path)` copy - about 25 µs
  at memcpy bandwidth, i.e. ~0.2% of an 11.9 ms iteration, for a ~40-line refactor of
  `finish`/`finish_closed`/`do_join`. Judged not worth the risk on this evidence; experiment 3
  bounds the whole allocation side at <1% anyway.
- A maintainer taking this patch is trading a 48% allocation-count reduction for a 40% increase in
  allocated byte volume and a 15% larger peak `output` buffer, with no measurable time or RSS
  effect. `BezPath::reserve` itself is useful independently of the `stroke_undashed` half, and the
  two halves of `patch.diff` can be separated.
- The bench crate here (`bench/`) is diagnostic (attribution + bound), not a timing harness; wall
  clock came from `harness/kurbo_bench` as usual.
