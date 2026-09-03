# pic-scale-safe rayon call-site gate - N=6/N=8 thread-count sweep for Triangle/Lanczos3

Follows up on `IDEAS.md`'s row dated 2026-08-25 ("`N=6` (candidate 027's finding for
`CatmullRom`/`Gaussian`) was never swept for `Triangle`/`Lanczos3`"): candidate 023 only tried
`num_threads(2)`/`num_threads(4)` for `Triangle`/`Lanczos3` (settling on `N=4`); candidate 027
later found `num_threads(6)` beats `N=4` by -6.7% to -9.6% for the sibling filters
`CatmullRom`/`Gaussian` on the identical `400x300->32x24` scenario using the identical gate
mechanism, but `Triangle`/`Lanczos3` were never re-checked past `N=4`. This candidate sweeps
`N=6` and `N=8` (the same two values candidate 027 found to matter - `N=6` as the winner,
`N=8` as the point where the curve fell back off) for `Triangle`/`Lanczos3` at the same scenario.

## What was tried

Same base as candidates 021-024/026/027: candidate 002's `pic-scale-safe/rayon` Cargo.toml
forward + candidate 027's `bench_resize_dynamic` criterion harness (already has `Triangle`/
`Lanczos3` cases at `400x300->32x24`, unmodified - no new bench work needed). "A" = that base with
no gate at all. "N4"/"N6"/"N8" = A plus candidate 023's `run_resize` gate
(`SMALL_RESIZE_SOURCE_PIXEL_THRESHOLD = 500_000`, filter-blind,
`rayon::ThreadPoolBuilder::new().num_threads(N)` cached pool), varying only the `N` literal. Each
of the 3 `N` values was built as its own binary from the same source tree (only `resize.rs`'s
`num_threads(N)` line changed between builds). Comparisons run via `raw/paired_pic_scale.sh`
(candidates 023/027's script, reused unmodified, invoked via explicit `./` paths per candidate
027's noted workaround) - 6 paired runs for single-core sanity checks, 10 for multi-core
comparisons, matching this workspace's established convention. `taskset -c 3` for single-core;
unpinned for multi-core (24 logical cores, `uptime` load average 6.4-8.8 across this session,
re-checked periodically to stay below the 24-core threshold). A same-session direct N4-vs-N6
head-to-head (both filters, multi-core, 10 runs, no intervening "A" re-baseline) was run twice
(once alongside the sweep, once as an independent repeat afterward) to confirm the result against
noise, the same discipline candidate 027 used to confirm its own winning `N`. A direct N4-vs-N8
head-to-head (both filters, multi-core, 10 runs) was also run once.

| Variant / scenario | single-core p50 (`taskset -c 3`, 6 runs) | multi-core p50 (unpinned, 10 runs) | B-faster pairs |
|---|---|---|---|
| A, Triangle | 73.776 us | 71.953 us | - |
| N4, Triangle | 74.846 us (+1.5%) | 41.561 us (**-42.2%**) | single: 0/6; multi: 10/10 |
| N6, Triangle | 75.784 us (+0.6%, different A-run) | 41.832 us (**-42.3%**) | single: 4/6; multi: 10/10 |
| N8, Triangle | 75.558 us (+0.7%, different A-run) | 45.656 us (**-36.9%**) | single: 2/6; multi: 10/10 |
| A, Lanczos3 | 210.250 us | 145.620 us | - |
| N4, Lanczos3 | 214.590 us (+2.1%) | 126.890 us (**-12.9%**) | single: 0/6; multi: 10/10 |
| N6, Lanczos3 | 212.210 us (+0.8%, different A-run) | 132.640 us (**-9.9%**) | single: 2/6; multi: 9/10 |
| N8, Lanczos3 | 212.790 us (+0.7%, different A-run) | 134.540 us (**-11.6%**) | single: 0/6; multi: 8/10 |
| N4 vs N6 direct, Triangle (multi-core) | - | 43.285 us (+0.8%) | multi: 5/10 |
| N4 vs N6 direct, Triangle, repeat (multi-core) | - | 41.002 us (+3.3%) | multi: 2/10 |
| N4 vs N8 direct, Triangle (multi-core) | - | 44.292 us (**+13.5%**) | multi: 0/10 |
| N4 vs N6 direct, Lanczos3 (multi-core) | - | 118.470 us (-0.1%) | multi: 6/10 |
| N4 vs N6 direct, Lanczos3, repeat (multi-core) | - | 119.800 us (-0.0%) | multi: 5/10 |
| N4 vs N8 direct, Lanczos3 (multi-core) | - | 129.610 us (**+6.7%**) | multi: 0/10 |

Note the single-core "A" baseline differs between rows (e.g. Triangle 73.776 us vs 75.784 us) for
the same reason noted in candidates 023/026/027: these are separately re-launched `bench_A` runs
minutes apart, subject to this machine's own documented mode-lottery noise (see
`projects/image.md`) - each row's own A/B pairing is still internally consistent since
`paired_pic_scale.sh` alternates the same two binaries within one run, which is why the direct
N4-vs-N6/N4-vs-N8 head-to-heads (run without an intervening "A" re-baseline) are the more
trustworthy same-session confirmation for the actual question this candidate asks (does `N=6`/`N=8`
beat `N=4`?). Raw paired output: `raw/singlecore_*.log`, `raw/multicore_*.log`,
`raw/repeat_N4_vs_N6_*.log`.

## Result

**`N=6` is a wash against `N=4` for both `Triangle` (+0.8%/+3.3% across two same-session direct
head-to-heads, 5/10 and 2/10 pairs favoring N6) and `Lanczos3` (-0.1%/-0.0%, 6/10 and 5/10 pairs) -
no repeatable improvement, unlike candidate 027's clear -6.7%/-9.6% `N=6` win for
`CatmullRom`/`Gaussian` on the identical scenario and gate mechanism. `N=8` is a clear, repeatable
regression relative to `N=4` for both filters (+13.5%, 0/10 pairs for `Triangle`; +6.7%, 0/10 pairs
for `Lanczos3`).** The A-vs-N sweep in the first table shows the same pattern less precisely (noisier
because of the separately-re-baselined "A" runs): `Triangle`'s A-vs-N multi-core win is
essentially flat from N4 to N6 (-42.2% to -42.3%) then drops at N8 (-36.9%); `Lanczos3`'s A-vs-N
win is largest at N4 (-12.9%) and slightly smaller at N6/N8 (-9.9%/-11.6%), consistent with the
direct head-to-heads showing N6/N8 not beating N4. `N=4` (candidates 023/024's already-established
value) remains the best or joint-best choice found so far for `Triangle`/`Lanczos3` specifically -
this is the opposite conclusion from candidate 027's finding for `CatmullRom`/`Gaussian`, where
`N=6` was a clear, repeatable improvement over `N=4`.

Single-core (`taskset -c 3`) is a wash for every `N` and both filters (+0.6% to +2.1%, 0-4 of 6
pairs favoring the patch), matching the pattern every prior candidate in this thread-count-sweep
line (021-024, 026, 027) already established.

## Does it change behavior/output?

- **Bit-exact?** Yes. A temporary standalone example (`examples/_correctness_tmp_028.rs`, built and
  run, not committed - removed before the working tree was reverted) hashed (`DefaultHasher`, over
  pixel bytes via `to_rgba8()` and dimensions) the resized image for all 5 `FilterType` variants
  across 4 scenarios (`RgbImage`/`RgbaImage` at `400x300->32x24`, and a large 4032x3024 `RgbImage`
  source resized to `64x48` and to `400x300`), built four times: once with no gate at all ("A"),
  once each at `num_threads(4)`/`num_threads(6)`/`num_threads(8)`. All 20 hashes matched exactly
  across all four builds (`raw/hashes_A.txt`, `raw/hashes_N4.txt`, `raw/hashes_N6.txt`,
  `raw/hashes_N8.txt`, identical apart from filter/scenario labels) - expected, since changing how
  many rayon workers a call is dispatched to changes nothing about which chunks compute which
  output bytes.
- **Is this a breaking change?** No public API changes; behavior only differs in wall-clock time,
  not output, and only when the `rayon` feature is enabled (not currently forwarded into
  pic-scale-safe on `main` - this candidate's Cargo.toml change re-applies candidate 002's proposed
  forward, still not itself applied to `main`).
- Project test suite (`cargo test --release --features rayon`): 550/551 passed, both before
  (Cargo.toml forward + bench harness only, no gate) and after (full patch, `num_threads(6)`
  variant), with the same single pre-existing unrelated failure both times
  (`resizing_with_catmul` in `tests/regression.rs` - same failure noted in candidates
  007/008/011/016/019/020/021/022/023/024/026/027's `RESULT.md`s, not touched by this candidate).

## Why the result looks the way it does

Candidate 027's own theory for why `N=6` peaked for `CatmullRom`/`Gaussian` (the pool's thread
count happening to line up closely with however many chunks pic-scale-safe's own
`par_chunks_exact`/`par_chunks_exact_mut` actually splits a 32x24-pixel output into for those two
filters' specific row-height/kernel-width combination) directly predicts a per-filter-dependent
optimum, not a single universal best `N` - which is exactly what this candidate's result shows:
`Triangle`/`Lanczos3` do not share `CatmullRom`/`Gaussian`'s `N=6` peak, and stay at (or best) their
own already-established `N=4`. This is consistent with the two filter pairs' very different
per-row work profiles (`Triangle`/bilinear has almost no kernel work per row; `CatmullRom`/
`Gaussian` sit in between; `Lanczos3` has the most per-row convolution work of the three), which
would plausibly change how a fixed 32x24 output actually gets chunked/scheduled across a pool of a
given size. Not confirmed by `perf`/disassembly or by reading pic-scale-safe's own chunking logic
directly (same gap candidate 027 already left open) - inferred from the measured curve shape alone.

## What's not done / open questions

- `N` values other than 4/6/8 (e.g. 2, 3, 5, 7 - candidate 027's fuller sweep for
  `CatmullRom`/`Gaussian`) were not tried for `Triangle`/`Lanczos3` - it remains possible some other
  `N` between 4 and 8 beats `N=4` for these two filters even though 6 and 8 do not; a full sweep
  matching candidate 027's `N=2/3/4/6/8` was not repeated here since the two values most likely to
  matter (027's own winner and its fallback point) were the ones actually tested.
- The "pool size should roughly match the real chunk count for this output shape" theory (carried
  over from candidate 027) was not confirmed by reading pic-scale-safe's own
  `par_chunks_exact`/`par_chunks_exact_mut` chunk-size calculation
  (`~/.cargo/registry/src/.../pic-scale-safe-0.1.11/`) or via `perf`/disassembly (unavailable in
  this environment, see `README.md`) - doing so could explain directly why `Triangle`/`Lanczos3`'s
  optimum differs from `CatmullRom`/`Gaussian`'s, rather than relying on the "different per-row
  work profile" inference above.
- Only the `400x300->32x24` scenario was checked (same open item every prior thread-count-sweep
  candidate in this line - 021-024, 026, 027 - already left open); whether the optimal `N` shifts
  with a differently-shaped small output was not investigated here either.
- `Nearest` is still only bit-exact-checked, never separately timed or swept over `N` - same gap
  every prior candidate in this line already left open.
