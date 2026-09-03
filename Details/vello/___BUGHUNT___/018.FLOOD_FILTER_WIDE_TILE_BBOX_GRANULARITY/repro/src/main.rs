// Reproduction for issue #1423: the flood filter (`<feFlood>`) is supposed to only flood the
// filter region of the filtered content (conceptually the tight bounding box of the filtered
// layer, expanded by any filter padding), not the entire renderer-internal bucket/tile row the
// content happens to fall into.
//
// This drives the real, public `vello_cpu::RenderContext` API end-to-end: push a flood filter
// layer, fill a tiny 3x3 rectangle inside it, pop the layer, and rasterize to a real pixmap - then
// checks whether pixels far away from the 3x3 rectangle (but still inside the render target) were
// also flooded, which they should not be.

use vello_common::color::palette::css::{REBECCA_PURPLE, TOMATO};
use vello_common::filter_effects::{Filter, FilterPrimitive};
use vello_common::kurbo::Rect;
use vello_cpu::{Pixmap, RasterizerSettings, RenderContext, Resources};

fn main() {
    let width: u16 = 512;
    let height: u16 = 64;

    let mut ctx = RenderContext::new(width, height);
    let mut resources = Resources::new();

    let filter_flood = Filter::from_primitive(FilterPrimitive::Flood { color: TOMATO });

    // A tiny 3x3 rectangle, positioned away from x=0 so that "flooded because it's in the same
    // tile-row as x=0" isn't a confound - and far from the right edge too.
    let rect = Rect::new(250.0, 30.0, 253.0, 33.0);

    ctx.push_filter_layer(filter_flood);
    ctx.set_paint(REBECCA_PURPLE);
    ctx.fill_rect(&rect);
    ctx.pop_layer();

    let mut pixmap = Pixmap::new(width, height);
    ctx.render_with(&mut pixmap, &mut resources, RasterizerSettings::default());

    // Sample a pixel that is clearly inside the 3x3 rect (should be flooded / non-transparent).
    let inside = pixmap.sample(251, 31);
    // Sample pixels far away, at the very left and very right edges of the render target, well
    // outside the 3x3 rect and outside any reasonable filter padding for a plain flood filter
    // (which has zero filter_expansion).
    let far_left = pixmap.sample(2, 31);
    let far_right = pixmap.sample(509, 31);

    println!("inside 3x3 rect (251,31): {inside:?}");
    println!("far left        (2,31):   {far_left:?}");
    println!("far right     (509,31):   {far_right:?}");

    let inside_flooded = inside.a > 0;
    let far_left_flooded = far_left.a > 0;
    let far_right_flooded = far_right.a > 0;

    if !inside_flooded {
        eprintln!("UNEXPECTED: the 3x3 rect itself was not flooded at all - test setup is broken");
        std::process::exit(2);
    }

    if far_left_flooded || far_right_flooded {
        eprintln!(
            "REPRODUCED: flood filter painted pixels far outside the tiny 3x3 filtered rect \
             (far_left flooded = {far_left_flooded}, far_right flooded = {far_right_flooded}), \
             not just the filter region."
        );
        std::process::exit(1);
    }

    println!("did not reproduce - flood stayed within the filtered content's region");
}
