// Scratch repro for bug-hunt candidate 010: does vello_hybrid's paint_visible
// early-out (skipping fill_rect when the solid paint's alpha is 0) break
// destructive blend modes (Compose::Clear / Compose::Copy)?

use vello_common::color::palette::css::{BLUE, TRANSPARENT, YELLOW};
use vello_common::kurbo::Rect;
use vello_common::peniko::{BlendMode, Compose, Mix};
use vello_common::pixmap::Pixmap;
use vello_hybrid::Scene;

fn render(scene: &Scene) -> Pixmap {
    let width = scene.width();
    let height = scene.height();

    let instance = wgpu::Instance::default();
    let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::default(),
        force_fallback_adapter: false,
        compatible_surface: None,
    }))
    .expect("Failed to find an appropriate adapter");
    let (device, queue) = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor {
        label: Some("Device"),
        required_features: wgpu::Features::empty(),
        ..Default::default()
    }))
    .expect("Failed to create device");

    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Render Target"),
        size: wgpu::Extent3d {
            width: width.into(),
            height: height.into(),
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

    let (mut renderer, mut resources) = vello_hybrid::Renderer::new(
        &device,
        &vello_hybrid::RenderTargetConfig {
            format: texture.format(),
            width: width.into(),
            height: height.into(),
        },
    );
    let render_size = vello_hybrid::RenderSize {
        width: width.into(),
        height: height.into(),
    };
    let depth_texture_view =
        vello_hybrid::Renderer::create_depth_texture_view(&device, &render_size);

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Vello Render To Buffer"),
    });
    renderer
        .render(
            scene,
            &mut resources,
            &device,
            &queue,
            &mut encoder,
            &render_size,
            &texture_view,
            Some(&depth_texture_view),
            &vello_hybrid::TextureBindings::new(),
        )
        .unwrap();

    let bytes_per_row = (u32::from(width) * 4).next_multiple_of(256);
    let texture_copy_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Output Buffer"),
        size: u64::from(bytes_per_row) * u64::from(height),
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
            buffer: &texture_copy_buffer,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(bytes_per_row),
                rows_per_image: None,
            },
        },
        wgpu::Extent3d {
            width: width.into(),
            height: height.into(),
            depth_or_array_layers: 1,
        },
    );
    queue.submit([encoder.finish()]);

    texture_copy_buffer
        .slice(..)
        .map_async(wgpu::MapMode::Read, move |result| {
            if result.is_err() {
                panic!("Failed to map texture for reading");
            }
        });
    device.poll(wgpu::PollType::wait_indefinitely()).unwrap();

    let mut img_data = Vec::with_capacity(usize::from(width) * usize::from(height) * 4);
    for row in texture_copy_buffer
        .slice(..)
        .get_mapped_range()
        .chunks_exact(bytes_per_row as usize)
    {
        img_data.extend_from_slice(&row[0..usize::from(width) * 4]);
    }
    texture_copy_buffer.unmap();

    Pixmap::from_parts(
        img_data,
        width,
        height,
        vello_common::pixmap::PixelMetadata::default(),
    )
}

/// Builds a scene: draw an opaque yellow rect as backdrop, then push a
/// blend layer with the given `compose` mode and draw a rect with the given
/// `paint`. If `paint` is fully transparent, `fill_rect` hits vello_hybrid's
/// `paint_visible` early-out and records zero strips for that call.
fn build_scene(compose: Compose, paint: vello_common::peniko::Color) -> Scene {
    let mut scene = Scene::new(100, 100);
    scene.set_paint(YELLOW.with_alpha(1.0));
    scene.fill_rect(&Rect::new(10.0, 10.0, 70.0, 70.0));

    scene.push_blend_layer(BlendMode::new(Mix::Normal, compose));
    scene.set_paint(paint);
    scene.fill_rect(&Rect::new(30.0, 30.0, 90.0, 90.0));
    scene.pop_layer();

    scene
}

fn check_pixel(pixmap: &Pixmap, x: u16, y: u16) -> [u8; 4] {
    let p = pixmap.sample(x, y);
    [p.r, p.g, p.b, p.a]
}

fn main() {
    let mut any_bug = false;

    for compose in [
        Compose::Clear,
        Compose::Copy,
        Compose::SrcIn,
        Compose::DestIn,
        Compose::SrcOut,
        Compose::DestAtop,
    ] {
        println!("=== compose = {compose:?} ===");

        let opaque_scene = build_scene(compose, BLUE.with_alpha(1.0));
        let opaque_px = render(&opaque_scene);

        let transparent_scene = build_scene(compose, TRANSPARENT);
        let transparent_px = render(&transparent_scene);

        // Sample a point that is only covered by the yellow backdrop rect,
        // NOT by the blue/transparent rect inside the blend layer
        // (yellow rect: 10..70, inner rect: 30..90 -> (20,20) is yellow-only).
        let opaque_yellow_only = check_pixel(&opaque_px, 20, 20);
        let transparent_yellow_only = check_pixel(&transparent_px, 20, 20);

        println!(
            "  yellow-only pixel (20,20): opaque-source={opaque_yellow_only:?} transparent-source={transparent_yellow_only:?}"
        );

        // Sample a point covered only by the inner rect (not by yellow):
        // (80, 80) is inside 30..90 but outside 10..70.
        let opaque_inner_only = check_pixel(&opaque_px, 80, 80);
        let transparent_inner_only = check_pixel(&transparent_px, 80, 80);
        println!(
            "  inner-only pixel (80,80): opaque-source={opaque_inner_only:?} transparent-source={transparent_inner_only:?}"
        );

        // The claimed bug: with the paint_visible early-out, a fully-transparent
        // paint causes fill_rect to record zero strips, same as never drawing at
        // all. If schedule_child_layer's is_destructive() handling did not exist,
        // this would leave the destructive layer's bbox empty and the backdrop
        // (yellow rect outside the blue rect's own footprint) would wrongly
        // survive untouched instead of being erased by Clear/Copy.
        if transparent_yellow_only == opaque_yellow_only {
            println!(
                "  OK: transparent-source layer erased the yellow backdrop exactly like the opaque-source reference"
            );
        } else {
            any_bug = true;
            println!(
                "  BUG: transparent-source layer left the yellow backdrop different from the opaque-source reference (opaque={opaque_yellow_only:?} transparent={transparent_yellow_only:?})"
            );
        }
    }

    if any_bug {
        println!("RESULT: BUG REPRODUCED");
        std::process::exit(1);
    } else {
        println!("RESULT: NO BUG (transparent-source draws inside destructive-blend layers erase the backdrop exactly like opaque-source draws do)");
    }
}
