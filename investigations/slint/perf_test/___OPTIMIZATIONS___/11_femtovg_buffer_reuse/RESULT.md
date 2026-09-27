# 11 - Reuse femtovg's path and tessellation buffers across primitives

**Status: win, applied to the tree (together with candidate 06, which it makes worth 6x more),
validated.**

## Why this exists

Candidate 06 pooled the `femtovg::Path` objects the renderer builds per rectangle, saving their
`verbs`/`coords` vectors - measured at -2 allocations per rectangle. What it could not save was
the **tessellation** those paths carry: `Path::clear()` dropped the `PathCache`, and
`Path::cache()` built a fresh one (`Vec<Point>`, `Vec<Contour>`) whenever the transform key did
not match, which for a per-frame path is always.

The GPU number the user measured made this worth chasing: `rects` 3 000 runs at **5.99 ms/frame
with 50 359 allocations** on real hardware, so at 13.4 ns per allocation (`just allocbench`)
roughly **11% of that frame is the allocator** - not the fraction of a percent that llvmpipe's
46 ms frame suggested.

## The change

In the femtovg checkout (`femtovg_pathcache_reuse.diff`):

- `PathCache::rebuild(&mut self, verbs, transform, tess_tol, dist_tol)` - tessellates into the
  buffers the cache already owns (clear, keep capacity). `PathCache::new` is now
  `default() + rebuild()`.
- `Path::cache()` calls `rebuild` on the existing cache instead of replacing it.
- The cache's transform key became `Option<u64>`: `Path::clear()` sets it to `None`, which means
  "buffers kept, contents stale", instead of dropping the cache. No sentinel key, so a stale
  tessellation can never be served.

In `i-slint-renderer-femtovg` (`slint_scratch_path.diff`, candidate 06): the thread-local pool of
scratch paths, handed out through an RAII `ScratchPath` guard that clears and returns them.

The two only pay off together: the pool keeps the `Path` alive so that its tessellation buffers
survive, and the reuse keeps them from being dropped on `clear()`.

## Measured (FemtoVG under Xvfb/llvmpipe; allocation counts are hardware independent)

| scene | before (candidates 08-10) | with 06+11 | delta |
| --- | --- | --- | --- |
| `rects` 3 000 | 50 193 | **35 035** | **-15 158 (-30.2%)** |
| `rects` 3 000 `--gradient` | 51 304 | **36 146** | **-15 158 (-29.5%)** |
| `list` 100 000 | 3 189 | **2 064 (-35.3%)** | |

~13.8 allocations per visible rectangle: each rectangle draws a background and a border path, and
each path was allocating verbs, coords, points and contours.

Frame times under llvmpipe stay inside the noise (46.6 vs 45.9 ms) because CPU rasterization
dominates there. Expected on real hardware: -15 158 x 13.4 ns = **~200 us of the 5.99 ms frame,
about -3.4%**. Run `just gpu-report` to get the real number.

## Validation

- femtovg's own tests: 61 passed. `i-slint-core` 361, `slint` 77, interpreter driver 818, Rust
  driver 745 - all pass, 0 failures.
- Pixels: `rects`, `rects --gradient`, `list` and `scroll` all **byte-identical** to the
  pre-candidate-11 build over 5 frames of moving content (compared above the status line).

## Tree change this required

The renderer patch needs `Path::clear()`/`PathCache::rebuild()`, which only exist in the local
femtovg checkout, so **every workspace now patches femtovg to it**:
`Cargo.toml`, `examples/Cargo.toml`, `demos/Cargo.toml`, `tests/Cargo.toml` each gained

```toml
[patch.crates-io]
femtovg = { path = "../femtovg" }
```

All four resolve and build; the tests workspace's suites pass. Revert with
`git checkout Cargo.toml examples/Cargo.toml demos/Cargo.toml tests/Cargo.toml` (plus the
lockfiles) if you would rather keep the published femtovg.
