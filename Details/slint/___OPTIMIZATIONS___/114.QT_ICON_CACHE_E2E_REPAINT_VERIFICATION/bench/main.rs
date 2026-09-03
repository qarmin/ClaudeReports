// End-to-end probe for candidate 114 (follow-up to candidate 113): drives a real, live
// QApplication paint cycle (offscreen QPA) through a compiled `Button { icon: ... }`
// (`slint_build::CompilerConfiguration::with_style("qt")`), forcing repeated repaints via
// alternating pointer hover/press and keyboard-focus changes on a *second*, icon-less button -
// i.e. exactly the "something else about the window changed" scenario GitHub #7511 describes,
// where the icon and its size never change across the whole run.
//
// `i_slint_backend_qt::bench_probe_counters()` (temporary, `bench-probe`-feature-gated, see
// bench_probe.patch) reports two counts read after the run:
//   - CALLS   = how many times `NativeButton::actual_icon()` was invoked (from both `render()`
//               and `layout_info()`) for the icon-bearing button - i.e. how often a real repaint
//               actually re-entered this code path.
//   - MISSES  = how many times `image_to_pixmap` (the expensive SVG rasterization) actually ran.
//
// Baseline (no cache): CALLS == MISSES, every invocation re-rasterizes.
// Patched (candidate 113's fix): CALLS grows with every repaint; MISSES stays at the number of
// distinct (icon, icon_size) pairs actually used (1, here) once the icon has been rasterized once.
//
// Requires QT_QPA_PLATFORM=offscreen and a real Qt on PATH/LD_LIBRARY_PATH (see the candidate's
// own justfile). Usage: qt_icon_e2e_probe <ticks>

slint::include_modules!();

use slint::platform::{Key, WindowEvent};
use slint::{LogicalPosition, TimerMode};
use std::time::Duration;

fn main() {
    let ticks: u32 = std::env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(200);

    let app = AppWindow::new().expect("failed to instantiate AppWindow (is Qt initialized?)");
    app.show().expect("show failed");

    let hover_pos = LogicalPosition::new(60.0, 40.0); // inside icon-button
    let away_pos = LogicalPosition::new(280.0, 190.0); // far corner, outside both buttons

    let zero = i_slint_backend_qt::bench_probe_counters();
    assert_eq!(zero, (0, 0, 0, 0), "counters not zero before the run");

    let tick_count = std::rc::Rc::new(std::cell::Cell::new(0u32));
    let app_weak = app.as_weak();
    let timer = slint::Timer::default();
    let tick_count_for_closure = tick_count.clone();
    timer.start(TimerMode::Repeated, Duration::from_millis(4), move || {
        let Some(app) = app_weak.upgrade() else { return };
        let window = app.window();
        let i = tick_count_for_closure.get();
        tick_count_for_closure.set(i + 1);

        if i >= ticks {
            slint::quit_event_loop().ok();
            return;
        }

        if std::env::var("NOOP").is_ok() {
            return;
        }

        // Hover on/off the icon button every tick.
        if i % 2 == 0 {
            window.dispatch_event(WindowEvent::PointerMoved { position: hover_pos });
        } else {
            window.dispatch_event(WindowEvent::PointerMoved { position: away_pos });
        }

        // Press+release (a real click) on the icon button every 3rd tick.
        if i % 3 == 0 {
            window.dispatch_event(WindowEvent::PointerMoved { position: hover_pos });
            window.dispatch_event(WindowEvent::PointerPressed {
                position: hover_pos,
                button: slint::platform::PointerEventButton::Left,
            });
            window.dispatch_event(WindowEvent::PointerReleased {
                position: hover_pos,
                button: slint::platform::PointerEventButton::Left,
            });
        }

        // Move keyboard focus away from / back onto the icon button every 5th tick (Tab
        // toggles between the two buttons in this two-widget scene).
        if i % 5 == 0 {
            window.dispatch_event(WindowEvent::KeyPressed { text: Key::Tab.into() });
            window.dispatch_event(WindowEvent::KeyReleased { text: Key::Tab.into() });
        }
    });

    let t0 = std::time::Instant::now();
    slint::run_event_loop().expect("event loop failed");
    let elapsed = t0.elapsed();

    let (calls_icon, misses_icon, calls_empty, misses_empty) =
        i_slint_backend_qt::bench_probe_counters();
    println!("ticks         = {ticks}");
    println!("elapsed       = {:8.3} ms", elapsed.as_secs_f64() * 1000.0);
    println!("icon-button:  calls = {calls_icon:5}  misses = {misses_icon:5}");
    println!("other-button: calls = {calls_empty:5}  misses = {misses_empty:5}");
}
