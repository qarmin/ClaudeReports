# AVIF_10BIT_UNALIGNED_BUFFER_ALLOC_UNACCOUNTED

Follow-up audit of `src/codecs/avif/decoder.rs`'s `max_alloc` (`Limits::reserve_usize`)
enforcement, from `IDEAS.md`'s `AVIF_DECODER_RESHAPE_PLANE_MAX_ALLOC_AUDIT` row: candidate 011 only
fixed the missing `max_image_width`/`max_image_height` check in `set_limits`; this candidate
cross-checks every plane/alpha allocation path in the file against `self.limits` for the separate
`max_alloc` mechanism.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: every allocation site in the file (`reshape_plane`, `transmute_y_plane16`, `transmute_chroma_plane16`, both branches of `read_image`'s 8-bit and 10/12-bit paths, `process_16bit_picture`) cross-checked against every `limits.reserve_usize`/`reserve` call | Static read of current `main` (`44ce9226`) | `reshape_plane` (used by both `transmute_y_plane16`/`transmute_chroma_plane16`) correctly calls `limits.reserve_usize(2 * count)` before allocating. One allocation was found with no matching reserve call at all: `read_image`'s 10/12-bit branch (~line 583), the `aligned_store` fallback taken when the caller-supplied output `buf` fails `bytemuck::try_cast_slice_mut` (i.e. is not 2-byte aligned) - `vec![0u16; buf.len() / 2]`, sized identically to the whole output image, allocated unconditionally with no call to `self.limits.reserve`/`reserve_usize` anywhere before or after. |
| Manual boundary-value construction + real end-to-end run against the crate's actual public API (`avif-native` feature, built via the local link-only dav1d pkg-config shim documented in `README.md`), using a real 10-bit AVIF produced by `ffmpeg`'s `libaom-av1` encoder (this crate's own `AvifEncoder` cannot produce anything but 8-bit AVIF, so an external real encoder was used to get a genuine 10-bit bitstream) | `cargo run --release` against a standalone `repro/`, decoder configured via the real, documented `AvifDecoder::set_limits(Limits{max_alloc: Some(0), ..})`, `read_image()` called with a caller-owned output buffer deliberately misaligned to 2 bytes (a legitimate value under the public API's contract - nothing requires an even-address buffer) | Confirms the gap precisely: `read_image()` succeeds and writes a full valid image despite `max_alloc: Some(0)` - the `aligned_store` allocation (9600 bytes for this 40x30 `Rgba16` test case) happens with zero enforcement. 3/3 identical runs (fresh clone, pristine `main`). |
| Same probe, but with a naturally 2-byte-aligned output buffer (no `aligned_store` needed) plus temporary `eprintln!` instrumentation (added and reverted within the session) in `transmute_y_plane16`/`transmute_chroma_plane16` to observe which branch actually runs against a real dav1d-decoded picture | Same build/input | Also succeeds despite `max_alloc: Some(0)`, but for an *expected* reason, not a bug: dav1d always returned even, properly aligned plane strides (256/128 bytes for this test image) for every Y/U/V plane, so `bytemuck::try_cast_slice` always took the zero-copy `Cow::Borrowed` branch - no allocation happens there at all, so `reshape_plane`'s `reserve_usize` (which *is* correctly gated) is simply never reached for real dav1d output. This is not itself a defect (nothing needs to be reserved when nothing is allocated); it does mean `reshape_plane`'s own accounting is effectively unreachable in practice and the file's *only* practically-reachable `max_alloc` gap for the 10/12-bit path is the `aligned_store` one above. |

## Status

**Confirmed.** Reproduces 3/3 runs, deterministic (no randomness/seed involved), both in-tree
against the main project checkout and via the standalone `repro/` run through `just test` against
a fresh clone of `base_commit`.

**Fully reproducible: Yes.** `repro/` drives the real, public `image::codecs::avif::AvifDecoder`
API (`set_limits` + `read_image`) end-to-end against a real AVIF file (encoded by a real external
10-bit AV1 encoder, decoded by the crate's own real dav1d backend). This is a single-participant
finding (a caller configuring `Limits` on the decoder it itself owns, then calling that same
decoder's own public method) - CONFIRMATION.md section 11's "real second participant" requirement
doesn't add anything beyond "run the real public API end-to-end", which this repro does.

## Class

security (resource-limit bypass: `max_alloc` silently not enforced for one reachable allocation)

## Root cause

`src/codecs/avif/decoder.rs`'s `ImageDecoder::read_image`, in the 10/12-bit-depth branch (the
`else` arm after `if bit_depth == 8`), handles a caller-supplied output buffer that isn't 2-byte
aligned by allocating a same-sized scratch `Vec<u16>` (`aligned_store`) to decode into instead,
then copying the result back into the caller's buffer byte-by-byte. This allocation is exactly the
same kind of "reshape FFI/output data into a Rust-constrained buffer" operation that
`reshape_plane` exists to guard via `limits.reserve_usize` - but it was never routed through
`self.limits` at all, unlike every other allocation added in this file for the same general
purpose. Since it happens unconditionally and before `process_16bit_picture`'s own (correctly
guarded) plane-reshaping calls, a caller who sets a tight `max_alloc` specifically to bound
decoder-owned allocations gets no protection at all against this one whenever their own output
buffer happens to be misaligned - the decoder just duplicates the full output buffer's worth of
memory for free.

Whether a real caller's buffer is ever actually misaligned in practice depends on how it obtained
that buffer: `ImageReader::decode()`'s own internal `Vec<u8>` allocation is effectively always
2-byte aligned on this platform's allocator, so this path is not reachable through the crate's own
"normal" decode entry point - it is reachable through the public `ImageDecoder::read_image` /
`decode_into`-style APIs, where the caller supplies their own buffer (e.g. a view into a larger
arena or a manually offset slice), which is a legitimate, unrestricted use of the public contract.

## Introduced by

3432fb35 "Defensive programming for avif decoder limits" (2026-05-21) - this commit added `self.limits`/`reserve_usize` guards to every other allocation in the file (e.g. `reshape_plane`) but did not add one for the pre-existing `aligned_store` allocation (originally introduced by fd73671c "Refactoring and readability improvements", 2024-11-01, before any limits accounting existed in this file).

## The fix

Added the missing `self.limits.reserve_usize(buf.len())?` call immediately before the
`aligned_store` allocation, mirroring `reshape_plane`'s own pattern (reserve first, then allocate):

```rust
} else {
    // If buffer from Decoder is unaligned. This allocation duplicates the
    // caller-provided output buffer, so it must be accounted for against
    // `self.limits` just like `reshape_plane`'s own allocations are, or a
    // misaligned output buffer silently bypasses `max_alloc` entirely.
    self.limits.reserve_usize(buf.len())?;
    let mut aligned_store = vec![0u16; buf.len() / 2];
    ...
```

Verified against the same repro (misaligned buffer, `max_alloc: Some(0)`): `read_image()` now
correctly returns `Err(Memory limit exceeded)` instead of succeeding, 3/3 runs. Also verified the
naturally-aligned-buffer case (no `aligned_store` needed) is unaffected - it still succeeds even
with `max_alloc: Some(0)`, which is correct: no decoder-owned allocation happens on that path (the
output is written directly into the caller's own already-allocated buffer), so there is nothing to
reserve.

- Project test suite (`avif-native` feature, via the dav1d pkg-config shim):
  - `tests/limits.rs::avif`: failing before -> failing after, **unrelated to this patch** - this
    is candidate 011's already-documented, separately-fixed `max_image_width`/`max_image_height`
    gap in `set_limits` (011's own fix was not applied in this session's tree); confirmed identical
    failure with and without this candidate's patch applied.
  - `tests/avif_termination.rs`, `tests/metadata.rs::test_read_avif_compatible_brands`,
    `tests/truncate_images.rs`: 1/1, 1/1, 14/14 passed, unchanged before and after.
  - `cargo test --lib` (default features, no `avif-native`): 331/331 passed, unchanged before and
    after.

## What's not done / open questions

- `reshape_plane`'s own `limits.reserve_usize` calls (for the Y/U/V/alpha plane reshapes) were
  found to be correctly guarded in principle, but observed to be **practically unreachable**
  through real dav1d output on this platform (dav1d always returns even/aligned plane strides, so
  the zero-copy `bytemuck` cast always succeeds and no reshape allocation ever happens). This isn't
  a bug by itself, but it does mean this file's `max_alloc` accounting for the 10/12-bit path
  currently amounts to "the one `aligned_store` check added by this patch" for all practically
  reachable inputs - worth noting for anyone auditing this area again in the future.
  Also worth noting since it is the primary discussion point remaining regarding candidate 011's
  own root-cause issue (1) (still open, not addressed here): all of the plane/alpha data these
  functions read from is dav1d's own already-allocated `Picture` memory, produced by the full
  unconditional decode in `AvifDecoder::new()` before any `Limits` exist at all - none of this
  candidate's fix, nor `reshape_plane`'s own accounting, touches that memory at all; both are
  strictly about `image`-crate-owned Rust allocations made *after* construction.
- The 8-bit `read_image` path (no `reshape_plane`/`transmute_*` calls at all - it writes directly
  into planes/`buf` with no intermediate allocation) was read and has nothing to guard; not a gap.
- Same class of "duplicate-the-output-buffer-for-alignment" fallback was checked only in this file;
  not swept across other decoders in this session (out of scope for this candidate, which was
  specifically about `AvifDecoder`'s own `reshape_plane`/`max_alloc` mechanism per the `IDEAS.md`
  row).
