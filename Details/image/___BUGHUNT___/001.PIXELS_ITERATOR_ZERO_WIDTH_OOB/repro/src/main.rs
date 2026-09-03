// Standalone reproduction: `GenericImageView::pixels()` panics with an out-of-bounds pixel
// access on a zero-width, non-zero-height image, driven entirely through the crate's own public
// API (`ImageBuffer::new` + `GenericImageView::pixels`).
//
// Exit code convention: 0 = did not reproduce (bug absent), non-zero = reproduced (bug present).

use image::{GenericImageView, ImageBuffer, Rgba};

fn main() {
    // A trivially constructable, non-malicious buffer: 0 columns, 5 rows. No file/codec
    // parsing involved - any caller of the public API can build this by hand.
    let buf = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(0, 5);

    let result = std::panic::catch_unwind(|| {
        // Explicit `GenericImageView::pixels` call: `ImageBuffer` also has an *inherent*
        // `pixels()` method (a plain slice iterator) that would otherwise shadow the trait
        // method and never hit the buggy code path.
        GenericImageView::pixels(&buf).count()
    });

    match result {
        Ok(count) => {
            println!("did not reproduce - iterated {count} pixels without panicking");
        }
        Err(panic_payload) => {
            eprintln!("REPRODUCED - panicked: {:?}", panic_payload);
            std::process::exit(1);
        }
    }
}
