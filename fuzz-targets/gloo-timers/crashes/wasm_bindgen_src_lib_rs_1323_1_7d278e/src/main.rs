use gloo_timers::future::IntervalStream;

fn main() {
    let _ = IntervalStream::new(0u32);
}
