// Reproduction for candidate 044: GLIFO_FONTDATA_AS_SKRIFA_UNWRAP_PANIC.
//
// glifo/src/glyph.rs's FontDataExt::as_skrifa() does:
//   FontRef::from_index(self.data.data(), self.index).unwrap()
// `FontRef::from_index` returns `Result<FontRef, ReadError>` and fails whenever the requested
// collection index is out of range for the given font data (or the sfnt/ttc header itself is
// malformed). `peniko::FontData::new(data, index)` is a fully public, unchecked constructor - it
// never validates that `index` is actually a valid font-collection index for `data`. This program
// hands a perfectly well-formed, unmodified single-font (non-collection) TTF but with index 1
// instead of 0 - a caller mistake or a font whose collection membership assumption doesn't hold -
// and drives this through vello_cpu's real public API (`RenderContext::glyph_run(...).fill_glyphs(...)`),
// the same entry point any real caller (e.g. a text layout engine feeding a user-supplied font
// file/index pair) would use.
//
// This panics inside `as_skrifa()` itself (line 1419 in glifo/src/glyph.rs), which is called
// (unconditionally, for every mode) before the previously-found `.head().unwrap()` panic
// (candidate 040) is even reached - a fix for candidate 040 alone would not fix this.
//
// Exit code convention: 0 = did not reproduce, non-zero = reproduced (panicked).

use std::iter;

use vello_common::kurbo::Affine;
use vello_common::peniko::{Blob, FontData};
use vello_cpu::{Glyph, RenderContext, Resources};

fn main() {
    let original: &[u8] = include_bytes!("../assets/Roboto-Regular.ttf");

    // Sanity check: this is a real, unmodified, single (non-collection) sfnt font - not a
    // font collection (which would start with "ttcf"), so index 1 is genuinely out of range.
    assert_ne!(&original[0..4], b"ttcf", "fixture is a TTC, not representative");

    // Not a single byte was touched - the only "corruption" is the caller-supplied collection
    // index, which peniko::FontData::new never validates against the actual font data.
    let font = FontData::new(Blob::from(original.to_vec()), 1);

    let mut ctx = RenderContext::new(200, 100);
    let mut resources = Resources::new();
    ctx.set_transform(Affine::translate((10.0, 50.0)));

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        ctx.glyph_run(&mut resources, &font)
            .font_size(32.0)
            .fill_glyphs(iter::empty::<Glyph>());
    }));

    match result {
        Ok(()) => {
            println!(
                "did not reproduce - fill_glyphs completed normally on FontData with out-of-range index"
            );
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
