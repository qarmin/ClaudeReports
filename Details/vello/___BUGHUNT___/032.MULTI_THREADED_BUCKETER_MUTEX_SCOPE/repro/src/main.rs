// Candidate 032: does a real second caller path exist that could contend for
// `MultiThreadedDispatcher::bucketer`'s Mutex while `rasterize_with` (sparse_strips/vello_cpu/
// src/dispatch/multi_threaded.rs) holds it across the whole parallel rasterization region?
//
// The only way that would matter in practice is if two OS threads could concurrently hold a
// shared reference into the same `RenderContext` (and thus the same `MultiThreadedDispatcher` /
// `Mutex<CommandBucketer>`), with one of them inside `rasterize_with` while another tries to lock
// `bucketer` from somewhere else. Rust's type system makes that impossible unless
// `RenderContext: Sync` (sharing `&RenderContext` across a thread boundary, e.g. via
// `Arc<RenderContext>` + `thread::spawn`, or `std::thread::scope`, requires `Sync`).
//
// This proves, using the real public API type (`vello_cpu::RenderContext`) - not a stand-in - that
// it is NOT `Sync`. That rules out the suspected "second caller serializes on the same lock"
// scenario entirely: there can be no second caller in the first place, because no second thread
// can ever observe the same `RenderContext`/dispatcher instance concurrently through safe Rust.
use static_assertions::assert_not_impl_any;

assert_not_impl_any!(vello_cpu::RenderContext: Sync);

fn main() {
    println!(
        "confirmed: vello_cpu::RenderContext is NOT Sync (assert_not_impl_any! compiled clean) \
         -> no two threads can ever share a `&RenderContext`/`&MultiThreadedDispatcher`, so the \
         suspected second-caller contention path on `bucketer` cannot exist through safe Rust."
    );
}
