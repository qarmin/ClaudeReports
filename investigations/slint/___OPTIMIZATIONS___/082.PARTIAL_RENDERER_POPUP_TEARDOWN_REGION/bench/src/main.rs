//! Standalone bench for candidate 082.
//!
//! Question: after closing a `ChildWindow` popup, does the partial renderer redraw only the
//! popup's own small rect, or does it fall back to a full-screen refresh?
//!
//! Headless (`MinimalSoftwareWindow`, `RepaintBufferType::ReusedBuffer`), so it needs no display
//! and is deterministic. Reports two independent signals:
//!  - `PhysicalRegion::bounding_box_size()` of the frame right after `close()` - a direct,
//!    non-noisy measurement of how much screen area the renderer decided to redraw.
//!  - wall-clock time of that same frame's `render()` call, over many repeated open/close
//!    cycles (min/p50/mean/p95/max, ~10% warmup discarded), since redrawing more pixels costs
//!    more time on real hardware.

use std::rc::Rc;
use std::time::{Duration, Instant};

use slint::platform::software_renderer::{MinimalSoftwareWindow, RepaintBufferType};
use slint::platform::{Platform, PlatformError, WindowAdapter};
use slint::PhysicalSize;

slint::include_modules!();

struct BenchPlatform {
    window: Rc<MinimalSoftwareWindow>,
}

impl Platform for BenchPlatform {
    fn create_window_adapter(&self) -> Result<Rc<dyn WindowAdapter>, PlatformError> {
        Ok(self.window.clone())
    }
}

fn percentile(sorted: &[Duration], p: f64) -> Duration {
    let idx = ((sorted.len() - 1) as f64 * p).round() as usize;
    sorted[idx]
}

fn main() -> Result<(), PlatformError> {
    let args: Vec<String> = std::env::args().collect();
    let row_count: i32 =
        args.iter().position(|a| a == "--rows").and_then(|i| args.get(i + 1)).and_then(|s| s.parse().ok()).unwrap_or(10000);
    let cycles: usize =
        args.iter().position(|a| a == "--cycles").and_then(|i| args.get(i + 1)).and_then(|s| s.parse().ok()).unwrap_or(200);

    let width = 800u32;
    let height = 600u32;

    let window = MinimalSoftwareWindow::new(RepaintBufferType::ReusedBuffer);
    slint::platform::set_platform(Box::new(BenchPlatform { window: window.clone() }))
        .map_err(|error| format!("cannot install the benchmark platform: {error}"))?;
    window.set_size(PhysicalSize::new(width, height));

    let app = AppWindow::new()?;
    app.set_row_count(row_count);

    let mut buffer = vec![slint::Rgb8Pixel::default(); (width * height) as usize];

    // Frame 0: initial paint, populates the partial-renderer cache for every background item.
    window.draw_if_needed(|renderer| {
        renderer.render(&mut buffer, width as usize);
    });

    let mut close_frame_durations = Vec::with_capacity(cycles);
    let mut last_close_region = slint::PhysicalSize::default();
    let mut last_open_region = slint::PhysicalSize::default();

    for _ in 0..cycles {
        app.invoke_open_popup();
        // Render the "popup appeared" frame - not measured, just needed so the popup's cached
        // geometry exists before it's closed again below (mirroring real usage: a popup is
        // always shown for at least one frame before it closes).
        window.draw_if_needed(|renderer| {
            let region = renderer.render(&mut buffer, width as usize);
            last_open_region = region.bounding_box_size();
        });

        app.invoke_close_popup();

        let start = Instant::now();
        window.draw_if_needed(|renderer| {
            let region = renderer.render(&mut buffer, width as usize);
            last_close_region = region.bounding_box_size();
        });
        close_frame_durations.push(start.elapsed());
    }

    // Discard the first ~10% as warmup.
    let warmup = close_frame_durations.len() / 10;
    let mut timed: Vec<Duration> = close_frame_durations[warmup..].to_vec();
    timed.sort_unstable();

    let min = timed[0];
    let p50 = percentile(&timed, 0.50);
    let mean = timed.iter().sum::<Duration>() / timed.len() as u32;
    let p95 = percentile(&timed, 0.95);
    let max = *timed.last().unwrap();

    println!("rows={row_count} cycles={cycles}");
    println!(
        "open-frame dirty bounding box:  {}x{}",
        last_open_region.width, last_open_region.height
    );
    println!(
        "close-frame dirty bounding box: {}x{}  (full screen is {width}x{height})",
        last_close_region.width, last_close_region.height
    );
    println!(
        "close-frame render() time: min={min:?} p50={p50:?} mean={mean:?} p95={p95:?} max={max:?}"
    );

    // FNV-1a over the final frame's pixel buffer: proves the *content* is unaffected by this
    // patch (which only changes how much of the buffer gets redrawn, never what ends up in it).
    let mut hash: u64 = 0xcbf29ce484222325;
    for pixel in &buffer {
        for byte in [pixel.r, pixel.g, pixel.b] {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(0x100000001b3);
        }
    }
    println!("final frame pixel hash: {hash:016x}");

    Ok(())
}
