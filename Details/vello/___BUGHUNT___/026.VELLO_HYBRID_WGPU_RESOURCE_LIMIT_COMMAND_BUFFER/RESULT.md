# VELLO_HYBRID_WGPU_RESOURCE_LIMIT_COMMAND_BUFFER

Three related-but-distinct `vello_hybrid` wgpu-backend robustness reports (linebender/vello#1381,
#1405, #1246), all about the Hybrid backend not degrading gracefully when it runs into a GPU
resource limit or a heavy per-frame `CommandEncoder` workload. This candidate investigates all
three but only reaches a full, run reproduction for #1381's shape of report ("5-10 images with
dimensions in the 1000-3000px range" causing an "out of texture memory" panic) - see "What's not
done" for #1405/#1246.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read `vello_hybrid`'s image-atlas code (`sparse_strips/vello_common/src/{image_cache,multi_atlas}.rs`, `sparse_strips/vello_hybrid/src/render/wgpu/mod.rs`) against the three linked issues | Code reading, ~40 min | Found `Renderer::upload_image`/`upload_image_with` (the wgpu backend's public entry point for registering a decoded CPU-side image with the GPU atlas cache) ends in `image_cache.allocate(width, height, padding).unwrap()` - any `AtlasError` (atlas count limit reached, or a single image too large for one atlas layer) becomes a hard `unwrap()` panic. The identical pattern exists in the WebGL backend's own `upload_image_with`, which even carries an explicit `// TODO: upload_image should return a Result rather than unwrapping here, so callers can handle images that are too large for the atlas instead of panicking.` comment acknowledging the gap. `vello_hybrid`'s own `RenderError` enum already has a ready-made `AtlasError(#[from] AtlasError)` variant with a matching `// TODO: Consider expanding RenderError to replace some .unwrap and .expect.` - i.e. the fix shape was already anticipated in the code, just not wired up for this call site. |
| Environment setup: found a usable software Vulkan adapter in this sandbox (Google SwiftShare CPU implementation, ICD JSON at `/tmp/swiftshader_icd/vk_swiftshader_icd.json`, discovered via `find / -iname 'libvk_swiftshader*'`) - contradicts a prior candidate's (024) note that no GPU/Vulkan adapter was available. `VK_ICD_FILENAMES=/tmp/swiftshader_icd/vk_swiftshader_icd.json` + `wgpu::Backends::VULKAN` reliably yields a real `wgpu::Adapter`/`Device`/`Queue`, letting this session actually drive `vello_hybrid`'s real wgpu `Renderer` end-to-end instead of relying on CPU-only reasoning. | One-off standalone `wgpu` smoke test, ~5 min | Adapter: `SwiftShader Device (Subzero)`, backend Vulkan, `max_texture_dimension_2d: 8192`, `max_texture_array_layers: 2048` (both above `vello_hybrid`'s `AtlasConfig::default()` of `atlas_size: (4096, 4096)`, `max_atlases: 8`, so the default atlas config is unaffected by this adapter's limits). |
| Manual boundary-value construction + real end-to-end repro: real `Renderer::new` (default `RenderSettings`) + real `Renderer::upload_image` in a loop, fed 10 real `vello_common::pixmap::Pixmap`s of 3000x3000px each (`Pixmap` is the crate's own public CPU-side image type; `upload_image` is its own public entry point - this *is* the real trigger path, no faked participant) | Release build, real `wgpu::Device`/`Queue` from the SwiftShader adapter above | **Reproduced**: the 9th upload (`AtlasConfig::default()`: `max_atlases: 8`, `atlas_size: 4096x4096` - each padded 3000x3000 image leaves only a 4096x1096 strip per atlas layer, too small to share a layer with another 3000x3000 image, so 8 layers hold at most 8 such images) panics with `called Result::unwrap() on an Err value: AtlasLimitReached { .. }` at `sparse_strips/vello_hybrid/src/render/wgpu/mod.rs:593`. Reproduces 3/3 runs (2 against a persistent checkout, 1 more against a completely fresh `git clone` via this candidate's own `justfile`) - see `output.txt`. |

## Status

**Confirmed** (for #1381's shape of report). Reproduces 3/3 runs, deterministic (the guillotine
atlas allocator's placement for a fixed sequence of same-sized allocation requests has no
randomness), release build, real (software) Vulkan adapter via SwiftShader.

**Fully reproducible**: **Yes**. `Pixmap` is `vello_hybrid`'s own public CPU-side image
representation and `Renderer::upload_image` is its own public entry point for registering decoded
image data with the GPU atlas cache - the same call a real embedder (Blitz/Servo) makes after
decoding a source image. There is no second participant being faked here (see `CONFIRMATION.md`
section 11's own carve-out for exactly this shape of finding: a real external artifact - decoded
image data - handed to the crate's real public API, with no "other side" to impersonate).
`repro/` runs the actual `Renderer::new` + `Renderer::upload_image` call sequence, driven by a
real `wgpu::Device`/`Queue` obtained from a real Vulkan adapter, exactly as a real caller would.

## Class

crash

## Root cause

`Renderer::upload_image`/`upload_image_with` (`sparse_strips/vello_hybrid/src/render/wgpu/mod.rs`,
around line 593) calls `image_cache.allocate(width, height, padding).unwrap()`. `ImageCache::allocate`
(`sparse_strips/vello_common/src/image_cache.rs`) already returns `Result<ImageId, AtlasError>` -
`AtlasError::AtlasLimitReached` when the atlas-count limit (`AtlasConfig::max_atlases`, default 8)
is reached and no existing/new atlas layer has room, or `AtlasError::TextureTooLarge` when a single
image doesn't fit in one atlas layer at all (default `atlas_size` 4096x4096) - but the wgpu
backend's convenience `upload_image` wrapper discards that `Result` via `.unwrap()`, turning any
such resource-exhaustion condition into a hard process panic instead of a value the caller could
react to (skip the image, downscale it, evict older atlas entries, etc.). The WebGL backend's own
`upload_image_with` has the identical pattern, explicitly flagged by its own inline `TODO`
comment - this is a design gap the project was already aware of, just not yet closed.

This matches linebender/vello#1381's report shape closely: "5-10 images with dimensions in the
1000-3000px range" causing a crash. With the default atlas configuration, images in that size range
consume roughly one atlas layer each once several are uploaded (guillotine fragmentation leaves too
narrow a remaining strip to place a second same-sized image in the same layer), so the fixed
`max_atlases = 8` budget is exhausted well before "5-10" uploads complete - exactly the panic this
candidate reproduces.

## Introduced by

805986f1 "[vello_hybrid] Image rendering (#1064)" (2025-07-02) - the original PR that added image
rendering to `vello_hybrid` already had this shape of bug: its `ImageCache::allocate` returned a
bare `ImageId` and used `.expect("Failed to allocate texture")` internally on the single,
fixed-size atlas of that era. The code has since evolved substantially (multi-atlas support,
auto-grow, a proper `Result<ImageId, AtlasError>` return type from `ImageCache::allocate` itself,
lazy atlas-texture allocation, richer allocation diagnostics), and `ImageCache::allocate` itself
was fixed to return a real `Result` at some point in that evolution - but the wgpu backend's
`upload_image`/`upload_image_with` convenience wrapper (most recently touched for this exact
signature by 65cdf58c "vello_hybrid: Add support for adding padding around images (#1489)",
2026-03-04, though that commit only added the `padding` parameter, not the `.unwrap()` itself)
never got updated to propagate the `Result` instead of unwrapping it. So: the underlying design gap
("resource exhaustion terminates the process") traces to 805986f1, but today's exact panic call
site (`image_cache.allocate(width, height, padding).unwrap()`) has existed in its current form
since 65cdf58c.

## The fix

Changed `Renderer::upload_image`/`upload_image_with` (wgpu backend only, per this candidate's
scope) to return `Result<ImageId, RenderError>` instead of a bare `ImageId`, using the `?` operator
(`RenderError` already has an `AtlasError(#[from] AtlasError)` variant, so no new error type was
needed). Updated the 4 in-repo call sites that broke (`examples/winit/src/main.rs`,
`examples/wgpu_webgl/src/lib.rs`, `vello_sparse_tests/tests/renderer.rs`) to `.expect(...)`
appropriately - none of them were themselves testing this failure mode, so surfacing the error
loudly there (rather than silently swallowing it) is the correct behavior for example/test code.

Verified against the *original* triggering input (10x 3000x3000 uploads: was a panic, now a
`RenderError::AtlasError` returned to the caller with no panic - see `output.txt`) **and** a
second input in the same bug class per `CONFIRMATION.md` section 7 (a single 5000x5000 image,
which hits the *other* `AtlasError` variant, `TextureTooLarge`, since it exceeds the 4096x4096
atlas-layer size outright): both now return gracefully instead of panicking (see `output.txt`).

- Project test suite, before vs. after (identical in both cases - no regression, no newly-fixed
  test either, since none of the existing tests exercised this exact panic path):
  - `cargo test -p vello_hybrid --all-features --lib`: 122 passed; 0 failed -> 122 passed; 0 failed.
  - `cargo test -p vello_common --all-features --lib`: 162 passed; 0 failed; 1 ignored -> same.
  - `cargo test -p vello_sparse_tests --test tests -- hybrid` (real wgpu backend via the same
    SwiftShader adapter): 11 passed; 536 failed; 63 ignored -> same counts, same failing test
    names, both before and after. The 536 failures are pre-existing and environment-caused, not a
    regression from this patch: this sandbox has no working `git-lfs` install, so every reference
    PNG under `sparse_strips/vello_sparse_tests/snapshots/` is an unresolved LFS pointer-stub text
    file rather than a real image, which fails every image-diff assertion in that test binary
    regardless of the patch (this workspace's own `README.md` documents the same `git-lfs`
    limitation independently, from an earlier session).
  - `cargo build --workspace --all-features`: succeeds, before and after.

## What's not done / open questions

- **linebender/vello#1246** ("A single encoder with too many commands can permanently stall (Metal
  backend)") was read and its architecture partially confirmed by code reading: `Renderer::render`
  takes a caller-owned `&mut CommandEncoder` and never submits internally (only `render_to_atlas`
  creates and submits its own encoder immediately, per its own doc comment); the example apps
  (`examples/winit/render_context.rs`) create one encoder per frame and submit once, meaning a
  single complex frame's entire draw/blend/filter workload is recorded into one `CommandEncoder`
  with no periodic mid-frame flush option exposed by the library. This matches the issue's own
  description of the workaround that fixed it (an experimental patch where "Vello Hybrid owns the
  `CommandEncoder` and periodically flushes it"). This was **not independently reproduced this
  session**: reproducing the actual stall requires the Metal backend specifically (macOS-only;
  this sandbox is Linux with no Metal availability at all, software or otherwise) - a genuine "not
  practical this session" per `CONFIRMATION.md` section 11's exception, not silently skipped. Not
  written up as its own candidate this session to keep one-bug-per-folder; worth a dedicated
  future candidate if a macOS runner becomes available.
- **linebender/vello#1405** ("Hang when rendering https://www.wikipedia.org") was read but not
  independently investigated with a real repro this session - it would require a full Blitz
  checkout and a real complex web page, well beyond a single session's scope, and the report itself
  doesn't pin down which of #1381's panic or #1246's stall (or a third, undiscovered cause) it is.
  Plausibly explained by either: a panic during image upload (matching #1381's class, if
  `catch_unwind` isn't used anywhere in Blitz's own render loop, an uncaught panic in a worker
  thread could present as a hang rather than a visible crash) or the command-buffer stall from
  #1246. Not confirmed either way.
- **The WebGL backend has the identical `.unwrap()` bug** in its own `upload_image_with`
  (`sparse_strips/vello_hybrid/src/render/webgl/mod.rs`, already flagged by its own inline `TODO`
  comment) - not fixed in this candidate to keep scope to the wgpu backend as named in this
  candidate's own title; the same fix shape (`Result<ImageId, ...>` instead of `.unwrap()`) would
  apply there too.
- This session did not check whether any *internal* caller of `ImageCache::allocate` for glyph
  atlas caching (`glifo`'s own atlas code, referenced from `sparse_strips/vello_hybrid/src/text.rs`)
  has a similar unguarded `.unwrap()`/`.expect()` on atlas exhaustion - out of scope for "wgpu
  resource limit for images" but the same bug class (a fixed-capacity GPU resource, exhausted by
  real content, panicking instead of degrading).
