// Repro for candidate 015 (issue https://github.com/linebender/vello/issues/1240).
//
// Renders the REAL `FunkyPathsScene` (sparse_strips/vello_example_scenes/src/path.rs) through the
// REAL, public `vello_cpu::RenderContext` API - the exact same entry point a real application
// embedding vello_cpu would use, and the exact scene shipped in vello's own example-scene gallery
// (rendered by default in both the native winit demo and the wasm/webgl demo).
//
// The scene's "missing_movetos" path has a second subpath that is never explicitly closed
// (`line_to` after `close_path`, with no intervening `move_to`, and no trailing `close_path` at
// all - see path.rs). At the historically-reported-broken commit, the fill for that subpath
// leaked out unbounded to the right edge of the tile/viewport instead of staying within the
// triangle's own footprint (matching the screenshot attached to issue #1240). At the commit this
// repro is run against (see `../justfile`'s `base_commit`), that no longer happens.
//
// This prints PASS/FAIL and also writes `funky_scene.png` for visual inspection.

use vello_cpu::kurbo::Affine;
use vello_cpu::{Level, Pixmap, RasterizerSettings, RenderContext, RenderSettings, Resources};
use vello_example_scenes::path::FunkyPathsScene;
use vello_example_scenes::ExampleScene;

fn main() {
    let width = 400_u16;
    let height = 600_u16;

    let settings = RenderSettings {
        level: Level::new(),
        num_threads: 0,
    };
    let mut ctx = RenderContext::new_with(width, height, settings);
    let mut resources = Resources::new();
    let mut scene = FunkyPathsScene::new();
    scene.render(&mut ctx, &mut resources, Affine::IDENTITY);
    ctx.flush();

    let mut pixmap = Pixmap::new(width, height);
    ctx.render_with(&mut pixmap, &mut resources, RasterizerSettings::default());

    // The second ("missing_movetos" bottom) subpath is, in device space (root_transform is
    // identity, and the scene applies `Affine::translate((100.0, 100.0))` before drawing it):
    //   (100,100) -> (100,500) -> (200,500)
    // Its rightmost extent is x=200. A point well to the right of that (x=350), but still well
    // within its vertical span (y=450), must be untouched background if winding is computed
    // correctly. If the pre-fix `closed`-flag bug from vello_common's flatten_simd.rs (see
    // RESULT.md) is present, this pixel gets incorrectly painted opaque blue because the
    // subpath's closing edge back to its start point is silently never emitted, so the winding
    // number never returns to 0 before the tile/row's right edge.
    let probe_x = 350_u16;
    let probe_y = 450_u16;
    let pixel = pixmap.sample(probe_x, probe_y);
    println!("pixel at ({probe_x}, {probe_y}): {pixel:?} (premultiplied RGBA)");

    let bug_reproduced = pixel.a > 10; // any real coverage/alpha at all is wrong here

    std::fs::write("funky_scene.png", pixmap.into_png().unwrap()).unwrap();
    println!("wrote funky_scene.png");

    if bug_reproduced {
        println!(
            "FAIL (bug reproduced): pixel at ({probe_x}, {probe_y}) has alpha {}, expected fully \
             transparent background",
            pixel.a
        );
        std::process::exit(1);
    } else {
        println!(
            "PASS (bug NOT reproduced): pixel at ({probe_x}, {probe_y}) is background \
             (alpha={}), as expected for a correctly closed/bounded fill",
            pixel.a
        );
    }
}
