use dicom_core::Tag;
use dicom_object::InMemDicomObject;
use dicom_transfer_syntax_registry::{TransferSyntaxIndex, TransferSyntaxRegistry};

fn short_vr_element(tag: Tag, vr: &[u8; 2], value: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(&tag.group().to_le_bytes());
    out.extend_from_slice(&tag.element().to_le_bytes());
    out.extend_from_slice(vr);
    out.extend_from_slice(&(value.len() as u16).to_le_bytes());
    out.extend_from_slice(value);
    out
}

fn main() {
    let ts = TransferSyntaxRegistry
        .get("1.2.840.10008.1.2.1")
        .expect("explicit VR little endian should be registered");

    // A two-element explicit-VR-LE data set:
    //   elem1: (0028,0106) VR=US, declared value length 3 - NOT a multiple of
    //          2 (the element size for US), which cannot occur from a
    //          spec-conformant writer but is trivially producible by a
    //          corrupted or malicious file.
    //   elem2: (0010,0010) VR=PN "AB" - a completely normal, well-formed
    //          element that should parse correctly on its own.
    let mut wire = Vec::new();
    wire.extend(short_vr_element(Tag(0x0028, 0x0106), b"US", &[0x01, 0x02, 0x03]));
    wire.extend(short_vr_element(Tag(0x0010, 0x0010), b"PN", b"AB"));

    println!("wire bytes ({} total): {:02x?}", wire.len(), wire);

    match InMemDicomObject::read_dataset_with_ts(&wire[..], ts) {
        Ok(obj) => {
            println!("did not reproduce - parsed successfully:");
            for elem in obj.iter() {
                println!(
                    "  {:?} vr={:?} value={:?}",
                    elem.header().tag,
                    elem.header().vr(),
                    elem.value()
                );
            }
        }
        Err(e) => {
            let msg = format!("{e:?}");
            println!("parse error (Display): {e}");
            println!("parse error (Debug, truncated): {}", &msg[..msg.len().min(400)]);

            // Pre-fix: the misaligned length silently truncates (n = len >> 1
            // = 1, so only 2 of the declared 3 value bytes are actually read
            // from the stream), while the decoder's own position bookkeeping
            // still advances by the full declared length (3). The leftover,
            // unconsumed byte then gets misinterpreted as the start of the
            // *next* element's header, corrupting the parse of elem2 (a
            // perfectly well-formed element) with an opaque, misleading
            // error - never mentioning the real cause (elem1's odd length).
            //
            // Post-fix: the misaligned length is rejected immediately, at
            // the exact element that caused it, with a clear, specific
            // error - and the source chain contains "InvalidValueLength",
            // not a generic header-decode/EOF failure blamed on elem2.
            if msg.contains("InvalidValueLength") {
                println!(
                    "FIXED - the malformed length was rejected immediately and specifically, \
                     with no corruption of the unrelated, well-formed second element"
                );
            } else {
                println!(
                    "REPRODUCED - the malformed length was not rejected at its own element; \
                     instead, parsing failed later while trying to interpret leftover, \
                     unconsumed bytes as a new element header - proof that the stream cursor \
                     desynchronized from the decoder's own position bookkeeping"
                );
                std::process::exit(1);
            }
        }
    }
}
