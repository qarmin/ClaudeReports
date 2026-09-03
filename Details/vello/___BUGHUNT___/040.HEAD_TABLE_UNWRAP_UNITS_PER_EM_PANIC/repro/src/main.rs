// Reproduction for candidate 040: HEAD_TABLE_UNWRAP_UNITS_PER_EM_PANIC.
//
// glifo/src/glyph.rs's `prepare_glyph_run` does:
//   run.font.as_skrifa().head().map(|h| h.units_per_em()).unwrap()
// `head()` returns `Result<Head, ReadError>` - it fails whenever the font's `head` table is
// missing or malformed. This program hands a real, well-formed sfnt (a valid table directory,
// so it still parses via `FontRef::from_index`), but with just the 4-byte `head` table tag
// corrupted so the table can no longer be found - a truncated/corrupted-download or
// deliberately crafted font is exactly this shape. It drives this through vello_cpu's real
// public API (`RenderContext::glyph_run(...).fill_glyphs(...)`), the same entry point any real
// caller (e.g. a text layout engine feeding a user-supplied font file) would use.
//
// Exit code convention: 0 = did not reproduce, non-zero = reproduced (panicked).

use std::iter;

use vello_common::kurbo::Affine;
use vello_common::peniko::{Blob, FontData};
use vello_cpu::{Glyph, RenderContext, Resources};

/// Corrupt the first table directory entry whose tag is `head`, so that skrifa's
/// `FontRef::head()` accessor can no longer find it (`ReadError::TableIsMissing`), while
/// leaving the rest of the sfnt table directory - and thus `FontRef::from_index` - intact.
fn corrupt_head_table_tag(mut font_bytes: Vec<u8>) -> Vec<u8> {
    const SFNT_HEADER_LEN: usize = 12;
    const RECORD_LEN: usize = 16;

    let num_tables = u16::from_be_bytes([font_bytes[4], font_bytes[5]]) as usize;

    for i in 0..num_tables {
        let record_off = SFNT_HEADER_LEN + i * RECORD_LEN;
        let tag = &font_bytes[record_off..record_off + 4];
        if tag == b"head" {
            // Flip one bit in the tag so it no longer matches "head" (nor any other real
            // sfnt table tag), without touching checksum/offset/length or any other table.
            font_bytes[record_off] = b'H';
            return font_bytes;
        }
    }
    panic!("test font has no 'head' table - fixture is not representative");
}

fn main() {
    let original: &[u8] = include_bytes!("../assets/Roboto-Regular.ttf");
    let corrupted = corrupt_head_table_tag(original.to_vec());

    // Sanity check: exactly one byte changed (the head tag), nothing else about the sfnt
    // table directory was touched - this isn't "garbage in, panic out".
    let diff_count = original
        .iter()
        .zip(corrupted.iter())
        .filter(|(a, b)| a != b)
        .count();
    assert_eq!(
        diff_count, 1,
        "corruption should change exactly the one head-tag byte"
    );

    let font = FontData::new(Blob::from(corrupted), 0);

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
            println!("did not reproduce - fill_glyphs completed normally on a font with a corrupted head table");
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
