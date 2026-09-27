# 017.peak_alloc_crosscheck

Methodology-validation only, no patch of its own: cross-checks candidate 016's custom
`peak_alloc` `GlobalAlloc`-wrapper peak-concurrently-live-bytes numbers (baseline ~20.76MB,
candidate 009's patch ~31.07MB, on `benches/large_image.jpg`) against an independent, external
memory measurement tool. This folder deliberately has **no `patch.diff`** - it does not modify
jpeg-decoder's own source as a lasting artifact; it only re-applies candidate 009's existing
`patch.diff` to a scratch copy for the duration of the measurement, then reverts.

## Tool selection

- `/usr/bin/time -v` is **not installed** on this machine (`/bin/bash: /usr/bin/time: No such
  file or directory` - no `time` package present).
- `valgrind` **is** installed (3.26.0), including `--tool=massif`. Chosen as the external tool.
- Needed `ulimit -n 1048576` first - the default open-file-descriptor limit on this container
  (`2147483584`) makes Valgrind fail immediately (`Private file creation failed`).

## What was tried

1. Built `harness/bench_decode` release binary against unpatched `src/` (base commit
   `eb2d7c0`) -> copied aside as `bench_decode_baseline`.
2. `git apply ___OPTIMIZATIONS___/009.gated_row_batching/patch.diff` to `src/decoder.rs`, rebuilt
   -> `bench_decode_009`. `git checkout -- src/` immediately after (no lasting change).
3. Ran both binaries under `valgrind --tool=massif --pages-as-heap=no` on
   `benches/large_image.jpg` with `iters=3` (3 decodes; massif's per-allocation instrumentation
   is slow, so a short run was used - peak heap doesn't need many iterations to manifest, and
   `bench_decode`'s own hash check still confirms bit-exactness across those 3 iterations).
   `--pages-as-heap=no` restricts massif to actual heap (`malloc`/`free`-style) allocations,
   which is the closest external analog to what `peak_alloc`'s `GlobalAlloc` wrapper tracks (as
   opposed to full process RSS, which `/usr/bin/time -v` would have reported had it been
   available - see "Why the result looks the way it does" below for why this turned out to be an
   advantage, not a compromise).
4. Took the maximum `mem_heap_B` value across all snapshots in each run's `massif.out.*` file as
   that binary's peak heap.
5. A secondary `--pages-as-heap=yes` run (tracks all `mmap`/`brk` page-level allocations,
   including Valgrind's own instrumentation footprint) was also tried as a rough RSS-ballpark
   sanity check - see note below on why its absolute numbers aren't usable directly.

## Result

| | candidate 016's `peak_alloc` (custom `GlobalAlloc`) | `valgrind --tool=massif --pages-as-heap=no` (external) |
|---|---|---|
| baseline (unpatched) | 20,759,358 bytes | 21,299,365 bytes |
| candidate 009 (patched) | 31,074,514 bytes | 31,574,677 bytes |
| delta (009 - baseline) | 10,315,156 bytes | 10,275,312 bytes |

Both the **direction** and the **magnitude** cross-check cleanly:

- **Direction**: both tools agree 009's patch has a higher peak than the unpatched baseline on
  this image - qualitatively consistent, as candidate 016 reported.
- **Magnitude**: massif's absolute peak-heap numbers are within **2.6%** (baseline) and **1.6%**
  (009) of `peak_alloc`'s figures - both external numbers are slightly *larger* than the custom
  allocator's, consistent with malloc-level bookkeeping/alignment overhead the wrapper's
  requested-size tracking doesn't count. The **delta** between patched and baseline - the actual
  quantity candidate 016's conclusion rests on - matches to within **0.4%** (10,275,312 vs.
  10,315,156 bytes), a far tighter match than "same order of magnitude."
- Bit-exact hash (`2f4deafc37e4ec2b`) was identical across both binaries and all 3 iterations
  under massif, confirming this reused the same known-correct decode path 016/009 already
  validated - not re-derived here, just a sanity check the harness was invoked correctly.
- The secondary `--pages-as-heap=yes` run (baseline 1,755,783,168 bytes; 009 1,776,852,992 bytes)
  is **not** a usable RSS proxy in absolute terms - Valgrind's own shadow-memory/JIT overhead
  dominates that figure under `pages-as-heap=yes`, inflating both numbers by roughly two orders
  of magnitude versus real RSS. It is mentioned only because its delta (21,069,824 bytes) is
  still the same order of magnitude as the heap-only delta above, which is at least not
  inconsistent, but this run was not relied on for the actual cross-check.

## Does it change behavior/output?

Not applicable in the usual sense - this candidate applies no lasting patch to jpeg-decoder's
own source (009's patch was applied only transiently, to a scratch build, and reverted
immediately with `git checkout -- src/`; see step 2 above). No `patch.diff` is included in this
folder for that reason. The only "output" of this candidate is the measurement comparison above.

## Why the result looks the way it does

`--pages-as-heap=no` massif tracks the same *kind* of quantity `peak_alloc`'s custom
`GlobalAlloc` wrapper does - live heap allocations via the process's allocator - rather than
whole-process RSS (stack, loaded library text/data, thread pool overhead for rayon's worker
threads, etc.), which is what `/usr/bin/time -v` would have reported. That turned out to be a
better cross-check for the *specific* question 016 needed answered (does the custom allocator's
peak-live-heap-bytes number reflect a real quantity, not an artifact of its own instrumentation)
than a full-RSS figure would have been, since RSS would have buried the ~10MB heap delta inside a
much larger, noisier process-wide number dominated by unrelated fixed costs. The two tools use
completely independent implementations (a hand-written `GlobalAlloc` impl compiled into the
binary vs. Valgrind's own instrumented-execution heap tracking) and agree on the patched-vs-
baseline delta to within 0.4%, which is strong evidence the custom-allocator technique introduced
in 016 measures a real, correctly-sized effect rather than an artifact of the wrapper itself.

## What's not done / open questions

- `/usr/bin/time -v` itself was never actually tried (not installed) - if a future session has a
  machine with the `time` package available, running it too would add a genuine RSS-level data
  point, which this session's `--pages-as-heap=yes` substitute could not cleanly provide.
- Only `large_image.jpg` was cross-checked (the one image candidate 016's own peak-memory finding
  was based on) - `tower.jpg`'s smaller peak-memory numbers (016: baseline 1,624,148 / 009
  2,416,976 bytes) were not independently re-checked under massif, though there's no reason to
  expect a different outcome given how closely `large_image.jpg` already agreed.
- This does not re-derive or change any of candidate 009's or 016's speed/correctness/memory
  conclusions - it only confirms the measurement *technique* those candidates' peak-memory claims
  rely on.
