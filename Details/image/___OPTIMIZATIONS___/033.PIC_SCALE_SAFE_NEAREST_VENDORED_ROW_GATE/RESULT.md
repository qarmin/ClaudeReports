# Vendor pic-scale-safe, add a minimum-row gate inside resize_nearest's own rayon dispatch

Follows the `IDEAS.md` row dated 2026-08-25 (sourced from candidates 031/032's open item):
`FilterType::Nearest` resizes at small sizes (e.g. `400x300->32x24`) are 26-37x slower than true
serial `main` once pic-scale-safe's `rayon` feature is forwarded, and no caller-side workaround
(thread-pool size, or skipping the pool entirely) fixes it, because pic-scale-safe 0.1.11's
`src/resize_nearest.rs` calls `par_chunks_exact_mut` unconditionally whenever its own `rayon`
Cargo feature is compiled in, with no minimum-row/size gate of its own - the dispatch cost is
paid regardless of which/how-large a rayon pool absorbs it. This candidate builds the fix
candidate 032 identified but didn't attempt: vendor pic-scale-safe 0.1.11 locally (per SKILL.md's
"Vendoring a suspect dependency") and add a `dst_height >= 32` minimum-row gate directly inside
`resize_nearest`'s own dispatch code, falling back to its existing serial `chunks_exact_mut` loop
for anything smaller, instead of ever calling `par_chunks_exact_mut` at all below that threshold.

## What was tried

Vendored `pic-scale-safe-0.1.11`'s full `src/` (unchanged files copied verbatim from
`~/.cargo/registry/src/index.crates.io-*/pic-scale-safe-0.1.11/`) into `image`'s own
`vendor/pic-scale-safe/`, with one function changed - `resize_nearest` in
`vendor/pic-scale-safe/src/resize_nearest.rs` - refactored to share its per-row closure between a
serial and a `#[cfg(feature = "rayon")]` parallel branch, and gated: `dst_height >= 32` uses
`par_chunks_exact_mut` (unchanged behavior above the threshold), anything smaller falls back to
the existing serial `chunks_exact_mut` loop even when the `rayon` feature is compiled in. Wired
in via `image`'s own `Cargo.toml`: a new `[patch.crates-io] pic-scale-safe = { path =
"vendor/pic-scale-safe" }` section, plus re-applying candidate 002's still-unmerged
`"pic-scale-safe/rayon"` feature forward (needed for the gate to matter at all - without it,
pic-scale-safe's `rayon` feature, and therefore this fix, is never compiled in on `main` today).
`benches/imageops.rs`'s `bench_resize_dynamic` group (candidates 002/031/032's harness, including
the `Nearest` case at `400x300->32x24`) was re-added since it isn't on `main`.

Three source states, same naming convention as candidates 029-032:

- **S (serial / current main)** - pristine `Cargo.toml` (no rayon forward, no `[patch]` section,
  registry pic-scale-safe used as-is), built with `--no-default-features --features
  default-formats` so pic-scale-safe's own `rayon` feature is never compiled in either. Literally
  what `main` does today.
- **A (parallel, ungated)** - candidate 002's `Cargo.toml` forward only (registry pic-scale-safe,
  no vendoring, no gate) - the already-measured regression from candidates 019-032.
- **E (this candidate)** - the vendored, gated `pic-scale-safe` wired in via `[patch.crates-io]`,
  built with `--features rayon`.

Binaries alternated via `raw/paired_pic_scale.sh` (this workspace's established
`--bench <id> --exact`, criterion `time: [.. mid ..]` extraction, reused unmodified from
candidates 023/026-032). `taskset -c 3` for single-core; unpinned for multi-core (24 logical
cores). `uptime` load average read 23-26 during this session (this workspace's `IDEAS.md` already
documents `/proc/loadavg` as host-wide/container-external on this machine, not a reliable signal
for this container's own contention - `top` inside the container showed only this session's own
processes at negligible CPU, so measurement proceeded as in every prior candidate in this line).
8 paired runs for single-core (each run twice), 10 for multi-core (each run twice), matching this
workspace's established convention.

| Variant / scenario | single-core p50 (`taskset -c 3`) | multi-core p50 (unpinned) | notes |
|---|---|---|---|
| S (serial/main), run 1 (vs E) | 1.207 us | 0.737 us | (paired against E) |
| E (this candidate), run 1 (vs S) | 1.196 us (**-0.9%**) | 0.753 us (**+2.3%**) | E-faster: 7/8 single, 4/10 multi |
| S (serial/main), run 2 (vs E) | 1.202 us | 0.720 us | (paired against E) |
| E, run 2 (vs S) | 1.194 us (**-0.6%**) | 0.746 us (**+3.6%**) | E-faster: 6/8 single, 2/10 multi |
| A (ungated), run 1 (vs E) | 9.170 us (median) | 18.213 us (median) | (paired against E) |
| E, run 1 (vs A) | 0.747 us (**-91.8%**) | 0.745 us (**-95.9%**) | E-faster: 8/8 single, 10/10 multi |

Raw paired output under `raw/`: `singlecore_S_vs_E_run{1,2}.log`, `multicore_S_vs_E_run{1,2}.log`
(S vs E), `singlecore_A_vs_E_run1.log`, `multicore_A_vs_E_run1.log` (A vs E).

## Result

**This closes the regression: E lands in the same band as true serial S, not the ~26-37x-slower
band candidates 019-032 measured for the ungated case (A).** E vs A: single-core median 9.170 us
-> 0.747 us (**-91.8%**, ~12.3x faster), multi-core median 18.213 us -> 0.745 us (**-95.9%**,
~24.4x faster), decisive in both directions (8/8 single-core and 10/10 multi-core pairs favored
E) - matching the magnitude of candidates 031/032's originally-measured regression, confirming
this fix removes essentially all of it. E vs S (the more important comparison - does the gated
vendored version actually recover serial performance, not just improve on the regression): a
small, inconsistent difference in both directions, within this workspace's established noise
band - single-core -0.9%/-0.6% (E slightly faster, 7/8 and 6/8 pairs), multi-core +2.3%/+3.6% (E
slightly slower, 4/10 and 2/10 pairs) across two repeat runs each. Neither axis clears this
workspace's own decisiveness bar (`wins >= n-1` or `<= 1` out of n) in either direction, so this
is read as a wash relative to true serial, not a residual win or loss - consistent with the gate
falling back to the exact same serial `chunks_exact_mut` loop pic-scale-safe's own
`#[cfg(not(feature = "rayon"))]` path already used, just reached via a runtime check instead of a
compile-time one.

## Does it change behavior/output?

- **Bit-exact?** Yes. A temporary standalone example (`examples/_correctness_tmp_033.rs`, built
  and run, not committed - removed before the working tree was reverted) hashed (`DefaultHasher`,
  over `to_rgba8()` bytes and dimensions) `DynamicImage::resize_exact` output for `Nearest` on 4
  scenarios (`RgbImage`/`RgbaImage` at `400x300->32x24`, and a `4032x3024` `RgbImage` resized to
  `64x48` and to `400x300` - the last two exercise the gate's *other* branch, since both stay at
  or above the `dst_height >= 32` threshold and should still take the `par_chunks_exact_mut`
  path). Built twice: once as **S** (`--no-default-features --features default-formats`, pristine
  `Cargo.toml`), once as **E** (`--features rayon`, vendored+gated `pic-scale-safe`). All 4 hashes
  matched exactly between the two builds (`raw/hashes_S.txt`, `raw/hashes_E.txt`) - expected,
  since the gate only changes *how* (serial vs. parallel dispatch) each output row is computed,
  never which source pixel maps to which output byte.
- **Is this a breaking change?** No public API changes to `image` itself. The change lives
  entirely in the vendored `pic-scale-safe` copy `image` would need to actually depend on (via
  the `[patch.crates-io]` override added to `Cargo.toml`) plus re-applying candidate 002's still
  unmerged `pic-scale-safe/rayon` feature forward - neither is on `main` today, so this candidate
  reapplies both alongside the actual fix, same as candidates 019-032 all had to.
- Project test suite: `cargo test --release` for S, `cargo test --release --features rayon` for
  E: **550/551 passed both before and after**, identical single pre-existing unrelated failure
  both times (`resizing_with_catmul` in `tests/regression.rs`, recorded in every prior candidate
  in this line back through candidate 007) - no new failures. Full output: `raw/test_S_full.txt`,
  `raw/test_E_full.txt`.

## Why the result looks the way it does

Candidate 032 already established (by reading pic-scale-safe's source after the fact) that the
~9-26us fixed cost candidates 019-032 kept measuring is `par_chunks_exact_mut`'s own internal
rayon dispatch/join-tree setup, not anything the caller's `ThreadPool::install()` wrapper
controls - so the only way to actually avoid it is to never call it at all for small inputs. This
candidate does exactly that inside pic-scale-safe's own `resize_nearest` function (the same file
candidate 032 read to reach that conclusion): the row-processing closure is now shared between a
serial and a parallel iterator, and a plain `dst_height >= 32` check picks which one to run,
compiled in whenever the `rayon` feature is present rather than replacing it entirely (unlike the
crate's own `#[cfg(not(feature = "rayon"))]` path, which removes the parallel branch from the
binary altogether). Below the threshold, this is byte-for-byte the same code path
`--features ""` (no rayon) already took, so it costs the same as true serial - which is exactly
what the E-vs-S numbers show (a wash, not a residual win or loss). Above the threshold (e.g. the
`64x48`/`400x300` correctness-check scenarios, both hashed identically to confirm the *other*
branch wasn't silently broken), nothing changed - `par_chunks_exact_mut` still runs, same as
before this patch, since candidates 002/019's original data showed real, large wins for
large-output resizes and this candidate only targets the small-output regression, not the
already-working large-output case.

## What's not done / open questions

- The `32`-row threshold was carried over as a round-number guess from this line's already-tested
  `400x300->32x24` scenario's own `dst_height` (24, so `32` is the next round number above it),
  not independently bisected the way `IDEAS.md`'s still-open row asks for (the exact
  source/output size crossover across all of 019-032's history was never pinned down either) -
  a real fix landing upstream would want a proper sweep (e.g. 8/16/32/64/128) rather than one
  round number picked to clear the one scenario this line has always measured.
- Only `Nearest` was gated. `Triangle`/`CatmullRom`/`Gaussian`/`Lanczos3` all showed real,
  sometimes very large (candidate 002: ~74-76% for `Lanczos3`) wins from pic-scale-safe's rayon
  path even at small-ish sizes (candidate 019's own data), so applying the same style of gate to
  every filter's dispatch site (not just `resize_nearest`) - each filter's own dispatch lives in
  `fixed_point_dispatch.rs`/`floating_point_dispatch.rs`, not read closely by this candidate - is
  a natural next step but wasn't attempted here; this candidate is scoped to the one filter/file
  candidates 031/032 already isolated as unfixable from the call site.
- Per SKILL.md's "vendoring a suspect dependency" guidance, this candidate does not commit to
  what happens next with the vendored copy - options are: open an upstream PR against
  `awxkee/pic-scale-safe` proposing this gate (or a swept/tuned version of it) directly in
  `resize_nearest.rs`, keep the local `[patch.crates-io]` override in `image`'s own `Cargo.toml`
  indefinitely, or drop it if a future pic-scale-safe release adds an equivalent gate itself. Not
  decided here - this candidate is a measured, working fix, not a decision about which path to
  take with it.
- The vendored copy was not given its own `___OPTIMIZATIONS___/` (SKILL.md's default guidance for
  vendoring a suspect dependency) - the fix is small, single-function, and directly motivated by
  measurements already on file in *this* workspace (candidates 019-032), so a second, separate
  workspace at pic-scale-safe's own root would only fragment one continuous investigation across
  two places; this `RESULT.md`/`patch.diff` is the complete record instead.
