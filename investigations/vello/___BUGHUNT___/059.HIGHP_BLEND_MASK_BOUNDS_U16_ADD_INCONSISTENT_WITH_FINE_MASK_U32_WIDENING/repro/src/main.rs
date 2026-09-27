// Candidate 059: real end-to-end check of whether a genuine caller of vello_cpu's public API can
// ever drive `HighPCompositeExt::blend`'s mask-bounds check (`start_y + 3 < height`, raw u16 add,
// `sparse_strips/vello_cpu/src/fine/highp/mod.rs:211`) to a `start_y` large enough to overflow.
//
// Exit code convention: 0 = did not reproduce (no panic - matches this session's finding that the
// real pipeline never supplies an out-of-invariant `start_y`), non-zero/panic = reproduced.
//
// `height` is set to 65532, the largest canvas height that is both representable in u16 and an
// exact multiple of `Tile::HEIGHT` (4) - i.e. the real maximum value `row_y` can ever take through
// the public API (see RESULT.md for why height=65535 itself hits a different, pre-existing,
// unrelated panic in `RectExt::snap_to_tile_coordinates` before ever reaching this code).
use vello_cpu::color::palette::css::{RED, WHITE};
use vello_cpu::kurbo::Rect;
use vello_cpu::{Mask, Pixmap, RenderContext, Resources};

fn main() {
    let width: u16 = 4;
    let height: u16 = 65532; // largest u16 multiple of Tile::HEIGHT=4

    // Build a full-canvas mask (must match main context's exact dimensions per push_layer/set_mask docs).
    let mask = {
        let mut mask_ctx = RenderContext::new(width, height);
        let mut mask_resources = Resources::new();
        let mut pixmap = Pixmap::new(width, height);
        mask_ctx.set_paint(RED);
        mask_ctx.fill_rect(&Rect::new(0.0, 0.0, width as f64, height as f64));
        mask_ctx.flush();
        mask_ctx.render(&mut pixmap, &mut mask_resources);
        Mask::new_luminance(&pixmap)
    };

    let mut ctx = RenderContext::new(width, height);
    ctx.set_mask(mask);
    ctx.set_paint(WHITE);
    // Fill the very bottom-most rows of the canvas, where row_y (a multiple of Tile::HEIGHT=4)
    // is at its maximum possible value before the mask-bounds check computes start_y + 3.
    ctx.fill_rect(&Rect::new(0.0, (height - 4) as f64, width as f64, height as f64));
    ctx.flush();

    let mut out = Pixmap::new(width, height);
    let mut resources = Resources::new();

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        ctx.render(&mut out, &mut resources);
    }));

    match result {
        Ok(()) => println!(
            "did not reproduce - real end-to-end render completed normally at the maximum tile-aligned canvas height (65532)"
        ),
        Err(e) => {
            eprintln!("REPRODUCED via real public API - panicked: {:?}", e);
            std::process::exit(1);
        }
    }
}
