// Standalone bench for candidate 055 (Window::move_focus's O(N^2) cycle detection).
//
// Two modes:
//   focus_bench <count> <iters>            - worst case: nothing in the scene is focusable, so
//                                             every Tab press walks the whole item tree before
//                                             giving up. Times `iters` consecutive Tab presses.
//   focus_bench <count> <iters> --check     - correctness check: one FocusScope is added after
//                                             the N rectangles. Verifies a single Tab press finds
//                                             it (scope_has_focus becomes true) both before and
//                                             after the patch - the optimization must not change
//                                             *which* item focus lands on, only how fast the
//                                             search that finds it (or gives up) runs.
//
// No display needed: uses the same headless MinimalSoftwareWindow platform perf_test's --bench
// uses.

slint::include_modules!();

use slint::platform::software_renderer::MinimalSoftwareWindow;
use slint::platform::{Platform, PlatformError, WindowAdapter};
use slint::{ComponentHandle, PhysicalSize};
use std::rc::Rc;
use std::time::{Duration, Instant};

struct BenchPlatform {
    window: Rc<MinimalSoftwareWindow>,
}

impl Platform for BenchPlatform {
    fn create_window_adapter(&self) -> Result<Rc<dyn WindowAdapter>, PlatformError> {
        Ok(self.window.clone())
    }
}

fn main() -> Result<(), PlatformError> {
    let mut args = std::env::args().skip(1);
    let count: i32 = args.next().and_then(|s| s.parse().ok()).unwrap_or(2000);
    let iters: usize = args.next().and_then(|s| s.parse().ok()).unwrap_or(20);
    let check = args.next().as_deref() == Some("--check");

    let window = MinimalSoftwareWindow::new(Default::default());
    slint::platform::set_platform(Box::new(BenchPlatform { window: window.clone() }))
        .map_err(|e| format!("cannot install platform: {e}"))?;
    window.set_size(PhysicalSize::new(400, 400));

    let app = MainWindow::new()?;
    app.set_count(count);
    app.set_focusable(check);
    app.show()?;

    if check {
        assert!(!app.get_scope_has_focus(), "scope must not start focused");
        app.window().dispatch_event(slint::platform::WindowEvent::KeyPressed {
            text: slint::platform::Key::Tab.into(),
        });
        assert!(
            app.get_scope_has_focus(),
            "Tab must move focus onto the FocusScope placed after the {count} rectangles"
        );
        println!("check OK: count={count}, FocusScope gained focus after one Tab press");
        return Ok(());
    }

    let mut times = Vec::with_capacity(iters);
    for _ in 0..iters {
        let t0 = Instant::now();
        app.window().dispatch_event(slint::platform::WindowEvent::KeyPressed {
            text: slint::platform::Key::Tab.into(),
        });
        times.push(t0.elapsed());
    }

    let warmup = (iters / 10).max(1).min(iters.saturating_sub(1));
    let mut stable: Vec<Duration> = times[warmup..].to_vec();
    stable.sort_unstable();
    let n = stable.len();
    let min = stable[0];
    let p50 = stable[n / 2];
    let mean = stable.iter().sum::<Duration>() / n as u32;
    let p95 = stable[(n * 95) / 100];
    let max = stable[n - 1];

    println!("count      = {count}");
    println!("iters      = {iters} (warmup discarded = {warmup})");
    println!(
        "min={:?} p50={:?} mean={:?} p95={:?} max={:?}",
        min, p50, mean, p95, max
    );
    Ok(())
}
