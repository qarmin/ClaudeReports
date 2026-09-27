use dicom_core::header::HasLength;
use dicom_object::InMemDicomObject;
use dicom_transfer_syntax_registry::entries::EXPLICIT_VR_LITTLE_ENDIAN;

/// Raw explicit-VR-LE dataset bytes reproducing the exact tail of the
/// original 324-byte fuzz-found crash (minus the file meta prefix, which
/// isn't needed to reproduce this - `read_dataset_with_ts` starts straight
/// at the data set). A bare Item marker (FFFE,E000) with undefined length
/// appears outside of any sequence (e.g. right after file meta parsing
/// handed off to the data set with a group length that didn't land exactly
/// on an element boundary), followed by 3 more bare delimiter-shaped byte
/// groups. Before the fix, the reader misreads the Item as the start of a
/// new, bogus sequence, then loses track of the structure entirely as it
/// processes what follows, eventually manufacturing an ordinary
/// `DataElement` out of the last, unrelated `SequenceDelimiter` bytes.
const STRAY_ITEM_THEN_DELIMITERS: &[u8] = &[
    0xfe, 0xff, 0x00, 0xe0, 0xff, 0xff, 0xff, 0xff, // Item, len undefined
    0xfe, 0xff, 0xdd, 0xe0, 0x00, 0x00, 0x00, 0x00, // SequenceDelimiter, len 0
    0xfe, 0xff, 0x0d, 0xe0, 0x00, 0x00, 0x00, 0x00, // ItemDelimiter, len 0
    0xfe, 0xff, 0xdd, 0xe0, 0x00, 0x00, 0x00, 0x00, // SequenceDelimiter, len 0
];

/// Same idea, but with a bare Sequence Delimitation Item (FFFE,E0DD),
/// defined length (always 0), outside of any sequence.
const STRAY_SEQUENCE_DELIMITER: &[u8] = &[
    0xfe, 0xff, 0xdd, 0xe0, // sequence delimiter, out of place
    0x00, 0x00, 0x00, 0x00, // length is always zero
];

/// Returns `true` if a bogus top-level `DataElement` with a reserved tag
/// was manufactured (the bug); `false` if the input was either cleanly
/// rejected or produced no elements at all (both acceptable outcomes).
fn try_read(label: &str, data: &[u8]) -> bool {
    let ts = EXPLICIT_VR_LITTLE_ENDIAN.erased();
    match InMemDicomObject::read_dataset_with_ts(data, &ts) {
        Ok(obj) => {
            let elements: Vec<_> = obj.iter().collect();
            if elements.is_empty() {
                println!("{label}: parsed to an empty object (delimiter cleanly ignored)");
                false
            } else {
                for elem in &elements {
                    println!(
                        "{label}: BUG - parsed to a top-level DataElement with reserved tag {} (vr {}, len {:?})",
                        elem.header().tag,
                        elem.header().vr(),
                        elem.header().length(),
                    );
                }
                true
            }
        }
        Err(e) => {
            println!("{label}: rejected with a clean parse error: {e}");
            false
        }
    }
}

fn main() {
    // Before the fix: the stray Item is misread as the start of a new,
    // bogus data set sequence (since decode_header() cannot tell it apart
    // from any other element with undefined length once VR is discarded),
    // and the stray SequenceDelimiter is misread as an ordinary top-level
    // element carrying a reserved tag - one that this crate's own encoder
    // refuses to write back out (see candidate 049), so any code path that
    // parses attacker-controlled bytes and later re-serializes what it read
    // (a common pattern - proxies, anonymizers, transcoders) can panic or
    // error on an input that "successfully" parsed moments earlier.
    //
    // After the fix: both are recognized for what they are - structural
    // markers with no business appearing here - and handled without ever
    // manufacturing an invalid `DataElement`.
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
             into an ordinary DataElement carrying a reserved tag."
        );
        std::process::exit(1);
    }

    println!("No corruption observed: stray markers are cleanly rejected or ignored.");
}
