# RGB -> RGBA expansion, re-tested against many small images (`img.pdf`'s actual shape)

Re-tests [`../001.hayro_image_rgba_scalar/RESULT.md`](../001.hayro_image_rgba_scalar/RESULT.md)'s
`map_array_collect` variant - a tie there against one large synthetic buffer - against a shape
that more closely matches real content.

## Why re-test

The pipeline breakdown (see `../README.md`) found `draw_image` at 13.5-14.7% of `render()`'s time
for `img.pdf`, whose 8 embedded images are all small (four 6x8, two 90x120, one 180x240, one
2154x90) - a very different shape from the single 4000x3000 buffer the original candidate was
benchmarked against. Fixed per-call overhead (allocation growth strategy) can matter more,
proportionally, for many small calls than for one big one.

## What was tried

`patch.diff` - same `map_array_collect` change as `001`'s canonical patch (collect into
`[u8;4]`, then `bytemuck::cast_vec`, instead of a per-byte `flat_map`).

## Benchmark: `bench/`

Self-contained project (own `Cargo.toml`, no dependency on `hayro` - pure functions, same as the
original candidate), using 8 synthetic buffers sized exactly like `img.pdf`'s real images
(258,852 total pixels):

```
cd bench
taskset -c 2 cargo run --release -- 2000
```

3 repeated runs, `taskset -c 2`, 2000 iterations (each iteration = all 8 images), hash-checked:

| Variant | single-core p50 | Δ% vs baseline |
|---|---|---:|
| baseline (current code before this fix) | 861.5-861.9 µs | - |
| `map_array_collect` (this patch) | 837.8-838.1 µs | -2.7% |
| `manual_loop_zip` | 902.97-903.14 µs | +4.8% |
| `manual_loop_u32_write` | 1.36 ms | +58% |

Consistent across all 3 runs (variance <0.1%) - a small but real, reproducible effect, unlike the
tie seen on the single-large-buffer test.

## Result

-2.7%, consistent across 3 repeated runs (variance <0.1%), on the isolated sub-operation. End-to-
end on `img.pdf` (below): within noise.

## Does it change behavior/output?

- **Bit-exact?** yes (same bytes, different allocation strategy).
- **Is this a breaking change?** no.
- Project test suite: 268/268 pass (the same isolation suite used throughout this audit), 0
  regressions, 0 new passes (expected for a bit-exact change).

## End-to-end impact on `img.pdf`

`hayro-bench render_bench` (measures parse+interpret+decode+rasterize, not PNG/JPEG encode),
`taskset -c 2`, 100 iterations each, 5 repeated runs:

| | Run 1 | Run 2 | Run 3 | Run 4 | Run 5 | Mean |
|---|---|---|---|---|---|---|
| Baseline | 3.162 | 3.153 | 3.132 | 3.154 | 3.138 | 3.148 ms |
| With fix | 3.138 | 3.192 | 3.187 | 3.226 | 3.176 | 3.184 ms |

No measurable end-to-end difference - the run-to-run spread (~1-2.5%) is larger than the isolated
~2.7% win on the one sub-operation this fix touches, which is itself only ~14% of total time
(same pattern as `003.hayro_vello_unpremultiply_wired_in`).

## Why the result looks the way it does

The win is real and reproducible in isolation but the sub-operation it touches (RGB->RGBA
expansion for small images) is too small a fraction of `img.pdf`'s total render time for a ~2.7%
local improvement to clear the end-to-end noise floor. A page with more or larger images would be
expected to show more benefit.

## What's not done / open questions

Did not also re-test `manual_loop_uninit` (needs `unsafe`, still blocked by
`#![forbid(unsafe_code)]`, same as the original candidate).
