// Standalone reproduction attempt for a suspected unbounded recursion in
// hayro-interpret/src/x_object/soft_mask.rs (a self-referential SMask ExtGState).
//
// self_softmask.pdf is fed straight to hayro::render, the crate's own public rendering entry
// point - the same one a real caller (a PDF viewer, a thumbnailer) would use to render an
// untrusted, malformed PDF. No second participant is faked here.
//
// A stack overflow aborts the whole process and cannot be caught with catch_unwind in-process,
// so the input is rendered in a fresh child process (re-exec of this binary with --child <file>)
// and the parent observes whether the child survives.

use std::env;
use std::process::Command;
use std::time::{Duration, Instant};

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
        let data = std::fs::read(&args[2]).expect("failed to read input file");
        render_pdf_bytes(data);
        return;
    }

    let file = "self_softmask.pdf";
    let self_exe = env::current_exe().expect("could not determine current executable path");

    let start = Instant::now();
    let status = Command::new(&self_exe)
        .arg("--child")
        .arg(file)
        .status()
        .expect("failed to spawn child process");
    let elapsed = start.elapsed();

    if status.success() {
        println!(
            "[self-referential soft mask] did not reproduce - render() completed normally (child status: {status}, took {:?})",
            elapsed
        );
        std::process::exit(0);
    } else if elapsed > Duration::from_secs(20) {
        println!(
            "[self-referential soft mask] TIMEOUT/HANG suspected - child status: {status}, took {:?}",
            elapsed
        );
        std::process::exit(2);
    } else {
        println!(
            "[self-referential soft mask] REPRODUCED - child process crashed (child status: {status}, took {:?})",
            elapsed
        );
        std::process::exit(1);
    }
}
