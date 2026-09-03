// Standalone reproduction for the unbounded Type3-glyph-in-itself recursion in
// hayro-interpret/src/font/type3.rs (`Type3::render_glyph`).
//
// `self_type3.pdf` is a minimal, hand-crafted PDF whose page shows one glyph ("A") using a Type3
// font, and whose Type3 font's own glyph proc for "A" shows text using the very same Type3 font
// object again (`/F1` in the glyph proc's inherited resources resolves back to font object `5 0
// R`, the font currently being rendered). The file is fed straight to `hayro::render`, the
// crate's own public rendering entry point - exactly the same entry point a real caller (a PDF
// viewer, a thumbnailer, hayro's own CLI) would use to render an untrusted, malformed PDF. No
// second participant is faked here: a malformed file handed to a renderer's own public entry
// point is the genuine external input, there is no "peer" being impersonated.
//
// A stack overflow aborts the whole process and cannot be caught with `catch_unwind` in-process,
// so the input is rendered in a fresh child process (re-exec of this same binary with `--child
// <file>`) and the parent observes whether the child survives.
//
// Exit code convention: 0 = did not reproduce, non-zero = reproduced.

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

    // Harness mode: spawn a fresh child process for the input (see module doc above for why).
    let file = "self_type3.pdf";
    let self_exe = env::current_exe().expect("could not determine current executable path");

    let status = Command::new(&self_exe)
        .arg("--child")
        .arg(file)
        .status()
        .expect("failed to spawn child process");

    if status.success() {
        println!(
            "[self-referential Type3 font] did not reproduce - render() completed normally (child status: {status})"
        );
    } else {
        println!(
            "[self-referential Type3 font] REPRODUCED - child process crashed (child status: {status})"
        );
        std::process::exit(1);
    }
}
