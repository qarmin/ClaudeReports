# jxl-image-rs-integration adapter

The adapter converts u32 sizes to usize, indexes buffers with `buf.len() / height` and has an unaligned-buffer fallback; panics there would hit every `image` crate user.

Base commit 7f539c8 (jxl-rs main); the main tree was not modified. Harness sources are in `../harnesses/`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Misaligned output buffer: decode 3x3_srgb_lossless.jxl into a `Vec<u8>` slice starting at offset 0,1,2,3 for U16/F16/F32 | `harnesses/diff_formats` subcommand `mis`, release | identical bytes at every offset, no panic (the `debug_assert!` in `ImageDataType::cast_slice_mut` is not a problem in release) |
| Read all test files + 38 conformance files + all fuzz corpora (14,000+ inputs) through `JxlDecoder::new` -> `read_image` with catch_unwind | `harnesses/img_int`, images over 2^24 pixels skipped | 712 ok, 14937 errors, 0 panics |

## Status

**Not reproducible**

**Fully reproducible**: n/a (no finding).

## Root cause

No panics; error paths return `ImageError`.

## What's not done / open questions

- The adapter was not fuzzed with coverage guidance, only replayed.
