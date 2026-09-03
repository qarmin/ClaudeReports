use dicom_dictionary_std::tags;
use dicom_object::InMemDicomObject;
use dicom_transfer_syntax_registry::{TransferSyntaxIndex, TransferSyntaxRegistry};

fn el_short(group: u16, elem: u16, vr: &[u8; 2], value: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(&group.to_le_bytes());
    out.extend_from_slice(&elem.to_le_bytes());
    out.extend_from_slice(vr);
    out.extend_from_slice(&(value.len() as u16).to_le_bytes());
    out.extend_from_slice(value);
    out
}

fn build_wire() -> Vec<u8> {
    let mut wire = Vec::new();
    wire.extend_from_slice(&0x0008u16.to_le_bytes());
    wire.extend_from_slice(&0x1110u16.to_le_bytes());
    wire.extend_from_slice(b"SQ");
    wire.extend_from_slice(&[0, 0]);
    wire.extend_from_slice(&0xFFFFFFFFu32.to_le_bytes());

    let mut item = Vec::new();
    item.extend(el_short(0x0008, 0x0005, b"CS", b"ISO_IR 126"));
    item.extend(el_short(0x0010, 0x0010, b"PN", &[0xE1, 0x20]));

    wire.extend_from_slice(&0xFFFEu16.to_le_bytes());
    wire.extend_from_slice(&0xE000u16.to_le_bytes());
    wire.extend_from_slice(&(item.len() as u32).to_le_bytes());
    wire.extend_from_slice(&item);

    wire.extend_from_slice(&0xFFFEu16.to_le_bytes());
    wire.extend_from_slice(&0xE0DDu16.to_le_bytes());
    wire.extend_from_slice(&0u32.to_le_bytes());

    // Top-level PatientName after the sequence - the top level never declares its own
    // SpecificCharacterSet, so this byte should decode under the default repertoire
    // (Latin-1: 0xE1 = 'a with acute').
    wire.extend(el_short(0x0010, 0x0010, b"PN", &[0xE1, 0x20]));
    wire
}

fn main() {
    let ts = TransferSyntaxRegistry
        .get("1.2.840.10008.1.2.1")
        .expect("explicit VR little endian should be registered");

    let wire = build_wire();
    let obj = InMemDicomObject::read_dataset_with_ts(&wire[..], ts).expect("should parse");

    let inner = obj
        .get(tags::REFERENCED_STUDY_SEQUENCE)
        .expect("sequence should be present")
        .items()
        .expect("should have items")[0]
        .element(tags::PATIENT_NAME)
        .expect("item should have PatientName")
        .to_str()
        .expect("should decode as string")
        .to_string();

    let top = obj
        .element(tags::PATIENT_NAME)
        .expect("top-level PatientName should be present")
        .to_str()
        .expect("should decode as string")
        .to_string();

    println!("item-scoped PatientName (Greek, ISO_IR 126 declared inside the item): {:?}", inner);
    println!("top-level PatientName (no charset declared at top level, should be default/Latin-1): {:?}", top);

    let mut failed = false;

    if top == inner {
        eprintln!(
            "REPRODUCED (decode leak) - the item's own SpecificCharacterSet leaked past the item \
             boundary: top-level PatientName decoded identically to the item-scoped one ({:?}) \
             instead of under the default character repertoire",
            top
        );
        failed = true;
    }

    // Round-trip check: write the correctly-decoded object back out and re-parse it. If the
    // encoder has the same leak (in the opposite direction: the item's charset switch bleeding
    // into the top-level element's *encoding*), this either errors outright (encoding a Latin-1
    // char under a Greek-only codec) or silently produces the wrong bytes.
    let mut out = Vec::new();
    match obj.write_dataset_with_ts(&mut out, ts) {
        Ok(()) => {
            let reparsed =
                InMemDicomObject::read_dataset_with_ts(&out[..], ts).expect("re-parse should succeed");
            let top_after = reparsed
                .element(tags::PATIENT_NAME)
                .expect("top-level PatientName should still be present")
                .to_str()
                .expect("should decode as string")
                .to_string();
            println!("top-level PatientName after write+reparse: {:?}", top_after);
            if top_after != top {
                eprintln!(
                    "REPRODUCED (encode leak) - round trip changed the top-level PatientName: {:?} -> {:?}",
                    top, top_after
                );
                failed = true;
            }
        }
        Err(e) => {
            eprintln!(
                "REPRODUCED (encode leak) - writing the correctly-decoded object failed: {:?}",
                e
            );
            failed = true;
        }
    }

    if failed {
        std::process::exit(1);
    }

    println!("did not reproduce - charset correctly scoped to the item on both decode and encode");
}
