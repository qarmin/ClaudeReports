// Standalone reproduction for vello candidate 003: drawing an Image with
// ImageSampler.alpha != 1.0 panics inside vello_common's Image::encode_into
// (unimplemented!("Applying opacity to image commands")).
//
// This drives the real, public vello_cpu API end-to-end: it registers an
// image, sets it as the current paint with a less-than-full-opacity sampler,
// fills a rect with it, and calls RenderContext::render (which is what any
// real caller of vello_cpu does to rasterize a scene). No internal/private
// API is called directly - `encode_into` is reached only via the normal
// public rendering path.

use std::sync::Arc;

use vello_cpu::kurbo::Rect;
use vello_cpu::peniko::{Extend, ImageQuality, ImageSampler};
use vello_cpu::{Image, ImageSource, Pixmap, RenderContext, Resources};

fn main() {
    let mut ctx = RenderContext::new(64, 64);
    let mut resources = Resources::new();

    // A tiny 2x2 source image to draw.
    let source_pixmap = Arc::new(Pixmap::new(2, 2));
    let image_id = resources.register_image(source_pixmap);

    ctx.set_paint(Image {
        image: ImageSource::OpaqueId {
            id: image_id,
            may_have_transparency: false,
        },
        sampler: ImageSampler {
            x_extend: Extend::Pad,
            y_extend: Extend::Pad,
            quality: ImageQuality::Low,
            // This is the trigger: any alpha other than 1.0 hits the
            // `unimplemented!()` guard in `Image::encode_into`.
            alpha: 0.5,
        },
    });
    ctx.fill_rect(&Rect::new(0.0, 0.0, 64.0, 64.0));

    let mut pixmap = Pixmap::new(64, 64);
    println!("about to render an image paint with sampler.alpha = 0.5 ...");
    ctx.render(&mut pixmap, &mut resources);
    println!("rendered successfully (bug is fixed)");
}
