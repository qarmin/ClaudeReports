use dicom_core::{DataElement, PrimitiveValue, Tag, VR, dicom_value};
use dicom_object::{FileDicomObject, FileMetaTableBuilder};
use dicom_pixeldata::PixelDecoder;

fn main() {
    // A DICOM object describing a legacy overlay plane embedded in the pixel
    // data (no Overlay Data element), with Rows/Columns near u16::MAX and
    // Number of Frames in Overlay near i32::MAX (both individually legal for
    // their respective VRs - US and IS), and a real, tiny PixelData buffer -
    // nowhere near what such a large declared overlay would actually need.
    let mut obj = FileDicomObject::new_empty_with_meta(
        FileMetaTableBuilder::new()
            .transfer_syntax("1.2.840.10008.1.2.1")
            .media_storage_sop_class_uid("1.2.840.10008.5.1.4.1.1.7")
            .media_storage_sop_instance_uid("2.25.221545913343461877066277885826230616209")
            .build()
            .unwrap(),
    );

    obj.put_element(DataElement::new(
        dicom_dictionary_std::tags::ROWS,
        VR::US,
        dicom_value!(U16, 65535),
    ));
    obj.put_element(DataElement::new(
        dicom_dictionary_std::tags::COLUMNS,
        VR::US,
        dicom_value!(U16, 65535),
    ));
    obj.put_element(DataElement::new(
        dicom_dictionary_std::tags::BITS_ALLOCATED,
        VR::US,
        dicom_value!(U16, 16),
    ));
    obj.put_element(DataElement::new(
        dicom_dictionary_std::tags::SAMPLES_PER_PIXEL,
        VR::US,
        dicom_value!(U16, 1),
    ));
    obj.put_element(DataElement::new(
        dicom_dictionary_std::tags::PIXEL_DATA,
        VR::OW,
        PrimitiveValue::U16(vec![0u16; 4].into()),
    ));

    let group = 0x6000u16;
    obj.put_element(DataElement::new(Tag(group, 0x0010), VR::US, dicom_value!(U16, 65535)));
    obj.put_element(DataElement::new(Tag(group, 0x0011), VR::US, dicom_value!(U16, 65535)));
    obj.put_element(DataElement::new(Tag(group, 0x0040), VR::CS, PrimitiveValue::from("R")));
    obj.put_element(DataElement::new(Tag(group, 0x0050), VR::SS, dicom_value!(I16, [1, 1])));
    obj.put_element(DataElement::new(Tag(group, 0x0100), VR::US, dicom_value!(U16, 16)));
    obj.put_element(DataElement::new(Tag(group, 0x0102), VR::US, dicom_value!(U16, 15)));
    obj.put_element(DataElement::new(
        Tag(group, 0x0015),
        VR::IS,
        PrimitiveValue::from(i32::MAX.to_string()),
    ));
    // Image Frame Origin encoded wider than its dictionary VR (US) would
    // carry - representative of a non-conformant/crafted file, which this
    // crate does not reject on VR-vs-dictionary mismatch.
    obj.put_element(DataElement::new(
        Tag(group, 0x0051),
        VR::UL,
        dicom_value!(U32, [i32::MAX as u32]),
    ));

    println!("decoding overlay group {group:#06X}...");
    let result = obj.decode_overlay(0);
    println!("result: {result:?}");
}
