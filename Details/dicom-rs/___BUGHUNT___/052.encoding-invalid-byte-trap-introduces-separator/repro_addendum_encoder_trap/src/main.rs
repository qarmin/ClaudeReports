//! Addendum repro for candidate 052: this candidate's own fix (`decode_text_trap` substituting
//! `U+FFFD` for an undecodable byte, instead of the old octal-escape sequence) makes a value
//! that had to be substituted permanently unwritable, if the active character set codec's own
//! `encode()` uses `EncoderTrap::Strict` (every codec in `encoding/src/text.rs` does) - `U+FFFD`
//! is outside every DICOM-supported character set's repertoire except UTF-8-based ones.
//!
//! This is a REAL crash discovered by this session's own stacked-fuzzing of the official,
//! tracked `object/fuzz/fuzz_targets/open_file.rs` target (with candidate 052's patch, among
//! others, applied) - `crash-input.bin` in this folder is the exact minimized artifact libFuzzer
//! wrote out. Independently re-confirmed afterward that candidate 052's patch ALONE (no other
//! candidate's patch) is sufficient to reproduce this: the crashing element is `(0008,0016)`
//! (VR UI, an "always default charset" VR per PS3.5 6.1.2.3), whose raw bytes contain one byte
//! `DefaultCharacterSetCodec`/ISO-8859-1 can decode, but which the encoding crate's ISO-8859-1
//! `Encoding` implementation still cannot re-encode as `U+FFFD` (the trap's own substitution
//! character) once decoded - see RESULT.md's Addendum section for the full mechanism.
//!
//! This exercises the real, public `dicom_object::OpenFileOptions::from_reader` +
//! `FileDicomObject::write_all` entry points on a single external artifact (an untrusted byte
//! stream) - no second participant to fake, so CONFIRMATION.md section 11 doesn't apply here.

fn main() {
    let data: &[u8] = include_bytes!("../crash-input.bin");

    let mut obj = dicom_object::OpenFileOptions::new()
        .read_preamble(dicom_object::file::ReadPreamble::Auto)
        .odd_length_strategy(dicom_object::file::OddLengthStrategy::Fail)
        .from_reader(data)
        .expect("this crafted input is expected to parse successfully");

    // matches the official fuzz harness's own preprocessing step
    for g in 0..=0x07FFu16 {
        obj.remove_element(dicom_object::Tag(g, 0x0000));
    }

    let mut bytes = Vec::new();
    match obj.write_all(&mut bytes) {
        Ok(()) => {
            println!(
                "OK: wrote {} bytes successfully (the lossily-decoded value was re-encoded, \
                 not rejected)",
                bytes.len()
            );
        }
        Err(e) => {
            println!("write failed: {e}");
            println!(
                "BUG REPRODUCED: a value that parsed successfully (via candidate 052's own \
                 lossy U+FFFD substitution for an undecodable byte) cannot be written back out \
                 at all, because U+FFFD is itself outside the active codec's own repertoire and \
                 the encoder uses EncoderTrap::Strict."
            );
            std::process::exit(1);
        }
    }
}
