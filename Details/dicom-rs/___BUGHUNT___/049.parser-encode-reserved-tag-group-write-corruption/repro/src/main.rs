use dicom_core::{DataElement, PrimitiveValue, Tag, VR};
use dicom_object::InMemDicomObject;
use dicom_transfer_syntax_registry::entries::EXPLICIT_VR_LITTLE_ENDIAN;

/// Writes a single data element with the given tag through the crate's own
/// encoder, then tries to read it back through the crate's own decoder.
///
/// Returns `true` if the round-trip is corrupted: either the write
/// "succeeds" but produces bytes the reader can't parse back (the bug,
/// pre-fix), or the write succeeds but decodes to the wrong tag/VR/value
/// (which would be an even worse silent corruption - not observed, but
/// checked for). Returns `false` if the write is cleanly rejected (the
/// fix) or if it round-trips correctly (the ordinary, non-reserved case).
fn write_and_read_back(tag: Tag) -> bool {
    let ts = EXPLICIT_VR_LITTLE_ENDIAN.erased();
    let obj = InMemDicomObject::from_element_iter([DataElement::new(
        tag,
        VR::UN,
        PrimitiveValue::Empty,
    )]);

    let mut buf = Vec::new();
    match obj.write_dataset_with_ts(&mut buf, &ts) {
        Err(e) => {
            println!("tag {tag}: write rejected: {e}");
            false
        }
        Ok(()) => {
            println!("tag {tag}: write succeeded, {} bytes: {:02x?}", buf.len(), buf);
            match InMemDicomObject::read_dataset_with_ts(buf.as_slice(), &ts) {
                Err(e) => {
                    println!("tag {tag}: BUG - wrote successfully but failed to read back: {e}");
                    true
                }
                Ok(read_back) => match read_back.element(tag) {
                    Ok(elem) if elem.header().tag == tag && elem.header().vr() == VR::UN => {
                        println!("tag {tag}: round-tripped correctly");
                        false
                    }
                    Ok(elem) => {
                        println!(
                            "tag {tag}: BUG - round-tripped to a different element: {:?}",
                            elem.header()
                        );
                        true
                    }
                    Err(e) => {
                        println!(
                            "tag {tag}: BUG - wrote successfully but element vanished on read back: {e}"
                        );
                        true
                    }
                },
            }
        }
    }
}

fn main() {
    // an ordinary, non-reserved tag: this must always round-trip correctly,
    // both before and after the fix.
    let ordinary = Tag(0x0009, 0x0001);

    // the three tags reserved for structural markers in group 0xFFFE. Any
    // header written with one of these tags is indistinguishable, on the
    // wire, from an Item / Item Delimitation / Sequence Delimitation marker
    // - which are always encoded as a bare tag+length with no VR field, in
    // every transfer syntax, even ones that otherwise always encode VR
    // explicitly.
    let item = Tag(0xFFFE, 0xE000);
    let item_delim = Tag(0xFFFE, 0xE00D);
    let seq_delim = Tag(0xFFFE, 0xE0DD);

    let ordinary_broken = write_and_read_back(ordinary);
    let item_broken = write_and_read_back(item);
    let item_delim_broken = write_and_read_back(item_delim);
    let seq_delim_broken = write_and_read_back(seq_delim);

    let bug_reproduced = !ordinary_broken && (item_broken || item_delim_broken || seq_delim_broken);

    if ordinary_broken {
        println!("UNEXPECTED: the ordinary, non-reserved tag itself failed to round-trip.");
        std::process::exit(2);
    }

    if bug_reproduced {
        println!(
            "BUG REPRODUCED: writing a data element with a reserved-group (0xFFFE) tag \
             produces output that the crate's own reader cannot parse back correctly."
        );
        std::process::exit(1);
    }

    println!("No corruption observed: reserved tags are now cleanly rejected at write time.");
}
