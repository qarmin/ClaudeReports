// Reproduction for candidate 070: GLIFO_AUTOHINT_SKRIFA_GSUB_GLYPH_STYLES_OOB_PANIC.
//
// This is a real skrifa (0.44.0, the version currently pinned by vello's root Cargo.toml) bug
// reachable through glifo/vello_cpu's public, ordinary hinted-glyph-run API - not a `.unwrap()`/
// `.expect()` inside glifo's own source, but a genuine out-of-bounds slice index deep inside
// skrifa's autohinting module (`outline::autohint::shape::GsubHandler::finish`,
// `self.glyph_styles[range]`) that fires the first time `vello_cpu`'s public
// `RenderContext::glyph_run(...).hint(true).fill_glyphs(...)` is called on a font whose `maxp`
// table's sfnt-directory record (offset/length fields) has been corrupted.
//
// `assets/mutated_noto_colr.ttf` is the project's own bundled test font
// (`examples/assets/noto_color_emoji/NotoColorEmoji-Subset.ttf`) with exactly ONE byte changed
// (offset 185, part of the `maxp` table's directory-entry `length` field: 0x00 -> 0xB3) - found by
// a short one-off single-byte-mutation fuzz sweep over that font, not hand-crafted to look like a
// plausible font. This corrupts `maxp` enough that skrifa's autohinting subsystem ends up
// constructing (or indexing) a zero-length `glyph_styles` array while a GSUB substitution rule
// still reports touching a real glyph id, and `GsubHandler::finish` does
// `for glyph in &mut self.glyph_styles[range] { ... }` with no bounds check against
// `glyph_styles.len()`.
//
// Only ONE glyph (id 1) is requested, and no other pre-existing glifo bug (e.g. candidate 039's
// `DrawError` unwrap at `glifo/src/glyph.rs:1985`) is reachable for this specific input/glyph
// combination - this isolates the fix so `reproduce-after` genuinely prints "did not reproduce"
// rather than merely trading one already-catalogued panic for another.
//
// Exit code convention: 0 = did not reproduce, non-zero = reproduced (panicked).

use vello_common::kurbo::Affine;
use vello_common::peniko::{Blob, FontData};
use vello_cpu::{Glyph, RenderContext, Resources};

fn main() {
    let bytes: &[u8] = include_bytes!("../assets/mutated_noto_colr.ttf");
    let font = FontData::new(Blob::from(bytes.to_vec()), 0);

    let mut ctx = RenderContext::new(64, 64);
    let mut resources = Resources::new();
    ctx.set_transform(Affine::IDENTITY);

    let glyphs = [Glyph {
        id: 1,
        x: 0.0,
        y: 0.0,
    }];

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        ctx.glyph_run(&mut resources, &font)
            .font_size(16.0)
            // Hinting must be requested: this is the only code path that constructs a
            // `skrifa::outline::hint::HintingInstance`, which is what reaches the buggy
            // autohint-shaping code inside skrifa.
            .hint(true)
            .fill_glyphs(glyphs.into_iter());
    }));

    match result {
        Ok(()) => {
            println!("did not reproduce - fill_glyphs completed normally on the mutated font");
        }
        Err(panic_payload) => {
            let msg = panic_payload
                .downcast_ref::<String>()
                .map(String::as_str)
                .or_else(|| panic_payload.downcast_ref::<&str>().copied())
                .unwrap_or("<non-string panic payload>");
            eprintln!("REPRODUCED - panicked: {msg}");
            std::process::exit(1);
        }
    }
}
