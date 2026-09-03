// Standalone reproduction of https://github.com/linebender/vello/issues/788:
// drawing very many images into a `vello::Scene` causes `RenderConfig::new` to
// underflow `buffer_sizes.bin_data.len() - layout.bin_data_start`.
//
// Uses only the real public API: `vello::Scene::draw_image` (exactly like the
// issue's own repro) followed by the same two calls `vello`'s own renderer
// makes internally (`Resolver::resolve` then `RenderConfig::new`), both public
// in `vello_encoding`. No GPU/wgpu device is needed since the panic happens on
// the CPU side before any GPU work is submitted.
//
// Run in a debug build (default `cargo run`, which has `overflow-checks = true`)
// to see the panic. In a release build the subtraction silently wraps instead
// (see README/RESULT.md for what that wrapped value means downstream).

use std::sync::Arc;

use vello::Scene;
use vello::kurbo::Affine;
use vello::peniko::{Blob, Color, ImageAlphaType, ImageData, ImageFormat};
use vello_encoding::{RenderConfig, Resolver};

fn build_scene(n_images: u32) -> Scene {
    let blob = Blob::new(Arc::new(vec![
        200u8;
        ImageFormat::Rgba8.size_in_bytes(100, 100).unwrap()
    ]));
    let img = ImageData {
        data: blob,
        format: ImageFormat::Rgba8,
        alpha_type: ImageAlphaType::Alpha,
        width: 100,
        height: 100,
    };

    let mut scene = Scene::new();
    for _ in 0..n_images {
        scene.draw_image(&img, Affine::IDENTITY);
    }
    scene
}

fn main() {
    let n_images: u32 = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(1_000_000);

    let scene = build_scene(n_images);

    let mut resolver = Resolver::new();
    let mut packed = vec![];
    let (layout, _ramps, _images) = resolver.resolve(scene.encoding(), &mut packed);

    println!(
        "n_images = {n_images}, layout.bin_data_start = {}",
        layout.bin_data_start
    );

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        RenderConfig::new(&layout, 100, 100, &Color::WHITE)
    }));

    match result {
        Ok(cfg) => {
            println!(
                "did not reproduce - RenderConfig::new completed, binning_size = {}",
                cfg.gpu.binning_size
            );
        }
        Err(panic_payload) => {
            eprintln!("REPRODUCED - RenderConfig::new panicked: {:?}", panic_payload);
            std::process::exit(1);
        }
    }
}
