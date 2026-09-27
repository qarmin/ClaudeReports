//! Correctness probe for candidate `070.VERTICALLAYOUT_TEXT_ORIENTATION_QUERY` (the font-metrics
//! NoWrap-height fast path in `Text::layout_info`'s `Orientation::Vertical` branch).
//!
//! Renders a `VerticalLayout` of fixed-width `NoWrap` `Text`s covering the styling axes the fast
//! path's font-metrics estimate has to match exactly (size/weight/italic/family/script/emoji/
//! letter-spacing), plus the one case it must *not* take the fast path for (an embedded `\n`,
//! which still hard-breaks under `NoWrap`), and prints an FNV-1a hash of the rendered pixels. Run
//! the same probe against a baseline-built and a patched-built `i-slint-core` and diff the hash -
//! any divergence means the fast path's estimate stopped matching the real shaped layout for one
//! of these cases. See `___OPTIMIZATIONS___/070.VERTICALLAYOUT_TEXT_ORIENTATION_QUERY/RESULT.md`.

use slint::platform::software_renderer::MinimalSoftwareWindow;
use slint::platform::{Platform, PlatformError};
use slint::{ComponentHandle, PhysicalSize};
use std::rc::Rc;
use std::time::{Duration, Instant};

struct TestPlatform {
    window: Rc<MinimalSoftwareWindow>,
    start: Instant,
}

impl Platform for TestPlatform {
    fn create_window_adapter(
        &self,
    ) -> Result<Rc<dyn slint::platform::WindowAdapter>, PlatformError> {
        Ok(self.window.clone())
    }
    fn duration_since_start(&self) -> Duration {
        self.start.elapsed()
    }
}

slint::slint! {
    export component VLayoutFixedText inherits Window {
        width: 800px;
        height: 800px;
        VerticalLayout {
            Text { width: 200px; text: "first line of text"; wrap: no-wrap; }
            Text { width: 200px; text: "second line of text"; wrap: no-wrap; }
            Text { width: 200px; text: "third line of text"; wrap: no-wrap; }
            Text { width: 200px; text: "tiny"; font-size: 6px; wrap: no-wrap; }
            Text { width: 200px; text: "huge"; font-size: 96px; wrap: no-wrap; }
            Text { width: 200px; text: "bold"; font-weight: 700; wrap: no-wrap; }
            Text { width: 200px; text: "italic"; font-italic: true; wrap: no-wrap; }
            Text { width: 200px; text: "letters"; letter-spacing: 3px; wrap: no-wrap; }
            Text { width: 200px; text: "serif family"; font-family: "serif"; wrap: no-wrap; }
            Text { width: 200px; text: "monospace"; font-family: "monospace"; wrap: no-wrap; }
            Text { width: 200px; text: "MixedCASE Ünïcödé 日本語"; wrap: no-wrap; }
            Text { width: 200px; text: ""; wrap: no-wrap; }
            Text { width: 200px; text: "🎉 emoji line"; wrap: no-wrap; }
            // The guard: an embedded hard break under `NoWrap` still starts a new paragraph
            // (only word-wrapping is suppressed), so this must NOT take the font-metrics fast path.
            Text { width: 200px; text: "line one\nline two\nline three"; wrap: no-wrap; }
            Text { width: 5px; text: "a very long line that would need to wrap if wrapping were on"; wrap: no-wrap; }
        }
    }
}

fn main() {
    let window = MinimalSoftwareWindow::new(Default::default());
    slint::platform::set_platform(Box::new(TestPlatform {
        window: window.clone(),
        start: Instant::now(),
    }))
    .unwrap();
    window.set_size(PhysicalSize::new(800, 800));
    let mut buffer = vec![slint::Rgb8Pixel::default(); 800 * 800];

    let app = VLayoutFixedText::new().unwrap();
    app.show().unwrap();
    window.draw_if_needed(|renderer| {
        renderer.render(&mut buffer, 800);
    });

    let bytes: &[u8] = bytemuck_cast(&buffer);
    let mut hash: u64 = 1469598103934665603;
    for &b in bytes {
        hash ^= b as u64;
        hash = hash.wrapping_mul(1099511628211);
    }
    println!("pixel fnv1a hash: {hash:016x}");
}

fn bytemuck_cast(pixels: &[slint::Rgb8Pixel]) -> &[u8] {
    // SAFETY: `Rgb8Pixel` is `#[repr(C)]` with 3 `u8` fields, no padding, no invalid bit patterns.
    unsafe {
        std::slice::from_raw_parts(pixels.as_ptr() as *const u8, std::mem::size_of_val(pixels))
    }
}
