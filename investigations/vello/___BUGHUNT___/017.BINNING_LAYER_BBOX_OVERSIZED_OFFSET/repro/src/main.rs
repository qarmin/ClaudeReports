// Standalone, end-to-end reproduction for vello issue #1061 ("Layer shapes are wrong"),
// candidate 017.BINNING_LAYER_BBOX_OVERSIZED_OFFSET.
//
// Drives the real public `vello` API (Scene + Renderer::render_to_texture) through the real
// GPU compute pipeline (a real wgpu device, Vulkan/Metal/DX12/software) - this is the exact
// same scene used by the project's own `vello_tests/tests/known_issues.rs::test_layer_size`
// (which is itself the smoke test linked from the issue, PR #1062).
//
// Scene: a 60x60 canvas filled green, then a red 20x20 rect at (20,20)-(40,40), then an empty
// layer (push_layer(Compose::Clear) immediately popped with nothing drawn inside) clipped to
// that same (20,20)-(40,40) rect. The empty Clear layer should erase (make transparent) exactly
// that 20x20 rect and nothing else. Instead, the renderer erases a 32x32 region from (16,16) to
// (47,47) - the rect's bounding box rounded *out* to the 16px tile grid.
//
// Exit code 0 = bug not present (region matches the true 20x20 rect exactly).
// Exit code 1 = bug reproduced (cleared region is larger than the true rect).

use std::num::NonZeroUsize;

use vello::kurbo::{Affine, Rect};
use vello::peniko::color::AlphaColor;
use vello::peniko::{Compose, Fill};
use vello::wgpu::{
    self, BufferDescriptor, BufferUsages, CommandEncoderDescriptor, Extent3d,
    TexelCopyBufferInfo, TexelCopyBufferLayout, TextureDescriptor, TextureDimension,
    TextureFormat, TextureUsages, TextureViewDescriptor,
};
use vello::{AaConfig, RenderParams, Renderer, RendererOptions, Scene};

const WIDTH: u32 = 60;
const HEIGHT: u32 = 60;
// The rect used both for the red fill and for the layer's clip shape.
const RECT_X0: f64 = 20.0;
const RECT_Y0: f64 = 20.0;
const RECT_X1: f64 = 40.0;
const RECT_Y1: f64 = 40.0;

fn build_scene() -> Scene {
    let mut scene = Scene::new();
    scene.fill(
        Fill::NonZero,
        Affine::IDENTITY,
        AlphaColor::from_rgb8(0, 255, 0),
        None,
        &Rect::from_origin_size((0.0, 0.0), (60., 60.)),
    );
    scene.fill(
        Fill::NonZero,
        Affine::IDENTITY,
        AlphaColor::from_rgb8(255, 0, 0),
        None,
        &Rect::new(RECT_X0, RECT_Y0, RECT_X1, RECT_Y1),
    );
    scene.push_layer(
        Fill::NonZero,
        Compose::Clear,
        1.0,
        Affine::IDENTITY,
        &Rect::new(RECT_X0, RECT_Y0, RECT_X1, RECT_Y1),
    );
    scene.pop_layer();
    scene
}

fn render(scene: &Scene) -> Vec<[u8; 4]> {
    pollster::block_on(async {
        let instance = wgpu::Instance::default();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await
            .expect("no compatible wgpu adapter found");
        eprintln!("wgpu adapter: {:?}", adapter.get_info());
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default())
            .await
            .expect("failed to create wgpu device");

        let mut renderer = Renderer::new(
            &device,
            RendererOptions {
                use_cpu: false,
                num_init_threads: NonZeroUsize::new(1),
                antialiasing_support: std::iter::once(AaConfig::Area).collect(),
                pipeline_cache: None,
            },
        )
        .expect("failed to create vello Renderer");

        let size = Extent3d {
            width: WIDTH,
            height: HEIGHT,
            depth_or_array_layers: 1,
        };
        let target = device.create_texture(&TextureDescriptor {
            label: Some("target"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8Unorm,
            usage: TextureUsages::STORAGE_BINDING | TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        let view = target.create_view(&TextureViewDescriptor::default());

        let params = RenderParams {
            base_color: vello::peniko::color::palette::css::BLACK,
            width: WIDTH,
            height: HEIGHT,
            antialiasing_method: AaConfig::Area,
        };
        renderer
            .render_to_texture(&device, &queue, scene, &view, &params)
            .expect("render_to_texture failed");

        let padded_bytes_per_row = (WIDTH * 4).next_multiple_of(256);
        let buffer_size = padded_bytes_per_row as u64 * HEIGHT as u64;
        let buffer = device.create_buffer(&BufferDescriptor {
            label: Some("readback"),
            size: buffer_size,
            usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let mut encoder =
            device.create_command_encoder(&CommandEncoderDescriptor { label: None });
        encoder.copy_texture_to_buffer(
            target.as_image_copy(),
            TexelCopyBufferInfo {
                buffer: &buffer,
                layout: TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(padded_bytes_per_row),
                    rows_per_image: None,
                },
            },
            size,
        );
        queue.submit([encoder.finish()]);

        let slice = buffer.slice(..);
        let (tx, rx) = std::sync::mpsc::channel();
        slice.map_async(wgpu::MapMode::Read, move |res| tx.send(res).unwrap());
        device.poll(wgpu::PollType::wait_indefinitely()).unwrap();
        rx.recv().unwrap().expect("failed to map readback buffer");

        let data = slice.get_mapped_range();
        let mut pixels = Vec::with_capacity((WIDTH * HEIGHT) as usize);
        for row in 0..HEIGHT {
            let start = (row * padded_bytes_per_row) as usize;
            for col in 0..WIDTH {
                let px = start + col as usize * 4;
                pixels.push([data[px], data[px + 1], data[px + 2], data[px + 3]]);
            }
        }
        pixels
    })
}

fn main() {
    let scene = build_scene();
    let pixels = render(&scene);

    // Bounding box of every pixel that differs from the green background (0,255,0,255).
    let mut min_x = WIDTH;
    let mut min_y = HEIGHT;
    let mut max_x = 0i64;
    let mut max_y = 0i64;
    let mut any = false;
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let px = pixels[(y * WIDTH + x) as usize];
            if px != [0, 255, 0, 255] {
                any = true;
                min_x = min_x.min(x);
                min_y = min_y.min(y);
                max_x = max_x.max(x as i64);
                max_y = max_y.max(y as i64);
            }
        }
    }

    assert!(any, "expected at least one non-green pixel (the cleared rect)");

    let (expect_min, expect_max) = (RECT_X0 as i64, RECT_X1 as i64 - 1);
    println!(
        "bounding box of affected (non-green) pixels: x=[{min_x},{max_x}] y=[{min_y},{max_y}]"
    );
    println!("expected (true rect, no oversizing): x=[{expect_min},{expect_max}] y=[{expect_min},{expect_max}]");

    let matches_true_rect = min_x as i64 == expect_min
        && max_x == expect_max
        && min_y as i64 == expect_min
        && max_y == expect_max;

    if matches_true_rect {
        println!("NOT REPRODUCED: cleared region exactly matches the true 20x20 rect.");
        std::process::exit(0);
    } else {
        println!(
            "REPRODUCED: cleared region is {}x{} instead of the true 20x20 rect - oversized/offset, \
             consistent with rounding out to the 16px tile grid.",
            max_x - min_x as i64 + 1,
            max_y - min_y as i64 + 1
        );
        std::process::exit(1);
    }
}
