use dicom_core::{DataElement, PrimitiveValue, VR};
use dicom_dictionary_std::{tags, uids};
use dicom_object::{FileMetaTableBuilder, InMemDicomObject};

fn main() {
    let out_path = std::env::args().nth(1).unwrap();
    let obj = InMemDicomObject::from_element_iter([
        DataElement::new(tags::SOP_CLASS_UID, VR::UI, uids::SECONDARY_CAPTURE_IMAGE_STORAGE),
        DataElement::new(tags::SOP_INSTANCE_UID, VR::UI, "1.2.3.4.5.6.7.8.9\0"),
        DataElement::new(tags::PATIENT_NAME, VR::PN, "Small^Test"),
        DataElement::new(tags::PATIENT_ID, VR::LO, "SMALL001"),
        DataElement::new(tags::MODALITY, VR::CS, "OT"),
        DataElement::new(tags::ROWS, VR::US, PrimitiveValue::from(1u16)),
        DataElement::new(tags::COLUMNS, VR::US, PrimitiveValue::from(1u16)),
        DataElement::new(tags::BITS_ALLOCATED, VR::US, PrimitiveValue::from(8u16)),
        DataElement::new(tags::BITS_STORED, VR::US, PrimitiveValue::from(8u16)),
        DataElement::new(tags::HIGH_BIT, VR::US, PrimitiveValue::from(7u16)),
        DataElement::new(tags::PIXEL_REPRESENTATION, VR::US, PrimitiveValue::from(0u16)),
        DataElement::new(tags::SAMPLES_PER_PIXEL, VR::US, PrimitiveValue::from(1u16)),
        DataElement::new(tags::PHOTOMETRIC_INTERPRETATION, VR::CS, "MONOCHROME2"),
        DataElement::new(tags::PIXEL_DATA, VR::OB, PrimitiveValue::from(vec![0x55u8])),
    ]);
    let meta = FileMetaTableBuilder::new()
        .media_storage_sop_class_uid(uids::SECONDARY_CAPTURE_IMAGE_STORAGE)
        .media_storage_sop_instance_uid("1.2.3.4.5.6.7.8.9")
        .transfer_syntax(uids::EXPLICIT_VR_LITTLE_ENDIAN)
        .build()
        .unwrap();
    obj.with_exact_meta(meta).write_to_file(&out_path).unwrap();
    println!("wrote small file to {out_path}");
}
