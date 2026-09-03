use std::io::Cursor;
use std::panic::{self, AssertUnwindSafe};

use dicom_parser::dataset::lazy_read::LazyDataSetReader;
use dicom_transfer_syntax_registry::entries::EXPLICIT_VR_LITTLE_ENDIAN;

/// A bare Item Delimitation Item (FFFE,E00D) at the top level (no sequence or item
/// currently open), immediately followed by a bare Item marker (FFFE,E000).
///
/// The first tag hits `LazyDataSetReader::advance()`'s unconditional
/// `Tag(0xFFFE, 0xE00D)` arm (no sequence/item was ever opened, so
/// `seq_delimiters` is empty), which unconditionally sets `in_sequence = true`
/// and pops the already-empty `seq_delimiters` stack (a silent no-op on `Vec`).
/// The second tag is then read via `decode_item_header()` (since `in_sequence`
/// is now true), which succeeds and reaches
/// `self.seq_delimiters.last().expect("item header should be read only inside
/// an existing sequence")` with an empty stack, panicking.
const STRAY_ITEM_DELIMITER_THEN_ITEM: &[u8] = &[
    0xfe, 0xff, 0x0d, 0xe0, 0x00, 0x00, 0x00, 0x00, // ItemDelimiter, len 0, top level
    0xfe, 0xff, 0x00, 0xe0, 0x00, 0x00, 0x00, 0x00, // Item, len 0
];

/// Variant: the item header carries a non-zero defined length (with matching
/// payload bytes present), still at the top level, to check that the fix
/// isn't narrowly tied to the exact zero-length item from the first case.
const STRAY_ITEM_DELIMITER_THEN_NONEMPTY_ITEM: &[u8] = &[
    0xfe, 0xff, 0x0d, 0xe0, 0x00, 0x00, 0x00, 0x00, // ItemDelimiter, len 0, top level
    0xfe, 0xff, 0x00, 0xe0, 0x04, 0x00, 0x00, 0x00, // Item, len 4
    0xaa, 0xbb, 0xcc, 0xdd, // item payload
];

fn run_case(label: &str, data: &'static [u8]) -> bool {
    let ts = EXPLICIT_VR_LITTLE_ENDIAN.erased();
    let result = panic::catch_unwind(AssertUnwindSafe(|| {
        let mut reader = LazyDataSetReader::new_with_ts(Cursor::new(data), &ts)
            .expect("reader should build fine");

        let mut tokens = Vec::new();
        while let Some(res) = reader.advance() {
            match res {
                Ok(token) => {
                    println!("{label}: token: {token:?}");
                    tokens.push(());
                }
                Err(e) => {
                    println!("{label}: rejected with a clean parse error: {e}");
                    break;
                }
            }
        }
        tokens.len()
    }));

    match result {
        Ok(n) => {
            println!("{label}: no panic; produced {n} token(s) cleanly");
            false
        }
        Err(payload) => {
            let msg = if let Some(s) = payload.downcast_ref::<&str>() {
                s.to_string()
            } else if let Some(s) = payload.downcast_ref::<String>() {
                s.clone()
            } else {
                "<non-string panic payload>".to_string()
            };
            println!("{label}: PANIC REPRODUCED: {msg}");
            true
        }
    }
}

fn main() {
    let panic_a = run_case(
        "zero-length item after stray item delimiter",
        STRAY_ITEM_DELIMITER_THEN_ITEM,
    );
    let panic_b = run_case(
        "non-zero-length item after stray item delimiter",
        STRAY_ITEM_DELIMITER_THEN_NONEMPTY_ITEM,
    );

    if panic_a || panic_b {
        std::process::exit(1);
    }
    println!("No panic in either case.");
}
