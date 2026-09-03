//! Standalone reproduction for Candidate 023.
//!
//! Drives the real, public `vello::Scene::draw_image` entry point with a real
//! `peniko::ImageData` whose `width`/`height` are legitimate `u32` values >= 65536 - the
//! pipeline's own encoded 16-bit field width. No internal struct is hand-constructed: the only
//! external input here is the u32 width/height a caller legitimately supplies via `ImageData`'s
//! public fields, exactly as a real application would.
//!
//! `DrawImage::width_height` packs `width` into the top 16 bits and `height` into the bottom 16
//! bits of a single u32 (see `vello_encoding/src/encoding.rs`'s `encode_image`, and the
//! corresponding unpack in `vello_shaders/shader/fine.wgsl`'s `read_image`:
//! `width = f32(width_height >> 16u)`, `height = f32(width_height & 0xffffu)`). The pristine
//! encode side never validates that `width`/`height` actually fit in 16 bits before packing them,
//! so any dimension >= 65536 silently corrupts the packed value instead of being rejected or
//! clamped.
//!
//! This also runs the REAL end-to-end `vello_encoding::Resolver::resolve` pipeline (the exact
//! function `vello::Renderer::render_to_texture`/`render_to_surface` calls before ever touching
//! the GPU) to check whether the corruption actually survives into the final GPU-submitted
//! buffer, or gets neutralized first by the atlas allocator's own, unrelated "image doesn't fit,
//! zero it out" safety net (`vello_encoding::image_cache`'s hardcoded `MAX_ATLAS_SIZE = 8192`,
//! always exceeded by any dimension that triggers this 16-bit overflow) - see RESULT.md for why
//! this matters for how to read the "end-to-end resolve()" lines below.

use peniko::{Blob, ImageAlphaType, ImageData, ImageFormat};
use vello::kurbo::Affine;
use vello::Scene;

/// Mirrors the shader's own unpacking logic exactly (`fine.wgsl`'s `read_image`).
fn decode_width_height(width_height: u32) -> (u32, u32) {
    let width = width_height >> 16;
    let height = width_height & 0xffff;
    (width, height)
}

fn encode_and_get_width_height(width: u32, height: u32) -> u32 {
    let mut scene = Scene::new();
    let data = vec![0_u8; 4]; // 1 RGBA8 pixel, deliberately mismatched vs width*height*4
    let image = ImageData {
        data: Blob::from(data),
        format: ImageFormat::Rgba8,
        alpha_type: ImageAlphaType::Alpha,
        width,
        height,
    };
    scene.draw_image(&image, Affine::IDENTITY);

    // Locate the just-encoded DrawImage's second u32 word (width_height) in the
    // real draw_data buffer produced by the real public encoding path.
    // DrawImage is 3 u32 words: [xy, width_height, sample_alpha].
    let draw_data = &scene.encoding().draw_data;
    assert_eq!(draw_data.len(), 3, "expected exactly one encoded DrawImage");
    draw_data[1]
}

fn check(width: u32, height: u32) -> bool {
    // Catch a possible `debug_assert!` panic from the patched tree so one out-of-range case
    // doesn't abort the whole run - each case is independent and should be reported on its own.
    let result = std::panic::catch_unwind(|| encode_and_get_width_height(width, height));
    let packed = match result {
        Ok(packed) => packed,
        Err(_) => {
            println!(
                "width={width:>10} height={height:>10} -> PANICKED (debug_assert caught the out-of-range dimensions)"
            );
            return true; // patched behavior: fails loudly instead of silently corrupting
        }
    };
    let (decoded_width, decoded_height) = decode_width_height(packed);
    let ok = decoded_width == width && decoded_height == height;
    println!(
        "width={width:>10} height={height:>10} -> packed=0x{packed:08x} decoded=({decoded_width}, {decoded_height}) {}",
        if ok { "OK" } else { "CORRUPTED" }
    );
    ok
}

/// Runs the REAL end-to-end resolve pipeline - see the module doc comment above.
fn check_end_to_end(width: u32, height: u32) {
    use vello_encoding::{DrawTag, Resolver};

    let result = std::panic::catch_unwind(|| {
        let mut scene = vello::Scene::new();
        let byte_len = (width as u64)
            .saturating_mul(height as u64)
            .saturating_mul(4)
            .min(64); // keep the fake pixel buffer small regardless of declared dimensions
        let data = vec![0_u8; byte_len as usize];
        let image = ImageData {
            data: Blob::from(data),
            format: ImageFormat::Rgba8,
            alpha_type: ImageAlphaType::Alpha,
            width,
            height,
        };
        scene.draw_image(&image, Affine::IDENTITY);

        let encoding = scene.encoding();
        let mut resolver = Resolver::new();
        let mut packed = Vec::new();
        let (layout, _ramps, _images) = resolver.resolve(encoding, &mut packed);

        // Find the DRAWTAG_IMAGE draw object's draw_data words in the final packed GPU buffer.
        let image_tag_index = encoding
            .draw_tags
            .iter()
            .position(|t| *t == DrawTag::IMAGE)
            .expect("expected one IMAGE draw tag");
        // draw_data_base is in words (u32); each draw object here is a single IMAGE with 3 u32
        // words at draw_data_offset 0 (xy, width_height, sample_alpha).
        let base = layout.draw_data_base as usize * 4;
        let draw_data_u32: &[u32] = bytemuck::cast_slice(&packed[base..]);
        draw_data_u32[image_tag_index * 3 + 1]
    });
    match result {
        Ok(width_height_word) => {
            let (decoded_width, decoded_height) = decode_width_height(width_height_word);
            println!(
                "  end-to-end resolve(): final GPU buffer width_height=0x{width_height_word:08x} decoded=({decoded_width}, {decoded_height})"
            );
        }
        Err(_) => println!("  end-to-end resolve(): PANICKED (debug_assert caught the out-of-range dimensions)"),
    }
}

fn main() {
    // Panics are expected and handled (see `check`/`check_end_to_end` above) once the fix's
    // `debug_assert!` is in place - suppress the default noisy panic hook so the output stays
    // readable.
    std::panic::set_hook(Box::new(|_| {}));

    // Boundary values around the 16-bit field width the packing code assumes.
    let cases = [
        (1_u32, 1_u32),    // trivially fits
        (65535, 1),        // max representable width (u16::MAX)
        (65536, 1),        // one past max: width's own low bit (bit 16) is lost entirely
        (65537, 1),        // width has both a "fits" bit and an "overflow" bit set
        (131072, 1),       // 2 * 65536: encodes identically to width=0 on the pristine tree
        (100_000, 50_000), // a realistic-looking "large but not absurd" image size
        (1, 65536),        // height overflow too - the pristine code masks height with & 0xFFFF,
                            // which is just as much a silent truncation as width's own overflow
    ];

    let mut any_corrupted = false;
    for (w, h) in cases {
        if !check(w, h) {
            any_corrupted = true;
        }
        check_end_to_end(w, h);
    }

    if any_corrupted {
        println!("REPRODUCED - width_height packing silently corrupts for width/height >= 65536");
        std::process::exit(1);
    } else {
        println!("did not reproduce");
    }
}
