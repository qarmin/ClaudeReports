// Standalone differential CPU-vs-`vello_hybrid`(GPU) rendering repro.
//
// Registers a small solid-red opaque image through `vello_hybrid`'s real public
// `Renderer::upload_image` API, fills a rectangle covering the whole canvas with it (also via the
// real public `Scene`/paint API), and renders the scene through a *real* `wgpu::Device` explicitly
// requested on the `Backends::GL` backend (Mesa's `llvmpipe` software rasterizer via
// `LIBGL_ALWAYS_SOFTWARE=1`, or any other GL/GLES driver present) - the same public entry point a
// real caller targeting WebGL/native-GL uses. The identical scene is also rendered through the
// real `vello_cpu::RenderContext` as a reference.
//
// On an unpatched checkout, the image comes out fully transparent/black on the `Backends::GL`
// path (the render target is left as its cleared background) even though the identical scene
// renders the solid-red rectangle correctly through `vello_cpu` and through `vello_hybrid` on
// `Backends::VULKAN`.

use std::sync::Arc;

use vello_common::kurbo::Rect;
use vello_common::paint::{Image, ImageSource};
use vello_common::peniko::{Extend, ImageQuality, ImageSampler};
use vello_common::pixmap::Pixmap;
use vello_cpu::{RenderContext, RenderMode, RenderSettings, Resources as CpuResources};
use vello_hybrid::{
    RenderSettings as HybridRenderSettings, RenderSize, RenderTargetConfig, Scene,
    TextureBindings,
};

const WIDTH: u16 = 64;
const HEIGHT: u16 = 64;

/// A small fully-opaque solid-red test image (premultiplied RGBA8, alpha = 255 so premultiplied
/// == straight).
fn solid_red_image() -> Pixmap {
    let mut pixmap = Pixmap::new(8, 8);
    let data = pixmap.data_as_u8_slice_mut();
    for px in data.chunks_exact_mut(4) {
        px.copy_from_slice(&[255, 0, 0, 255]);
    }
    pixmap
}

fn image_paint(source: ImageSource) -> Image {
    Image {
        image: source,
        sampler: ImageSampler {
            x_extend: Extend::Pad,
            y_extend: Extend::Pad,
            quality: ImageQuality::Low,
            alpha: 1.0,
        },
    }
}

fn render_cpu() -> Vec<u8> {
    let settings = RenderSettings {
        level: vello_cpu::Level::new(),
        num_threads: 0,
    };
    let mut ctx = RenderContext::new_with(WIDTH, HEIGHT, settings);
    let mut resources = CpuResources::new();

    let image = Arc::new(solid_red_image());
    let id = resources.register_image(Arc::clone(&image));
    ctx.set_paint(image_paint(ImageSource::opaque_id(id)));
    ctx.fill_rect(&Rect::new(0.0, 0.0, WIDTH.into(), HEIGHT.into()));
    ctx.flush();

    let mut pixmap = Pixmap::new(WIDTH, HEIGHT);
    ctx.render_with(
        &mut pixmap,
        &mut resources,
        vello_cpu::RasterizerSettings {
            render_mode: RenderMode::OptimizeQuality,
            ..Default::default()
        },
    );
    pixmap.data_as_u8_slice().to_vec()
}

/// Renders the identical scene through `vello_hybrid`, driven by a real `wgpu::Device` requested
/// on the given `backends` (e.g. `wgpu::Backends::GL` or `wgpu::Backends::VULKAN`).
fn render_hybrid(backends: wgpu::Backends) -> (Vec<u8>, String) {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends,
        ..wgpu::InstanceDescriptor::new_without_display_handle()
    });
    let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::default(),
        force_fallback_adapter: false,
        compatible_surface: None,
    }))
    .expect("Failed to find an adapter for the requested backend");
    let adapter_name = adapter.get_info().name;
    let (device, queue) = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor {
        label: Some("Device"),
        required_features: wgpu::Features::empty(),
        ..Default::default()
    }))
    .expect("Failed to create device");

    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Render Target"),
        size: wgpu::Extent3d {
            width: WIDTH.into(),
            height: HEIGHT.into(),
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    });
    let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

    let (mut renderer, mut resources) = vello_hybrid::Renderer::new_with(
        &device,
        &RenderTargetConfig {
            format: texture.format(),
            width: WIDTH.into(),
            height: HEIGHT.into(),
        },
        HybridRenderSettings::default(),
    );
    let render_size = RenderSize {
        width: WIDTH.into(),
        height: HEIGHT.into(),
    };
    let depth_texture_view = vello_hybrid::Renderer::create_depth_texture_view(&device, &render_size);

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Vello Render To Buffer"),
    });

    // Real public API: register the image through the renderer's own atlas-upload entry point.
    let id = renderer.upload_image(&mut resources, &device, &queue, &mut encoder, &solid_red_image());

    let mut scene = Scene::new_with(WIDTH, HEIGHT, vello_common::fearless_simd::Level::baseline());
    scene.set_paint(image_paint(ImageSource::opaque_id(id)));
    scene.fill_rect(&Rect::new(0.0, 0.0, WIDTH.into(), HEIGHT.into()));

    renderer
        .render(
            &scene,
            &mut resources,
            &device,
            &queue,
            &mut encoder,
            &render_size,
            &texture_view,
            Some(&depth_texture_view),
            &TextureBindings::new(),
        )
        .unwrap();

    let bytes_per_row = (u32::from(WIDTH) * 4).next_multiple_of(256);
    let output_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Output Buffer"),
        size: u64::from(bytes_per_row) * u64::from(HEIGHT),
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });
    encoder.copy_texture_to_buffer(
        wgpu::TexelCopyTextureInfo {
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        wgpu::TexelCopyBufferInfo {
            buffer: &output_buffer,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(bytes_per_row),
                rows_per_image: Some(HEIGHT.into()),
            },
        },
        wgpu::Extent3d {
            width: WIDTH.into(),
            height: HEIGHT.into(),
            depth_or_array_layers: 1,
        },
    );
    queue.submit([encoder.finish()]);

    let buffer_slice = output_buffer.slice(..);
    buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
        result.expect("Failed to map buffer for reading");
    });
    device.poll(wgpu::PollType::wait_indefinitely()).unwrap();

    let data = buffer_slice.get_mapped_range();
    let mut out = Vec::with_capacity(usize::from(WIDTH) * usize::from(HEIGHT) * 4);
    for row in 0..usize::from(HEIGHT) {
        let start = row * bytes_per_row as usize;
        let end = start + usize::from(WIDTH) * 4;
        out.extend_from_slice(&data[start..end]);
    }
    drop(data);
    output_buffer.unmap();

    (out, adapter_name)
}

/// Returns the fraction of pixels whose RGBA differs from `expected` by more than 2 per channel.
fn fraction_mismatching(pixels: &[u8], expected: [u8; 4]) -> f64 {
    let mut mismatches = 0usize;
    let mut total = 0usize;
    for px in pixels.chunks_exact(4) {
        total += 1;
        let bad = px
            .iter()
            .zip(expected.iter())
            .any(|(a, b)| a.abs_diff(*b) > 2);
        if bad {
            mismatches += 1;
        }
    }
    mismatches as f64 / total as f64
}

fn main() {
    let cpu = render_cpu();
    let cpu_mismatch = fraction_mismatching(&cpu, [255, 0, 0, 255]);
    println!("vello_cpu:               fraction of pixels != solid red = {cpu_mismatch:.4}");

    let (gl, gl_adapter) = render_hybrid(wgpu::Backends::GL);
    let gl_mismatch = fraction_mismatching(&gl, [255, 0, 0, 255]);
    println!(
        "vello_hybrid (Backends::GL, adapter={gl_adapter}): fraction of pixels != solid red = {gl_mismatch:.4}"
    );

    println!();
    if gl_mismatch > 0.5 {
        println!(
            "BUG REPRODUCED: vello_hybrid's Backends::GL render target is mostly NOT the \
             uploaded solid-red image (mismatch fraction {gl_mismatch:.4}), even though \
             vello_cpu (mismatch fraction {cpu_mismatch:.4}) renders the identical scene \
             correctly."
        );
        std::process::exit(1);
    } else {
        println!(
            "OK: vello_hybrid's Backends::GL output matches the uploaded image (mismatch \
             fraction {gl_mismatch:.4}), consistent with vello_cpu's reference render."
        );
    }
}
