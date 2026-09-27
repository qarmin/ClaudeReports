// Standalone reproduction: `GenericImage::sub_image()` (`src/images/generic_image.rs:328`),
// the trait-default method used directly on a plain `ImageBuffer` (not `SubImage`'s own inherent
// `sub_image()`/`view()` methods candidates 032/033/034 already exercised), mishandles a
// zero-width `Rect` exactly like its siblings did - reached through the last previously
// unexercised entry point of this defect class.
//
// Driven entirely through the crate's own public API:
//   RgbImage::from_fn -> GenericImage::sub_image (single level, zero-width rect, direct on the
//   ImageBuffer, no SubImage nesting) -> .pixels()
//
// Same two observable failure modes as candidates 001/032/033/034, depending on the zero-width
// view's x-offset relative to the backing image:
//
//   Case A (xoffset = 0): no panic - `get_pixel(0, y)` stays in-bounds of the *backing* image
//   for every row, so the iterator silently yields "phantom" pixels from column 0 of the backing
//   image instead of the correct answer of zero pixels for a zero-width view.
//
//   Case B (xoffset = backing width): panics, because `get_pixel(xoffset, y)` on the backing
//   image is out of bounds once the row-wrap logic increments y past what a zero-width row was
//   ever supposed to allow.
//
// Exit code convention: 0 = did not reproduce (bug absent in both cases), non-zero = reproduced.

use image::math::Rect;
use image::{GenericImage, GenericImageView, ImageBuffer, Rgb};

fn main() {
    let mut backing: ImageBuffer<Rgb<u8>, Vec<u8>> =
        ImageBuffer::from_fn(5, 5, |x, y| Rgb([x as u8, y as u8, 0]));

    let mut reproduced = false;

    // Case A: GenericImage::sub_image() called directly on a plain ImageBuffer with a
    // zero-width Rect at xoffset 0. Single level, no SubImage nesting.
    {
        let sub_a = backing.sub_image(Rect {
            x: 0,
            y: 0,
            width: 0,
            height: 5,
        });
        let pixels_a: Vec<_> = sub_a.pixels().collect();
        if pixels_a.is_empty() {
            println!(
                "Case A: did not reproduce - 0 pixels yielded, as expected for a zero-width sub_image"
            );
        } else {
            reproduced = true;
            println!(
                "Case A: REPRODUCED - zero-width sub_image yielded {} phantom pixel(s) instead of 0: {:?}",
                pixels_a.len(),
                pixels_a
            );
        }
    }

    // Case B: zero-width rect at xoffset == backing width (5).
    {
        let result_b = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let sub_b = backing.sub_image(Rect {
                x: 5,
                y: 0,
                width: 0,
                height: 5,
            });
            sub_b.pixels().count()
        }));
        match result_b {
            Ok(count) => {
                println!("Case B: did not reproduce - iterated {count} pixels without panicking");
            }
            Err(panic_payload) => {
                reproduced = true;
                eprintln!("Case B: REPRODUCED - panicked: {:?}", panic_payload);
            }
        }
    }

    if reproduced {
        std::process::exit(1);
    }
}
