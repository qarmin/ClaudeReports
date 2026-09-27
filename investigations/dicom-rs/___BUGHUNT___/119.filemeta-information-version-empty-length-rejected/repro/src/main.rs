//! Real, end-to-end reproduction of upstream issue `Enet4/dicom-rs#565`: a real file (from a
//! NewTom CT scanner, per the reporter) has a File Meta Information Version element (0002,0001)
//! that is *present* but declared with length 0 (empty) instead of the normal 2 bytes.
//! dicom-rs (as of `df2b9e42`, before this candidate's fix) rejects the *entire file* with
//! `UnexpectedDataValueLength`, even though a genuinely *missing* File Meta Information Version
//! element is already tolerated with a documented default fallback ([0, 1], see meta.rs's own
//! "See #28"/PR #29 comment) - the file being merely *present-but-empty* should not be treated
//! any more strictly than *absent*.
//!
//! Drives the real public `dicom_object::from_reader` entry point on a minimal, hand-encoded
//! but genuinely valid-shape file meta group (single external artifact, no second participant
//! to fake - CONFIRMATION.md section 11's single-process shape applies here, same as candidates
//! 053/108/118's own repros for meta-parsing/decode bugs).

#[rustfmt::skip]
const DATA: &[u8] = &[
    b'D', b'I', b'C', b'M',
    // File Meta Information Group Length (0002,0000) UL 4, value = 40 (0x28)
    0x02, 0x00, 0x00, 0x00, b'U', b'L', 0x04, 0x00, 0x28, 0x00, 0x00, 0x00,
    // File Meta Information Version (0002,0001) OB, reserved(2), length 0 (EMPTY - the bug
    // trigger, matching the real NewTom scanner file described in the issue)
    0x02, 0x00, 0x01, 0x00, b'O', b'B', 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    // Transfer Syntax UID (0002,0010) UI 20 "1.2.840.10008.1.2.1\0" (Explicit VR LE)
    0x02, 0x00, 0x10, 0x00, b'U', b'I', 0x14, 0x00,
    b'1', b'.', b'2', b'.', b'8', b'4', b'0', b'.', b'1', b'0', b'0', b'0', b'8',
    b'.', b'1', b'.', b'2', b'.', b'1', 0x00,
];

fn main() {
    match dicom_object::from_reader(DATA) {
        Ok(obj) => {
            println!(
                "OK: file parsed successfully. information_version = {:?} (expected the \
                 documented [0, 1] default, same as a genuinely missing element)",
                obj.meta().information_version
            );
            if obj.meta().information_version != [0, 1] {
                println!("UNEXPECTED: parsed, but with a surprising information_version value");
                std::process::exit(2);
            }
        }
        Err(e) => {
            println!("parse error: {e}");
            println!(
                "BUG REPRODUCED: the entire file was rejected because of a single, \
                 non-essential, present-but-empty File Meta Information Version element - \
                 even though a genuinely MISSING one is already tolerated with a documented \
                 default fallback."
            );
            std::process::exit(1);
        }
    }
}
