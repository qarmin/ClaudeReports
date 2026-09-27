# 11 - Reuse femtovg's path and tessellation buffers across primitives

Ported from `perf_test/___OPTIMIZATIONS___/11_femtovg_buffer_reuse/RESULT.md` on 2026-08-23 (original perf_test-relative session; source and any extra notes remain there untouched).

Applied to the tree (together with candidate `006.FEMTOVG_PATH_REUSE`, which it makes worth 6x
more), validated. This is a "vendoring a suspect dependency" case (see
`~/.claude/skills/perf-optimize/SKILL.md`'s "Vendoring a suspect dependency" section): the patch
touches femtovg's own source, not Slint's.

## Why this exists

Candidate `006.FEMTOVG_PATH_REUSE` pooled the `femtovg::Path` objects the renderer builds per
rectangle, saving their `verbs`/`coords` vectors - measured at -2 allocations per rectangle. What
it could not save was the **tessellation** those paths carry: `Path::clear()` dropped the
`PathCache`, and `Path::cache()` built a fresh one (`Vec<Point>`, `Vec<Contour>`) whenever the
transform key did not match, which for a per-frame path is always.

The GPU number measured for candidate 006 made this worth chasing: `rects` 3 000 runs at **5.99
ms/frame with 50 359 allocations** on real hardware, so at 13.4 ns per allocation (`just
allocbench`) roughly **11% of that frame is the allocator** - not the fraction of a percent that
llvmpipe's 46 ms frame suggested.

## What was tried

In the femtovg checkout (`patch.diff`):

- `PathCache::rebuild(&mut self, verbs, transform, tess_tol, dist_tol)` - tessellates into the
  buffers the cache already owns (clear, keep capacity). `PathCache::new` is now
  `default() + rebuild()`.
- `Path::cache()` calls `rebuild` on the existing cache instead of replacing it.
- The cache's transform key became `Option<u64>`: `Path::clear()` sets it to `None`, which means
  "buffers kept, contents stale", instead of dropping the cache. No sentinel key, so a stale
  tessellation can never be served.

In `i-slint-renderer-femtovg` (`patch.diff`, candidate `006.FEMTOVG_PATH_REUSE`): the
thread-local pool of scratch paths, handed out through an RAII `ScratchPath` guard that clears
and returns them.

The two only pay off together: the pool keeps the `Path` alive so that its tessellation buffers
survive, and the reuse keeps them from being dropped on `clear()`.

## Result

Measured (FemtoVG under Xvfb/llvmpipe; allocation counts are hardware independent):

| scene | before (candidates 008-010) | with 06+11 | delta |
| --- | --- | --- | --- |
| `rects` 3 000 | 50 193 | **35 035** | **-15 158 (-30.2%)** |
| `rects` 3 000 `--gradient` | 51 304 | **36 146** | **-15 158 (-29.5%)** |
| `list` 100 000 | 3 189 | **2 064 (-35.3%)** | |

~13.8 allocations per visible rectangle: each rectangle draws a background and a border path, and
each path was allocating verbs, coords, points and contours.

Frame times under llvmpipe stay inside the noise (46.6 vs 45.9 ms) because CPU rasterization
dominates there. Expected on real hardware: -15 158 x 13.4 ns = **~200 us of the 5.99 ms frame,
about -3.4%**.

## Does it change behavior/output?

- **Bit-exact?** Yes. Pixels: `rects`, `rects --gradient`, `list` and `scroll` all
  byte-identical to the pre-candidate-011 build over 5 frames of moving content (compared above the
  status line).
- **Is this a breaking change?** No public API break in the sense that `Path::clear()` and
  `PathCache::rebuild()` are additive/internal femtovg API. It does, however, require every
  downstream workspace that builds `i-slint-renderer-femtovg` to add a
  `[patch.crates-io] femtovg = { path = "../femtovg" }` override (see below), since it depends on
  femtovg internals not present in any published crates.io release.
- femtovg's own tests: 61 passed. `i-slint-core` 361, `slint` 77, interpreter driver 818, Rust
  driver 745 - all pass, 0 failures.

## Why the result looks the way it does

`Path::clear()` (candidate 006) dropped the `PathCache` entirely, so every cleared-and-reused path
still had to build a fresh tessellation (`Vec<Point>`, `Vec<Contour>`) on its next use - exactly
canceling out the savings a pooled path was meant to provide for the tessellation-side buffers.
Making `clear()` keep the cache's buffer capacity and re-tessellate in place (instead of dropping
and rebuilding) lets the pool's benefit extend to tessellation too, which is why 06 and 11 only
pay off in combination.

## What's not done / open questions

The renderer patch needs `Path::clear()`/`PathCache::rebuild()`, which only exist in the local
femtovg checkout, so **every workspace now patches femtovg to it**:
`Cargo.toml`, `examples/Cargo.toml`, `demos/Cargo.toml`, `tests/Cargo.toml` each gained

```toml
[patch.crates-io]
femtovg = { path = "../femtovg" }
```

All four resolve and build; the tests workspace's suites pass. Revert with
`git checkout Cargo.toml examples/Cargo.toml demos/Cargo.toml tests/Cargo.toml` (plus the
lockfiles) if you would rather keep the published femtovg. Upstreaming `Path::clear()` and
`PathCache::rebuild()` to femtovg itself (so a future crates.io release includes them) is the
step that would let this drop the local-path patch.

## Reproduce

```sh
just run          # allocations per frame with the buffer reuse in place
just run-xvfb     # same, under Xvfb
just revert       # revert the whole femtovg-side experiment (candidates 006+011) and rebuild
```

Fully reproducing the bench additionally needs `perf_test/` copied in manually, since it is
untracked in the main repo's git history.

## Reproducibility check after upstream update (2026-08-23)

`patch.diff` still applies cleanly against femtovg HEAD `df54a3c`. Not rebuilt/re-benched this round - re-verified via `git apply --check` only.
