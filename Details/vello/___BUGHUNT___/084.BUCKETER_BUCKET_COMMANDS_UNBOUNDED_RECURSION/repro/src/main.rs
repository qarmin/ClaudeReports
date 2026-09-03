// Standalone reproduction for candidate 084 (BUCKETER_BUCKET_COMMANDS_UNBOUNDED_RECURSION).
//
// Uses only `vello_cpu`'s real public API (`RenderContext::push_opacity_layer`/`pop_layer`/
// `render`) to build a scene with `depth` levels of nested layers - exactly the shape a real
// SVG-to-Vello frontend would produce for a deeply nested group tree (e.g. nested <g> elements
// each carrying opacity/clip/blend), then renders it. This is a single external artifact (the
// scene's own nesting structure) handed to the library's real public entry point - there is no
// second participant to fake here (see CONFIRMATION.md section 11).
//
// Exit code convention: 0 = did not reproduce, non-zero/abort = reproduced (stack overflow).

use vello_cpu::color::palette::css::BLUE;
use vello_cpu::kurbo::Rect;
use vello_cpu::{Pixmap, RenderContext, Resources};

fn main() {
    let depth: usize = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(100_000);

    let width = 64u16;
    let height = 64u16;
    let mut ctx = RenderContext::new(width, height);
    let mut resources = Resources::new();

    ctx.set_paint(BLUE);

    // Simulate a scene with `depth` levels of nested layers/groups - e.g. deeply nested SVG
    // <g> elements each with opacity/clip/blend, translated 1:1 into nested push_layer/pop_layer
    // calls the way a real SVG-to-Vello frontend would when walking a deeply nested group tree.
    for _ in 0..depth {
        ctx.push_opacity_layer(0.999);
    }

    ctx.fill_rect(&Rect::new(0.0, 0.0, 10.0, 10.0));

    for _ in 0..depth {
        ctx.pop_layer();
    }

    ctx.flush();

    let mut pixmap = Pixmap::new(width, height);
    // `render` triggers coarse rasterization, which walks the recorded node tree via
    // `CommandBucketer::bucket_commands` - recursively, on the unfixed tree, once per nesting
    // level of `depth` above.
    ctx.render(&mut pixmap, &mut resources);

    println!("did not reproduce - rendered successfully with depth={depth}");
}
