// Same class, different input: completely garbage bytes (not just a valid font with a bad
// index), so FontRef::from_index fails at the FileRef::new/table-directory-parsing stage
// instead of the InvalidCollectionIndex stage. Confirms the fix isn't narrowly specific to
// the out-of-range-index shape alone (CONFIRMATION.md section 7).

use std::iter;

use vello_common::kurbo::Affine;
use vello_common::peniko::{Blob, FontData};
use vello_cpu::{Glyph, RenderContext, Resources};

fn main() {
    let garbage: Vec<u8> = vec![0u8; 32];
    let font = FontData::new(Blob::from(garbage), 0);

    let mut ctx = RenderContext::new(200, 100);
    let mut resources = Resources::new();
    ctx.set_transform(Affine::translate((10.0, 50.0)));

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        ctx.glyph_run(&mut resources, &font)
            .font_size(32.0)
            .fill_glyphs(iter::empty::<Glyph>());
    }));

    match result {
        Ok(()) => println!("did not reproduce - fill_glyphs completed normally on garbage bytes"),
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
