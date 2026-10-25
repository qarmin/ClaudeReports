# set_pixel_format accepts a wrong-length extra_channel_format and the decoder panics later

`JxlDecoder::set_pixel_format` returns `Result<()>` (and has a TODO about missing validation) but does not check that `JxlPixelFormat::extra_channel_format` has one entry per extra channel of the image. The mismatch is only noticed by an `assert_eq!` when the first frame header is processed. Found while probing API misuse in `harnesses/diff_formats` (`misuse` subcommand) for candidate 001's neighbours.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Standalone repro through the public API | `repro/`, 3x3a_srgb_lossless.jxl (1 alpha extra channel), `JxlPixelFormat::rgb8(0)` | `set_pixel_format` returns Ok, next `process` panics at `jxl/src/frame/render.rs:699` (assertion `left == right`, left 0, right 1), 5/5 runs |
| Sweep of colour type x extra-channel count (0..3) x per-channel format x colour format on 5 files | `harnesses/diff_formats misuse`, release | every count != the image's number of extra channels panics (with the right count the API returns proper `NotGrayscale` / `NotCmyk` errors); with the patch all wrong counts return `ApiUsageError` |

## Status

**Confirmed** - 5/5 runs, release build, deterministic.

**Fully reproducible**: **Yes** - public API on a real 66 byte file; the "second participant" is just the caller's own `JxlPixelFormat`.

## Class

crash (panic reachable from safe API misuse; `set_pixel_format` promises to report errors)

## Root cause

`set_pixel_format` stores the format after checking only that BasicInfo exists and that the format is not changed after the first frame header. The length of `extra_channel_format` is first compared with `frame_header.num_extra_channels` inside `Frame::build_render_pipeline` (`assert_eq!`, `jxl/src/frame/render.rs:699`), i.e. in `process()` instead of in the setter.

## Introduced by

7a62f9d "Use JxlOutputBuffer in SaveStage (#378)" (2025-09-09) - the earliest commit containing the `extra_channel_format.len()` assertion (moved to render.rs by 2d70385); `set_pixel_format` has never validated the length.

## The fix

`set_pixel_format` returns `Error::ApiUsageError("extra_channel_format must have one entry per extra channel")` when the length differs from `image_metadata.extra_channel_info.len()` (12 added lines, `patch.diff`). `repro/` then prints the error instead of panicking; the sweep above shows no wrong-length panic left for lengths 0 to 3 on images with 0 or 1 extra channels.

- Project test suite: `cargo test --release -p jxl` 1561 passed / 0 failed before (measured for 001 on the same commit) -> 1561 / 0 after; `-p jxl-image-rs-integration` 4 / 0 -> 4 / 0; `-p jxl_cli` 2 / 0 -> 2 / 0.

## What's not done / open questions

- Whether an error from the setter or a more lenient behaviour (pad with `None`) is preferred is the maintainers' call; `jxl_cli` and the image integration already always pass a correct length.
- The sweep covered images with 0 or 1 extra channels (extra_channels.jxl, 3x3a, gray_alpha), not ones with several.
