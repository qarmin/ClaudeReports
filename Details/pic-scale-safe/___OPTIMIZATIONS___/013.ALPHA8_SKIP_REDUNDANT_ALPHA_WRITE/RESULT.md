# Skip the mathematically-redundant alpha-channel write in premultiply (Mixed - direction flips with build config)

**100% safe-Rust, no `unsafe`, no `#[target_feature]`.** New work this session, measured directly
against this repo (not ported) - a real, verified-exhaustively-correct simplification whose
performance effect turned out to depend entirely on the build's target-CPU setting, in a way that
matters specifically because of a fact about *this repo's own* build config discovered along the
way (see "A surprising prerequisite finding" below). No recommendation either way is baked in here
- see "Result" for both numbers and pick based on which build config your actual users get.

## The idea

`src/alpha.rs`'s in-place premultiply functions (`premultiply_rgba8`, `premultiply_la8`,
`premultiply_rgba16`, `premultiply_la16`) and their allocating counterparts
(`premultiplied_rgba8`, `premultiplied_la8`, `premultiplied_rgba16`, `premultiplied_la16`) all
compute the new alpha-channel value the same way as every other channel: `div_by_255(255 * a)` (or
the bit-depth-appropriate `div_by_2pn_m1(max_colors * a, n)`). This is mathematically always equal
to `a` itself - verified exhaustively in this session (every `a` in `0..=255` for the 8-bit
formula, and every `a` in `0..=max_colors` for `n = 9, 10, 11, 12, 16`, brute-forced with a
standalone Rust program, zero mismatches in every case). For the in-place functions, the channel
already holds `a` before the write, so the line is provably a complete no-op and can be deleted;
for the allocating functions, the call can be replaced with a plain copy (`dst[3] = src[3];`).

## What was tried

Removed the redundant `div_by_255`/`div_by_2pn_m1` call for the alpha channel in all 8 functions
listed above (patch.diff). Added a new criterion bench, `app/benches/premultiply_alpha` (this repo
had no bench reaching `alpha.rs` before this candidate - see `___OPTIMIZATIONS___/README.md`'s
instrumentation section), covering `premultiply_rgba8`/`unpremultiply_rgba8`/`premultiply_la8`/
`premultiply_rgba16` at 1920x1080.

**A surprising prerequisite finding, discovered while setting up the very first disassembly
check:** this repo's own `.cargo/config.toml` sets `rustflags = ["-Ctarget-cpu=native"]` for the
whole workspace (both the crate itself and `app/`). This means every `cargo build`/`cargo bench`
run from within this repo - including this candidate's own measurements below unless explicitly
overridden - already compiles with the host's full native instruction set (AVX-512 on the
measuring machine), **not** the generic x86-64 baseline that candidates 005/006 (measured via a
vendored copy inside a *different* project, `image`, which has no such config) assumed as "the
crate's default". Cargo does not propagate a dependency's own `.cargo/config.toml` to a downstream
consumer - only the invoking package/workspace's own config applies - so this native-cpu setting
benefits this repo's own local dev/bench workflow and nobody else. Disassembling
`premultiply_rgba8` confirmed the practical consequence directly: under this repo's own default
build, the unmodified function already batches **16 pixels per loop iteration using AVX-512
`zmm`/`vpermt2b` permutes** - genuinely excellent auto-vectorization, achieved with zero source
changes, simply because of the ambient build flags. Under `RUSTFLAGS=""` (forcing the generic
baseline any downstream `cargo add pic-scale-safe` user gets by default), the same unmodified
function processes **one pixel per iteration** using an SSE2 register-packing trick
(`movd`/`pinsrw` to build one 4-lane vector per pixel) - matching the file's own `// Almost all
loops are not auto-vectorised without doing anything dirty` comment, which is accurate for the
common case but stale/misleading for this repo's own dev builds.

## Result

Bench: `app/benches/premultiply_alpha`, 1920x1080 buffers, this session's own measuring machine.
Both configs paired-confirmed with repeated runs (2-4x each; magnitudes were stable, criterion's
own internal p-values all < 0.05):

| Function | native-cpu (this repo's own default) | `RUSTFLAGS=""` (downstream-consumer-equivalent) |
|---|---|---|
| `premultiply_rgba8` | 392 us -> 1234-1257 us (**+215% to +224%, regression**) | 3.27 ms -> 2.53-2.55 ms (**-22% to -23%, win**) |
| `premultiply_la8` | 160 us -> 392-976 us (**regression, +145% to +510%** depending on run - see caveat below) | 1.66-1.71 ms -> 955-976 us (**-42% to -44%, win**) |
| `premultiply_rgba16` bd10 | 629-663 us -> 1.24-1.26 ms (**+90% to +100%, regression**) | 2.97-3.09 ms -> 2.32-2.39 ms (**-22% to -24%, win**) |
| `unpremultiply_rgba8` (unchanged code, control) | ~2.0-3.6 ms both before/after, high run-to-run variance | ~1.93-1.98 ms both before/after, stable | 

**Mixed, and the direction is fully determined by target-CPU, not noise.** Under this repo's own
native-cpu build, the patch is a clear, repeatable regression (confirmed via `just test` from a
genuinely fresh clone, 3/3 and 4/4 tight paired comparisons for `premultiply_rgba8` and
`premultiply_rgba16` respectively). Under the generic baseline that most real-world consumers of
this crate get by default, the same patch is a clear, repeatable win (confirmed the same way).
`premultiply_la8`'s native-cpu numbers had more run-to-run spread than the other two (160us->392us
in one paired run, 160us->976us baseline-comparable in another single-shot run) - direction
(regression) was consistent every time, magnitude was not; not fully root-caused, plausibly
AVX-512-frequency-transition-related since `unpremultiply_rgba8` (completely unchanged code)
independently showed unusually high variance specifically on native-cpu runs too (1.98-3.6ms) while
staying tight and stable under `RUSTFLAGS=""` (1.93-1.98ms) in every run.

## Does it change behavior/output?

- **Bit-exact?** Yes - verified via a dedicated correctness-check example
  (`verify_correctness.rs`, kept in this folder for reference, not part of `patch.diff`): 200
  randomized trials of `premultiply_rgba8`/`premultiplied_rgba8`/`premultiply_la8`/
  `premultiplied_la8` (2048-4096 pixels each) plus 50 randomized trials per bit depth (9, 10, 11,
  12, 16) of `premultiply_rgba16`/`premultiplied_rgba16`/`premultiply_la16`/`premultiplied_la16`
  (1024-2048 pixels each), every result compared byte-for-byte against a reference implementation
  using the original (un-simplified) formula - **0 mismatches across all trials**. This rests on
  the exhaustively-verified mathematical identity `div_by_255(255 * a) == a` for all `a` in
  `0..=255`, and `div_by_2pn_m1(max_colors * a, n) == a` for all `a` in `0..=max_colors`, `n` in
  `{9, 10, 11, 12, 16}` - not a sampled approximation.
- **Is this a breaking change?** No - bit-exact output, no `unsafe` anywhere, no public API
  change.
- Project test suite: `cargo test --release --features rayon` - **5 passed, 0 failed**, both
  before and after (this crate's own suite doesn't exercise `alpha.rs` at all, hence the dedicated
  correctness example above).

## Why the result looks the way it does

The original code treats all 4 (or 2) channels of a pixel completely uniformly - same function
call, same shape, differing only in which byte is read. This uniformity is exactly what lets
LLVM's AVX-512 vectorizer batch many pixels per iteration using wide permute instructions
(`vpermt2b`) to de-interleave the 4 channels across 16 pixels at once, then operate on all of them
with identical wide arithmetic. Removing the alpha channel's write breaks that uniformity: the
loop body no longer treats all 4 lanes identically, and the AVX-512 vectorizer falls back to a
much less efficient strategy for the (now 3-way-uniform-plus-1-copy) pattern - hence the large
regression under native-cpu. Under the generic SSE2 baseline, the original code was never batching
multiple pixels per iteration in the first place (confirmed via disassembly - it was already
doing the best it could with one pixel per iteration, per-channel, via a register-packing trick),
so removing one `div_by_255` call per iteration is a straightforward reduction in real work with
no vectorization-shape cost to pay - hence the genuine win there.

## What's not done / open questions

- The same simplification was NOT tried for `unpremultiply_*` (which uses a 65536-entry lookup
  table, a completely different mechanism with no analogous redundant-write identity) or for the
  `_f32` premultiply/unpremultiply variants (which already write `chunk[3] = a`/`chunk[1] = a`
  directly, no redundant division there to begin with - these were already optimal in this
  specific respect).
- Whether this crate's own `.cargo/config.toml` `target-cpu=native` setting should be revisited
  (e.g. documented explicitly as "for local benchmarking only, not reflective of what published
  crate consumers get" or removed) is a separate, real question raised by this candidate but out
  of scope for it - flagged in `IDEAS.md`.
- `premultiply_la8`'s variance under native-cpu specifically (vs. the tighter `premultiply_rgba8`/
  `premultiply_rgba16` numbers) was not root-caused; a `perf stat -e power/energy-pkg/,cycles` run
  during both configs would help confirm or refute the AVX-512-downclocking-transition theory.
