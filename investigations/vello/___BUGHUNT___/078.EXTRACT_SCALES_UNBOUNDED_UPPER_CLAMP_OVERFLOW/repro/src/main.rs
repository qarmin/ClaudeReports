//! Real end-to-end reproduction for candidate 078
//! (EXTRACT_SCALES_UNBOUNDED_UPPER_CLAMP_OVERFLOW).
//!
//! Drives the actual public `vello_cpu::RenderContext` rendering pipeline - the same
//! `push_filter_layer`/`fill_rect`/`render` sequence a real application uses (see
//! `vello_cpu/examples/basic.rs`) - with a pathological but entirely legitimate affine
//! transform in place when a Gaussian blur filter layer is pushed.
//!
//! `vello_common::util::extract_scales` (used by `gaussian_blur::transform_blur_params` to
//! scale a blur's standard deviation by the current transform) downcasts the transform's
//! coefficients to `f32` and squares them. A transform with one very large scale factor (a
//! non-uniform scale is enough - a uniform scale gets masked by a different, already-known
//! NaN-absorption quirk, see candidate 019) overflows that squaring to `+inf`, and only the
//! *lower* bound is clamped (`.max(1e-6)`) - there is no upper clamp. The resulting `+inf`
//! scale feeds into `GaussianBlur::new`/`plan_decimated_blur`'s decimation loop
//! (`remaining_variance = (remaining_variance - 1.5) * 0.25`), which never converges below
//! its `4.0` exit threshold for infinite input and spins forever.
//!
//! Because a genuine hang can't be "waited out" by a test harness, this repro runs the render
//! call on a background thread and applies its own bounded wall-clock timeout to observe
//! whether it returns at all - it does not fabricate the hang, it only bounds how long this
//! process waits to report one.

use std::thread;
use std::time::{Duration, Instant};

use vello_common::filter_effects::{Filter, FilterFunction};
use vello_cpu::kurbo::{Affine, Rect};
use vello_cpu::{Pixmap, RenderContext, Resources};
use vello_cpu::color::palette::css::BLUE;

const HANG_TIMEOUT: Duration = Duration::from_secs(10);

fn main() {
    let start = Instant::now();
    let handle = thread::spawn(|| {
        let mut ctx = RenderContext::new(100, 100);
        let mut resources = Resources::new();

        // A real, plausible non-uniform transform: a huge horizontal scale, ordinary
        // vertical scale. This is exactly the shape `Affine::scale_non_uniform(x, y)`
        // parses a real SVG `transform="scale(x y)"` attribute into (see
        // `vello_common::pico_svg`) - nothing here is a fabricated internal state, it is
        // an ordinary (if extreme) caller-supplied transform.
        //
        // `radius` (the blur's std_deviation) is deliberately tiny so that the
        // *unrelated* padding computation in `FilterData::new` (which uses kurbo's own
        // f64 `transform_rect_bbox`, not `extract_scales`, and is already covered by a
        // separate, previously-confirmed candidate: 052) stays well under u16::MAX even
        // though the transform's own scale factor is extreme enough to overflow
        // `extract_scales`'s f32 computation (~1.8e19) - this isolates the specific bug
        // this candidate is about.
        ctx.set_transform(Affine::new([1e19, 0.0, 0.0, 1.0, 0.0, 0.0]));

        let filter = Filter::from_function(FilterFunction::Blur { radius: 1e-16 });
        ctx.push_filter_layer(filter);

        // Reset to identity so the actual drawn content lands squarely on-canvas - only
        // the filter's own *captured* transform above (captured at push time) is
        // pathological.
        ctx.set_transform(Affine::IDENTITY);
        ctx.set_paint(BLUE);
        ctx.fill_rect(&Rect::new(20.0, 20.0, 60.0, 60.0));
        ctx.pop_layer();

        ctx.flush();

        let mut pixmap = Pixmap::new(100, 100);
        ctx.render(&mut pixmap, &mut resources);
    });

    let deadline = Instant::now() + HANG_TIMEOUT;
    loop {
        if handle.is_finished() {
            handle.join().expect("render thread panicked");
            println!(
                "OK: RenderContext::render() returned after {:?} (bug not present / fixed)",
                start.elapsed()
            );
            return;
        }
        if Instant::now() > deadline {
            println!(
                "HANG: RenderContext::render() did not return within {:?} (bug reproduced)",
                HANG_TIMEOUT
            );
            std::process::exit(42);
        }
        thread::sleep(Duration::from_millis(50));
    }
}
