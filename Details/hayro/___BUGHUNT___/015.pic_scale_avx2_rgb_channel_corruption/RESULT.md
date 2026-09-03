# pic-scale's AVX2 RGB (CN=3) downscale convolution corrupts a small strip of pixels

Found while working `___OPTIMIZATIONS___/042.pic_scale_avx2_simd_bench_flag` (perf-optimize
candidate: `hayro-bench`'s own manifest never requested hayro's `simd` feature, so it was
benchmarking pic-scale's scalar fallback the whole time). That candidate's own correctness check
(comparing a `simd`-enabled render of `img.pdf` against a scalar-fallback render) found a real,
reproducible 26-pixel difference and saved both PNGs, but did not fully bisect or minimize it.
This candidate re-verifies that finding independently, locates the exact root cause and the
introducing/fixing commits in pic-scale's own history, and confirms it is already fixed upstream.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Re-render `img.pdf` via hayro's public API (`hayro::render`) with hayro's own default features vs. a separate standalone crate depending on the same path, each declaring its own `[workspace]` | Two independent, freshly-resolved dependency graphs (not respecting the main checkout's committed `Cargo.lock`) | Both variants pulled in pic-scale **0.7.11** (the newest version satisfying hayro's `^0.7.9` requirement) regardless of the AVX2 feature flag, and produced bit-identical output - did not reproduce candidate 042's finding. Misleading at first; root-caused below. |
| Re-run with pic-scale explicitly pinned to `=0.7.9` (the exact version candidate 042 investigated, and what the main checkout's own committed `Cargo.lock` still resolves to) | Same two variants (AVX2 handlers compiled in vs. only the scalar fallback), `img.pdf`, `compare -metric AE -fuzz 0%` | **Reproduces**: 26/500395 pixels differ, `compare -metric PAE` peak error 65535/65535 (full 8-bit-channel-scale deviation) - matches candidate 042's own count and severity exactly |
| Same test re-run with pic-scale pinned to `=0.7.10` | Same methodology | Still reproduces: 26 pixels differ, same magnitude |
| Same test re-run with pic-scale pinned to `=0.7.11` | Same methodology | Bit-identical (0 pixels differ) - bug is gone as of `0.7.11` |
| `git diff 0.7.10 0.7.11 -- src/avx2/` in the sibling `pic-scale` clone at `/home/rafal/Projekty/Obce/pic-scale` | Read diff | Only 7 files touched between those two tags; `src/avx2/rgb_u8.rs` is one of them - the exact file candidate 042 already suspected (`avx2::rgb_u8::convolve_horizontal_rgb_avx_row(s)_4`) |
| Read that file's diff directly | `git diff 0.7.10 0.7.11 -- src/avx2/rgb_u8.rs` | Found the exact bug (see Root cause) |
| `git log -S` on the buggy/fixed lines in the sibling clone | `git log -S "_mm_loadu_si16(src0.as_ptr())"` / `..."(src_ptr0)"` | Introduced by `fe54ab9`, fixed by `fdc2eca` (see Introduced by / Fixed by) |
| End-to-end reproduction via this candidate's real `justfile` (fresh clone, hayro's own `hayro/examples/render.rs` example, `--features simd` vs. `--no-default-features --features embed-fonts,embed-cmaps`) | `just test`: clone at commit `5a5f0e24...`, reproduce (26px, AE), apply `patch.diff` (bumps the clone's own `Cargo.lock` to pic-scale 0.7.11), reproduce again | Before: 26/500395 pixels differ (`5.1959e-05` normalized AE). After: 0 pixels differ. Full run captured in this candidate's own history - reran cleanly from a fresh `rm -rf clone && git clone` |
| `hayro-tests` render suite with the patch applied (pic-scale 0.7.9 -> 0.7.11) vs. without, same fresh clone | `cargo test --release -p hayro-tests --test tests -- render --test-threads=4` | Identical both ways: **274 passed / 1252 failed** (failures are the same pre-existing missing-external-corpus `NotFound`s documented in `___OPTIMIZATIONS___`'s candidates 007/009/042, not caused by either pic-scale version) |

## Status

**Confirmed.**

**Fully reproducible**: **Yes** - the real, published, `#![forbid(unsafe_code)]` `hayro` public
API (`hayro::render`, exercised via hayro's own `hayro/examples/render.rs`), rendering a real PDF
(`img.pdf`, already part of this repo), no fabricated intermediate state. `just test` in this
folder reproduces it end-to-end from a completely fresh clone.

## Class

logic (silent pixel corruption on real content - not a crash, not UB in the Rust-safety sense
since the intrinsics are inside pic-scale's own `unsafe` blocks and never actually go
out-of-bounds, just read the *wrong in-bounds address*).

## Root cause

`pic-scale`'s AVX2 RGB (`CN = 3`) horizontal-convolution "remainder" handlers - the code paths
that handle the last few pixels of a row that don't fill a full SIMD register width - build a
3-byte-wide (packed-as-`u16`+one-more-byte) source pixel by loading 2 bytes with
`_mm_loadu_si16` and inserting the 3rd byte separately. In `pic-scale` 0.7.9/0.7.10
(`src/avx2/rgb_u8.rs`, in `Row4ExecutionUnit::<HAS_DOT>`'s 2-row remainder path and in the
1-row `add_one_weight` function), the correctly offset pointer (`src_ptr0`/`src_ptr1`/`src_ptr`,
already advanced by `start_x * CN` to point at the actual remainder pixel) is computed but then
**not used** for the 2-byte load - the load instead reads from `src0.as_ptr()` /
`src1.as_ptr()` / `src.as_ptr()`, i.e. byte offset 0 of the *entire row*, regardless of which
pixel is actually being processed:

```rust
// pic-scale 0.7.9/0.7.10, src/avx2/rgb_u8.rs (both call sites have this same mistake)
let src_ptr0 = src0.get_unchecked((start_x * CN)..).as_ptr(); // correct pixel offset - computed...
let base_pixel0 = _mm_loadu_si16(src0.as_ptr());               // ...but not used here (bug)
let m_vl0 = _mm_insert_epi8::<2>(base_pixel0, src_ptr0.add(2).read_unaligned() as i32); // the
    // 3rd byte IS read from the correct offset (src_ptr0), so the first 2 channels of the wrong
    // pixel get blended with the 3rd channel of the right one - explaining the "R/G channels
    // inverted, B/A unchanged" symptom candidate 042 observed
```

This only affects the narrow "remainder" tail of a row's convolution (rows whose width isn't an
exact multiple of the SIMD stride), which is exactly why candidate 042 saw damage confined to a
2-pixel-wide vertical strip of one small legend icon rather than the whole image - most of any
given row is handled by the correctly-indexed bulk SIMD loop, only the last 1-2 pixels take this
buggy path.

## Introduced by

fe54ab916500d5995c833db0d5e0ff30d9623a2d "AVX RGB8 improvements" (2025-04-30), in the sibling
`pic-scale` clone at `/home/rafal/Projekty/Obce/pic-scale` - first release containing it: pic-scale
0.6.2 (per `git tag --contains fe54ab9`). Present through every 0.6.x and 0.7.0-0.7.10 release,
including the 0.7.9 that hayro's own committed `Cargo.lock` still pins today.

## Fixed by

fdc2ecab59dc9a9f78be899717fc7dd21ed0a858 "Bugfixes" (2026-07-28), upstream in `pic-scale` itself -
first released as pic-scale **0.7.11**. This is an **upstream fix already released and available**,
not something hayro's own source needs to change; hayro's `Cargo.toml` already permits it
(`pic-scale = { version = "0.7.9", default-features = false }`, a `^0.7.9` requirement). The only
reason hayro is still exposed today is that this repo's own `Cargo.lock` happens to have `pic-scale`
pinned at the older `0.7.9` (see `patch.diff`) - a fresh downstream consumer of the published
`hayro` crate who does not themselves have a stale lock of their own already gets the fix for free.

## The fix

`patch.diff`: a 4-line `Cargo.lock` change bumping `pic-scale` `0.7.9` -> `0.7.11` (`cargo update -p
pic-scale --precise 0.7.11`). No hayro source touched.

- Before: `hayro/examples/render.rs --features simd` vs. `--no-default-features --features
  embed-fonts,embed-cmaps` on `img.pdf` -> 26/500395 pixels differ (`compare -metric AE -fuzz 0%`),
  peak channel deviation 255/255 (`compare -metric PAE`).
- After: 0/500395 pixels differ, bit-identical PNGs.
- Project test suite: `cargo test --release -p hayro-tests --test tests -- render`: **274 passed
  / 1252 failed both before and after** (identical failing-test set; all failures are the
  pre-existing missing-external-corpus `NotFound`s already documented elsewhere in this repo's
  `___OPTIMIZATIONS___/`, unrelated to pic-scale).

## What's not done / open questions

- Only `CN = 3` (RGB, `src/avx2/rgb_u8.rs`) was checked. `src/avx2/rgba_u8.rs` (`CN = 4`) uses a
  different load helper (a full 4-byte load, since RGBA already packs into a clean 32-bit word) and
  was not separately audited for the identical "computed offset, unused" mistake - worth a quick
  read, though the fix commit's own diff (`git diff 0.7.10 0.7.11 -- src/avx2/`) did not touch
  `rgba_u8.rs`, suggesting it either never had this exact bug or wasn't part of this same fix pass.
- The SSE (`src/sse/rgb_u8.rs`) and NEON paths were not diffed the same way; candidate 042 already
  established (informally, not saved as a diff here) that an SSE-only build matched the scalar
  fallback bit-for-bit on this same input, consistent with this bug being AVX2-specific, but that
  was not independently re-verified in this candidate.
- Not reported upstream to the `pic-scale` maintainer (moot in the ordinary case since it's already
  fixed in a released version) - only worth doing if this repo, or another consumer, needs a
  written record for a security advisory / changelog reference.
