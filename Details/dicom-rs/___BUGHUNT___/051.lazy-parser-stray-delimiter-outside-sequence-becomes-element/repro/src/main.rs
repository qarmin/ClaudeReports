use std::io::Cursor;

use dicom_parser::dataset::lazy_read::LazyDataSetReader;
use dicom_parser::dataset::LazyDataToken;
use dicom_transfer_syntax_registry::entries::EXPLICIT_VR_LITTLE_ENDIAN;

/// Raw explicit-VR-LE dataset bytes: a bare Item marker (FFFE,E000), with
/// undefined length, appearing outside of any sequence, followed by 3 more
/// bare delimiter-shaped byte groups - the same shape used by candidate
/// 050's reproduction of this bug in the eager `DataSetReader`
/// (`parser/src/dataset/read.rs`). This candidate shows the same defect
/// independently exists in `LazyDataSetReader`
/// (`parser/src/dataset/lazy_read.rs`) - a separate, hand-written token
/// generation implementation used by `DicomCollector` and any other caller
/// that wants to skip/lazily read values.
const STRAY_ITEM_THEN_DELIMITERS: &[u8] = &[
    0xfe, 0xff, 0x00, 0xe0, 0xff, 0xff, 0xff, 0xff, // Item, len undefined
    0xfe, 0xff, 0xdd, 0xe0, 0x00, 0x00, 0x00, 0x00, // SequenceDelimiter, len 0
    0xfe, 0xff, 0x0d, 0xe0, 0x00, 0x00, 0x00, 0x00, // ItemDelimiter, len 0
    0xfe, 0xff, 0xdd, 0xe0, 0x00, 0x00, 0x00, 0x00, // SequenceDelimiter, len 0
];

/// Same idea, but with a bare Sequence Delimitation Item (FFFE,E0DD),
/// defined length (always 0), outside of any sequence, on its own.
const STRAY_SEQUENCE_DELIMITER: &[u8] = &[
    0xfe, 0xff, 0xdd, 0xe0, // sequence delimiter, out of place
    0x00, 0x00, 0x00, 0x00, // length is always zero
];

/// Returns `true` if a bogus top-level element header with a reserved tag
/// was manufactured (the bug); `false` if the input was either cleanly
/// rejected or produced no elements at all (both acceptable outcomes).
fn try_read(label: &str, data: &[u8]) -> bool {
    let ts = EXPLICIT_VR_LITTLE_ENDIAN.erased();
    let mut reader = match LazyDataSetReader::new_with_ts(Cursor::new(data), &ts) {
        Ok(r) => r,
        Err(e) => {
            println!("{label}: could not even build a reader: {e}");
            return false;
        }
    };

    let mut bug = false;
    while let Some(res) = reader.advance() {
        match res {
            Ok(LazyDataToken::ElementHeader(header)) if header.tag.group() == 0xFFFE => {
                println!(
                    "{label}: BUG - produced an ElementHeader token with reserved tag {} (vr {:?}, len {:?})",
                    header.tag, header.vr, header.len,
                );
                bug = true;
            }
            Ok(LazyDataToken::SequenceStart { tag, len }) if tag.group() == 0xFFFE => {
                println!(
                    "{label}: BUG - produced a SequenceStart token with reserved tag {tag} (len {len:?})",
                );
                bug = true;
            }
            Ok(token) => {
                println!("{label}: token: {token:?}");
            }
            Err(e) => {
                println!("{label}: rejected with a clean parse error: {e}");
                break;
            }
        }
    }
    if !bug {
        println!("{label}: no bogus reserved-tag token produced");
    }
    bug
}

fn main() {
    let bug_a = try_read(
        "stray Item followed by delimiters",
        STRAY_ITEM_THEN_DELIMITERS,
    );
    let bug_b = try_read(
        "stray SequenceDelimiter (FFFE,E0DD)",
        STRAY_SEQUENCE_DELIMITER,
    );

    if bug_a || bug_b {
        println!(
            "BUG REPRODUCED: a stray delimiter/item marker outside of a sequence was turned \
             into an ordinary token carrying a reserved tag."
        );
        std::process::exit(1);
    }

    println!("No corruption observed: stray markers are cleanly rejected or ignored.");
}
