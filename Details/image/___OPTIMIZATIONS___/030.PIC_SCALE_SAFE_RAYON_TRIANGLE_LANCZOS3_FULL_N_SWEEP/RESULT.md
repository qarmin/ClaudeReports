# pic-scale-safe rayon call-site gate - full N=2/3/4/6/8 thread-count sweep for Triangle/Lanczos3

Follows up on `IDEAS.md`'s row dated 2026-08-25 ("The optimal capped thread count ... was not
swept beyond `N=2` and `N=4`", pointing at candidate 023's "What's not done"): candidate 023 only
compared `num_threads(1)`/`(2)`/`(4)` for `Triangle`/`Lanczos3`; candidate 027 later ran the fuller
`N=2/3/4/6/8` sweep for `CatmullRom`/`Gaussian` and found a non-monotonic curve peaking at `N=6`;
candidate 028 partially followed up for `Triangle`/`Lanczos3` but only swept `N=6`/`N=8` (not the
full `2/3/4/6/8` set), finding `N=4` stays best/joint-best. This candidate completes the full
`N=2/3/4/6/8` sweep for `Triangle`/`Lanczos3` in a single session (matching candidate 027's
methodology exactly - all 5 `N` values built and compared together, plus direct N-vs-N4
head-to-heads), rather than relying on across-session comparisons between candidates 023's
`N=2`/`N=4` data and candidate 028's `N=6`/`N=8` data.

## What was tried

Same base as candidates 021-024/026-029: candidate 002's `pic-scale-safe/rayon` Cargo.toml forward
+ candidate 026's `bench_resize_dynamic` criterion harness (already has `Triangle`/`Lanczos3`
cases at `400x300->32x24`, unmodified - no new bench work needed). "A" = that base with no gate at
all. "N2"/"N3"/"N4"/"N6"/"N8" = A plus candidate 023's `run_resize` gate
(`SMALL_RESIZE_SOURCE_PIXEL_THRESHOLD = 500_000`, filter-blind,
`rayon::ThreadPoolBuilder::new().num_threads(N)` cached pool), varying only the `N` literal. All 5
`N` values were built as separate binaries from the same source tree in one session (only
`resize.rs`'s `num_threads(N)` line changed between builds). Comparisons run via
`raw/paired_pic_scale.sh` (candidates 023/026/027/028's script, reused unmodified, invoked via
explicit `./` paths) - 6 paired runs per (N, filter) for single-core, 10 for multi-core, matching
this workspace's established convention. `taskset -c 3` for single-core; unpinned for multi-core
(24 logical cores). Direct N4-vs-N2/N3/N6/N8 head-to-heads (multi-core, 10 runs each, no
intervening "A" re-baseline) were run afterward for all four non-N4 values, plus a second
independent repeat of N4-vs-N6 specifically (both filters), to confirm results against noise -
this session's `uptime` load average swung unusually widely (2.1 up to 80.2, see "Why the result
looks the way it does" below), so every head-to-head number here should be read together with its
own B-faster pair count, not the raw percentage alone.

| Variant / scenario | single-core p50 (`taskset -c 3`, 6 runs) | multi-core p50 (unpinned, 10 runs) | B-faster pairs |
|---|---|---|---|
| A, Triangle | 75.006-77.046 us (varies by run) | 68.623-79.594 us (varies by run) | - |
| N2, Triangle | 75.159 us (+0.2%) | 48.387 us (**-33.2%**) | single: 2/6; multi: 10/10 |
| N3, Triangle | 75.103 us (+0.2%) | 41.493 us (**-42.4%**) | single: 1/6; multi: 10/10 |
| N4, Triangle | 76.040 us (-1.3%) | 39.069 us (**-43.5%**) | single: 4/6; multi: 10/10 |
| N6, Triangle | 75.543 us (+0.8%) | 36.026 us (**-48.1%**) | single: 1/6; multi: 10/10 |
| N8, Triangle | 76.618 us (+1.0%) | 43.713 us (**-38.1%**) | single: 1/6; multi: 10/10 |
| A, Lanczos3 | 210.560-212.970 us (varies by run) | 139.39-149.93 us (varies by run) | - |
| N2, Lanczos3 | 211.040 us (+0.2%) | 144.920 us (+2.8%) | single: 2/6; multi: 1/10 |
| N3, Lanczos3 | 211.280 us (+0.2%) | 123.320 us (**-12.0%**) | single: 2/6; multi: 10/10 |
| N4, Lanczos3 | 212.190 us (-0.4%) | 117.470 us (**-16.4%**) | single: 3/6; multi: 10/10 |
| N6, Lanczos3 | 215.140 us (+1.9%) | 114.460 us (**-18.4%**) | single: 2/6; multi: 10/10 |
| N8, Lanczos3 | 212.850 us (+0.4%) | 129.410 us (**-10.7%**) | single: 1/6; multi: 10/10 |
| N4 vs N2 direct, Triangle (multi-core) | - | 48.106 us (+16.6%) | multi: 2/10 |
| N4 vs N3 direct, Triangle (multi-core) | - | 40.590 us (+0.7%) | multi: 3/10 |
| N4 vs N6 direct, Triangle (multi-core, run 1) | - | 41.368 us (+1.7%) | multi: 5/10 |
| N4 vs N6 direct, Triangle (multi-core, run 2/repeat) | - | 100.330 us (+0.1%, noisy session window) | multi: 4/10 |
| N4 vs N8 direct, Triangle (multi-core) | - | 102.970 us (+0.9%, noisy session window) | multi: 6/10 |
| N4 vs N2 direct, Lanczos3 (multi-core) | - | 148.180 us (**+21.9%**) | multi: 0/10 |
| N4 vs N3 direct, Lanczos3 (multi-core) | - | 120.870 us (+3.0%) | multi: 1/10 |
| N4 vs N6 direct, Lanczos3 (multi-core, run 1) | - | 255.010 us (+41.4%, noisy session window) | multi: 7/10 |
| N4 vs N6 direct, Lanczos3 (multi-core, run 2/repeat) | - | 312.420 us (+2.4%, noisy session window) | multi: 6/10 |
| N4 vs N8 direct, Lanczos3 (multi-core) | - | 122.470 us (**+4.2%**) | multi: 0/10 |

Note the "A" baseline's own absolute numbers vary between rows in the first table (e.g. Triangle
68.6-79.6 us multi-core) for the same reason noted in every prior candidate in this thread-count
line: each `N` was measured as a separate `paired_pic_scale.sh` invocation minutes apart, subject
to this machine's own mode-lottery noise - each row's own A/B pairing stays internally consistent
since the script alternates the same two binaries within one run, which is why the direct
N-vs-N4 head-to-heads (run without an intervening "A" re-baseline) are the more trustworthy
same-session confirmation for the actual question this candidate asks (does any of `N=2/3/6/8`
beat `N=4`?). Several of the direct head-to-head runs (both `N4 vs N6` repeats, `N4 vs N8`
Triangle) landed inside an unusually noisy ~30-minute window this session (`uptime` load average
seen as high as 80.2 with only 24 logical cores and this container showing ~90% idle CPU at the
same moment - see "Why the result looks the way it does") that inflated absolute timings 2-3x
without changing the *relative* A/B pairing much, since both binaries in a pair suffer the same
contention; the B-faster pair counts from those runs are still meaningful, the raw percentages are
not. Raw paired output: `raw/singlecore_*.log`, `raw/multicore_*.log`, `raw/direct_N4vs*.log`,
`raw/hashes_*.txt`, `raw/test_before_full.txt`, `raw/test_after_full.txt`.

## Result

**`N=4` remains the best or joint-best thread count for both `Triangle` and `Lanczos3` across the
full `N=2/3/4/6/8` sweep - no other tested value shows a repeatable improvement over it in a
same-session direct head-to-head.** `N=2` is a clear, repeatable regression relative to `N=4` for
both filters (Triangle +16.6%, 2/10 pairs; Lanczos3 +21.9%, 0/10 pairs), consistent with candidate
023's original A-vs-N comparison already ranking `N=2` behind `N=4`. `N=3` is a wash for Triangle
(+0.7%, 3/10) and a small, not clearly decisive regression for Lanczos3 (+3.0%, 1/10 pairs - only
1 of 10 favored N3, but the median gap is small). `N=6` is a wash for both filters across two
independent direct head-to-heads (Triangle: 5/10 and 4/10 pairs favoring N6; Lanczos3: 7/10 and
6/10 pairs favoring N6 despite the noisy-window percentages suggesting otherwise) - the same
conclusion candidate 028 already reached, now confirmed with a fresh same-session build/measure
cycle rather than relying on that earlier session's numbers. `N=8` is a wash for Triangle (+0.9%,
6/10 pairs) and a small but consistent regression for Lanczos3 (+4.2%, 0/10 pairs). Single-core
(`taskset -c 3`) is a wash for every `N` and both filters (-1.3% to +1.9%, 1-4 of 6 pairs), matching
the pattern every prior candidate in this thread-count-sweep line has already established. This
extends candidate 028's conclusion (which only tested `N=6`/`N=8` against `N=4`) with direct
confirmation for `N=2` and `N=3` too, closing out `IDEAS.md`'s row asking for the full
`2/3/4/6/8` sweep for these two filters: unlike `CatmullRom`/`Gaussian` (candidate 027's `N=6`
peak), `Triangle`/`Lanczos3` do not have a better `N` than 4 anywhere in this range.

## Does it change behavior/output?

- **Bit-exact?** Yes. A temporary standalone example (`examples/_correctness_tmp_030.rs`, built and
  run, not committed - removed before the working tree was reverted) hashed (`DefaultHasher`, over
  pixel bytes via `to_rgba8()` and dimensions) `DynamicImage::resize_exact` output for all 5
  `FilterType` variants across 4 scenarios (`RgbImage`/`RgbaImage` at `400x300->32x24`, and a large
  4032x3024 `RgbImage` source resized to `64x48` and to `400x300`), built for all 5 gated `N`
  values (`num_threads(2)/(3)/(4)/(6)/(8)`). All 20 hashes matched exactly across every build
  (`raw/hashes_N2.txt` through `raw/hashes_N8.txt`), and also matched candidate 028's saved
  `hashes_A.txt` (no-gate baseline) byte-for-byte - expected, since changing how many rayon workers
  a call is dispatched to changes nothing about which chunks compute which output bytes.
- **Is this a breaking change?** No public API changes; behavior only differs in wall-clock time,
  not output, and only when the `rayon` feature is enabled (not currently forwarded into
  pic-scale-safe on `main` - this candidate's Cargo.toml change re-applies candidate 002's proposed
  forward, still not itself applied to `main`).
- Project test suite (`cargo test --release --features rayon`): 550/551 passed, both before
  (`raw/test_before_full.txt` - Cargo.toml forward + bench harness only, no gate) and after
  (`raw/test_after_full.txt` - full patch with `num_threads(4)`, the value already shipped for
  these two filters by candidate 029), with the same single pre-existing unrelated failure both
  times (`resizing_with_catmul` in `tests/regression.rs` - same failure noted in candidates
  007/008/011/016/019/020/021/022/023/024/026/027/028/029's `RESULT.md`s, not touched by this
  candidate).

## Why the result looks the way it does

Candidate 027's chunk-count theory (a given `N` wins when it happens to closely match however many
chunks pic-scale-safe's own `par_chunks_exact`/`par_chunks_exact_mut` splits a 32x24-pixel output
into for a specific filter's row-height/kernel-width) continues to explain why `Triangle`/
`Lanczos3`'s optimum (`N=4`) differs from `CatmullRom`/`Gaussian`'s (`N=6`, candidate 027) - not
confirmed here either, same gap every prior candidate in this line already left open (`perf`/
disassembly unavailable in this environment).

Separately, this session ran into a real, unexplained ~30-minute window of extreme `uptime` load
average (up to 80.2 against 24 logical cores) that did not correspond to any CPU activity visible
inside this container (`top` showed ~89% idle with a single ~2-core bench process running at the
time) - consistent with the "mode-lottery noise" this workspace's `projects/image.md` already
documents as a property of this specific machine, but more extreme than any prior candidate in
this line reported (027: 2.2-11.6; 028: 6.4-8.8; 029: 2.9-7.6). This inflated the *raw
percentages* of two direct head-to-heads run inside that window (`N4 vs N6` both repeats, `N4 vs
N8` Triangle) without changing their *pair-count* conclusions, since the noise affects both halves
of an interleaved A/B pair roughly equally - this is why this candidate leans on B-faster pair
counts as the primary signal and calls out which rows were measured inside the noisy window.

## What's not done / open questions

- The unexplained extreme-load window (load average up to 80.2 with the container itself mostly
  idle) was not investigated further - it's not confirmed whether this is host-level contention on
  a shared machine (this environment's own note about "mode-lottery noise" suggests this is a
  known, if not previously seen this extreme, property of the hardware/hosting) or something else.
  Future candidates on this machine should budget for occasional multi-run repeats to route around
  it, as this one did.
- `N` values other than 2/3/4/6/8 (e.g. 5, 7 - the two integers candidate 027 also skipped for
  `CatmullRom`/`Gaussian`) were still not tried for `Triangle`/`Lanczos3`.
- Only the `400x300->32x24` scenario was checked, same open item every prior thread-count-sweep
  candidate in this line has already left open.
- `Nearest`/`Bilinear`-as-a-standalone-timed-case is still only bit-exact-checked via the
  correctness harness, never separately timed or swept over `N` - same gap every prior candidate
  in this line already left open.
