//! Callgrind probe for candidate 022 (reused unchanged from candidate 021): renders
//! `worst_case_cache.pdf` (326 `Tf` calls, every one a *distinct* resource name never reused
//! anywhere else on the page) a fixed, small number of times so callgrind's exact instruction
//! count (deterministic, unlike wall-clock) can be diffed between baseline (no
//! `resolved_font_cache`, same as candidate 021's baseline) and patched (`resolved_font_cache`
//! with an `Entry`-API miss path instead of candidate 021's separate `get()`+`insert()`) builds.

use hayro::hayro_interpret::InterpreterSettings;
use hayro::hayro_syntax::Pdf;
use hayro::{RenderCache, RenderSettings, render};

const WORST_CASE_PDF: &[u8] = include_bytes!("../../testdata/worst_case_cache.pdf");

fn main() {
    let iterations: usize = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(1);

    let pdf = Pdf::new(WORST_CASE_PDF.to_vec()).unwrap();
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
