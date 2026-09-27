// Standalone reproduction for the unbounded pattern-in-pattern recursion in
// hayro-interpret/src/pattern.rs (`TilingPattern::interpret`).
//
// Both `crash_input.pdf` (the original fuzzer-found crash file,
// hayro-fuzz/artifacts/fuzz_pdf_render/crash-51b715b89668b0bd90d214f9249269abac0b05a2) and
// `self_pattern.pdf` (an independently hand-crafted, minimal PDF whose tiling pattern's own
// content stream fills a shape with itself) are fed straight to `hayro::render`, the crate's own
// public rendering entry point - exactly the same entry point a real caller (a PDF viewer, a
// thumbnailer, hayro's own CLI) would use to render an untrusted, malformed PDF. No second
// participant is faked here: a malformed file handed to a renderer's own public entry point is
// the genuine external input, there is no "peer" being impersonated.
//
// A stack overflow aborts the whole process and cannot be caught with `catch_unwind` in-process,
// so each input is rendered in a fresh child process (re-exec of this same binary with
// `--child <file>`) and the parent observes whether the child survives.
//
// Exit code convention: 0 = neither input reproduced, non-zero = at least one reproduced.

use std::env;
use std::process::Command;

fn render_pdf_bytes(data: Vec<u8>) {
    use hayro::hayro_interpret::InterpreterSettings;
    use hayro::hayro_interpret::hayro_syntax::Pdf;
    use hayro::{RenderCache, RenderSettings, render};

    let pdf = Pdf::new(data).expect("failed to parse PDF");
    let page = pdf.pages().iter().next().expect("PDF has no pages");

    let settings = InterpreterSettings::default();
    let render_settings = RenderSettings {
        x_scale: 1.0,
        y_scale: 1.0,
        ..Default::default()
    };
    let cache = RenderCache::new();

    let pixmap = render(page, &cache, &settings, &render_settings);
    println!(
        "rendered a {}x{} pixmap without crashing",
        pixmap.width(),
        pixmap.height()
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 && args[1] == "--child" {
        // Child mode: actually run the real end-to-end render path on the given input file.
        let data = std::fs::read(&args[2]).expect("failed to read input file");
        render_pdf_bytes(data);
        return;
    }

    // Harness mode: spawn a fresh child process per input (see module doc above for why).
    let inputs = [
        (
            "original fuzzer-found crash file (crash-51b715b89668b0bd90d214f9249269abac0b05a2)",
            "crash_input.pdf",
        ),
        (
            "independently hand-crafted self-referential tiling pattern",
            "self_pattern.pdf",
        ),
    ];

    let self_exe = env::current_exe().expect("could not determine current executable path");
    let mut any_reproduced = false;

    for (label, file) in inputs {
        let status = Command::new(&self_exe)
            .arg("--child")
            .arg(file)
            .status()
            .expect("failed to spawn child process");

        if status.success() {
            println!("[{label}] did not reproduce - render() completed normally (child status: {status})");
        } else {
            println!("[{label}] REPRODUCED - child process crashed (child status: {status})");
            any_reproduced = true;
        }
    }

    if any_reproduced {
        std::process::exit(1);
    }
}
