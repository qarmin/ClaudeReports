use dicom_encoding::adapters::{
    EncodeOptions, EncodeResult, PixelDataObject, PixelDataWriter, RawPixelData,
};
use dicom_core::ops::AttributeOp;
use std::borrow::Cow;

/// A minimal 2-frame object; only `number_of_frames` is actually consulted by
/// `PixelDataWriter::encode`'s default implementation.
struct TwoFrameObject;

impl PixelDataObject for TwoFrameObject {
    fn transfer_syntax_uid(&self) -> &str {
        "1.2.3.4"
    }
    fn rows(&self) -> Option<u16> {
        Some(1)
    }
    fn cols(&self) -> Option<u16> {
        Some(1)
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
        Some(2)
    }
    fn number_of_fragments(&self) -> Option<u32> {
        None
    }
    fn fragment(&self, _fragment: usize) -> Option<Cow<'_, [u8]>> {
        None
    }
    fn offset_table(&self) -> Option<Cow<'_, [u32]>> {
        None
    }
    fn raw_pixel_data(&self) -> Option<RawPixelData> {
        None
    }
}

/// Writes frame 0 as 4 bytes and frame 1 as 6 bytes - distinct lengths so a
/// wrong cumulative computation is obvious.
struct IdentityPixelDataWriter;

impl PixelDataWriter for IdentityPixelDataWriter {
    fn encode_frame(
        &self,
        _src: &dyn PixelDataObject,
        frame: u32,
        _options: EncodeOptions,
        dst: &mut Vec<u8>,
    ) -> EncodeResult<Vec<AttributeOp>> {
        let len = if frame == 0 { 4 } else { 6 };
        dst.extend(std::iter::repeat(frame as u8).take(len));
        Ok(Vec::new())
    }
}

fn main() {
    let obj = TwoFrameObject;
    let writer = IdentityPixelDataWriter;
    let mut dst = Vec::new();
    let mut offset_table = Vec::new();

    writer
        .encode(&obj, EncodeOptions::default(), &mut dst, &mut offset_table)
        .unwrap();

    println!("frame lengths: {:?}", dst.iter().map(|f| f.len()).collect::<Vec<_>>());
    println!("offset table: {offset_table:?}");

    // per PS3.5 Annex A.4: entry[0] must always be 0, and entry[n] is the
    // cumulative byte offset (including the 8-byte item header of each
    // preceding fragment) to the start of frame n's own fragment.
    let expected = vec![0u32, 4 + 8];
    assert_eq!(
        offset_table, expected,
        "BOT entries must be cumulative offsets, not each frame's own length + a running count"
    );
    println!("offset table is correctly cumulative");
}
