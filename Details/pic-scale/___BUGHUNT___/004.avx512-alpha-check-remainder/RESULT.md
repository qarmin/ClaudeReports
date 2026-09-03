# AVX-512 non-constant-alpha check drops the row remainder, silently skipping premultiply

Top unchecked backlog row: "unsafe SIMD kernels in `src/avx512/*`, `src/avx2/*`, `src/sse/*`,
hardware-exercisable on this machine". Manual code reading first ruled out most of `src/avx512/`
as reachable at all on this specific CPU (see "What was tried"), then focused on the one avx512
kernel that *is* reachable here: `avx512_has_non_constant_cap_alpha_rgba8` in
`src/avx512/check_alpha.rs`, which decides whether premultiply/unpremultiply-alpha is needed
before/after resampling an RGBA u8 image.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual CPU-feature check (`is_x86_feature_detected!`) for every gate used by `#[cfg(feature = "avx512")]` code across the repo | plain `rustc`, this host | Found that essentially every `avx512`-feature-gated fast path in this repo (`src/avx2/rgb_u8.rs`, `vertical_u8.rs`, `rgba_u16_lb.rs`, `rgb_u16_lb.rs`, `plane_u16_lb.rs`, `vertical_u16_lb.rs`, and `src/avx512/rgba_u8_dot_vnni.rs`) additionally gates on `is_x86_feature_detected!("avxvnni")` (AVX-VNNI, VEX-encoded), which is **false** on this host even though `avx512vnni`/`avx512bw`/`avx512f` are all true (this is an AMD Zen4/5-class chip: full AVX-512 but no separate AVX-VNNI extension, which is mostly an Intel client-CPU feature). So most of `src/avx512/` is dead code on this specific machine, contrary to the backlog row's assumption that it's all directly exercisable here. Noted in `IDEAS.md` as a new suspect (dispatch/naming gap, not itself a bug this session root-caused). The one avx512-gated path that *is* reachable here (gates only on `avx512bw`) is `avx512_has_non_constant_cap_alpha_rgba8` (`src/image_store.rs:888`, `src/avx512/check_alpha.rs`). |
| Manual boundary-value code reading of `avx512_has_non_constant_cap_alpha_rgba8_impl` (the one reachable avx512 kernel), cross-referenced against the equivalent, known-correct AVX2 implementation (`src/avx2/check_alpha.rs`) in the same file layout | reading only | Found `for chunk in row.as_chunks::<64>().0.iter()`: `<[u8]>::as_chunks::<64>()` returns `(&[[u8;64]], &[u8])` - `.0` is *only* the full 64-byte chunks; the tail remainder (`.1`, 0-63 leftover bytes when the row isn't a multiple of 64 bytes / 16 RGBA pixels) is never iterated at all. The `if chunk.len() == 64 {...} else {...}` masked-load branch right below it can never take the `else` arm, because every `chunk: &[u8; 64]` from `.0` always has `.len() == 64` - so the partial/masked-load code that was clearly written to handle a short last chunk is unreachable dead code. Any alpha variation confined to that dropped tail (or, when a whole row is `< 64` bytes i.e. width `<= 15` RGBA pixels, the *entire* row) is invisible to this function, which returns `false` ("alpha is constant") when it should return `true`. |
| Standalone repro against the public API (`Scaler::plan_rgba_resampling(.., premultiply_alpha: true)`), 8x1 RGBA u8 image, `width*4 = 32 < 64` so the whole row falls in the dropped region, last pixel bright+translucent (`(250,250,250,40)`) next to opaque dark pixels, `Bilinear` 8->4 downscale | release build, `default-features = false, features = ["avx","sse","avx512","threading"]`, this host (`avx512bw` detected: true) | `resample(premultiply_alpha=true)` and `resample(premultiply_alpha=false)` produced **byte-identical** output (`[..., 113,113,113,163]` for the last pixel in both cases) - confirms premultiplication was silently skipped despite being explicitly requested. Rebuilding the same repro with `avx512` feature disabled (forcing the AVX2 path, which *does* handle the tail correctly via its own chunked-cascade-down-to-scalar remainder handling) gave a **different**, correct result for `premultiply_alpha=true` (`[..., 34,34,34,163]`) vs. `false` (`[..., 113,113,113,163]`) on the same input - proving the AVX-512 path specifically is the one that's wrong, not the test input. 5/5 repeat runs of the AVX-512-enabled repro were bit-for-bit identical (deterministic). |

## Status

**Confirmed** - deterministic, 5/5 runs, release build, this host's real AVX-512BW hardware
(`is_x86_feature_detected!("avx512bw")` true). Also reproduces from a completely fresh clone via
`just test` in this candidate's own folder (see `output.txt`).

## Class

logic

## Root cause

`avx512_has_non_constant_cap_alpha_rgba8_impl` (`src/avx512/check_alpha.rs`) is meant to answer
"does this RGBA8 image have non-constant alpha", used by `ImageStore::is_alpha_premultiplication_needed`
(`src/image_store.rs`) to decide, at `src/plan/alpha_plan.rs:54`
(`maybe_premultiply_alpha`), whether to actually premultiply alpha before resampling and
unpremultiply it afterward - a real behavioral difference the public API exposes explicitly via
the `premultiply_alpha: bool` argument of `Scaler::plan_rgba_resampling` (its own doc comment:
"pre-multiplied ... to avoid dark fringing").

The per-row scan iterates `row.as_chunks::<64>().0.iter()`. `<[u8]>::as_chunks::<N>()` splits a
slice into `(full N-sized chunks, remainder)` - `.0` never includes a short last chunk, that's
what `.1` is for. Because only `.0` is iterated, any row whose byte length (`width * 4`) isn't an
exact multiple of 64 has its last 0-63 bytes (up to the last 15 RGBA pixels of that row) never
inspected, and a row shorter than 64 bytes (`width <= 15`) is skipped entirely, contributing
nothing to `sums`. The `if chunk.len() == 64 { .. } else { partial mask .. }` branch a few lines
below is dead code as a direct consequence - it was clearly written expecting a *variable-length*
last chunk (exactly what `.chunks(64)` gives, and exactly what the equivalent, working AVX2
version in `src/avx2/check_alpha.rs` does via its own cascade of `as_chunks::<128>()` ->
`as_chunks::<32>()` -> scalar remainder), but `.as_chunks::<64>().0` never produces one.

Net effect: on any x86_64 host with `avx512bw` and the `avx512` cargo feature enabled, resizing an
RGBA u8 image whose width isn't a multiple of 16 pixels (the overwhelmingly common case) can
silently skip alpha premultiplication/unpremultiplication even when the caller explicitly asked
for it, if the only non-constant alpha in the image happens to fall in the last (width % 16)
pixels of some row (or the image is narrower than 16 pixels altogether) - producing incorrect
"dark fringing" pixel values near the right edge of translucent regions instead of the alpha-aware
blend the caller requested.

## Introduced by

69b140f "Alpha check AVX-512" (2025-01-06) - the remainder-dropping shape was present from this function's very first commit (`row.chunks_exact(64)`, which also silently drops a short last chunk, exactly like the `.as_chunks::<64>().0` it was later changed to). A later commit, 76533f2 "Code review" (2026-07-05), swapped `chunks_exact(64)` for `as_chunks::<64>().0.iter()` - functionally equivalent for this bug (both drop the remainder) - so it did not introduce the defect, only preserved it under a different API. Confirmed via `git show` on both commits.

## The fix

One-line change: iterate the row as `row.chunks(64)` (variable-length chunks, matching what the
already-present `chunk.len() == 64` mask-selection logic downstream was written to expect) instead
of `row.as_chunks::<64>().0.iter()` (fixed-length chunks that drop the remainder). This makes the
pre-existing masked-load branch (`_mm512_mask_loadu_epi8` with a lane count matching the actual
remaining bytes) reachable for the first time - no new unsafe code, no new intrinsics, just makes
dead code live.

```diff
-            for chunk in row.as_chunks::<64>().0.iter() {
+            for chunk in row.chunks(64) {
```

Verified: rebuilt `pic-scale` with this change (`--no-default-features --features
avx,sse,avx512,threading`), re-ran the standalone repro against the patched build - the
`premultiply_alpha=true` output changed from matching `premultiply_alpha=false`
(`[...,113,113,113,163]`) to matching the always-correct AVX2 reference value exactly
(`[...,34,34,34,163]`), 3/3 repeat runs, bit-for-bit identical to the AVX2 path's result.

- Project test suite (`cargo test --no-default-features --features avx,sse,avx512,threading
  --release`): **68 passed, 0 failed** before the fix, **68 passed, 0 failed** after - no
  regressions. (The existing `check_alpha_exists_rgba8`/`check_alpha_not_exists_rgba8` unit tests
  in `src/avx512/check_alpha.rs` use a 256x256 image - `256*4 = 1024` bytes/row, an exact multiple
  of 64 - so they never exercised the remainder path and didn't already catch this.)

## What's not done / open questions

- Not checked whether the equivalent risk exists in any other avx512-gated kernel in this repo -
  moot for correctness-impact purposes right now, since (per "What was tried" above) every other
  `avx512`-feature-gated fast path additionally requires `avxvnni`, which this host's CPU (and,
  per public CPU-feature databases, most current AMD Zen4/5 and many Intel Xeon avx512 parts)
  doesn't have - so they're simply not reachable here to check the same way. Added to `IDEAS.md`
  as its own suspect: a future session on a host that actually has `avxvnni` (e.g. an Intel Alder
  Lake/Raptor Lake client chip) could re-run this same style of check against
  `src/avx512/rgba_u8_dot_vnni.rs` and the various `avx2/*_u16_lb.rs`/`rgb_u8.rs`/`vertical_u8.rs`
  avx512-gated branches.
- Only the u8 RGBA avx512 alpha-check was in scope (the only one this host can reach); the
  scalar/AVX2/SSE alpha-check implementations for the same purpose were not re-audited here beyond
  using the AVX2 one as this candidate's own correctness reference (which it passed).
  `avx512_has_non_constant_cap_alpha_rgba16`/`_f16`/`_f32` do not exist in this codebase at all
  (grep found no avx512 u16/f16/f32 alpha-check functions), so there's no equivalent function to
  check for the same bug in other bit depths - not a gap this candidate introduced.
- Didn't check whether the same `.as_chunks::<N>().0`-drops-remainder shape (as opposed to this
  exact function) appears anywhere else in the codebase as a copy-paste source - worth a targeted
  grep (`as_chunks::<`) across `src/avx512`, `src/avx2`, `src/sse` in a future session; not done
  here since the soft 3-approach cap for this candidate was already reached with a confirmed,
  fixed, and verified bug.
