// Second, independent check for candidate 028: attempts to run the REAL GPU shader path (the
// actual `flatten.wgsl` compute shader, naga-compiled to real SPIR-V and dispatched through
// `wgpu`, NOT the CPU fallback used by `main.rs`) via whatever Vulkan/GL adapter is available on
// the machine running this - a software Vulkan implementation (e.g. SwiftShader/lavapipe) if no
// real GPU is present. This is still not Metal, so it cannot exercise Metal's own
// `fastMathEnabled` specifically, but it is a genuine non-CPU compute-shader dispatch (real
// SPIR-V, real Vulkan implementation, not a CPU reimplementation) - a strictly closer
// approximation to the reported bug's own trigger path than the pure-CPU-fallback check in
// `main.rs`, worth trying on whatever GPU backend is actually available before concluding this is
// unreproducible in a given environment.
//
// Renders both the real GPU-shader path and the real CPU-fallback path for the same scene
// (near-cusp cubic from `tricky_strokes`, centered on the cubic's own cusp point so it stays
// on-canvas at every scale) and diffs them pixel-by-pixel: a large, structured difference (as
// opposed to antialiasing-level noise) would indicate a genuine GPU-vs-CPU divergence
// reproducible on this hardware/backend. PNGs are written to `out/` for visual inspection.

use vello::kurbo::{Affine, BezPath, Cap, Join, PathEl, Point, Stroke};
use vello::peniko::Color;
use vello::peniko::ImageData;
use vello::util::RenderContext;
use vello::{AaConfig, RendererOptions, Scene};

const NEAR_CUSP: [(f64, f64); 4] = [(0., 0.01), (128., 127.999), (128., 0.01), (0., 127.99)];
const STROKE_WIDTH: f64 = 30.;
const WIDTH: u32 = 400;
const HEIGHT: u32 = 400;
/// The t=0.5 De Casteljau point of `NEAR_CUSP`, i.e. this symmetric cubic's own cusp location in
/// local coordinates - used to keep the interesting region on-canvas at every scale (a fixed
/// translation would zoom the actual cusp off-screen for large `s`).
const CUSP_LOCAL: (f64, f64) = (96.0, 64.0);

fn main() {
    pollster::block_on(run());
}

fn build_scene(s: f64) -> Scene {
    let mut path = BezPath::new();
    path.push(PathEl::MoveTo(Point::new(NEAR_CUSP[0].0, NEAR_CUSP[0].1)));
    path.push(PathEl::CurveTo(
        Point::new(NEAR_CUSP[1].0, NEAR_CUSP[1].1),
        Point::new(NEAR_CUSP[2].0, NEAR_CUSP[2].1),
        Point::new(NEAR_CUSP[3].0, NEAR_CUSP[3].1),
    ));
    let stroke = Stroke::new(STROKE_WIDTH / s)
        .with_caps(Cap::Butt)
        .with_join(Join::Miter);
    let tx = (WIDTH as f64 / 2.0) - s * CUSP_LOCAL.0;
    let ty = (HEIGHT as f64 / 2.0) - s * CUSP_LOCAL.1;
    let transform = Affine::new([s, 0.0, 0.0, s, tx, ty]);
    let mut scene = Scene::new();
    scene.stroke(&stroke, transform, Color::from_rgb8(0, 0, 0), None, &path);
    scene
}

fn render(
    renderer: &mut vello::Renderer,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    scene: &Scene,
) -> ImageData {
    let render_params = vello::RenderParams {
        base_color: Color::WHITE,
        width: WIDTH,
        height: HEIGHT,
        antialiasing_method: AaConfig::Area,
    };
    let size = wgpu::Extent3d {
        width: WIDTH,
        height: HEIGHT,
        depth_or_array_layers: 1,
    };
    let target = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Target texture"),
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    });
    let view = target.create_view(&wgpu::TextureViewDescriptor::default());
    renderer
        .render_to_texture(device, queue, scene, &view, &render_params)
        .expect("render_to_texture failed");

    let padded_byte_width = (WIDTH * 4).next_multiple_of(256);
    let buffer_size = padded_byte_width as u64 * HEIGHT as u64;
    let buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("val"),
        size: buffer_size,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Copy out buffer"),
    });
    encoder.copy_texture_to_buffer(
        target.as_image_copy(),
        wgpu::TexelCopyBufferInfo {
            buffer: &buffer,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(padded_byte_width),
                rows_per_image: None,
            },
        },
        size,
    );
    queue.submit([encoder.finish()]);
    let buf_slice = buffer.slice(..);
    buf_slice.map_async(wgpu::MapMode::Read, |v| v.unwrap());
    device.poll(wgpu::PollType::wait_indefinitely()).unwrap();
    let data = buf_slice.get_mapped_range();
    let mut result_unpadded = Vec::<u8>::with_capacity((WIDTH * HEIGHT * 4) as usize);
    for row in 0..HEIGHT {
        let start = (row * padded_byte_width) as usize;
        result_unpadded.extend(&data[start..start + (WIDTH * 4) as usize]);
    }
    ImageData {
        data: vello::peniko::Blob::new(std::sync::Arc::new(result_unpadded)),
        format: vello::peniko::ImageFormat::Rgba8,
        width: WIDTH,
        height: HEIGHT,
        alpha_type: vello::peniko::ImageAlphaType::Alpha,
    }
}

fn write_png(path: &str, image: &ImageData) {
    let mut data = Vec::new();
    let mut encoder = png::Encoder::new(&mut data, WIDTH, HEIGHT);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(image.data.data()).unwrap();
    writer.finish().unwrap();
    std::fs::write(path, &data).unwrap();
}

/// Counts pixels differing by more than 8/255 in any channel, and the largest single-channel
/// difference observed - a real "flattening artifact" glitch (a visible spike/notch) would show
/// up as a large, spatially-structured difference; antialiasing-level rounding noise between two
/// independent (but both-correct) rasterizer implementations does not.
fn diff(a: &ImageData, b: &ImageData) -> (usize, u8) {
    let da = a.data.data();
    let db = b.data.data();
    let mut n_diff = 0;
    let mut max_diff = 0u8;
    for (x, y) in da.iter().zip(db.iter()) {
        let d = x.abs_diff(*y);
        if d > 8 {
            n_diff += 1;
        }
        max_diff = max_diff.max(d);
    }
    (n_diff, max_diff)
}

async fn run() {
    let mut context = RenderContext::new();
    let device_id = match context.device(None).await {
        Some(id) => id,
        None => {
            println!(
                "NO GPU/Vulkan adapter available on this machine - cannot attempt the real \
                 GPU-shader-dispatch check here (see RESULT.md; this is a legitimate 'not \
                 attempted' outcome, not a negative result)"
            );
            return;
        }
    };
    let info = context.devices[device_id].adapter().get_info();
    println!("Using adapter: {info:?}");

    let device_handle = &mut context.devices[device_id];
    let device = device_handle.device.clone();
    let queue = device_handle.queue.clone();
    let mut gpu_renderer = vello::Renderer::new(
        &device,
        RendererOptions {
            use_cpu: false, // real GPU compute-shader dispatch, not the CPU fallback
            num_init_threads: std::num::NonZeroUsize::new(1),
            antialiasing_support: std::iter::once(AaConfig::Area).collect(),
            pipeline_cache: None,
        },
    )
    .expect("gpu renderer creation failed");
    let mut cpu_renderer = vello::Renderer::new(
        &device,
        RendererOptions {
            use_cpu: true,
            num_init_threads: std::num::NonZeroUsize::new(1),
            antialiasing_support: std::iter::once(AaConfig::Area).collect(),
            pipeline_cache: None,
        },
    )
    .expect("cpu renderer creation failed");

    std::fs::create_dir_all("out").ok();
    for s in [1.0_f64, 5.3332, 8.0, 32.0, 64.0, 113.5797, 200.0] {
        let scene = build_scene(s);
        let gpu_img = render(&mut gpu_renderer, &device, &queue, &scene);
        let cpu_img = render(&mut cpu_renderer, &device, &queue, &scene);
        let (n_diff, max_diff) = diff(&gpu_img, &cpu_img);
        println!(
            "scale={s:>10.4}  differing_pixels(>8/255)={n_diff:>6}  max_channel_diff={max_diff:>3}"
        );
        write_png(&format!("out/gpu_scale_{s}.png"), &gpu_img);
        write_png(&format!("out/cpu_scale_{s}.png"), &cpu_img);
    }
}
