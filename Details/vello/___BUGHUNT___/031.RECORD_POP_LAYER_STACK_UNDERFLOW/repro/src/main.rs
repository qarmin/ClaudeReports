// Standalone reproduction for candidate 031 (RECORD_POP_LAYER_STACK_UNDERFLOW).
//
// Calls `vello_cpu::RenderContext::pop_layer()` - the real public API a document-format
// translation layer (SVG/PDF importer, etc.) would call once per closing group/layer tag -
// without a matching prior `push_layer`/`push_clip_layer`/etc. call, exactly the situation an
// unbalanced or malformed group-nesting input in such a caller would produce. No second
// participant/wire format is involved here: the caller misusing this exact public API *is* the
// real external input, there is nothing else to fake.
//
// Exit code convention: 0 = did not reproduce (bug absent), non-zero/panic = reproduced.

use vello_cpu::RenderContext;

fn main() {
    let result = std::panic::catch_unwind(|| {
        let mut ctx = RenderContext::new(100, 100);
        // No push_layer() call preceded this - simulates a malformed/unbalanced document
        // (e.g. an extra closing "layer/group" marker with no matching opening one).
        ctx.pop_layer();
    });

    match result {
        Ok(()) => {
            println!("did not reproduce - pop_layer() returned normally with no matching push_layer()");
        }
        Err(panic_payload) => {
            eprintln!("REPRODUCED - pop_layer() panicked: {:?}", panic_payload);
            std::process::exit(1);
        }
    }
}
