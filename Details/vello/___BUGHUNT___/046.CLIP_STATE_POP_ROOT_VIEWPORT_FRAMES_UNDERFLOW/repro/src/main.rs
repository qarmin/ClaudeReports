//! Candidate 046: does `vello_common::clip::ClipState::pop_root_viewport`'s unchecked
//! `self.frames.pop().expect("filter clip stack underflow")` (clip.rs:234) fire through any real
//! public entry point, independently of `CommandRecorder::pop_layer`'s own already-panicking
//! layer-stack check (record.rs:307, the underflow candidate 031 already confirmed)?
//!
//! Both `vello_cpu::RenderContext::pop_layer` and `vello_hybrid::Scene::pop_layer` only ever call
//! into `ViewportState::pop_root_viewport` (-> `ClipState::pop_root_viewport`) from inside the
//! `match self.recorder.pop_layer() { PoppedLayer::Filter => ... }` arm - and `recorder.pop_layer()`
//! itself panics first (`layer_stack.pop().unwrap()`) whenever there's no matching `push_layer`
//! call at all. Since every `push_layer` call that carries `filter_data` unconditionally does
//! `viewport.push_root_viewport(..)` *before* `recorder.push_layer(..)`, and both effects are only
//! ever undone together inside `pop_layer`, `frames` can never end up "behind" `layer_stack" through
//! this API: it either mirrors it 1:1, or (if a panic somehow interrupted a `push_layer` call
//! between the two pushes) has strictly *more* entries than warranted, never fewer.
//!
//! This program exercises three unbalanced-pop shapes (a lone extra pop, an extra pop after
//! correctly-balanced filter-layer nesting, and a pop with no push at all) against both
//! `vello_cpu::RenderContext` and `vello_hybrid::Scene`, wraps each in `catch_unwind`, and prints
//! which line panicked. If the suspected bug were independently reachable, at least one of these
//! would panic at `clip.rs:234` ("filter clip stack underflow") or the sibling
//! `viewport.rs:114` ("root viewport stack underflow") message instead of `record.rs:307`.

use std::panic::{self, AssertUnwindSafe};
use vello_common::filter_effects::{Filter, FilterFunction};

fn make_filter() -> Filter {
    Filter::from_function(FilterFunction::Blur { radius: 1.0 })
}

fn panic_message(payload: Box<dyn std::any::Any + Send>) -> String {
    payload
        .downcast_ref::<String>()
        .cloned()
        .or_else(|| payload.downcast_ref::<&str>().map(|s| s.to_string()))
        .unwrap_or_else(|| "<non-string panic payload>".to_string())
}

fn run_cpu_case(name: &str, f: impl FnOnce(&mut vello_cpu::RenderContext)) {
    let mut ctx = vello_cpu::RenderContext::new(64, 64);
    let result = panic::catch_unwind(AssertUnwindSafe(|| f(&mut ctx)));
    match result {
        Ok(()) => println!("[vello_cpu] {name}: no panic (unexpected)"),
        Err(payload) => println!("[vello_cpu] {name}: panicked with: {}", panic_message(payload)),
    }
}

fn run_hybrid_case(name: &str, f: impl FnOnce(&mut vello_hybrid::Scene)) {
    let mut scene = vello_hybrid::Scene::new(64, 64);
    let result = panic::catch_unwind(AssertUnwindSafe(|| f(&mut scene)));
    match result {
        Ok(()) => println!("[vello_hybrid] {name}: no panic (unexpected)"),
        Err(payload) => println!("[vello_hybrid] {name}: panicked with: {}", panic_message(payload)),
    }
}

fn main() {
    println!("== vello_cpu::RenderContext ==");

    run_cpu_case("no push, single pop_layer()", |ctx| {
        ctx.pop_layer();
    });

    run_cpu_case("balanced filter push/pop, then one extra pop_layer()", |ctx| {
        ctx.push_filter_layer(make_filter());
        ctx.pop_layer(); // balanced: consumes the filter layer.
        ctx.pop_layer(); // extra: no matching push at all.
    });

    run_cpu_case(
        "two filter pushes, three pops (one extra after balanced nesting)",
        |ctx| {
            ctx.push_filter_layer(make_filter());
            ctx.push_filter_layer(make_filter());
            ctx.pop_layer();
            ctx.pop_layer();
            ctx.pop_layer(); // extra.
        },
    );

    println!("== vello_hybrid::Scene ==");

    run_hybrid_case("no push, single pop_layer()", |scene| {
        scene.pop_layer();
    });

    run_hybrid_case("balanced filter push/pop, then one extra pop_layer()", |scene| {
        scene.push_filter_layer(make_filter());
        scene.pop_layer();
        scene.pop_layer(); // extra.
    });

    println!();
    println!(
        "Conclusion: every panic above should be at record.rs:307 (CommandRecorder::pop_layer's \
         layer_stack.pop().unwrap(), already confirmed as candidate 031), never at clip.rs:234 \
         (ClipState::pop_root_viewport's frames.pop()) or viewport.rs:114 (root viewport stack \
         underflow) - confirming those two are unreachable given the current call graph."
    );
}
