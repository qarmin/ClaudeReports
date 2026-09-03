use dicom_encoding::adapters::{PixelDataObject, RawPixelData};
use dicom_transfer_syntax_registry::{TransferSyntaxIndex, TransferSyntaxRegistry};
use std::borrow::Cow;

struct FakePixelData {
    fragment: Vec<u8>,
}

impl PixelDataObject for FakePixelData {
    fn transfer_syntax_uid(&self) -> &str {
        "1.2.840.10008.1.2.5"
    }
    fn rows(&self) -> Option<u16> {
        Some(4)
    }
    fn cols(&self) -> Option<u16> {
        Some(4)
    }
    fn samples_per_pixel(&self) -> Option<u16> {
        Some(1)
    }
    fn bits_allocated(&self) -> Option<u16> {
        Some(8)
    }
    fn bits_stored(&self) -> Option<u16> {
        Some(8)
    }
    fn photometric_interpretation(&self) -> Option<&str> {
        Some("MONOCHROME2")
    }
    fn number_of_frames(&self) -> Option<u32> {
        Some(1)
    }
    fn number_of_fragments(&self) -> Option<u32> {
        Some(1)
    }
    fn fragment(&self, fragment: usize) -> Option<Cow<'_, [u8]>> {
        if fragment == 0 {
            Some(Cow::Borrowed(&self.fragment))
        } else {
            None
        }
    }
    fn offset_table(&self) -> Option<Cow<'_, [u32]>> {
        None
    }
    fn raw_pixel_data(&self) -> Option<RawPixelData> {
        None
    }
}

fn main() {
    // A well-formed RLE header (1 segment, matching BitsAllocated=8/SamplesPerPixel=1) whose
    // single segment's PackBits-encoded content decodes to just 1 byte - far short of the 16
    // bytes (rows=4 * cols=4) the pixel-interleaving loop expects from it.
    let mut fragment = Vec::new();
    fragment.extend_from_slice(&1u32.to_le_bytes()); // NumberOfSegments = 1
    fragment.extend_from_slice(&8u32.to_le_bytes()); // segment 0 starts at byte 8
    fragment.push(0x00); // PackBits header: copy 1 literal byte
    fragment.push(0xAB); // the 1 literal byte

    let ts = TransferSyntaxRegistry
        .get("1.2.840.10008.1.2.5")
        .expect("RLE Lossless should be registered");
    let reader = ts
        .pixel_data_reader()
        .expect("RLE Lossless should have a pixel data reader (the `rle` feature is enabled)");

    let src = FakePixelData { fragment };
    let mut dst = Vec::new();

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        reader.decode(&src, &mut dst)
    }));

    match result {
        Ok(Ok(())) => println!("did not reproduce - decode unexpectedly succeeded"),
        Ok(Err(e)) => println!(
            "did not reproduce - decode failed gracefully with a normal Result::Err: {:?}",
            e
        ),
        Err(panic_payload) => {
            eprintln!("REPRODUCED - panicked: {:?}", panic_payload);
            std::process::exit(1);
        }
    }
}
