# TILE_RADIX_SORT

`Tiles::sort_tiles` (`sparse_strips/vello_common/src/tile.rs`) sorts the per-scene `Vec<Tile>`
with `sort_unstable`, whose `Ord` impl is a straight `u64` comparison (`Tile::to_bits`, itself a
documented no-op bit-reinterpretation of `Tile`'s in-memory layout). A profile of a near-native
render scale showed this sort alone at 8-9% of total samples (`quicksort::<Tile>`), a real,
resolution-independent cost per the task brief (roughly the same share at scale 1.5x and scale
8x) rather than a sampling artifact, and the comparator's own doc comment already frames the
current approach as a deliberate "sort more, render faster" trade-off - worth re-checking whether
a sort that exploits the sort key's structure (bounded `(y, x)` tile-grid coordinates in the high
bits, a wider but still fixed-width `packed_winding_line_idx` in the low 32 bits) beats a generic
comparison sort. This candidate replaces `sort_unstable()` with a hand-written LSD radix sort
(8-bit digits, 8 passes, skipping any digit that is zero across every tile) over the same `u64`
key, everything else in the pipeline (flattening, tiling, strip rendering) left untouched.

Toolchain: `rustc 1.98.0 (88d9e12ae 2026-08-18)`, `cargo 1.98.0 (797e8a9bc 2026-08-05)`.

## What was tried

Bench: the existing `sort` Criterion group in `sparse_strips/vello_bench` (`cargo bench --bench
main --features extended -- sort`), which clones a `Tiles` container already tiled from the
Ghostscript Tiger SVG (the only scene currently available in this harness - `vello_bench/data/`
ships empty, gitignored, in this checkout) and times only `Tiles::sort_tiles()` per iteration.
That scene tiles into 15,971 `Tile`s (`n_tiles = 15971`, `max_key = 0x003f003f0003fff0` measured
via a temporary debug print, removed before the final patch) - of the 8 possible 8-bit digits,
digits 3, 5 and 7 are all-zero for this scene and get skipped by the radix sort's early-out, so 5
of 8 passes actually run.

Two real binaries were built and compared directly (`baseline_main` / `patched_main`, distinct
sizes/hashes confirmed) rather than rebuilding per run, to avoid stale-binary risk. Paired,
alternating runs, `taskset -c 0`, both against the identical scene:

| Variant | single-core p50 (early runs, heavy sibling load) | single-core p50 (later runs, quiet system) | notes |
|---|---|---|---|
| baseline (`sort_unstable`) | 119.1 / 151.6 / 107.8 / 106.8 µs | 97.8 / 97.9 µs | 6 paired runs total |
| patched (radix sort) | 105.9 / 110.4 / 106.5 / 101.3 µs | 100.9 / 101.7 µs | same 6 paired runs |

The first 4 of those 6 pairs were measured while sibling `___OPTIMIZATIONS___` candidates in this
same batch were building/benchmarking concurrently (`uptime` showed load average ~23 on a 24-core
machine); the shared `flock` serializes actual heavy commands but does not stop the OS from
scheduling other work onto the pinned core in between, matching MEASUREMENT.md's documented
"`taskset` alone doesn't guarantee isolation" failure mode - baseline visibly absorbed more of
that noise (119-152 µs) than patched did (106-110 µs) in those 4 pairs. Once the system quieted
(pairs 5-6, and a separate follow-up unpinned/multi-core round below), both binaries settled into
a tight, low-variance band and the direction reversed and became consistent:

| Variant | single-core p50 (quiet, 2 pairs) | unpinned "multi-core" p50 (quiet, 3 pairs) |
|---|---|---|
| baseline (`sort_unstable`) | 97.80, 97.90 µs | 97.80, 97.72, 98.16 µs |
| patched (radix sort) | 100.94, 101.75 µs | 101.48, 101.08, 101.74 µs |

Under quiet-system conditions (5 total pairs across the pinned and unpinned rounds), the patched
build is slower in all 5, by a consistent ~3-4 µs (~3.5%).

## Result

At this scene's tile count (15,971 tiles/call, 5 of 8 radix passes actually executed), the radix
sort is **~3.5% slower** than `sort_unstable()`, consistently, once system noise is controlled for
(5/5 quiet paired runs favor baseline; the 4 noisy early pairs that appeared to favor the patch are
attributed to asymmetric contention hitting the baseline run harder, not a real effect - see
above). This is a single-scene, single-tile-count measurement (~16K tiles); no larger scene was
available in the current bench harness to check whether a larger tile count (as implied by the
task brief's "scale 8" profiling, where the sort's cost share held rather than shrank) shifts the
crossover in the radix sort's favor - see "What's not done" below. Only wall-clock time was
measured; no allocation-count or peak-RSS claim is made (the radix sort does add one `O(n)`
scratch-buffer allocation per call that `sort_unstable()` doesn't need, which is a plausible partial
explanation for the constant-factor loss at this size, but this was not separately quantified).

## Does it change behavior/output?

- **Bit-exact?** Yes. `radix_sort_tiles` sorts by the exact same key (`Tile::to_bits()`) that
  `Tile`'s `Ord` impl already used, so the resulting tile order is identical to the previous
  `sort_unstable()` order for any input (verified indirectly: the full `vello_common` test suite,
  which includes `tile::tests::sort_test` and every strip-rendering test that depends on tile
  order, passes unchanged - see below).
- **Is this a breaking change?** No. `sort_tiles`/`Tiles` are otherwise untouched; `radix_sort_tiles`
  is a private helper.
- Project test suite:
  - `cargo test -p vello_common --release`: **159 passed, 0 failed, 1 ignored** before (baseline)
    and after (patched) - identical.
  - `cargo test -p vello_cpu --release --lib --tests`: **81 passed, 0 failed** before and after -
    identical. (`cargo test -p vello_cpu --release --doc` fails to *build* on both baseline and
    patched trees alike - an unrelated, pre-existing issue: `vello_cpu/src/text_debug.rs` imports
    `glifo::atlas::GlyphCacheStats`, which `glifo` only exports under
    `cfg(all(debug_assertions, feature = "std"))`, and doctests build in a profile without
    `debug_assertions`. Confirmed present on a clean, unpatched checkout of the same commit, so
    this is not caused by this candidate.)

## Why the result looks the way it does

`Tile`'s sort key is already a plain `u64` by construction (the struct's field order is chosen
specifically so `to_bits()` compiles to a no-op), so `sort_unstable()` on `Vec<Tile>` is
effectively sorting an array of primitive 64-bit integers - exactly the case Rust's pattern-
defeating quicksort (the current `sort_unstable` implementation) is well-tuned for, including
branchless comparisons and good cache behavior for a single in-place buffer. The radix sort
instead: (1) allocates a same-size scratch `Vec<Tile>` up front, (2) for each of up to 8 passes,
builds a 256-entry count histogram (a full read pass) and then scatters every element into `dst`
at `counts[digit]` (a write pattern that is not sequential in `src` order, so it doesn't get the
same cache/prefetch benefit as `sort_unstable`'s in-place partitioning). For ~16K elements, the
extra allocation plus 5 executed passes' worth of histogram-then-scatter traffic outweighs radix
sort's better asymptotic complexity - the crossover point (if one exists in this codebase's actual
usage range) is apparently above the tile count this bench's only available scene produces.

## What's not done / open questions

- Not tested at a substantially larger tile count. The task brief's own profiling notes that this
  sort's cost *share* barely changed between a 1.5x and an 8x render scale (8.27% vs 9.31%),
  which is also consistent with radix sort not helping - if the real bottleneck scaled favorably
  for radix sort, its relative share should have dropped at the higher tile count, not held
  steady. This candidate could not directly confirm or refute that at a much larger tile count
  (millions of tiles) without a bigger test scene; `vello_bench/data/` is empty in this checkout
  (gitignored, presumably populated by a separate asset-fetch step not run here), and constructing
  a large synthetic scene was out of scope for this candidate's time budget.
- A smaller radix digit width was chosen deliberately (8 bits/256 buckets) specifically to keep
  the per-pass histogram cache-resident, on the assumption that a wider digit (e.g. 16 bits/65536
  buckets, 4 passes) would trade fewer passes for a histogram that no longer fits in L2 cache; this
  was reasoned about but not itself measured as a separate variant, so it's possible a different
  digit width performs better at this or a larger tile count.
- No attempt was made at a hybrid approach (e.g. bucket/counting sort only on the bounded `(y, x)`
  prefix, falling back to a short per-bucket sort/insertion-sort on the remaining
  `packed_winding_line_idx` suffix, which the code comment already frames as the "trailing tie-
  break" field) - this is closer to what the task brief's framing suggested and might avoid the
  full-width radix sort's extra passes over the wide `packed_winding_line_idx` bits; noted as a
  follow-up idea rather than tried here.
- The `flatten::fill`/`flatten_cubic_simd` and `make_tiles_analytic_aa_impl` hotspots named
  alongside the sort in the task brief were not touched by this candidate (one fix per candidate,
  per SKILL.md) - they remain open leads for a separate candidate.
