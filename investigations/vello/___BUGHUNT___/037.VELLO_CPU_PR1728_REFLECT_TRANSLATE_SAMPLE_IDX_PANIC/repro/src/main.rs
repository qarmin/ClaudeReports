// Standalone reproduction: rendering a small image with `Extend::Reflect` and a translating
// affine transform panics with an out-of-bounds `Pixmap` index, on AVX2, once open PR
// linebender/vello#1728 ("vello_cpu: Avoid continuously splatting positions") is applied on top
// of this repro's `clone/`. Does not reproduce against pristine `clone/` (i.e. this is a bug in
// the PR's own patch, not present in current upstream `HEAD`) - see RESULT.md.
//
// This mirrors `sparse_strips/vello_sparse_tests/tests/image.rs`'s
// `image_bilinear_with_translation` test (`quality()` helper + `rgb_img_2x2()`), using vello_cpu's
// public API directly instead of the internal test harness, driving a real end-to-end render.

use std::io::Cursor;
use std::sync::Arc;

use vello_cpu::kurbo::{Affine, Rect};
use vello_cpu::peniko::{Extend, ImageQuality, ImageSampler};
use vello_cpu::{Image, ImageSource, Pixmap, RenderContext, Resources};

fn main() {
    let png_bytes = include_bytes!("../rgb_image_2x2.png");
    let pixmap = Pixmap::from_png(Cursor::new(png_bytes.as_slice())).unwrap();
    let image_source = ImageSource::Pixmap(Arc::new(pixmap));

    let mut ctx = RenderContext::new(100, 100);
    let mut resources = Resources::new();

    // Same transform/quality/extend combination as the failing
    // `image_bilinear_with_translation_cpu_u8_avx2` snapshot test.
    ctx.set_paint_transform(Affine::scale(5.0) * Affine::translate((10.0, 10.0)));
    ctx.set_paint(Image {
        image: image_source,
        sampler: ImageSampler {
            x_extend: Extend::Reflect,
            y_extend: Extend::Reflect,
            quality: ImageQuality::Medium,
            alpha: 1.0,
        },
    });
    ctx.fill_rect(&Rect::new(10.0, 10.0, 90.0, 90.0));
    ctx.flush();

    let mut out = Pixmap::new(100, 100);
    // Panics here (with PR #1728 applied): "index out of bounds: the len is 4 but the index is
    // 4294967293/4294967295" in `Pixmap::sample_idx` (`vello_common/src/pixmap.rs`).
    ctx.render(&mut out, &mut resources);

    println!("no crash - rendered {}x{} pixmap OK", out.width(), out.height());
}
