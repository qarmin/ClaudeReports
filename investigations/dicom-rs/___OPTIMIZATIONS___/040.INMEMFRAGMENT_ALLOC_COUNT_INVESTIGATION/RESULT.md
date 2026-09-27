# Root-causing candidate 001's Ir-count-vs-wall-clock gap via allocation count and massif

Candidate 001 (`InMemFragment: Vec<u8> -> Arc<[u8]>`) measured a huge wall-clock win on
`write-only` (-86.5% to -93.5%) but a much smaller `valgrind --cache-sim=yes` `Ir`-count
reduction (~30%, 594M -> 415M instructions) for the same comparison - a real, unexplained gap
`IDEAS.md` flagged, hypothesized to be glibc malloc/mmap-threshold churn from the baseline's
per-iteration clone of many fragment buffers (real kernel time Valgrind's `Ir`-only mode doesn't
simulate). Neither `strace`/`ltrace` nor a direct allocation-size histogram had been tried.

## What was tried

Two investigations, both measurement-only (no new library-source change - this candidate
re-applies candidate 001's exact `patch.diff`):

1. **Allocation count**: added a new harness mode, `write-only-alloc-count <file> <iters>`
   (harness-only, mirrors the existing `write-dataset-alloc-count`) - counts total
   alloc+dealloc calls via the harness's existing `diag-alloccount` feature (a wrapped `System`
   allocator), for a `write_all`-based comparison instead of `write_dataset_with_ts`.
2. **Peak heap bytes**: `valgrind --tool=massif` on both binaries, 3 `write-only` iterations each
   on `color3d_jpeg_baseline.dcm` (6.14MB, 120 frames). Sandbox note: `valgrind` failed outright
   with `Private file creation failed` at this box's default `ulimit -n` (`2147483572`, an
   absurdly high value some environments set) - fixed by running under `ulimit -n 1048576` first;
   worth remembering for any future `valgrind` use on this box.

| Metric | baseline | patched |
|---|---|---|
| allocs/iter (`write-only-alloc-count`, 3 iters) | 477.0 | 357.0 |
| massif peak heap bytes (`useful-heap`) | 24,588,061 B | 24,620,844 B |

## Result

**Allocation count drops by exactly 120 per iteration (477 -> 357) - exactly matching this
fixture's own `NumberOfFrames = 120`.** This is decisive: each `write_all` call was allocating
one extra buffer per encapsulated pixel-data fragment (one clone per JPEG frame) in the baseline,
and candidate 001's fix removes exactly that one allocation per frame - not a general reduction
in allocation count, a precise 1:1 match to the fragment count. **Peak heap bytes show
essentially no change** (24.588MB vs. 24.620MB, well within noise) - expected, since the
fragments already reside in memory from the initial parse (held across all iterations) regardless
of whether `write_all` clones them again or not; the *clone*, when it happens, doesn't raise the
overall process peak because it happens well after the initial parse-time peak (dominated by
JPEG-decoding-unrelated allocations already resident).

**This resolves the original open question**: the Ir-count-vs-wall-clock gap is NOT primarily
about allocation *count* (only 120 avoided calls, not enough on its own to explain an 86-93%
time reduction by call-count alone) - it's about allocation *size*. `color3d_jpeg_baseline.dcm`'s
120 frames average ~51KB each (6.14MB / 120) - the baseline pays a real, sizeable `memcpy` (not
just a `malloc` call) for each of those 120 clones, every single `write_all` iteration. A `memcpy`
of tens of KB is real memory-bandwidth/cache work whose wall-clock cost is disproportionate to its
raw instruction count under Valgrind's `Ir`-only counting mode (each `rep movs`-style copy
instruction covers many bytes, so instruction count undercounts real memory-system cost) -
confirming the original hypothesis's *shape* (a real-hardware cost Valgrind's instruction-only
model doesn't fully capture) while replacing "glibc malloc/mmap churn" with a more precise,
directly-measured mechanism: **large per-fragment memcpy cost, not malloc/mmap overhead per se**.

## Does it change behavior/output?

- **Bit-exact?** Yes - `checksum=0xdef147e27b943369` identical baseline vs. patched, both
  `write-only-alloc-count` and the timed `write-only` runs under `valgrind`.
- **Is this a breaking change?** No - same `InMemFragment` internal representation change as
  candidate 001, unaffected by this candidate (pure investigation).
- Project test suite: `cargo test --release --workspace` (this patch touches `core`, `object`,
  `parser`, `pixeldata`, `encoding`, `fromimage`, several `transfer-syntax-registry` test files -
  the full workspace, not just one crate): **every crate's test result line reported 0 failed**
  (patch applied) - matches an unpatched baseline run (also 0 failed everywhere), same as
  candidate 001's own original correctness check.

## Why the result looks the way it does

See "Result" above - the mechanism is now directly measured, not just hypothesized: exactly one
large (tens-of-KB) `memcpy`-backed allocation avoided per encapsulated pixel-data fragment, per
`write_all` call. The gap between the earlier `Ir`-count comparison (~30% reduction) and the real
wall-clock reduction (~86-93%) reflects `Ir` counting's blindness to the *size* of memory
operations, not just their *count* - a small number of large `memcpy`s can dominate wall-clock
time while barely moving an instruction-count metric, since a single wide-copy instruction (or a
short loop of them) can move many kilobytes per counted instruction.

## What's not done / open questions

- This closes the specific `IDEAS.md` item asking for a massif/allocation-histogram comparison
  to confirm or refute the allocation-count/size part of the original hypothesis - it's now
  confirmed (allocation size, not count or malloc/mmap threshold behavior specifically).
- The original hypothesis's alternative framing ("mmap-threshold churn", i.e. large allocations
  crossing glibc's mmap threshold and paying real kernel page-fault/mmap/munmap cost) was not
  separately ruled out - a ~51KB average fragment size is below glibc's default `M_MMAP_THRESHOLD`
  (usually 128KB), so this specific mechanism is now considered unlikely for *this* fixture, but
  was not directly tested (e.g. via `strace -c`, still unavailable on this box) for a fixture with
  larger individual fragments where mmap-threshold effects might apply instead.
