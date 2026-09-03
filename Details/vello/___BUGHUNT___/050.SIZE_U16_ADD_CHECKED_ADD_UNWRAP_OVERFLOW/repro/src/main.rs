// Candidate 050: does any real, user-reachable path in vello_hybrid ever call
// `impl Add for SizeU16` / `impl Add<u16> for SizeU16` (sparse_strips/vello_common/src/geometry.rs)
// with operands close enough to u16::MAX to trigger the `.checked_add(..).unwrap()` panic?
//
// This binary demonstrates two things using the real, public API:
//
// 1. The operator itself is a genuine, exploitable-looking footgun in isolation: calling it
//    directly with boundary values does panic, exactly as the candidate suspected.
// 2. The actual public entry points a real application would use to feed "user-supplied scene
//    content" into vello_hybrid's texture/layer/atlas sizing (`Scene::new` with extreme
//    dimensions, `MemorySettings`/`LayersConfig` with extreme min/max texture sizes) never
//    exercise this operator - vello_hybrid's own internal sizing code
//    (`schedule/allocate.rs::RegionProps::allocation_size`,
//    `render/common.rs::LayersConfig::required_intermediate_texture_size`) uses
//    `SizeU16::checked_add` (returns `Option`, safe) or manual `u32` arithmetic instead, never
//    the `Add`/`Add<u16>` operator impls that panic.

use std::panic::{self, AssertUnwindSafe};
use vello_common::geometry::SizeU16;
use vello_common::kurbo::Shape;
use vello_hybrid::{MemorySettings, Scene};

fn main() {
    println!("=== Part 1: direct calls to the public `Add` impl at the u16 boundary ===");
    for i in 0..3 {
        let a = SizeU16::from_wh(u16::MAX - 1, u16::MAX - 1);
        let b = SizeU16::from_wh(2, 2);
        let result = panic::catch_unwind(AssertUnwindSafe(|| a + b));
        match result {
            Ok(size) => println!("run {i}: unexpectedly did NOT panic, got {size:?}"),
            Err(payload) => {
                let msg = payload
                    .downcast_ref::<&str>()
                    .map(|s| s.to_string())
                    .or_else(|| payload.downcast_ref::<String>().cloned())
                    .unwrap_or_else(|| "<non-string panic payload>".to_string());
                println!("run {i}: panicked as expected: {msg}");
            }
        }
    }

    println!();
    println!("=== Part 2: real public entry points with extreme, user-controlled sizes ===");

    // A near-maximum canvas size. `65532` (a multiple of the internal tile width of 4) is used
    // instead of the literal `u16::MAX` (65535) to stay clear of the separate, already
    // acknowledged and already-tracked `RectU16::snap_to_tile_coordinates` edge panic (see
    // RESULT.md's "New suspects noticed" - unrelated to this candidate).
    const NEAR_MAX: u16 = 65532;
    let mut scene = Scene::new(NEAR_MAX, NEAR_MAX);
    println!(
        "Scene::new({NEAR_MAX}, {NEAR_MAX}) succeeded: {}x{}",
        scene.width(),
        scene.height()
    );

    // Push and pop a filter layer whose clip covers the whole (near-u16::MAX) canvas - this is
    // the kind of "user-supplied scene content" the candidate worried about, exercised through
    // the same public Scene API a real application would use.
    let clip = vello_common::kurbo::Rect::new(0.0, 0.0, NEAR_MAX as f64, NEAR_MAX as f64);
    scene.push_layer(
        Some(&vello_common::kurbo::BezPath::from_iter(
            clip.path_elements(0.1),
        )),
        None,
        None,
        None,
        None,
    );
    scene.pop_layer();
    println!("push_layer/pop_layer with a near-u16::MAX bbox succeeded without panicking.");

    // A MemorySettings with both min and max texture size pinned to u16::MAX - the most extreme
    // user-configurable values the candidate flagged as a possible source of overflow.
    let mut settings = MemorySettings::default();
    settings.layers_config.min_texture_size = SizeU16::new(u16::MAX);
    settings.layers_config.max_texture_size = SizeU16::new(u16::MAX);
    println!(
        "MemorySettings with min_texture_size = max_texture_size = SizeU16::new(u16::MAX) constructed \
         without panicking: {settings:?}"
    );

    println!();
    println!(
        "No internal vello_hybrid code path reached the `Add`/`Add<u16>` impls for SizeU16 in \
         this run (confirmed separately in RESULT.md by rebuilding the entire workspace with \
         both impls removed: it compiles and every existing test still passes)."
    );
}
