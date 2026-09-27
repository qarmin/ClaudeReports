//! Real, end-to-end reproduction using the public `dicom_encoding::adapters::PixelDataObject`
//! trait's default `frame_pixel_data` method, on a custom implementor built with the exact
//! shape given in upstream issue `Enet4/dicom-rs#817` ("dicom-encoding: unchecked usize
//! arithmetic leading to silent frame corruption") - a real, if adversarially-sized, set of
//! attribute values (Rows/Columns/SamplesPerPixel/BitsAllocated, all individually legal per
//! their own dictionary VR, plus an ordinary-looking large frame index) reaching the crate's
//! real, public, unauthenticated entry point.
//!
//! This is a single external artifact (the object's own declared attributes) with no second
//! participant to fake - CONFIRMATION.md section 11's real-second-participant requirement does
//! not apply here (compare candidates 086/087, the sibling bug in `pixeldata/src/lib.rs`, which
//! used the same single-process shape).

use std::borrow::Cow;

use dicom_encoding::adapters::{PixelDataObject, RawPixelData};

struct Obj {
    rows: u16,
    columns: u16,
    samples_per_pixel: u16,
    bits_allocated: u16,
    photometric_interpretation: &'static str,
    number_of_frames: u32,
    flat_pixel_data: Vec<u8>,
}

impl PixelDataObject for Obj {
    fn transfer_syntax_uid(&self) -> &str {
        "1.2.840.10008.1.2.1"
    }
    fn rows(&self) -> Option<u16> {
        Some(self.rows)
    }
    fn cols(&self) -> Option<u16> {
        Some(self.columns)
    }
    fn samples_per_pixel(&self) -> Option<u16> {
        Some(self.samples_per_pixel)
    }
    fn bits_allocated(&self) -> Option<u16> {
        Some(self.bits_allocated)
    }
    fn bits_stored(&self) -> Option<u16> {
        Some(self.bits_allocated)
    }
    fn photometric_interpretation(&self) -> Option<&str> {
        Some(self.photometric_interpretation)
    }
    fn number_of_frames(&self) -> Option<u32> {
        Some(self.number_of_frames)
    }
    fn number_of_fragments(&self) -> Option<u32> {
        None
    }
    fn fragment(&self, fragment: usize) -> Option<Cow<'_, [u8]>> {
        if fragment == 0 {
            Some(Cow::Borrowed(&self.flat_pixel_data))
        } else {
            None
        }
    }
    fn offset_table(&self) -> Option<Cow<'_, [u32]>> {
        None
    }
    fn raw_pixel_data(&self) -> Option<RawPixelData> {
        Some(RawPixelData {
            fragments: vec![self.flat_pixel_data.clone()].into(),
            offset_table: Default::default(),
        })
    }
}

fn main() {
    // Every attribute value here is individually legal for its own DICOM VR:
    // Rows/Columns/SamplesPerPixel/BitsAllocated are all `US` (max 65535), NumberOfFrames
    // is `IS`/`UL`-backed (max u32::MAX as read by this crate). No single field is
    // out-of-range - only their *product*, combined with an ordinary-looking large frame
    // index, overflows `usize` arithmetic in `frame_pixel_data`'s native-pixel-data branch.
    let obj = Obj {
        rows: 65535,
        columns: 65535,
        samples_per_pixel: 3,
        bits_allocated: 16,
        photometric_interpretation: "RGB",
        number_of_frames: u32::MAX,
        flat_pixel_data: vec![0u8; 1024],
    };

    let last_frame = obj.number_of_frames().unwrap() - 1;
    println!(
        "Calling frame_pixel_data({last_frame}) on a RGB/16-bit/65535x65535/{}-frame object \
         (each attribute individually legal for its own VR)...",
        obj.number_of_frames().unwrap()
    );

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        obj.frame_pixel_data(last_frame)
    }));

    match result {
        Ok(None) => {
            println!(
                "OK: no panic, returned None (the overflowing computation was rejected \
                 gracefully instead of wrapping/panicking)."
            );
        }
        Ok(Some(value)) => {
            println!(
                "UNEXPECTED: no panic, but returned Some({} bytes) instead of None - possible \
                 silent frame corruption (wrapped offset happened to land in bounds)",
                value.len()
            );
            std::process::exit(4);
        }
        Err(payload) => {
            let message = payload
                .downcast_ref::<String>()
                .map(String::as_str)
                .or_else(|| payload.downcast_ref::<&str>().copied())
                .unwrap_or("<non-string panic payload>");
            println!("PANICKED: {message}");
            if message.contains("multiply with overflow") || message.contains("add with overflow") {
                println!(
                    "BUG REPRODUCED: frame_pixel_data's native-pixel-data offset arithmetic \
                     overflowed usize on an ordinary (if large) ,individually-legal frame index, \
                     instead of returning a graceful None/error."
                );
                std::process::exit(1);
            } else {
                println!("Panicked, but not with the expected overflow message.");
                std::process::exit(3);
            }
        }
    }
}
