//! Callgrind probe for candidate 020: renders `cache_defeating.pdf` (326 `Tf` calls, every one a
//! cache miss for candidate 018's `resolved_font_cache`) a fixed, small number of times so
//! callgrind's exact instruction count (deterministic, unlike wall-clock) can be diffed between
//! baseline and patched builds. Iteration count is a CLI arg so both variants can be run at
//! whatever count is convenient (1 for a quick smoke check under callgrind's own overhead, more
//! for a stabler total count).

use hayro::hayro_interpret::InterpreterSettings;
use hayro::hayro_syntax::Pdf;
use hayro::{RenderCache, RenderSettings, render};

const CACHE_DEFEATING_PDF: &[u8] = include_bytes!("../../testdata/cache_defeating.pdf");

fn main() {
    let iterations: usize = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(1);

    let pdf = Pdf::new(CACHE_DEFEATING_PDF.to_vec()).unwrap();
    let cache = RenderCache::new();
    let settings = InterpreterSettings::default();
    let page = pdf.pages().iter().next().unwrap();

    let mut last_hash: u64 = 0;
    for _ in 0..iterations {
        let pixmap = render(
            &page,
            &cache,
            &settings,
            &RenderSettings {
                x_scale: 1.0,
                y_scale: 1.0,
                ..Default::default()
            },
        );
        let mut hash: u64 = 0xcbf29ce484222325;
        for px in pixmap.data() {
            for b in px.to_u8_array() {
                hash ^= b as u64;
                hash = hash.wrapping_mul(0x100000001b3);
            }
        }
        last_hash = hash;
        std::hint::black_box(&pixmap);
    }
    println!("hash: {last_hash:#018x}");
}
