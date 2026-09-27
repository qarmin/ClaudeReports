// Standalone bench for the candidate 100 follow-up: `ItemRc::is_visible_or_clipped_by_flickable`
// (internal/core/item_tree.rs) calling `absolute_clip_rect_and_geometry` redundantly while
// deciding whether a not-visible item is only hidden by a Flickable ancestor.
//
// `count` copies of `HiddenChain` (a TextInput nested 15 `clip: true` levels deep, itself
// `visible: false`) sit before one always-visible `tail` TextInput. A single Tab press with
// nothing focused walks `move_focus`'s forward chain (internal/core/window.rs), calling
// `is_visible_or_clipped_by_flickable()` on every `HiddenChain` leaf before reaching `tail`.
//
// No display needed: the same headless MinimalSoftwareWindow platform perf_test/other bench
// candidates in this workspace use.

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
    let count: i32 = args.next().and_then(|s| s.parse().ok()).unwrap_or(500);
    let iters: usize = args.next().and_then(|s| s.parse().ok()).unwrap_or(20);

    let window = MinimalSoftwareWindow::new(Default::default());
    slint::platform::set_platform(Box::new(BenchPlatform { window: window.clone() }))
        .map_err(|e| format!("cannot install platform: {e}"))?;
    window.set_size(PhysicalSize::new(200, 200));

    let app = MainWindow::new()?;
    app.set_count(count);
    app.show()?;

    let mut times = Vec::with_capacity(iters);
    for i in 0..iters {
        let t0 = Instant::now();
        app.window().dispatch_event(slint::platform::WindowEvent::KeyPressed {
            text: slint::platform::Key::Tab.into(),
        });
        times.push(t0.elapsed());
        if i % 2 == 0 {
            assert!(
                app.get_tail_has_focus(),
                "count={count}: Tab press #{i} (odd->even landing) must focus `tail` after \
                 walking past all {count} hidden leaves"
            );
        }
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
    println!("tail_has_focus_after_last = {}", app.get_tail_has_focus());
    println!("min={:?} p50={:?} mean={:?} p95={:?} max={:?}", min, p50, mean, p95, max);
    Ok(())
}
