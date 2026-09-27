use dicom_core::{DataElement, PrimitiveValue, VR, value::DataSetSequence};
use dicom_dictionary_std::tags;
use dicom_object::{InMemDicomObject, open_file};
use dicom_pixeldata::PixelDecoder;

fn main() {
    let test_file = dicom_test_files::path("pydicom/CT_small.dcm")
        .expect("test file should be available");
    let mut obj = open_file(&test_file).expect("test file should parse");

    // Build a conformant VOI LUT Sequence item per PS3.3 C.7.6.3.1.5: LUT Descriptor's first
    // value of 0 means the LUT has 65536 entries (not "0 entries" - 0 is a documented special
    // case, not an invalid/empty value).
    let lut_data: Vec<u16> = (0..=u16::MAX).collect(); // 65536 entries, one per u16 value
    assert_eq!(lut_data.len(), 65536);

    let voi_lut_item = InMemDicomObject::from_element_iter([
        DataElement::new(
            tags::LUT_DESCRIPTOR,
            VR::US,
            PrimitiveValue::U16(vec![0u16, 0u16, 16u16].into()), // [0 -> 65536 entries, min_pixel_value=0, bits_stored=16]
        ),
        DataElement::new(
            tags::LUT_DATA,
            VR::OW,
            PrimitiveValue::U16(lut_data.into()),
        ),
    ]);

    obj.put_element(DataElement::new(
        tags::VOILUT_SEQUENCE,
        VR::SQ,
        DataSetSequence::from(vec![voi_lut_item]),
    ));

    let decoded = obj.decode_pixel_data().expect("pixel data should decode");

    match decoded.voi_lut_sequence() {
        Ok(Some(seq)) => {
            println!(
                "did not reproduce - VOI LUT Sequence correctly recognized, {} entr{}",
                seq.len(),
                if seq.len() == 1 { "y" } else { "ies" }
            );
        }
        Ok(None) => {
            eprintln!(
                "REPRODUCED - a conformant VOI LUT Sequence (LUT Descriptor[0]=0, meaning 65536 \
                 entries per PS3.3 C.7.6.3.1.5) was silently dropped: voi_lut_sequence() returned \
                 None instead of Some(1 entry)"
            );
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("REPRODUCED (different symptom) - voi_lut_sequence() returned an error: {:?}", e);
            std::process::exit(1);
        }
    }
}
