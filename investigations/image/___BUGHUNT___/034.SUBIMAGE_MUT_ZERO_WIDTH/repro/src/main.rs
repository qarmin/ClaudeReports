// Standalone reproduction: `SubImage::sub_image()` (`src/images/sub_image.rs:161`), the
// *mutable* inherent counterpart of candidate 032/033's `SubImage::view()`, mishandles a
// zero-width `Rect` exactly like its read-only siblings did - reached through a previously
// unexercised entry point (the `GenericImage::sub_image` -> `SubImage::sub_image` mutable path,
// not `GenericImageView::view`/`SubImage::view`).
//
// Driven entirely through the crate's own public API:
//   RgbImage::from_fn -> GenericImage::sub_image (level 1, full-size, mutable)
//     -> SubImage::sub_image (level 2, zero-width rect, mutable) -> .pixels()
//
// Same two observable failure modes as candidates 001/032/033, depending on the zero-width
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

    // Case A: mutable sub-view via GenericImage::sub_image (level 1, full-size), then
    // SubImage::sub_image (level 2, zero-width rect at xoffset 0).
    {
        let mut level1 = backing.sub_image(Rect {
            x: 0,
            y: 0,
            width: 5,
            height: 5,
        });
        let sub_a = level1.sub_image(Rect {
            x: 0,
            y: 0,
            width: 0,
            height: 5,
        });
        let pixels_a: Vec<_> = sub_a.pixels().collect();
        if pixels_a.is_empty() {
            println!(
                "Case A: did not reproduce - 0 pixels yielded, as expected for a zero-width mutable sub_image"
            );
        } else {
            reproduced = true;
            println!(
                "Case A: REPRODUCED - zero-width mutable sub_image yielded {} phantom pixel(s) instead of 0: {:?}",
                pixels_a.len(),
                pixels_a
            );
        }
    }

    // Case B: zero-width rect at xoffset == backing width (5).
    {
        let result_b = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut level1 = backing.sub_image(Rect {
                x: 0,
                y: 0,
                width: 5,
                height: 5,
            });
            let sub_b = level1.sub_image(Rect {
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
