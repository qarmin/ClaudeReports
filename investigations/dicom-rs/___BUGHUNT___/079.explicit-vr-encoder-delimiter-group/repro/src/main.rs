//! Standalone reproduction for candidate 079: the Explicit VR LE/BE encoders' shared
//! `Encode::encode_element_header` fast-path has no guard for tags in the reserved group
//! 0xFFFE (Item / Item Delimitation Item / Sequence Delimitation Item), which on the wire are
//! *always* encoded as a bare tag + 4-byte length (8 bytes total, no VR field, no reserved
//! bytes) - in every transfer syntax, including ones that otherwise always write VR explicitly.
//!
//! This drives `dicom-encoding`'s own public `Encode`/`Decode` traits directly (the same public
//! API a caller of this crate - independent of `dicom-parser` - would use), with no dependency
//! on any part of `dicom-parser` or `dicom-object`. It is a genuine round trip using this
//! crate's real encoder and its own matching real decoder, not a hand-authored stand-in for
//! either side.

use dicom_core::header::{DataElementHeader, HasLength, Header, Length};
use dicom_core::{Tag, VR};
use dicom_encoding::decode::Decode;
use dicom_encoding::decode::explicit_be::ExplicitVRBigEndianDecoder;
use dicom_encoding::decode::explicit_le::ExplicitVRLittleEndianDecoder;
use dicom_encoding::encode::Encode;
use dicom_encoding::encode::explicit_be::ExplicitVRBigEndianEncoder;
use dicom_encoding::encode::explicit_le::ExplicitVRLittleEndianEncoder;
use std::io::Cursor;

/// Encodes `de` with `enc`, then decodes the resulting bytes back with `dec`.
/// Returns `true` if the round trip preserved the tag and length (the correct
/// outcome), `false` if it was silently corrupted along the way.
macro_rules! check_roundtrip {
    ($label:expr, $enc:expr, $dec:expr, $de:expr) => {{
        let de = $de;
        let mut buf = Vec::new();
        match $enc.encode_element_header(&mut buf, de) {
            Ok(written) => {
                println!(
                    "[{}] wrote {written} bytes for tag {:?} vr={:?} len={:?}: {:02x?}",
                    $label,
                    de.tag(),
                    de.vr(),
                    de.length(),
                    buf
                );

                let mut cursor = Cursor::new(buf.clone());
                match $dec.decode_header(&mut cursor) {
                    Ok((header, consumed)) => {
                        println!(
                            "[{}] decoded back: tag={:?} vr={:?} len={:?}, consumed={consumed} of {} bytes written",
                            $label,
                            header.tag(),
                            header.vr(),
                            header.length(),
                            buf.len()
                        );
                        let ok = header.tag() == de.tag() && header.length() == de.length();
                        if !ok {
                            println!("[{}] MISMATCH: round trip corrupted the element", $label);
                        }
                        ok
                    }
                    Err(e) => {
                        println!("[{}] decode of the just-written bytes FAILED: {e}", $label);
                        false
                    }
                }
            }
            Err(e) => {
                println!("[{}] encode rejected the header outright: {e}", $label);
                // Rejecting the write is the *fixed* behavior: there is no valid
                // way to represent an ordinary element with a group-0xFFFE tag on
                // the wire, so refusing to write it at all is correct.
                true
            }
        }
    }};
}

fn main() {
    let le_enc = ExplicitVRLittleEndianEncoder::default();
    let le_dec = ExplicitVRLittleEndianDecoder::default();
    let be_enc = ExplicitVRBigEndianEncoder::default();
    let be_dec = ExplicitVRBigEndianDecoder::default();

    let mut all_ok = true;

    // Item (FFFE,E000), long-form VR (UN) - falls into the encoder's generic
    // 12-byte branch (tag + VR + 2 reserved bytes + 4-byte length) instead of
    // the required bare 8-byte tag+length shape.
    all_ok &= check_roundtrip!(
        "LE Item/UN",
        le_enc,
        le_dec,
        DataElementHeader::new(Tag(0xFFFE, 0xE000), VR::UN, Length(0))
    );

    // Item Delimitation Item (FFFE,E00D), short-form VR (UI) - happens to also
    // total 8 bytes, but the byte layout still differs: 2 bytes of VR text plus
    // a 2-byte short length replace what should be a 4-byte length field.
    all_ok &= check_roundtrip!(
        "BE ItemDelim/UI",
        be_enc,
        be_dec,
        DataElementHeader::new(Tag(0xFFFE, 0xE00D), VR::UI, Length(0))
    );

    // Sequence Delimitation Item (FFFE,E0DD), long-form VR (OB) on LE.
    all_ok &= check_roundtrip!(
        "LE SeqDelim/OB",
        le_enc,
        le_dec,
        DataElementHeader::new(Tag(0xFFFE, 0xE0DD), VR::OB, Length(0))
    );

    // Control case: an ordinary tag round-trips correctly either way, proving
    // the mismatch above is specific to group 0xFFFE, not a general encoder bug.
    all_ok &= check_roundtrip!(
        "LE ordinary tag",
        le_enc,
        le_dec,
        DataElementHeader::new(Tag(0x0010, 0x0010), VR::PN, Length(0))
    );

    if all_ok {
        println!("all round trips preserved tag+length: BUG FIXED");
        std::process::exit(0);
    } else {
        println!("BUG REPRODUCED: at least one group-0xFFFE header round-tripped incorrectly");
        std::process::exit(1);
    }
}
