// Reproduction for: Zero-sized Pixmaps are constructible and panic on sampling (#1269).
//
// Uses vello_cpu's public API end-to-end: construct a zero-width Pixmap, use it as an
// `Image` paint source, fill a rect with it, and render. The panic happens inside the real
// rendering/sampling path, not in hand-constructed internal state.

use std::sync::Arc;
use vello_cpu::kurbo::Rect;
use vello_cpu::peniko::Extend;
use vello_cpu::{Image, ImageSource, Pixmap, RenderContext, Resources};

fn main() {
    let result = std::panic::catch_unwind(|| {
        let width = 10;
        let height = 5;
        let mut context = RenderContext::new(width, height);
        let mut resources = Resources::new();

        // A zero-width Pixmap is happily constructed by the public API - nothing rejects it.
        let zero_pixmap = Pixmap::new(0, 3);

        context.set_paint(Image {
            image: ImageSource::Pixmap(Arc::new(zero_pixmap)),
            sampler: vello_cpu::peniko::ImageSampler {
                x_extend: Extend::Pad,
                y_extend: Extend::Pad,
                ..Default::default()
            },
        });
        context.fill_rect(&Rect::from_points((0., 0.), (width as f64, height as f64)));

        let mut target = Pixmap::new(width, height);
        context.flush();
        context.render(&mut target, &mut resources);
        target
    });

    match result {
        Ok(_) => {
            println!("did not reproduce - rendering with a zero-width Pixmap image completed normally");
        }
        Err(panic_payload) => {
            eprintln!("REPRODUCED - panicked: {:?}", panic_payload);
            std::process::exit(1);
        }
    }
}
