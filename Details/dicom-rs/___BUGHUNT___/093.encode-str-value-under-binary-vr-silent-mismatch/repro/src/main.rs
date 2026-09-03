use dicom_core::{DataElement, PrimitiveValue, Tag, VR};
use dicom_dictionary_std::uids;
use dicom_object::{FileDicomObject, FileMetaTableBuilder, InMemDicomObject};

fn main() {
    let meta = FileMetaTableBuilder::new()
        .media_storage_sop_class_uid(uids::SECONDARY_CAPTURE_IMAGE_STORAGE)
        .media_storage_sop_instance_uid("2.25.222222222222222222222222222222222222222")
        .transfer_syntax(uids::EXPLICIT_VR_LITTLE_ENDIAN)
        .build()
        .unwrap();

    let mut obj: FileDicomObject<InMemDicomObject> = FileDicomObject::new_empty_with_meta(meta);

    // A private tag, declared as VR::US (a binary "unsigned short" VR per PS3.5), but the
    // actual stored PrimitiveValue is a textual `Str` - a mismatch the public `DataElement::new`
    // constructor explicitly documents that it will NOT check. This is the mirror direction of
    // candidate 092 (binary value under a textual VR): here a textual value is put under a
    // binary VR instead.
    let tag = Tag(0x0009, 0x0010);
    let original_text = "hello".to_string();
    obj.put_element(DataElement::new(
        tag,
        VR::US,
        PrimitiveValue::Str(original_text.clone()),
    ));

    let mut buf = Vec::new();
    let write_result = obj.write_dataset_with_ts(
        &mut buf,
        &dicom_transfer_syntax_registry::entries::EXPLICIT_VR_LITTLE_ENDIAN.erased(),
    );

    let buf = match write_result {
        Err(e) => {
            println!(
                "did not reproduce - write_dataset_with_ts returned a graceful error instead \
                 of silently writing a text value under a binary-VR header: {e}"
            );
            std::process::exit(0);
        }
        Ok(()) => buf,
    };

    println!(
        "write succeeded with no error/panic - {} bytes written",
        buf.len()
    );
    println!("raw element bytes: {:02x?}", &buf);

    // Header per PS3.5 7.1.2 for a "short form" VR like US: 4 bytes tag, 2 bytes VR, 2 bytes length.
    let tag_bytes = &buf[0..4];
    let vr_bytes = &buf[4..6];
    let len_bytes = &buf[6..8];
    let value_bytes = &buf[8..];
    let declared_len = u16::from_le_bytes([len_bytes[0], len_bytes[1]]);
    println!(
        "declared header: tag={:02x?} vr={:?} len={}",
        tag_bytes,
        std::str::from_utf8(vr_bytes).unwrap(),
        declared_len
    );
    println!("value bytes on the wire: {:02x?}", value_bytes);

    // PS3.5 6.2 requires VR US's value field to be one or more 2-byte unsigned integers -
    // by construction the value field here is the ASCII text "hello" plus a padding byte,
    // not a well-formed sequence of 16-bit integers in any meaningful sense.
    println!(
        "value bytes are the literal ASCII text {:?} instead of binary u16 values",
        std::str::from_utf8(value_bytes).unwrap_or("<not valid utf8>")
    );

    // Round-trip through the crate's own reader to show this isn't just a byte-level
    // curiosity: the same mismatch corrupts what a real reader observes too.
    let mut file_buf = Vec::new();
    obj.write_all(&mut file_buf)
        .expect("full file write should succeed");

    let read_back = FileDicomObject::<InMemDicomObject>::from_reader(&file_buf[..]);
    match read_back {
        Ok(obj2) => {
            let elem = obj2.element(tag).expect("element should be present");
            println!(
                "round-tripped element value (VR={:?}): {:?}",
                elem.vr(),
                elem.value()
            );
            println!(
                "REPRODUCED: element declared as VR::US (binary u16) was written with the \
                 original text {:?}'s raw bytes as the value field, and the crate's own reader \
                 happily parsed those bytes back as {} native-endian u16 values instead of \
                 raising any error anywhere in the write-then-read cycle - the original text \
                 is gone, silently replaced by nonsense numbers.",
                original_text,
                value_bytes.len() / 2
            );
            std::process::exit(1);
        }
        Err(e) => {
            println!(
                "round-trip read failed with: {e} -- this too demonstrates the corruption \
                 (write produced a file the crate's own reader cannot cleanly interpret as the \
                 declared VR expects), though the write side raised no error at all."
            );
            std::process::exit(1);
        }
    }
}
