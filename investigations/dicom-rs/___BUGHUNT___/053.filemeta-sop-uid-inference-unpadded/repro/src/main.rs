/// A minimal DICOM stream (no 128-byte preamble needed for `from_reader`):
/// file meta group with only the mandatory Transfer Syntax UID (Media
/// Storage SOP Class UID is deliberately omitted), followed by a data set
/// containing just a SOPClassUID (0008,0016) element whose value is
/// "1.2.3.4\0" - 8 bytes, already padded to even length with one trailing
/// NUL, as any correctly-encoded DICOM value must be.
#[rustfmt::skip]
const DATA: &[u8] = &[
    b'D', b'I', b'C', b'M',
    // File Meta Information Group Length (0002,0000) UL 4
    0x02, 0x00, 0x00, 0x00, b'U', b'L', 0x04, 0x00, 0x1c, 0x00, 0x00, 0x00,
    // Transfer Syntax UID (0002,0010) UI 20 "1.2.840.10008.1.2.1\0" (Explicit VR LE)
    0x02, 0x00, 0x10, 0x00, b'U', b'I', 0x14, 0x00,
    b'1', b'.', b'2', b'.', b'8', b'4', b'0', b'.', b'1', b'0', b'0', b'0', b'8',
    b'.', b'1', b'.', b'2', b'.', b'1', 0x00,
    // data set: SOPClassUID (0008,0016) UI 8 "1.2.3.4\0"
    0x08, 0x00, 0x16, 0x00, b'U', b'I', 0x08, 0x00,
    b'1', b'.', b'2', b'.', b'3', b'.', b'4', 0x00,
];

fn main() {
    let obj = dicom_object::from_reader(DATA).expect("should parse");

    // Media Storage SOP Class UID is missing from the file meta group, so
    // it gets inferred from the data set's own SOP Class UID.
    let sop_class_uid = obj.meta().media_storage_sop_class_uid.clone();
    println!(
        "inferred media_storage_sop_class_uid = {:?} (byte len {})",
        sop_class_uid,
        sop_class_uid.len()
    );

    let mut bytes = Vec::new();
    obj.write_all(&mut bytes).expect("should write");
    let obj2 = dicom_object::from_reader(bytes.as_slice()).expect("should re-parse");
    let sop_class_uid2 = obj2.meta().media_storage_sop_class_uid.clone();
    println!(
        "after round trip: media_storage_sop_class_uid = {:?} (byte len {})",
        sop_class_uid2,
        sop_class_uid2.len()
    );

    let mut bug = false;
    if sop_class_uid.len() % 2 != 0 {
        println!(
            "BUG - the inferred Media Storage SOP Class UID has odd length ({}), which is \
             invalid DICOM and will silently grow by one byte the next time this object is \
             written out",
            sop_class_uid.len()
        );
        bug = true;
    }
    if sop_class_uid != sop_class_uid2 {
        println!("BUG - Media Storage SOP Class UID did not survive a write/re-read round trip");
        bug = true;
    }

    if bug {
        println!(
            "BUG REPRODUCED: the SOP Class UID inference fallback stores an unpadded, \
             odd-length value that does not survive its own round trip."
        );
        std::process::exit(1);
    }

    println!("No corruption observed: the inferred UID has even length and round-trips stably.");
}
