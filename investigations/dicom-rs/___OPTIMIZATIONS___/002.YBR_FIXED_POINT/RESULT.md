# YBR_FULL/YBR_FULL_422 -> RGB colorspace conversion: f32+floor -> fixed-point

`convert_colorspace_u8`/`convert_colorspace_u16` (`pixeldata/src/lib.rs`) convert a decoded
YBR_FULL/YBR_FULL_422 pixel buffer to RGB in-place, one pixel at a time. The original
implementation does the whole matrix multiply in `f32`, then `.floor()` (a real libm call) and
`.clamp()` per channel per pixel. Since the coefficients and inputs are simple bounded integers,
a fixed-point integer implementation should avoid the `floorf` call entirely and do only integer
multiply/shift/clamp. Carried over as a well-specified lead from a prior session's notes
(candidate 002 in `~/.claude/skills/perf-optimize/projects/dicom-rs.md`) whose real artifacts
weren't found on disk this session - re-implemented and re-measured fresh here (base commit
`df2b9e4236298efa3fa37a1f360fc4bf284d586a`), independently deriving the fixed-point coefficients
rather than copying remembered code.

## What was tried

Rewrote both functions to Q16.16 fixed-point: coefficients (1.402, 0.114*1.772/0.587,
-0.299*1.402/0.587, 1.772) rounded to the nearest 1/65536 as compile-time `i32`/`i64` constants,
replicating the original's `(... ) + 0.5` then `.floor()` behavior exactly via "add a
half-unit-of-precision, then arithmetic-shift right" (Rust's `>>` on signed integers is already
floor division for two's complement, matching `f32::floor()`'s rounding direction). `u8` uses an
`i32` accumulator (max intermediate magnitude ~28M, far under `i32::MAX`); `u16` needs `i64`
(`y << 16` alone can reach ~4.3B for 16-bit samples, already over `i32::MAX` before adding the
other terms). The `#[cfg(feature = "rayon")]`/`chunks_mut` dispatch structure is unchanged.

Bench: `___OPTIMIZATIONS___/harness/bench_pipeline`, new `colorspace` mode (`obj.decode_pixel_data()`
+ `to_dynamic_image(0)`, hashing the output) against two new 2,000,000-pixel synthetic
`YBR_FULL`/native/`PlanarConfiguration=0` fixtures built from the real
`pydicom/SC_ybr_full_uncompressed.dcm` (100x100) fixture via two new harness one-shot modes:
`gen-ybr` (tiles the real 100x100 content 200x vertically - `ybr_big.dcm`) and `gen-ybr-random`
(same metadata scaffold, pixel data replaced with a fixed-seed xorshift PRNG - `ybr_random.dcm`,
needed because the tiled-real-content file turned out to be bit-exact between builds, so it can't
show the rounding difference this candidate needs to characterize). `dump-image` mode dumps the
raw decoded RGB bytes for byte-diffing.

Single-core numbers use a `--no-default-features --features image,native` build (no `rayon`
linked, confirmed via `nm <bin> | grep -c rayon_core` = 0 both builds) under `taskset -c 0`, per
this workspace's own candidate-001-adjacent finding (and the prior session's note) that pinning a
rayon-enabled binary to one core does not give a valid single-threaded number by itself.

| Variant | single-core p50 (no-rayon build, taskset -c 0) | multi-core p50 (default rayon build, unpinned) | notes |
|---|---|---|---|
| baseline (f32+floor) | 22.560 ms / 16.111 ms (2 rounds) | 6.002 ms / 6.095 ms (2 rounds) | libm `floorf` call per channel per pixel |
| patched (fixed-point) | 8.043 ms / 7.330 ms (2 rounds) | 4.955 ms / 4.954 ms (2 rounds) | integer-only per pixel |

## Result

**Single-core: -64.3% and -54.5% across two independent rounds, 9/9 paired wins both times,
consistent.** **Multi-core (unpinned, default rayon-enabled build): -17.5% and -18.7% across two
rounds, 9/9 paired wins both times, consistent.** Direction and rough magnitude match the prior
session's remembered numbers (~47-57% single-core, ~16-17% multi-core) closely enough to trust as
a genuine re-derivation, not a fluke - this session's single-core win is somewhat larger, plausibly
just fixture/box-dependent (2M random-content pixels here vs. whatever exact fixture size the
original session used).

## Does it change behavior/output?

- **Bit-exact?** No, for random content - **yes**, for the tiled-real-photo fixture. On
  `ybr_big.dcm` (2,000,000 pixels, real tiled photographic content), baseline and patched output
  are byte-identical (`sha256sum` match). On `ybr_random.dcm` (same pixel count, pseudo-random
  byte content), **0.027% of output bytes differ, always by exactly 1 (max diff = 1 LSB)** - 1,630
  of 6,000,000 bytes. Root cause: the fixed-point coefficients are each rounded to the nearest
  1/65536, a strictly less precise representation than an `f32`'s 23-bit mantissa, so a small
  fraction of inputs land exactly on a rounding boundary the two representations resolve
  differently. This only becomes visible on inputs with enough distinct/extreme pixel values to
  hit those boundaries - real photographic content, which is spatially smooth and rarely near a
  boundary at every pixel, did not show it in this test; synthetic random content did.
- **Is this a breaking change?** No public API/type signature changes - both functions are private
  (`fn`, not `pub fn`) and the change is purely internal. The only externally-observable effect is
  the 1-LSB output difference described above for pixel values that happen to land on a rounding
  boundary.
- Project test suite: **537 passed / 0 failed / 6 ignored, both before and after** (full
  `cargo test --workspace`, including doctests) - no new failures.

## Why the result looks the way it does

The single-core win is large because `floorf` is a real function call (not inlined, confirmed by
this workspace's candidate-002-adjacent disassembly technique from the prior session's notes,
not independently re-verified this session) executed twice per pixel (once for `cr`, `cg`, `cb`
each - three calls) on every single pixel; replacing it with a shift is pure ALU work with no call
overhead. The multi-core win is much smaller because `par_chunks_mut`'s own per-chunk dispatch
overhead (one call per single pixel, 3-byte chunks) dominates the per-pixel arithmetic cost either
way at this pixel count - this matches this workspace's own candidate-004/008/009 findings (not
re-verified this session) that rayon's fixed per-call cost is often the larger term next to cheap
per-element work, and is consistent with why a pixel-count threshold before dispatching to rayon
(a separate, not-yet-re-tried backlog item) would compound with this fix rather than being made
redundant by it.

## What's not done / open questions

- Not independently re-verified this session: whether the patched version auto-vectorizes (the
  prior session's notes report "no" for both float and fixed-point, due to the `chunks_mut(3)`
  call-per-pixel boundary) - left as a backlog item (SIMD rewrite) if a future session wants a
  larger win here. See `031.VERIFY_SIMD_YBR_ASM` for a follow-up disassembly-based verification of
  this: confirmed no packed SIMD across pixels either before or after this patch, in both the
  rayon-enabled and no-rayon builds.
- The exact rounding-boundary root cause (each coefficient being ~a few parts-per-million off its
  true value) was reasoned about but not empirically isolated per-coefficient (e.g. by testing
  which specific coefficient's rounding causes which specific differing pixels) - the aggregate
  1-LSB/0.027% characterization is solid, a finer per-coefficient breakdown was not attempted.
- Whether increasing fixed-point precision (e.g. Q16.24 for `u8`, still fits `i32`) would recover
  bit-exactness at no extra runtime cost was not tried - a plausible, cheap follow-up if
  bit-exactness turns out to matter to a reader of this patch.
