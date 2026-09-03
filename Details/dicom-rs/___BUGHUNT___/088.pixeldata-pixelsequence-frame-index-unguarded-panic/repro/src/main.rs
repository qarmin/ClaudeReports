//! Standalone reproduction for candidate 088.
//!
//! Builds an ordinary, syntactically well-formed DICOM object (via this crate's own public
//! `FileMetaTableBuilder`/`InMemDicomObject`/`DataElement` API) whose Pixel Data element is
//! encapsulated (undefined length, item fragments) even though the declared transfer syntax is
//! plain Explicit VR Little Endian (a codec-free, non-encapsulated transfer syntax). The DICOM
//! parser decides whether an element is "encapsulated pixel data" purely from the element's own
//! tag + undefined length (see `dicom_core::header::HasLength::is_encapsulated_pixeldata`), not
//! from the declared transfer syntax, so this is a real object the parser will happily hand back
//! from `dicom_object::open_file` on a round-tripped file - not an internal-state hack.
//!
//! `NumberOfFrames` is set to 2, matching the 2 fragments actually present (so the buggy branch's
//! own `number_of_frames as usize == fragments.len()` guard passes) - then the real public
//! `PixelDecoder::decode_pixel_data_frame` entry point is called with an ordinary one-past-the-end
//! frame index (2), which is exactly the kind of off-by-one a caller could trivially make e.g. by
//! iterating `0..=number_of_frames` instead of `0..number_of_frames`.

use dicom_core::header::Length;
use dicom_core::value::PixelFragmentSequence;
use dicom_core::{DataElement, PrimitiveValue, Tag, VR, dicom_value};
use dicom_dictionary_std::tags;
use dicom_object::{FileDicomObject, FileMetaTableBuilder, InMemDicomObject};
use dicom_pixeldata::PixelDecoder;
use dicom_transfer_syntax_registry::entries::EXPLICIT_VR_LITTLE_ENDIAN;

fn build_obj() -> FileDicomObject<InMemDicomObject> {
    let meta = FileMetaTableBuilder::new()
        .transfer_syntax(EXPLICIT_VR_LITTLE_ENDIAN.uid())
        .media_storage_sop_class_uid("1.2.840.10008.5.1.4.1.1.7")
        .media_storage_sop_instance_uid("2.25.11111111111111111111111111111111111111")
        .build()
        .unwrap();
    let mut obj = FileDicomObject::new_empty_with_meta(meta);

    obj.put(DataElement::new(tags::ROWS, VR::US, PrimitiveValue::from(2u16)));
    obj.put(DataElement::new(tags::COLUMNS, VR::US, PrimitiveValue::from(2u16)));
    obj.put(DataElement::new(tags::BITS_ALLOCATED, VR::US, PrimitiveValue::from(8u16)));
    obj.put(DataElement::new(tags::BITS_STORED, VR::US, PrimitiveValue::from(8u16)));
    obj.put(DataElement::new(tags::HIGH_BIT, VR::US, PrimitiveValue::from(7u16)));
    obj.put(DataElement::new(tags::SAMPLES_PER_PIXEL, VR::US, PrimitiveValue::from(1u16)));
    obj.put(DataElement::new(tags::PIXEL_REPRESENTATION, VR::US, PrimitiveValue::from(0u16)));
    obj.put(DataElement::new(
        tags::PHOTOMETRIC_INTERPRETATION,
        VR::CS,
        dicom_value!(Str, "MONOCHROME2"),
    ));
    obj.put(DataElement::new(tags::NUMBER_OF_FRAMES, VR::IS, dicom_value!(Str, "2")));

    // Pixel Data, encapsulated (undefined length + fragments), 2 fragments -
    // matches NumberOfFrames above.
    obj.put(DataElement::new_with_len(
        Tag(0x7FE0, 0x0010),
        VR::OB,
        Length::UNDEFINED,
        PixelFragmentSequence::new(vec![], vec![vec![0u8; 4], vec![1u8; 4]]),
    ));

    obj
}

fn main() {
    // Round-trip through the real file writer/reader, so the object handed to
    // `decode_pixel_data_frame` is genuinely what `dicom_object::open_file` would hand back for
    // this byte stream, not merely an in-memory construction.
    let path = std::env::temp_dir().join("dicom_candidate_088_repro.dcm");
    build_obj().write_to_file(&path).unwrap();
    let obj = dicom_object::open_file(&path).unwrap();
    std::fs::remove_file(&path).ok();

    println!("== valid frame indices (0 and 1) ==");
    for f in 0..2u32 {
        match obj.decode_pixel_data_frame(f) {
            Ok(_) => println!("frame {f}: Ok"),
            Err(e) => println!("frame {f}: Err({e})"),
        }
    }

    println!("== out-of-range frame index (2) - fragments.len() == number_of_frames == 2 ==");
    println!("calling decode_pixel_data_frame(2)...");
    match obj.decode_pixel_data_frame(2) {
        Ok(_) => println!("frame 2: Ok (unexpected)"),
        Err(e) => println!("frame 2: Err({e}) (graceful error, bug is fixed)"),
    }
    println!("no panic occurred");
}
