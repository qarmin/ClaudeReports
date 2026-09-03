// Standalone reproduction for candidate 045 (CLIP_STATE_POP_CLIP_STACK_UNDERFLOW).
//
// Calls `vello_cpu::RenderContext::pop_clip_path()` - the real, fully public API a document
// importer/translation layer (SVG/PDF/HTML-canvas clip-stack replay, etc.) would call once per
// closing "restore clip" marker - without a matching prior `push_clip_path()` call. That is
// exactly the situation an unbalanced or malformed clip-nesting input in such a caller would
// produce. No second participant/wire format is involved: the caller misusing this exact public
// API (calling `pop_clip_path()` one time too many) *is* the real external input, there is
// nothing else to fake.
//
// Two variants are exercised, matching both of `vello_cpu`'s dispatch backends, since the panic
// fires in two different (but sibling) functions depending on which is active:
//   - default (single-threaded) dispatcher -> viewport.rs -> ClipState::pop_clip
//     (vello_common/src/clip.rs, "ClipState::pop_clip")
//   - multi-threaded dispatcher (`multithreading` feature, `num_threads: 1`) -> its own
//     `clip_context: ClipContext` field -> ClipContext::pop_clip
//     (vello_common/src/clip.rs, "ClipContext::pop_clip")
//
// Exit code convention: 0 = neither variant reproduced, non-zero = at least one reproduced.

use vello_cpu::kurbo::{Rect, Shape};
use vello_cpu::{RenderContext, RenderSettings};

fn main() {
    let mut reproduced = false;

    let result_st = std::panic::catch_unwind(|| {
        // num_threads: 0 explicitly selects the single-threaded dispatcher - required because
        // this binary is built with the `multithreading` feature enabled (to also exercise the
        // multi-threaded variant below), which changes `RenderSettings::default()`'s own
        // `num_threads` to `available_parallelism()` instead of 0.
        let settings = RenderSettings {
            num_threads: 0,
            ..Default::default()
        };
        let mut ctx = RenderContext::new_with(100, 100, settings);
        // No push_clip_path() call preceded this - simulates a malformed/unbalanced document
        // (e.g. an extra "restore clip" marker with no matching "save clip" one).
        ctx.pop_clip_path();
    });
    match result_st {
        Ok(()) => {
            println!(
                "[single-threaded] did not reproduce - pop_clip_path() returned normally with no matching push_clip_path()"
            );
        }
        Err(panic_payload) => {
            eprintln!(
                "[single-threaded] REPRODUCED - pop_clip_path() panicked: {panic_payload:?}"
            );
            reproduced = true;
        }
    }

    let result_mt = std::panic::catch_unwind(|| {
        let settings = RenderSettings {
            num_threads: 1,
            ..Default::default()
        };
        let mut ctx = RenderContext::new_with(100, 100, settings);
        ctx.pop_clip_path();
    });
    match result_mt {
        Ok(()) => {
            println!(
                "[multi-threaded] did not reproduce - pop_clip_path() returned normally with no matching push_clip_path()"
            );
        }
        Err(panic_payload) => {
            eprintln!("[multi-threaded] REPRODUCED - pop_clip_path() panicked: {panic_payload:?}");
            reproduced = true;
        }
    }

    // A different input in the same bug class (CONFIRMATION.md section 7): an extra pop *after*
    // otherwise-balanced push/pop nesting, rather than a pop with zero preceding pushes.
    let result_extra = std::panic::catch_unwind(|| {
        let mut ctx = RenderContext::new(100, 100);
        let path1 = Rect::new(0.0, 0.0, 10.0, 10.0).to_path(0.1);
        let path2 = Rect::new(0.0, 0.0, 5.0, 5.0).to_path(0.1);
        ctx.push_clip_path(&path1);
        ctx.push_clip_path(&path2);
        ctx.pop_clip_path();
        ctx.pop_clip_path();
        // One extra pop beyond the balanced nesting above.
        ctx.pop_clip_path();
    });
    match result_extra {
        Ok(()) => {
            println!(
                "[extra-pop-after-balanced] did not reproduce - the extra pop_clip_path() returned normally"
            );
        }
        Err(panic_payload) => {
            eprintln!(
                "[extra-pop-after-balanced] REPRODUCED - pop_clip_path() panicked: {panic_payload:?}"
            );
            reproduced = true;
        }
    }

    if reproduced {
        std::process::exit(1);
    }
}
