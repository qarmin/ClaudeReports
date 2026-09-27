# RGB -> RGBA scalar expansion (`hayro/src/image.rs`)

Part of stage 2 (interpret+decode, see `../README.md`) - runs once per embedded raster image
without an alpha channel that doesn't need resizing (the common case for a plain opaque JPEG
photo). Exact site: `hayro/src/image.rs:378-382`, the `draw_image` function's opaque-RGB branch,
sitting next to an existing hand-written SIMD `premultiply_rgba` later in the same file.

This candidate tested 4 implementation variants in one investigation, before the current
"one fix per candidate" convention existed - kept as one folder rather than retroactively split
into 4. `patch.diff` (== `candidate_patches/map_array_collect.diff`) is the variant later
re-tested against a more representative input shape and applied - see
[`../005.rgba_small_images/RESULT.md`](../005.rgba_small_images/RESULT.md).

## What was tried

`candidate_patches/*.diff` - each is a real, standalone patch against the exact line range above
(`git apply` each one individually against a clean checkout to reproduce; don't apply more than
one at a time, they all touch the same lines). Each was built, and independently benchmarked via
`bench/` (formerly `bench_rgba_convert/`), a self-contained project (own `Cargo.toml`, no
dependency on the `hayro` crate - these candidates are pure functions over a synthetic buffer):

```
cd bench
taskset -c 2 cargo run --release -- 30
```

30 iterations on a synthetic 4000x3000 RGB buffer, hash-checked every iteration:

| Variant | Patch file | single-core p50 | vs. baseline |
|---|---|---|---|
| baseline (current code) | - | 19.09 ms | - |
| `map_array_collect` (collect into `[u8;4]`, then `bytemuck::cast_vec`) | `candidate_patches/map_array_collect.diff` | 19.07 ms | tie (noise) |
| `manual_loop_zip` (explicit `.zip()` + index writes) | `candidate_patches/manual_loop_zip.diff` | 21.40 ms | +12% slower |
| `manual_loop_uninit` (`Vec::spare_capacity_mut` + `set_len`) | not applicable - see below | 21.37 ms (isolation only) | +12% slower, needs `unsafe` |
| `manual_loop_u32_write` (pack as `u32`, double-buffered) | not re-applied - see below | 40.65 ms (isolation only) | +113% slower - algorithmic bug (double allocation) |

Two of the four tested variants (`manual_loop_uninit`, `manual_loop_u32_write`) don't have a
`candidate_patches/*.diff`:
- `manual_loop_uninit` requires `unsafe { out.set_len(...) }`, which cannot even compile inside
  this crate - `hayro/src/lib.rs:33` has `#![forbid(unsafe_code)]`. It only exists as a standalone
  measurement in `bench/`, a separate project without that crate attribute.
- `manual_loop_u32_write` was already ~2x slower in isolation due to an algorithmic mistake
  (double-buffered: builds a `Vec<u32>` then copies it again into a `Vec<u8>`) - applying it as a
  real patch would just reproduce a known-bad result, so it wasn't repeated as an in-crate patch.

## Result

On this single-large-buffer benchmark, `map_array_collect` is a tie with baseline (within noise);
the other two real-patch variants are regressions (+12%, +113%). Re-tested later against many
*small* images (`img.pdf`'s actual content shape) in
[`../005.rgba_small_images/RESULT.md`](../005.rgba_small_images/RESULT.md): `map_array_collect`
is there a real, reproducible ~2.7% win, never worse.

## Does it change behavior/output?

- **Bit-exact?** yes, for all four variants (same bytes, different allocation/loop strategy).
- **Is this a breaking change?** no.
- Project test suite: not run at this stage (correctness verified via hash-check in the isolated
  bench only) - see `../005.rgba_small_images/RESULT.md` for the full 268-test isolation-suite run
  against the applied variant.

## Why the result looks the way it does

The only thing that would plausibly beat the baseline by a wide margin on this large-buffer shape
is a real SIMD byte shuffle (`pshufb`/`vld3`-style 3-to-4 channel deinterleave). Blocked by two
independent constraints:
- `hayro/src/lib.rs:33` has `#![forbid(unsafe_code)]`.
- `fearless_simd` (hayro's SIMD abstraction) does not expose a byte-level permute/shuffle
  primitive in the version pinned by hayro's `Cargo.lock`.

`map_array_collect`'s allocation-strategy win (single `collect` into a known-size `[u8;4]` element
vs. per-byte `flat_map`) doesn't show up on one large buffer because the fixed per-call overhead
it avoids is amortized across a huge number of elements - it becomes visible once there are many
smaller allocations instead (see `005.rgba_small_images`).

## What's not done / open questions

The remaining unexplored lever (a real SIMD byte shuffle) is still blocked by the two constraints
above; revisiting that needs either a policy decision to allow `unsafe` for one function, or a new
primitive contributed upstream to `fearless_simd`.
