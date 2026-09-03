// Standalone bench for candidate 100 (`ItemRc::absolute_clip_rect_and_geometry`'s redundant
// recursive recomputation of nested clipping-ancestor chains, internal/core/item_tree.rs:496-514).
//
// `process_mouse_input` (internal/core/window.rs) calls `is_visible()` on the currently focused
// item at the very top of *every* dispatched mouse event, unconditionally - and `is_visible()`
// calls `absolute_clip_rect_and_geometry()`. With the pre-patch recursive definition this costs
// O(2^depth) for `depth` nested `clip: true` ancestors above the focused item, because every
// clipping ancestor's own geometry is obtained by recursively calling the same function on it
// (which redundantly re-walks the overlapping sub-chain above it, and so on).
//
// This bench focuses a `TextInput` nested `depth` levels deep inside `clip: true` Rectangles
// (`ui/main.slint` unrolls one `ClipChain<depth>` component per depth - Slint has no recursion),
// then times `iters` consecutive `MouseMoved` events, each of which re-triggers the
// `is_visible()` check on the focused item. No display needed: same headless
// `MinimalSoftwareWindow` platform perf_test's `--bench` uses.
//
// Correctness check embedded directly: the leaf must still report `has_focus` after every
// dispatched event on both the pristine and patched binary - if `is_visible()` ever
// miscomputed `false` for the (always fully visible) focused item, `process_mouse_input` would
// drop the focus via `take_focus_item`, which is exactly the kind of behavior change a
// timing-only patch must not introduce.

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
    let depth: i32 = args.next().and_then(|s| s.parse().ok()).unwrap_or(18);
    let iters: usize = args.next().and_then(|s| s.parse().ok()).unwrap_or(30);

    let window = MinimalSoftwareWindow::new(Default::default());
    slint::platform::set_platform(Box::new(BenchPlatform { window: window.clone() }))
        .map_err(|e| format!("cannot install platform: {e}"))?;
    window.set_size(PhysicalSize::new(200, 200));

    let app = MainWindow::new()?;
    app.set_depth(depth);
    app.show()?;

    // One Tab press: the leaf TextInput is the only focusable item in the scene, so this gives
    // it focus regardless of `depth`.
    app.window().dispatch_event(slint::platform::WindowEvent::KeyPressed {
        text: slint::platform::Key::Tab.into(),
    });
    assert!(
        app.get_leaf_has_focus(),
        "depth={depth}: Tab must focus the leaf TextInput nested {depth} clip levels deep"
    );

    let mut times = Vec::with_capacity(iters);
    for i in 0..iters {
        let x = 10.0 + (i % 5) as f32;
        let y = 10.0 + (i % 3) as f32;
        let t0 = Instant::now();
        app.window().dispatch_event(slint::platform::WindowEvent::PointerMoved {
            position: slint::LogicalPosition::new(x, y),
        });
        times.push(t0.elapsed());
    }

    assert!(
        app.get_leaf_has_focus(),
        "depth={depth}: leaf must still have focus after {iters} mouse-move events - a \
         mis-detected `is_visible() == false` would have dropped it via take_focus_item"
    );

    let warmup = (iters / 10).max(1).min(iters.saturating_sub(1));
    let mut stable: Vec<Duration> = times[warmup..].to_vec();
    stable.sort_unstable();
    let n = stable.len();
    let min = stable[0];
    let p50 = stable[n / 2];
    let mean = stable.iter().sum::<Duration>() / n as u32;
    let p95 = stable[(n * 95) / 100];
    let max = stable[n - 1];

    println!("depth      = {depth}");
    println!("iters      = {iters} (warmup discarded = {warmup})");
    println!("leaf_has_focus_after = {}", app.get_leaf_has_focus());
    println!("min={:?} p50={:?} mean={:?} p95={:?} max={:?}", min, p50, mean, p95, max);
    Ok(())
}
