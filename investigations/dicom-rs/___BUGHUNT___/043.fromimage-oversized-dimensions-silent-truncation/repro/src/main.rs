use dicom_core::{DataElement, VR, value::PrimitiveValue};
use dicom_object::{FileDicomObject, FileMetaTableBuilder};
use image::{DynamicImage, GenericImageView, GrayImage};

// `dicom-fromimage` is a binary-only crate (no library target), so this repro
// can't call its private `update_from_img` function directly the way other
// candidates' repro/ crates call into a library. Instead, this replicates
// the exact vulnerable computation from `fromimage/src/main.rs` verbatim
// (see patch.diff's context for the real source lines this mirrors):
//
//     obj.put(DataElement::new(tags::COLUMNS, VR::US, PrimitiveValue::from(width as u16)));
//     obj.put(DataElement::new(tags::ROWS, VR::US, PrimitiveValue::from(height as u16)));
//     ...
//     let pixeldata = img.into_bytes();
//     obj.put(DataElement::new(tags::PIXEL_DATA, ..., PrimitiveValue::from(pixeldata)));
//
// demonstrating that the stored Columns/Rows silently disagree with the
// actual PixelData buffer size for an oversized image, exactly as
// `fromimage`'s own `update_from_img` does before the fix.
fn main() {
    let width = 70_000u32;
    let img = DynamicImage::ImageLuma8(GrayImage::new(width, 1));
    let (real_width, real_height) = img.dimensions();

    let mut obj = FileDicomObject::new_empty_with_meta(
        FileMetaTableBuilder::new()
            .transfer_syntax("1.2.840.10008.1.2.1")
            .media_storage_sop_class_uid("1.2.840.10008.5.1.4.1.1.7")
            .media_storage_sop_instance_uid("2.25.221545913343461877066277885826230616209")
            .build()
            .unwrap(),
    );

    // the exact pre-fix computation
    obj.put(DataElement::new(
        dicom_dictionary_std::tags::COLUMNS,
        VR::US,
        PrimitiveValue::from(real_width as u16),
    ));
    obj.put(DataElement::new(
        dicom_dictionary_std::tags::ROWS,
        VR::US,
        PrimitiveValue::from(real_height as u16),
    ));

    let pixeldata = img.into_bytes();
    obj.put(DataElement::new(
        dicom_dictionary_std::tags::PIXEL_DATA,
        VR::OB,
        PrimitiveValue::from(pixeldata.clone()),
    ));

    let stored_columns: u16 = obj
        .element(dicom_dictionary_std::tags::COLUMNS)
        .unwrap()
        .to_int()
        .unwrap();

    println!("real image width: {real_width}, stored Columns: {stored_columns}");
    println!("PixelData bytes: {} (matches real width, not stored Columns)", pixeldata.len());

    if stored_columns as u32 == real_width {
        println!("no truncation occurred for this input");
        std::process::exit(0);
    }

    println!(
        "BUG: Columns ({stored_columns}) does not describe the actual PixelData buffer \
         ({} bytes, real width {real_width}) - the file is self-inconsistent.",
        pixeldata.len()
    );
    std::process::exit(1);
}
