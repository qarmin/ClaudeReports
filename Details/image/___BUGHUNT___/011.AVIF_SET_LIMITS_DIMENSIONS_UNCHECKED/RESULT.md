# AVIF_SET_LIMITS_DIMENSIONS_UNCHECKED

`src/codecs/avif/decoder.rs`: `AvifDecoder::new()` (~line 94) runs a full dav1d decode (`send_data`
+ `read_until_ready`, for both primary and alpha pictures) to completion before `Limits` is even
known, and `AvifDecoder`'s `ImageDecoder::set_limits` override (~line 422) never checks
`max_image_width`/`max_image_height` at all - unlike every sibling decoder (PNG/JPEG/GIF/TIFF/HDR).
Candidate from `IDEAS.md`, tracking upstream issue https://github.com/image-rs/image/issues/3076.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `AvifDecoder::set_limits` vs. the default trait method (`io/decoder.rs:44-47`) and every sibling override (`gif.rs:145`, `tiff.rs:430`, `hdr/decoder.rs:312`, `png.rs:388`, `jpeg/decoder.rs:173`) | Static read of current `main` (`44ce9226`) | `AvifDecoder::set_limits` is `self.limits = limits; Ok(())` - no `check_support`/`check_dimensions` call at all. Every other listed decoder calls `limits.check_dimensions(...)` (directly or via `check_layout_dimensions`) inside its own `set_limits`. This is a structural deviation, not just a timing issue. |
| Manual boundary-value construction + real end-to-end run against the crate's actual public API, using a real AVIF encoded by the crate's own real encoder (`AvifEncoder`/ravif) and decoded by the crate's own real dav1d-backed `AvifDecoder` (`avif-native` feature) | `cargo run --release` against `___BUGHUNT___/_reference/probe` (path-dependency on the real checkout), `avif-native` built via a local link-only pkg-config shim for the system's already-installed `libdav1d.so.7` (no dev headers/meson/ninja/sudo needed - see README note below) | Confirms the bug precisely: `AvifDecoder::new()` on a real 400x400 AVIF takes ~5-6ms (full dav1d decode, vs. ~2ms for an 8x8 baseline); `decoder.set_limits(Limits{max_image_width:10,max_image_height:10})` on the resulting decoder returns `Ok(())` unconditionally. 3/3 identical runs. Also confirmed the documented *defensive* usage (`ImageReaderOptions::new(..).limits(10x10).decode()`, limits set **before** calling `decode()`) still pays the full dav1d decode cost inside `make_decoder()` before the limit is ever consulted - the final `Err` only comes from an unrelated, redundant `ImageReader`-level dimension re-check in `decode_to_dynimage`, not from anything AVIF-specific. |
| Ran the project's own pre-existing test `tests/limits.rs::avif` (already asserts `decoder.set_limits(width_height_limits()).is_err()`) | `cargo test --no-default-features --features "avif avif-native" --test limits avif` | **FAILED** on pristine `main` (`thread 'avif' panicked at tests/limits.rs:190:5: assertion failed: decoder.set_limits(width_height_limits()).is_err()`). This is an existing, correctly-written regression test that has simply never been exercised in CI - `.github/workflows/rust.yml`'s `test_avif_decoding` job only runs `cargo build` with `avif-native`, never `cargo test`. |

## Status

**Confirmed.** Reproduces 3/3 runs (real end-to-end probe, deterministic - no randomness/seed
involved) and also reproduces as an outright test failure in the project's own pre-existing (but
CI-unexercised) `tests/limits.rs::avif` test, 1/1.

**Fully reproducible: Yes.** `repro/` exercises the real, public `image::codecs::avif::AvifDecoder`
and `image::ImageReaderOptions`/`ImageDecoder` API, driven by a real AVIF file produced by the
crate's own real encoder (not hand-authored bytes) and decoded by the crate's own real dav1d
backend (not a stub). This is a single-participant finding (a caller vs. the decoder it configured
itself), so CONFIRMATION.md section 11's "real second participant" requirement doesn't add anything
beyond "run the real public API end-to-end", which this repro does.

## Class

security (resource-limit bypass / partial denial-of-service via unbounded decode cost)

## Root cause

Two compounding issues in `src/codecs/avif/decoder.rs`:

1. `AvifDecoder::new()` performs the *entire* AV1 frame decode (dav1d `send_data` +
   `read_until_ready`, for both the primary and, if present, alpha picture) unconditionally, before
   any `Limits` value is ever available to the decoder - there is no lazy/deferred decode path the
   way header-only-parsing formats (PNG/JPEG/GIF/TIFF headers) have. This matches the crate's own
   documented design assumption at `image_reader_type.rs`'s `make_decoder()` doc comment ("For all
   formats except PNG, the limits are ignored and can be set with `ImageDecoder::set_limits` after
   calling this function") - an assumption that silently breaks for AVIF because its "cheap
   construction, defer real work" contract isn't upheld: construction *is* the real work.
2. Independently of (1), `AvifDecoder::set_limits` never calls `Limits::check_dimensions` (or
   `check_layout_dimensions`) at all, unlike every sibling decoder. Even if a caller does everything
   "right" by the trait's own documented contract (call `set_limits` right after construction, before
   requesting pixel data), the width/height limit is silently accepted and never enforced for AVIF
   specifically. This second issue is bounded and independently fixable; the first is not, without a
   larger architectural change to defer dav1d's decode until dimensions are needed.

## Introduced by

3432fb3 "Defensive programming for avif decoder limits" (2026-05-21) - this commit added
`AvifDecoder`'s first-ever `set_limits` override, storing `limits` without calling
`Limits::check_dimensions`/`check_layout_dimensions`. Before it, `AvifDecoder` had no override and
used `ImageDecoder`'s default `set_limits` (`src/io/decoder.rs:44`), which does call
`check_layout_dimensions` - so this commit is a regression, not a pre-existing gap.

## The fix

Added the missing dimension check to `AvifDecoder::set_limits`, matching the pattern used by every
sibling decoder (`limits.check_support(...)` + `limits.check_dimensions(width, height)`, using the
already-known `self.picture` dimensions since decode has already happened by this point):

```rust
fn set_limits(&mut self, limits: Limits) -> ImageResult<()> {
    limits.check_support(&crate::LimitSupport::default())?;
    limits.check_dimensions(self.picture.width(), self.picture.height())?;
    self.limits = limits;
    Ok(())
}
```

This closes root-cause issue (2): a caller using the documented `ImageDecoder::set_limits` contract
(e.g. via `into_decoder()`) now gets real width/height enforcement for AVIF, consistent with every
other format. Verified against more than the one triggering input (CONFIRMATION.md section 7): a
64x32 real AVIF with a limit exactly matching (accepted, correctly) and limits one-under on width
only / height only (both correctly rejected) - see `output.txt`'s "Boundary-value check" section.

This does **not** fix root-cause issue (1): `AvifDecoder::new()` still unconditionally pays the full
dav1d decode cost regardless of any limit, including in the documented defensive usage pattern
(`ImageReaderOptions::limits(...)` set before `.decode()`) - confirmed still true after the patch
(see `output.txt`, "Scenario B" timing is unchanged before/after). Fixing that would require
deferring the dav1d decode itself until dimensions are needed (e.g. a cheap header-only parse via
`mp4parse` first, or restructuring the decode entry point to accept limits up front) - an
architecture-level change to how `AvifDecoder` is constructed, out of scope for a bounded fix this
session, and the reason this exact class of finding is still open upstream as
https://github.com/image-rs/image/issues/3076.

- Project test suite:
  - `tests/limits.rs::avif` (avif-native feature): **failing before -> passing after** (the only
    change in outcome; this is the specific pre-existing test this bug already violates).
  - `tests/avif_termination.rs`, `tests/metadata.rs::test_read_avif_compatible_brands`,
    `tests/truncate_images.rs` (avif-native feature): 1/1, 1/1, 14/14 passed, unchanged before and
    after.
  - `cargo test --lib` (default features, no avif-native): 331/331 passed, unchanged before and
    after.

## What's not done / open questions

- Root-cause issue (1) above (the actual "CPU/memory cost already spent" resource-exhaustion shape
  the original candidate named) is **not fixed** - only the independently-broken dimension check
  is. A maintainer would need to decide whether to restructure `AvifDecoder::new()` to defer the
  dav1d decode (e.g. parse `mp4parse`'s header/dimension info first, without invoking dav1d, then
  let `set_limits`/`prepare_image` gate the actual decode) - this is the same open question as
  upstream issue #3076, not resolved by this session's patch.
- `max_alloc` is exercised elsewhere (`reshape_plane`'s `limits.reserve_usize`) but wasn't
  independently re-audited this session for AVIF; only `max_image_width`/`max_image_height` were in
  scope here.
- Why `tests/limits.rs::avif` never caught this in CI is now understood (`test_avif_decoding` in
  `.github/workflows/rust.yml` only `cargo build`s with `avif-native`, never `cargo test`s) - not
  fixed here since it's a CI-workflow change, not a source-code bug, and out of this session's
  scope; noted in `IDEAS.md`.
- This environment cannot install `libdav1d`-dev/pkg-config or `meson`/`ninja` (no sudo, no apt
  package for `libdav1d-dev`), so `avif-native` normally fails to build here (as candidate 002 also
  found). Worked around this session with a local, non-system, link-only pkg-config shim pointing
  at the already-installed runtime `libdav1d.so.7` (dav1d-sys ships pre-generated FFI bindings, so
  no header parsing is actually needed to build - only linking) - see `justfile`'s `shim` recipe.
  This shim technique is worth keeping as a documented, reusable trick for any future AVIF-native
  candidate in this environment.
