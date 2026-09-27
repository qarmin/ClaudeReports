//! What does having many timers cost per tick?
//!
//! `Timers::maybe_activate_timers` takes the active-timer `Vec`, partitions it and rebuilds it on
//! every call, and `update_timers_and_animations()` is called once per frame. That is fine for a
//! handful of timers; this measures the slope.

use std::rc::Rc;
use std::time::{Duration, Instant};

use slint::platform::{Platform, PlatformError, WindowAdapter};
use slint::platform::software_renderer::MinimalSoftwareWindow;

struct P(Rc<MinimalSoftwareWindow>);
impl Platform for P {
    fn create_window_adapter(&self) -> Result<Rc<dyn WindowAdapter>, PlatformError> {
        Ok(self.0.clone())
    }
}

fn measure(count: usize, ticks: usize) -> f64 {
    let timers: Vec<slint::Timer> = (0..count)
        .map(|i| {
            let t = slint::Timer::default();
            // Long intervals: nothing should actually fire, so this measures the walk, not the work.
            t.start(slint::TimerMode::Repeated, Duration::from_secs(3600 + i as u64), || {});
            t
        })
        .collect();

    let start = Instant::now();
    for _ in 0..ticks {
        slint::platform::update_timers_and_animations();
    }
    let elapsed = start.elapsed().as_secs_f64() * 1e6 / ticks as f64;
    drop(timers);
    elapsed
}

fn main() {
    let window = MinimalSoftwareWindow::new(Default::default());
    slint::platform::set_platform(Box::new(P(window))).expect("cannot install platform");

    println!("{:>8}  {:>12}  {:>14}", "timers", "us per tick", "ns per timer");
    for count in [0, 1, 10, 100, 1_000, 10_000] {
        let us = measure(count, 2_000);
        let per = if count > 0 { us * 1000.0 / count as f64 } else { 0.0 };
        println!("{count:>8}  {us:>12.3}  {per:>14.1}");
    }
}
