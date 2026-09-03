// Standalone reproduction: `SubImage::pixels()` (reached through `GenericImageView::view` /
// `SubImageInner`'s own `GenericImageView` impl, deref-coerced from `SubImage`) mishandles a
// zero-width `Rect` view into a non-zero-width backing image, driven entirely through the
// crate's own public API (`ImageBuffer::from_fn` + `GenericImageView::view` + `.pixels()`).
//
// This is the same root cause as candidate 001 (`Pixels::next()`'s row-wrap logic assumes every
// row has at least one column) but reached through a different, previously-unexercised public
// entry point (`SubImage`/`SubImageInner`, not a plain `ImageBuffer`), and it has two distinct
// observable failure modes depending on the view's x-offset relative to the backing image:
//
//   Case A (xoffset = 0): no panic at all - `get_pixel(0, y)` stays in-bounds of the *backing*
//   image for every row, so the iterator silently yields "phantom" pixels from column 0 of the
//   backing image instead of the correct answer of zero pixels for a zero-width view. This is a
//   silent wrong-data bug, not a crash.
//
//   Case B (xoffset = backing width): panics, because `get_pixel(xoffset, y)` on the backing
//   image is now out of bounds once the row-wrap logic increments y past what a zero-width row
//   was ever supposed to allow.
//
// Exit code convention: 0 = did not reproduce (bug absent in both cases), non-zero = reproduced.

use image::math::Rect;
use image::{GenericImageView, ImageBuffer, Rgba};

fn main() {
    let backing: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::from_fn(5, 5, |x, y| Rgba([x as u8, y as u8, 0, 255]));

    let mut reproduced = false;

    // Case A: zero-width view at xoffset 0 into a 5x5 backing image.
    let rect_a = Rect {
        x: 0,
        y: 0,
        width: 0,
        height: 5,
    };
    let view_a = backing.view(rect_a);
    let pixels_a: Vec<_> = view_a.pixels().collect();
    if pixels_a.is_empty() {
        println!("Case A: did not reproduce - 0 pixels yielded, as expected for a zero-width view");
    } else {
        reproduced = true;
        println!(
            "Case A: REPRODUCED - zero-width view yielded {} phantom pixel(s) instead of 0: {:?}",
            pixels_a.len(),
            pixels_a
        );
    }

    // Case B: zero-width view at xoffset == backing width (5) into the same 5x5 backing image.
    let rect_b = Rect {
        x: 5,
        y: 0,
        width: 0,
        height: 5,
    };
    let view_b = backing.view(rect_b);
    let result_b = std::panic::catch_unwind(|| view_b.pixels().count());
    match result_b {
        Ok(count) => {
            println!("Case B: did not reproduce - iterated {count} pixels without panicking");
        }
        Err(panic_payload) => {
            reproduced = true;
            eprintln!("Case B: REPRODUCED - panicked: {:?}", panic_payload);
        }
    }

    if reproduced {
        std::process::exit(1);
    }
}
