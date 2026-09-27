// Real end-to-end repro: an actual PNG file (the kind an attacker/user could supply as an
// embedded image, e.g. in an SVG or a standalone image asset) is decoded with the real `image`
// crate decode path (the same pattern vello's own examples/scenes/src/images.rs uses), producing
// a genuine `peniko::ImageData` whose width/height come straight from the decoded raster. That
// value is then passed into vello_common's real public API, `ImageSource::from_peniko_image_data`.
//
// No internal state is hand-constructed here: the PNG bytes are the sole external artifact, and
// everything downstream (decode -> ImageData -> from_peniko_image_data) is the real production
// code path a real application would run.
//
// This same source is used for both "reproduce-before" (default, no features) and
// "reproduce-after" (`--features fixed`) - the patch changes
// `from_peniko_image_data`'s return type from `ImageSource` to `Option<ImageSource>`, so the two
// runs call it slightly differently (see the `#[cfg(feature = "fixed")]` branch below), but both
// exercise the identical decoded, oversized `ImageData`.

use std::io::Cursor;
use std::sync::Arc;

use peniko::{Blob, ImageAlphaType, ImageData, ImageFormat};
use vello_common::paint::ImageSource;

/// Build a real, valid PNG file (as bytes) whose width exceeds u16::MAX. This models an
/// attacker/user-supplied image asset (an unusually wide but very short raster, e.g. a 1px-tall
/// "barcode"-style image) - decoding it is cheap, but the width alone already exceeds u16::MAX.
fn build_oversized_png() -> Vec<u8> {
    let width: u32 = 70_000;
    let height: u32 = 1;
    let mut png_bytes = Vec::new();
    {
        let mut encoder = png::Encoder::new(&mut png_bytes, width, height);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();
        let pixel_data = vec![0xAAu8; (width * height * 4) as usize];
        writer.write_image_data(&pixel_data).unwrap();
    }
    png_bytes
}

/// Same decode logic as `examples/scenes/src/images.rs::decode_image` in this repo: the real
/// `image` crate decode entry point that any vello-based app loading an image file would call.
fn decode_image(data: &[u8]) -> ImageData {
    let image = image::ImageReader::new(Cursor::new(data))
        .with_guessed_format()
        .expect("guess format")
        .decode()
        .expect("decode image");
    let width = image.width();
    let height = image.height();
    let data = Arc::new(image.into_rgba8().into_vec());
    let blob = Blob::new(data);
    ImageData {
        data: blob,
        format: ImageFormat::Rgba8,
        width,
        height,
        alpha_type: ImageAlphaType::Alpha,
    }
}

fn main() {
    let png_bytes = build_oversized_png();
    println!("Built a real PNG file, {} bytes on disk/wire.", png_bytes.len());

    let image_data = decode_image(&png_bytes);
    println!(
        "Decoded real PNG via `image` crate: width={}, height={} (u16::MAX = {})",
        image_data.width,
        image_data.height,
        u16::MAX
    );

    println!("Calling vello_common's public ImageSource::from_peniko_image_data(...)...");

    #[cfg(not(feature = "fixed"))]
    {
        // Pre-fix API: `-> ImageSource`. This is expected to panic (the bug).
        let _source = ImageSource::from_peniko_image_data(&image_data);
        println!("No panic occurred (unexpected - bug is NOT present).");
    }

    #[cfg(feature = "fixed")]
    {
        // Post-fix API: `-> Option<ImageSource>`. This is expected to return `None` instead of
        // panicking.
        match ImageSource::from_peniko_image_data(&image_data) {
            None => println!(
                "Correctly rejected the oversized image: from_peniko_image_data returned None \
                 instead of panicking."
            ),
            Some(_) => println!("Unexpectedly got Some(..) for an oversized image (unexpected)."),
        }
    }
}
