//! Candidate 077: does `vello_common::transforms::RootTransforms::pop_root`'s unguarded
//! `self.transforms.pop()` (transforms.rs:126) ever underflow through any real public entry
//! point, independently of `CommandRecorder::pop_layer`'s own already-panicking layer-stack
//! check (record.rs:307, the underflow candidate 031 already confirmed) or the multi-threaded
//! dispatcher's own `layer_depth` underflow check (multi_threaded.rs:584)?
//!
//! Unlike candidate 046 (which checked `ClipState::pop_root_viewport`, only pushed/popped for
//! *filter* layers), `RootTransforms::push_root`/`pop_root` are called unconditionally for
//! *every* `push_layer`/`pop_layer` call (filter or not) in both `vello_cpu::RenderContext` and
//! `vello_hybrid::Scene`. Both call `pop_root()` only *after* the underlying dispatcher's own
//! `pop_layer()` call returns successfully - and that call panics first (record.rs:307, or
//! multi_threaded.rs:584) on any unbalanced/extra pop. Since every `push_layer` call pushes
//! both a `layer_stack` entry and a `root_transforms` entry (in that order: push_root, then
//! recorder.push_layer), and both are only ever popped together inside `pop_layer` (dispatcher
//! pop first, then pop_root), `root_transforms.len()` can never end up smaller than
//! `layer_stack.len() + 1` (the permanent identity element) through this API - so `pop_root()`
//! should never be reached with fewer than 2 elements left.
//!
//! Each case below is run in its own subprocess (candidate 031's multi-threaded dispatcher path
//! is known to sometimes cause a whole-process abort/second-panic on `Drop`, so cases must not
//! share a process or a later case's signal would be lost). The driver (`main`, no args) spawns
//! itself once per case with `--case N` and reports each case's exit status and captured
//! stdout/stderr. If the suspected bug were independently reachable, at least one case would
//! panic at transforms.rs:105 ("root transform stack should never be empty", the delayed symptom
//! of an unguarded `pop_root()` having silently emptied the stack - `pop_root` itself discards
//! `Vec::pop()`'s `None` and does not panic synchronously) instead of at record.rs:307 or
//! multi_threaded.rs:584.

use std::process::Command;
use vello_common::filter_effects::{Filter, FilterFunction};
use vello_cpu::{RenderContext, RenderSettings};

fn make_filter() -> Filter {
    Filter::from_function(FilterFunction::Blur { radius: 1.0 })
}

fn single_threaded() -> RenderSettings {
    RenderSettings {
        num_threads: 0,
        ..Default::default()
    }
}

fn multi_threaded() -> RenderSettings {
    RenderSettings {
        num_threads: 1,
        ..Default::default()
    }
}

const CASE_NAMES: &[&str] = &[
    "vello_cpu/single-threaded: no push, single pop_layer() (regular)",
    "vello_cpu/single-threaded: balanced push_blend_layer/pop_layer, then one extra pop_layer()",
    "vello_cpu/single-threaded: two regular pushes, three pops (one extra after balanced nesting)",
    "vello_cpu/single-threaded: balanced filter push/pop, then one extra pop_layer()",
    "vello_cpu/multi-threaded: no push, single pop_layer() (regular)",
    "vello_cpu/multi-threaded: balanced push_blend_layer/pop_layer, then one extra pop_layer()",
    "vello_hybrid: no push, single pop_layer() (regular)",
    "vello_hybrid: balanced push_blend_layer/pop_layer, then one extra pop_layer()",
    "vello_hybrid: balanced filter push/pop, then one extra pop_layer()",
];

fn run_case(case: usize) {
    match case {
        0 => {
            let mut ctx = RenderContext::new_with(64, 64, single_threaded());
            ctx.pop_layer();
        }
        1 => {
            let mut ctx = RenderContext::new_with(64, 64, single_threaded());
            ctx.push_blend_layer(Default::default());
            ctx.pop_layer(); // balanced
            ctx.pop_layer(); // extra: no matching push at all
        }
        2 => {
            let mut ctx = RenderContext::new_with(64, 64, single_threaded());
            ctx.push_blend_layer(Default::default());
            ctx.push_blend_layer(Default::default());
            ctx.pop_layer();
            ctx.pop_layer();
            ctx.pop_layer(); // extra
        }
        3 => {
            let mut ctx = RenderContext::new_with(64, 64, single_threaded());
            ctx.push_filter_layer(make_filter());
            ctx.pop_layer(); // balanced
            ctx.pop_layer(); // extra
        }
        4 => {
            let mut ctx = RenderContext::new_with(64, 64, multi_threaded());
            ctx.pop_layer();
        }
        5 => {
            let mut ctx = RenderContext::new_with(64, 64, multi_threaded());
            ctx.push_blend_layer(Default::default());
            ctx.pop_layer(); // balanced
            ctx.pop_layer(); // extra
        }
        6 => {
            let mut scene = vello_hybrid::Scene::new(64, 64);
            scene.pop_layer();
        }
        7 => {
            let mut scene = vello_hybrid::Scene::new(64, 64);
            scene.push_blend_layer(Default::default());
            scene.pop_layer(); // balanced
            scene.pop_layer(); // extra
        }
        8 => {
            let mut scene = vello_hybrid::Scene::new(64, 64);
            scene.push_filter_layer(make_filter());
            scene.pop_layer(); // balanced
            scene.pop_layer(); // extra
        }
        _ => unreachable!(),
    }
    println!("no panic (unexpected)");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() >= 3 && args[1] == "--case" {
        let case: usize = args[2].parse().unwrap();
        run_case(case);
        return;
    }

    // Driver mode: run every case in its own subprocess.
    let self_path = std::env::current_exe().unwrap();
    let mut any_transforms_panic = false;
    for (i, name) in CASE_NAMES.iter().enumerate() {
        let output = Command::new(&self_path)
            .args(["--case", &i.to_string()])
            .output()
            .expect("failed to spawn subprocess");
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        let panic_line = stderr
            .lines()
            .find(|l| l.contains("panicked at"))
            .unwrap_or("<no panic line found>");
        println!("--- case {i}: {name} ---");
        println!("exit status: {:?}", output.status);
        println!("stdout: {}", stdout.trim());
        println!("panic location: {panic_line}");
        if panic_line.contains("transforms.rs") {
            any_transforms_panic = true;
            println!("!!! REPRODUCED: panic in transforms.rs (RootTransforms) !!!");
        }
    }

    println!();
    if any_transforms_panic {
        println!(
            "CONCLUSION: at least one case panicked inside transforms.rs (RootTransforms) - the \
             suspected underflow IS independently reachable."
        );
        std::process::exit(1);
    } else {
        println!(
            "CONCLUSION: every case above panicked (or aborted) at record.rs:307 \
             (CommandRecorder::pop_layer's layer_stack.pop().unwrap(), candidate 031) or \
             multi_threaded.rs's layer_depth checked_sub().expect(\"layer stack underflow\"), \
             never inside transforms.rs (RootTransforms' own stack) - RootTransforms::pop_root's \
             missing underflow guard is masked/unreachable given the current call graph."
        );
    }
}
