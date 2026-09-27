//! Windowed counterpart of the headless benchmark: renders a fixed number of frames
//! with the real backend (femtovg by default), then quits. This is the mode to profile
//! the GPU renderers with, since they need a window and a GL context.

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::{Duration, Instant};


use slint::{ComponentHandle, RenderingState, Timer, TimerMode};

use crate::MainWindow;
use crate::alloc_stats;

thread_local! {
    /// Recorded once the window exists, so the report says what was actually measured.
    static WINDOW_INFO: RefCell<Option<String>> = const { RefCell::new(None) };
}

pub struct FrameRunOptions {
    pub frames: usize,
    /// Physical size to force on the window: the window manager may otherwise open or maximize it
    /// differently between runs, which changes how many rows are visible and makes two runs
    /// incomparable (observed as a 2x difference in allocations per frame).
    pub size: (u32, u32),
    pub scroll_step: f32,
    /// Sample one in N allocations of the last frame.
    #[cfg_attr(not(feature = "alloc-profile"), allow(dead_code))]
    pub alloc_profile: Option<u64>,
    /// Where to write a PPM snapshot of the last frame, for before/after comparisons.
    pub snapshot: Option<String>,
}

pub fn install(app: &MainWindow, options: &FrameRunOptions) -> Result<(), slint::PlatformError> {
    // Everything from here to the first rendered frame: showing the window, the layout pass, and
    // the first render. This is the windowed analogue of the headless `show + layout` line, and it
    // is where a scene that shapes a whole document up front spends its seconds -- the per-frame
    // timings below start at `BeforeRendering` and so miss all of it.
    let install_start = Instant::now();
    let time_to_first_frame = Rc::new(Cell::new(None::<Duration>));
    let frames = options.frames;
    let scroll_step = options.scroll_step;
    #[cfg(feature = "alloc-profile")]
    let alloc_profile = options.alloc_profile;

    let started = Cell::new(Instant::now());
    let allocations = Cell::new(0u64);
    // Paragraphs shaped, read as a running total: a large document shapes on demand, so this is
    // the only column that says how much text work a frame really did. The FemtoVG renderer has no
    // reachable `TextLayoutCache`, hence the free function.
    let shapes = Cell::new(0u64);
    let durations = Rc::new(RefCell::new(Vec::<(Duration, u64, u64)>::with_capacity(frames)));

    app.window().set_size(slint::PhysicalSize::new(options.size.0, options.size.1));
    let snapshot = options.snapshot.clone();
    let weak_for_snapshot = app.as_weak();
    let done = Rc::new(Cell::new(false));
    let notifier = app.window().set_rendering_notifier({
        let durations = durations.clone();
        let done = done.clone();
        let time_to_first_frame = time_to_first_frame.clone();
        move |state, _| match state {
            RenderingState::BeforeRendering => {
                #[cfg(feature = "alloc-profile")]
                if let Some(rate) = alloc_profile
                    && durations.borrow().len() + 1 == frames
                {
                    crate::alloc_profile::start(rate);
                }
                started.set(Instant::now());
                allocations.set(alloc_stats::allocations());
                shapes.set(0);
            }
            RenderingState::AfterRendering => {
                if done.get() {
                    return;
                }
                if time_to_first_frame.get().is_none() {
                    time_to_first_frame.set(Some(install_start.elapsed()));
                }
                let frame = (
                    started.get().elapsed(),
                    alloc_stats::allocations() - allocations.get(),
                    0 - shapes.get(),
                );
                durations.borrow_mut().push(frame);
                if durations.borrow().len() >= frames {
                    // Just mark it here; `take_snapshot()` below renders again synchronously,
                    // and this callback still holds `SkiaRenderer`'s rendering-notifier RefCell
                    // borrowed, so calling it from here panics with "RefCell already mutably
                    // borrowed" (Skia only - FemtoVG's/software's notifier isn't reentered this
                    // way). Do the actual finalization from a timer instead, once this call
                    // stack has unwound.
                    done.set(true);
                }
            }
            _ => {}
        }
    });
    // The software renderer has no rendering notifier - use --bench for that one.
    notifier.map_err(|error| {
        format!("--frames needs a renderer with a rendering notifier ({error}); try --bench")
    })?;

    {
        let durations = durations.clone();
        let finish_timer = Timer::default();
        finish_timer.start(TimerMode::Repeated, Duration::from_millis(1), {
            let done = done.clone();
            move || {
                if !done.get() {
                    return;
                }
                done.set(false);
                #[cfg(feature = "alloc-profile")]
                if alloc_profile.is_some() {
                    crate::alloc_profile::stop();
                }
                if let Some(path) = &snapshot {
                    match weak_for_snapshot.upgrade().map(|app| app.window().take_snapshot()) {
                        Some(Ok(buffer)) => {
                            write_ppm(path, &buffer)
                                .unwrap_or_else(|error| eprintln!("cannot write {path}: {error}"));
                            println!("snapshot written to {path}");
                        }
                        other => eprintln!("no snapshot: {other:?}"),
                    }
                }
                report(&durations.borrow(), time_to_first_frame.get());
                #[cfg(feature = "alloc-profile")]
                if alloc_profile.is_some() {
                    crate::alloc_profile::report_by_site(20);
                    crate::alloc_profile::report(6, 8);
                }
                slint::quit_event_loop().unwrap();
            }
        });
        Box::leak(Box::new(finish_timer));
    }

    {
        let window = app.window();
        let size = window.size();
        WINDOW_INFO.with(|info| {
            *info.borrow_mut() = Some(format!(
                "window         {}x{} physical, scale factor {}",
                size.width,
                size.height,
                window.scale_factor()
            ))
        });
    }

    // Properties must not be written from the rendering notifier, so the scrolling is driven
    // from a timer that also keeps asking for the next frame. The offset follows the number of
    // frames already rendered, not the number of timer ticks: otherwise a faster or slower build
    // scrolls a different distance per frame and snapshots of the same run are not comparable.
    let timer = Timer::default();
    timer.start(TimerMode::Repeated, Duration::from_millis(1), {
        let weak = app.as_weak();
        let durations = durations.clone();
        let done = done.clone();
        move || {
            // Stop asking for redraws once the counted frames are in: an extra redraw here
            // races the finish timer's own `take_snapshot()` call below against the window's
            // (double-buffered) swapchain and makes which of two buffers gets read back
            // flaky, independent of anything the rendered content actually changed.
            if done.get() {
                return;
            }
            let Some(app) = weak.upgrade() else { return };
            let rendered = durations.borrow().len() as f32;
            app.set_scroll_y(-rendered * scroll_step);
            app.window().request_redraw();
        }
    });
    // The timer must outlive this function; it stops when the event loop ends.
    Box::leak(Box::new(timer));

    Ok(())
}

fn report(frames: &[(Duration, u64, u64)], time_to_first_frame: Option<Duration>) {
    println!();
    if let Some(window) = WINDOW_INFO.with(|info| info.borrow().clone()) {
        println!("{window}");
    }
    if let Some(elapsed) = time_to_first_frame {
        println!("show + layout  {elapsed:.2?} (up to the first rendered frame)");
    }
    // Worth stating, because a `0` in the paragraph-shapes column reads like "no text work" when
    // it means "none of it happened while rendering": the window between `BeforeRendering` and
    // `AfterRendering` excludes the layout pass, which is where `list` and `scroll` do all of
    // their shaping and where `text` does 2 of its 13. The headless `--bench` report charges a
    // frame from before the property writes instead, so its counts are layout + render.
    println!("per-frame numbers below cover rendering only, not the layout pass before it");
    println!("{:>5}  {:>12}  {:>14}  {:>17}", "frame", "time", "allocations", "paragraph shapes");
    for (index, (duration, allocations, shapes)) in frames.iter().enumerate() {
        println!(
            "{index:>5}  {:>12}  {allocations:>14}  {shapes:>17}",
            format!("{duration:.2?}")
        );
    }

    // The headline numbers of candidate 03 are the first frame (which includes the layout pass
    // that shapes the document) and what stays resident afterwards, so the windowed report has to
    // carry both - the headless one already did.
    if let Some(resident) = alloc_stats::resident_bytes() {
        println!();
        println!(
            "resident memory {:.1} MB after the last frame",
            resident as f64 / (1024.0 * 1024.0)
        );
    }

    if let Some((first, rest)) = frames.split_first() {
        println!();
        println!(
            "first frame    {:.2?} ({} allocations, {} paragraph shapes)",
            first.0, first.1, first.2
        );
        if !rest.is_empty() {
            // Median, not mean. On a real compositor a single frame can render before the window
            // is fully mapped, or stall on it: one such frame out of ten moved a `text` row from
            // 2.42 ms to 8.80 ms and its allocations from 1317 to 1499, which is enough to invent
            // a regression that isn't there. The spread line below makes those frames visible
            // rather than letting them average in.
            let time = crate::median(rest.iter().map(|frame| frame.0));
            let allocations = crate::median(rest.iter().map(|frame| frame.1));
            let shapes = crate::median(rest.iter().map(|frame| frame.2));
            println!(
                "later frames   {time:.2?} median, {:.1} fps ({allocations} allocations, \
{shapes} paragraph shapes per frame)",
                1.0 / time.as_secs_f64(),
            );
            let slowest = rest.iter().map(|frame| frame.0).max().unwrap_or_default();
            let fastest = rest.iter().map(|frame| frame.0).min().unwrap_or_default();
            let most = rest.iter().map(|frame| frame.1).max().unwrap_or_default();
            let least = rest.iter().map(|frame| frame.1).min().unwrap_or_default();
            println!(
                "               spread {fastest:.2?}..{slowest:.2?} over {} frames, \
allocations {least}..{most}",
                rest.len()
            );
        }
    }
}

/// Minimal PPM writer: enough for `cmp`/ImageMagick-free before/after comparisons.
fn write_ppm(
    path: &str,
    buffer: &slint::SharedPixelBuffer<slint::Rgba8Pixel>,
) -> std::io::Result<()> {
    use std::io::Write;
    let mut out = std::io::BufWriter::new(std::fs::File::create(path)?);
    write!(out, "P6\n{} {}\n255\n", buffer.width(), buffer.height())?;
    for pixel in buffer.as_slice() {
        out.write_all(&[pixel.r, pixel.g, pixel.b])?;
    }
    out.flush()
}
