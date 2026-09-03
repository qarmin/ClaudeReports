//! Headless Skia benchmark (`--skia-bench`): drives `i_slint_renderer_skia::SkiaRenderer`
//! directly through its own `SoftwareSurface`/`RenderBuffer` API, bypassing the winit backend
//! entirely.
//!
//! Why this exists instead of just using `--frames` with `--renderer skia`: the windowed
//! (`--frames`) path times frames via `Window::set_rendering_notifier`, and
//! `i_slint_renderer_skia`'s own `partial_rendering_state()` unconditionally disables partial
//! rendering whenever a rendering notifier is registered ("We don't know where the application
//! might render to, so disable partial rendering" - see `internal/renderers/skia/lib.rs`). That
//! makes the dirty-region clip path (the code this mode exists to measure) unreachable through
//! `--frames`, regardless of `SLINT_SKIA_PARTIAL_RENDERING`. Driving the renderer's own
//! `render()` method directly - the same pattern `api/rs/slint/tests/partial_renderer.rs` uses -
//! needs no notifier and keeps partial rendering on, timed with a plain `Instant` around each
//! `render()` call instead.
//!
//! Only implements what candidates touching the dirty-region clip path need: the `scroll` and
//! `rects` scenes, plain scrolling/animation, no mutate/resize/image/text modes. Extend rather
//! than duplicate if a future candidate needs more of `bench.rs`'s surface here.

use std::cell::{Cell, RefCell};
use std::num::NonZeroU32;
use std::rc::{Rc, Weak};
use std::time::{Duration, Instant};

use i_slint_renderer_skia::software_surface::{RenderBuffer, SoftwareSurface};
use i_slint_renderer_skia::{SkiaRenderer, SkiaSharedContext, skia_safe};
use slint::platform::{Platform, PlatformError, WindowAdapter};
use slint::{ComponentHandle, PhysicalSize, Rgba8Pixel, SharedPixelBuffer};

use crate::alloc_stats;
use crate::bench::BenchOptions;
use crate::{Logic, MainWindow, PerfMode, Settings};

/// Reuses one pixel buffer across frames (like a real "reused buffer" swapchain), so
/// `back_buffer_age` is 1 from the second frame on and the renderer only has to repaint what
/// actually changed - the common case the dirty-region clip path is meant for.
#[derive(Default)]
struct HeadlessRenderBuffer {
    pixels: RefCell<Option<SharedPixelBuffer<Rgba8Pixel>>>,
}

impl RenderBuffer for HeadlessRenderBuffer {
    fn with_buffer(
        &self,
        _window: &slint::Window,
        size: PhysicalSize,
        render_callback: &mut dyn FnMut(
            NonZeroU32,
            NonZeroU32,
            skia_safe::ColorType,
            u8,
            &mut [u8],
        ) -> Result<
            Option<i_slint_core::partial_renderer::DirtyRegion>,
            PlatformError,
        >,
    ) -> Result<(), PlatformError> {
        let Some((width, height)): Option<(NonZeroU32, NonZeroU32)> =
            size.width.try_into().ok().zip(size.height.try_into().ok())
        else {
            return Ok(());
        };

        let mut shared = self.pixels.borrow_mut().take();
        if shared
            .as_ref()
            .is_some_and(|b| b.width() != width.get() || b.height() != height.get())
        {
            shared = None;
        }

        let mut age = 1u8;
        let pixels = shared.get_or_insert_with(|| {
            age = 0;
            SharedPixelBuffer::new(width.get(), height.get())
        });

        let bytes = bytemuck::cast_slice_mut(pixels.make_mut_slice());
        render_callback(width, height, skia_safe::ColorType::RGBA8888, age, bytes)?;

        *self.pixels.borrow_mut() = shared;
        Ok(())
    }
}

struct SkiaHeadlessWindow {
    window: slint::Window,
    renderer: SkiaRenderer,
    needs_redraw: Cell<bool>,
    size: Cell<PhysicalSize>,
    render_buffer: Rc<HeadlessRenderBuffer>,
}

impl SkiaHeadlessWindow {
    fn new() -> Rc<Self> {
        let render_buffer = Rc::new(HeadlessRenderBuffer::default());
        let renderer = SkiaRenderer::new_with_surface(
            &SkiaSharedContext::default(),
            Box::new(SoftwareSurface::from(render_buffer.clone())),
        );
        Rc::new_cyclic(|w: &Weak<Self>| Self {
            window: slint::Window::new(w.clone()),
            renderer,
            needs_redraw: Default::default(),
            size: Default::default(),
            render_buffer,
        })
    }

    /// PPM (P6) bytes of the last rendered frame, for a before/after pixel comparison.
    fn snapshot_ppm(&self) -> Option<Vec<u8>> {
        let pixels = self.render_buffer.pixels.borrow();
        let pixels = pixels.as_ref()?;
        let mut ppm = format!("P6\n{} {}\n255\n", pixels.width(), pixels.height()).into_bytes();
        ppm.extend(pixels.as_bytes().chunks_exact(4).flat_map(|rgba| [rgba[0], rgba[1], rgba[2]]));
        Some(ppm)
    }

    /// Renders if a redraw was requested since the last call, timing only the `render()` call
    /// itself (not the property writes/layout that led up to it).
    fn draw_if_needed(&self) -> Option<Duration> {
        if self.needs_redraw.replace(false) {
            let start = Instant::now();
            let _ = self.renderer.render().unwrap();
            Some(start.elapsed())
        } else {
            None
        }
    }
}

impl WindowAdapter for SkiaHeadlessWindow {
    fn window(&self) -> &slint::Window {
        &self.window
    }

    fn size(&self) -> PhysicalSize {
        self.size.get()
    }

    fn renderer(&self) -> &dyn slint::platform::Renderer {
        &self.renderer
    }

    fn set_size(&self, size: slint::WindowSize) {
        self.size.set(size.to_physical(1.));
        self.window
            .dispatch_event(slint::platform::WindowEvent::Resized { size: size.to_logical(1.) })
    }

    fn request_redraw(&self) {
        self.needs_redraw.set(true);
    }
}

struct SkiaHeadlessPlatform {
    window: Rc<SkiaHeadlessWindow>,
}

impl Platform for SkiaHeadlessPlatform {
    fn create_window_adapter(&self) -> Result<Rc<dyn WindowAdapter>, PlatformError> {
        Ok(self.window.clone())
    }
}

/// Headless Skia benchmark: same content/scroll/animate logic `bench::run` uses for `scroll` and
/// `rects`, but rendered through real Skia (software surface) instead of the software renderer -
/// see the module doc for why this needs its own platform/window rather than reusing `--frames`.
pub fn run(options: &BenchOptions) -> Result<(), PlatformError> {
    let window = SkiaHeadlessWindow::new();
    slint::platform::set_platform(Box::new(SkiaHeadlessPlatform { window: window.clone() }))
        .map_err(|error| format!("cannot install the skia-bench platform: {error}"))?;
    window.set_size(slint::WindowSize::Physical(PhysicalSize::new(options.width, options.height)));

    let app = MainWindow::new()?;
    let records = crate::logic::wire_up(&app);
    app.set_mode(options.mode);
    app.set_probe_opacity(options.probe_opacity);
    app.set_probe_animate(options.probe_animate);
    app.set_probe_count(options.probe_count);
    app.global::<Settings>().set_row_style(options.row_style);

    match options.mode {
        PerfMode::List | PerfMode::Scroll => app.global::<Logic>().invoke_add_records(options.count),
        PerfMode::Rects => {
            app.set_rect_count(options.count);
            app.set_rect_opacity(options.item_opacity);
            app.set_rect_animate(options.animate);
            app.set_rect_animate_color(options.animate_color);
            app.set_rect_animate_count(options.animate_count);
            app.set_rect_use_path(options.use_path);
        }
        other => return Err(format!("--skia-bench does not support --mode {other:?} yet").into()),
    }
    let _ = &records;

    app.show()?;

    let mut render_times = Vec::with_capacity(options.frames);
    let mut allocations_per_frame = Vec::with_capacity(options.frames);

    for frame in 0..options.frames {
        let allocations_before = alloc_stats::allocations();

        app.set_scroll_y(-(frame as f32) * options.scroll_step);
        slint::platform::update_timers_and_animations();
        window.request_redraw();

        if let Some(duration) = window.draw_if_needed() {
            render_times.push(duration);
            allocations_per_frame.push(alloc_stats::allocations() - allocations_before);
        }
    }

    let skip = render_times.len() / 10;
    let later = &render_times[skip.min(render_times.len().saturating_sub(1))..];
    let later_allocs = &allocations_per_frame[skip.min(allocations_per_frame.len().saturating_sub(1))..];

    let mut sorted: Vec<Duration> = later.to_vec();
    sorted.sort();
    let median = sorted.get(sorted.len() / 2).copied().unwrap_or_default();
    let mean_allocs = if later_allocs.is_empty() {
        0
    } else {
        later_allocs.iter().sum::<u64>() / later_allocs.len() as u64
    };

    println!("frames rendered: {}", render_times.len());
    println!(
        "later frames median: {:.3}ms   allocations/frame: {mean_allocs}",
        median.as_secs_f64() * 1000.0
    );

    if let Some(path) = &options.snapshot {
        match window.snapshot_ppm() {
            Some(ppm) => match std::fs::write(path, ppm) {
                Ok(()) => println!("snapshot written to {path}"),
                Err(error) => eprintln!("no snapshot: {error}"),
            },
            None => eprintln!("no snapshot: nothing rendered yet"),
        }
    }

    Ok(())
}
