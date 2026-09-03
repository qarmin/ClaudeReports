// Standalone reproduction: `SubImage`'s own *inherent* `view()` method
// (`src/images/sub_image.rs:140`, for constructing a sub-view of an existing `SubImage`, i.e.
// nesting a view inside a view) mishandles a zero-width `Rect`, exactly like candidate 032's
// `GenericImageView::view()` (trait method, on a plain `ImageBuffer`) did - reached through a
// different, previously-unexercised public entry point (nested `SubImage::view`, not a
// single-level trait `view()` call).
//
// Driven entirely through the crate's own public API:
//   ImageBuffer::from_fn -> GenericImageView::view (level 1, full-size) -> SubImage::view
//   (level 2, INHERENT method, zero-width rect) -> .pixels()
//
// Same two observable failure modes as candidate 032, depending on the nested view's x-offset
// relative to the backing image:
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
use image::{GenericImageView, ImageBuffer, Rgba};

fn main() {
    let backing: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::from_fn(5, 5, |x, y| Rgba([x as u8, y as u8, 0, 255]));

    let mut reproduced = false;

    // Level 1: a normal, full-size SubImage via the trait method GenericImageView::view().
    let level1 = backing.view(Rect {
        x: 0,
        y: 0,
        width: 5,
        height: 5,
    });

    // Case A: level 2, nested view via SubImage's own INHERENT view() method, zero-width rect
    // at xoffset 0 relative to the backing image.
    let rect_a = Rect {
        x: 0,
        y: 0,
        width: 0,
        height: 5,
    };
    let view_a = level1.view(rect_a); // SubImage::view (inherent), not GenericImageView::view
    let pixels_a: Vec<_> = view_a.pixels().collect();
    if pixels_a.is_empty() {
        println!("Case A: did not reproduce - 0 pixels yielded, as expected for a zero-width nested view");
    } else {
        reproduced = true;
        println!(
            "Case A: REPRODUCED - zero-width nested view yielded {} phantom pixel(s) instead of 0: {:?}",
            pixels_a.len(),
            pixels_a
        );
    }

    // Case B: level 2, nested view, zero-width rect at xoffset == backing width (5).
    let rect_b = Rect {
        x: 5,
        y: 0,
        width: 0,
        height: 5,
    };
    let view_b = level1.view(rect_b);
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
