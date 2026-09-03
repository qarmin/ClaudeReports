use dicom_core::header::Header;

/// A minimal DICOM stream (no 128-byte preamble needed for `from_reader`):
/// file meta group with only the mandatory Transfer Syntax UID (Media
/// Storage SOP Class UID is deliberately omitted, so it gets inferred from
/// the data set below), followed by a data set containing just a
/// SOPClassUID (0008,0016) element.
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
    // it gets inferred from the data set's own SOP Class UID - changing
    // `information_group_length`'s correct value, since
    // `FileMetaTable::build()` already computed it before this inference
    // step had a chance to run.
    let group_length_before = obj.meta().information_group_length;
    println!("information_group_length after first parse: {group_length_before}");

    let mut bytes = Vec::new();
    obj.write_all(&mut bytes).expect("should write");
    let obj2 = dicom_object::from_reader(bytes.as_slice()).expect("should re-parse");

    let group_length_after = obj2.meta().information_group_length;
    println!("information_group_length after round trip: {group_length_after}");

    let spilled: Vec<_> = obj2
        .iter()
        .filter(|elem| elem.tag().group() == 0x0002)
        .collect();

    let mut bug = false;
    if !spilled.is_empty() {
        println!(
            "BUG - {} file meta element(s) spilled into the main data set after a round trip: {:?}",
            spilled.len(),
            spilled.iter().map(|e| e.tag()).collect::<Vec<_>>()
        );
        bug = true;
    }
    if group_length_before != group_length_after {
        println!(
            "BUG - information_group_length changed across a round trip ({group_length_before} \
             -> {group_length_after}), meaning the first parse's value was already stale"
        );
        bug = true;
    }

    if bug {
        println!(
            "BUG REPRODUCED: the SOP Class UID inference fallback leaves \
             information_group_length stale, truncating the meta group on write."
        );
        std::process::exit(1);
    }

    println!("No corruption observed: information_group_length stays correct and stable.");
}
