use dicom_core::value::C;
use dicom_core::{DataElement, PrimitiveValue, Tag, VR};
use dicom_dictionary_std::uids;
use dicom_object::{FileDicomObject, FileMetaTableBuilder, InMemDicomObject};

fn main() {
    let meta = FileMetaTableBuilder::new()
        .media_storage_sop_class_uid(uids::SECONDARY_CAPTURE_IMAGE_STORAGE)
        .media_storage_sop_instance_uid("2.25.111111111111111111111111111111111111111")
        .transfer_syntax(uids::EXPLICIT_VR_LITTLE_ENDIAN)
        .build()
        .unwrap();

    let mut obj: FileDicomObject<InMemDicomObject> = FileDicomObject::new_empty_with_meta(meta);

    // A private tag, declared as VR::UI (a textual UID VR per PS3.5), but the actual
    // stored PrimitiveValue is a binary U32 array - a mismatch the public `DataElement::new`
    // constructor explicitly documents that it will NOT check.
    let tag = Tag(0x0009, 0x0010);
    let original_numbers: Vec<u32> = vec![0xDEADBEEF, 0x00000001, 0xFFFFFFFF];
    obj.put_element(DataElement::new(
        tag,
        VR::UI,
        PrimitiveValue::U32(C::from_vec(original_numbers.clone())),
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
                 of silently writing corrupted bytes: {e}"
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

    // Header per PS3.5 7.1.2 for a "short form" VR like UI: 4 bytes tag, 2 bytes VR, 2 bytes length.
    let tag_bytes = &buf[0..4];
    let vr_bytes = &buf[4..6];
    let len_bytes = &buf[6..8];
    let value_bytes = &buf[8..];
    println!(
        "declared header: tag={:02x?} vr={:?} len={}",
        tag_bytes,
        std::str::from_utf8(vr_bytes).unwrap(),
        u16::from_le_bytes([len_bytes[0], len_bytes[1]])
    );
    println!("value bytes on the wire: {:02x?}", value_bytes);

    // PS3.5 6.2 requires VR UI's value field to consist only of characters from the
    // DICOM default character repertoire (basically printable ASCII plus backslash),
    // used to encode one or more UIDs. Demonstrate the actual bytes violate that.
    let is_valid_ui_text = value_bytes
        .iter()
        .all(|&b| b == 0 || (0x20..0x7f).contains(&b));
    println!("value bytes are valid DICOM UI text bytes: {is_valid_ui_text}");

    if !is_valid_ui_text {
        println!(
            "REPRODUCED: element declared as VR::UI contains raw binary bytes of the original \
             U32 values {:x?}, not a valid UID text string - the file written to disk is \
             spec-violating even though write_dataset_with_ts() returned Ok(()) with no error.",
            original_numbers
        );
    }

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
            if !is_valid_ui_text {
                println!(
                    "REPRODUCED (round-trip): the reader happily parsed the corrupted bytes as \
                     VR::UI text under the declared header, silently reinterpreting the original \
                     numeric bytes as a nonsense string, with no error raised anywhere in the \
                     write-then-read cycle."
                );
                std::process::exit(1);
            } else {
                println!("did not reproduce - value bytes were valid UI text after all");
            }
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
