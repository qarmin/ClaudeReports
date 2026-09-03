//! Real, end-to-end reproduction of upstream issue `Enet4/dicom-rs#698`
//! ("Potential mismatch between self.position and reader position in StatefulDecoder"): a
//! real, non-malicious DICOM file with a multi-valued UL tag whose declared value length (10
//! bytes) is 2 bytes larger than what its element count actually needs (2 x UL = 8 bytes) -
//! dcmtk and dcmjs both parse this file fine (skipping the 2 trailing garbage bytes), but
//! dicom-rs (as of `df2b9e42`, before this candidate's fix) fails right after this tag,
//! because it never consumes those 2 bytes from the actual stream even though its own
//! position bookkeeping assumes it did - corrupting the parse of the next, perfectly
//! well-formed element. This is the exact live, real-world manifestation of candidate 108's
//! already-confirmed root cause (parser/src/stateful/decode.rs's `len >> k` VR readers), just
//! discovered independently via a real user's file rather than fuzzing.
//!
//! Drives the real public `InMemDicomObject::read_dataset_with_ts` entry point on
//! hand-encoded but genuinely valid-shape Explicit VR Little Endian wire bytes (this is a
//! single external artifact - the file itself - with no second participant to fake; per
//! CONFIRMATION.md section 11 this single-process shape is the correct one here, same as
//! candidate 108's own repro).

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

    // elem1: a private-looking tag, VR=UL, declared value length 10 - not a multiple of 4
    //        (the element size for UL). 2 whole UL values (8 bytes: 0x00000001, 0x00000002)
    //        plus 2 trailing garbage bytes (0xAA, 0xBB), matching the shape of the real file
    //        attached to #698 (a VM=2 UL tag declaring length 10 instead of 8).
    // elem2: (0010,0010) VR=PN "AB" - a completely ordinary, well-formed element that a
    //        correct/lenient reader (dcmtk, dcmjs) parses without any trouble.
    let mut elem1_value = Vec::new();
    elem1_value.extend_from_slice(&1u32.to_le_bytes());
    elem1_value.extend_from_slice(&2u32.to_le_bytes());
    elem1_value.extend_from_slice(&[0xAA, 0xBB]); // 2 trailing garbage bytes
    assert_eq!(elem1_value.len(), 10);

    let mut wire = Vec::new();
    wire.extend(short_vr_element(Tag(0x0009, 0x0010), b"UL", &elem1_value));
    wire.extend(short_vr_element(Tag(0x0010, 0x0010), b"PN", b"AB"));

    println!("wire bytes ({} total): {:02x?}", wire.len(), wire);

    match InMemDicomObject::read_dataset_with_ts(&wire[..], ts) {
        Ok(obj) => {
            let elem1 = obj.element(Tag(0x0009, 0x0010));
            let elem2 = obj.element(Tag(0x0010, 0x0010));
            println!("elem1 (0009,0010) = {elem1:?}");
            println!("elem2 (0010,0010) = {elem2:?}");

            let elem1_ok = matches!(
                elem1.ok().and_then(|e| e.value().to_multi_int::<u32>().ok()),
                Some(v) if v == vec![1u32, 2u32]
            );
            let elem2_ok = matches!(
                elem2.ok().and_then(|e| e.value().to_str().ok()),
                Some(s) if s.trim() == "AB"
            );

            if elem1_ok && elem2_ok {
                println!(
                    "FIXED: the misaligned length was tolerated (2 trailing garbage bytes \
                     skipped) - elem1 correctly parsed as [1, 2], and the following \
                     well-formed elem2 parsed correctly too, matching dcmtk/dcmjs's own \
                     lenient behavior on this exact file shape."
                );
            } else {
                println!("UNEXPECTED: parsed, but not with the expected values.");
                std::process::exit(2);
            }
        }
        Err(e) => {
            println!("parse error: {e}");
            println!(
                "BUG REPRODUCED: the file failed to parse at all (either via a corrupted \
                 parse of elem2 caused by the 2 unconsumed garbage bytes from elem1 - the \
                 original desync bug, candidate 108 - or via a strict rejection of elem1's \
                 own misaligned length) - a real-world file that other DICOM toolkits (dcmtk, \
                 dcmjs) parse without issue."
            );
            std::process::exit(1);
        }
    }
}
