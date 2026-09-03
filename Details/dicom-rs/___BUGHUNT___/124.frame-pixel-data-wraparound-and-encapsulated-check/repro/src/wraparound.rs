//! Real, end-to-end, 64-bit genuine-wraparound-into-bounds reproduction for the native-
//! pixel-data branch of `dicom_encoding::adapters::PixelDataObject::frame_pixel_data`'s default
//! method (candidate 117's fix target, `Enet4/dicom-rs#817`).
//!
//! Candidate 117 fixed the panic (debug builds, overflow-checks on) but never independently
//! built a *release-mode* demonstration of the thing the original issue actually alleged is the
//! worse failure mode: `frame as usize * frame_size` wrapping `usize` all the way back around to
//! a SMALL value that still passes the existing `end <= pixel_data.len()` bounds check, so the
//! pre-fix code returns `Some(wrong_bytes)` instead of panicking or erroring. That needs a
//! `frame` (u32, so < 2**32) and `frame_size` (derived from Rows/Columns/SamplesPerPixel/
//! BitsAllocated, each individually legal per its own 16-bit DICOM VR) whose product is *just*
//! over a multiple of 2**64 - solved here via modular arithmetic (see the comment below for the
//! worked numbers), not picked by guesswork.
//!
//! Because `frame` is capped at u32::MAX, `frame * frame_size` can only ever reach 2**64 if
//! `frame_size` itself is already >= roughly 2**64 / 2**32 = 2**32 (~4.3 GB) - meaning the
//! resulting demonstration inherently requires a multi-gigabyte "pixel data" buffer to actually
//! exist (as opposed to the 32-bit case in the original issue, which could wrap with a modest
//! buffer). The buffer below is allocated zero-filled (`vec![0u8; N]`); on Linux this is backed
//! by lazily-committed anonymous pages, so the ~4.84 GiB virtual allocation does not require
//! ~4.84 GiB of *resident* memory up front - only the handful of bytes actually written/read are
//! ever faulted in.
//!
//! This is a single external artifact (the object's own declared attributes plus its own pixel
//! data bytes) with no second participant to fake - CONFIRMATION.md section 11's real-second-
//! participant requirement does not apply here, same as candidate 117's own repro.

use std::borrow::Cow;

use dicom_encoding::adapters::{PixelDataObject, RawPixelData};

// Worked modular-arithmetic solution (found by brute-force/CRT search over legal
// Rows/Columns/SamplesPerPixel/BitsAllocated combinations, see the bug-hunt session notes):
//
//   rows = 32768, columns = 20000, samples_per_pixel = 7, bits_allocated = 2 (=> 1 byte/sample)
//   frame_size = rows * columns * samples_per_pixel * bytes_per_sample
//              = 32768 * 20000 * 7 * 1 = 4_587_520_000
//   frame = 4_021_071_096   (< u32::MAX, an "ordinary-looking" large frame index)
//
//   true_product = frame * frame_size = 18_446_744_074_321_920_000
//                = 1 * 2**64 + 612_368_384
//   => on a 64-bit target with wrapping (release, overflow-checks off) arithmetic:
//        start = true_product % 2**64 = 612_368_384
//        end   = start + frame_size   = 5_199_888_384   (still < 2**64, no second wrap)
//
//   end (5_199_888_384 bytes, ~4.84 GiB) is small enough to actually allocate as the object's
//   real `pixel_data`, so pre-fix code's `end <= pixel_data.len()` check is satisfied and the
//   overflowed offset is used to slice real bytes out of the buffer - i.e. `Some(wrong_bytes)`,
//   not a panic or a graceful rejection.
const ROWS: u16 = 32768;
const COLUMNS: u16 = 20000;
const SAMPLES_PER_PIXEL: u16 = 7;
const BITS_ALLOCATED: u16 = 2;
const FRAME: u32 = 4_021_071_096;
const FRAME_SIZE: usize = 32768 * 20000 * 7 * 1;
const START: usize = 612_368_384;
const END: usize = START + FRAME_SIZE;

const MARKER: &[u8] = b"WRAPPED!";

struct Obj {
    pixel_data: Vec<u8>,
}

impl PixelDataObject for Obj {
    fn transfer_syntax_uid(&self) -> &str {
        "1.2.840.10008.1.2.1"
    }
    fn rows(&self) -> Option<u16> {
        Some(ROWS)
    }
    fn cols(&self) -> Option<u16> {
        Some(COLUMNS)
    }
    fn samples_per_pixel(&self) -> Option<u16> {
        Some(SAMPLES_PER_PIXEL)
    }
    fn bits_allocated(&self) -> Option<u16> {
        Some(BITS_ALLOCATED)
    }
    fn bits_stored(&self) -> Option<u16> {
        Some(BITS_ALLOCATED)
    }
    fn photometric_interpretation(&self) -> Option<&str> {
        Some("MONOCHROME2")
    }
    fn number_of_frames(&self) -> Option<u32> {
        Some(FRAME + 1)
    }
    fn number_of_fragments(&self) -> Option<u32> {
        None
    }
    fn fragment(&self, fragment: usize) -> Option<Cow<'_, [u8]>> {
        if fragment == 0 {
            Some(Cow::Borrowed(&self.pixel_data))
        } else {
            None
        }
    }
    fn offset_table(&self) -> Option<Cow<'_, [u32]>> {
        None
    }
    fn raw_pixel_data(&self) -> Option<RawPixelData> {
        Some(RawPixelData {
            fragments: vec![self.pixel_data.clone()].into(),
            offset_table: Default::default(),
        })
    }
}

fn main() {
    assert_eq!(FRAME_SIZE, 4_587_520_000);
    assert_eq!(START, 612_368_384);
    assert_eq!(END, 5_199_888_384);

    // Sanity-check the modular arithmetic in the same 64-bit wrapping semantics the actual
    // (unfixed) code performs, so a mismatch here (rather than a mismatch inside the library)
    // would fail loudly before we even allocate the multi-gigabyte buffer.
    let wrapped_start = (FRAME as u64).wrapping_mul(FRAME_SIZE as u64);
    assert_eq!(wrapped_start, START as u64, "modular arithmetic prediction is wrong");

    println!(
        "Allocating a {END} byte (~{:.2} GiB) zero-filled buffer (lazily committed on Linux, \
         only the marker bytes are actually touched)...",
        END as f64 / (1024.0 * 1024.0 * 1024.0)
    );
    let mut pixel_data = vec![0u8; END];
    // Plant a marker at the predicted wrapped `start` offset so the returned bytes (if any) can
    // be checked for exact provenance, not just "some Some(..) came back".
    pixel_data[START..START + MARKER.len()].copy_from_slice(MARKER);

    let obj = Obj { pixel_data };

    println!(
        "Calling frame_pixel_data({FRAME}) on a MONOCHROME2/{BITS_ALLOCATED}-bit/{ROWS}x{COLUMNS}/\
         samples_per_pixel={SAMPLES_PER_PIXEL} object (frame_size={FRAME_SIZE}, each attribute \
         individually legal for its own VR) - predicted wrapped start={START}, end={END}..."
    );

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        obj.frame_pixel_data(FRAME)
    }));

    match result {
        Ok(None) => {
            println!(
                "OK: no panic, returned None (the overflowing computation was rejected \
                 gracefully instead of wrapping into a small, spuriously in-bounds offset)."
            );
        }
        Ok(Some(value)) => {
            let starts_with_marker = value.len() >= MARKER.len() && &value[..MARKER.len()] == MARKER;
            println!(
                "UNEXPECTED: no panic, returned Some({} bytes); first {} bytes match the planted \
                 marker at the predicted wrapped offset {START}: {starts_with_marker}",
                value.len(),
                MARKER.len(),
            );
            if starts_with_marker {
                println!(
                    "WRAPAROUND CONFIRMED: frame_pixel_data({FRAME}) silently returned bytes read \
                     from usize-wrapped offset {START}..{END} instead of rejecting the request - \
                     exactly the release-mode 'silent frame corruption' the original issue alleged, \
                     now proven with a genuine 64-bit wraparound (true product = 2**64 + {START})."
                );
                std::process::exit(1);
            } else {
                std::process::exit(4);
            }
        }
        Err(payload) => {
            let message = payload
                .downcast_ref::<String>()
                .map(String::as_str)
                .or_else(|| payload.downcast_ref::<&str>().copied())
                .unwrap_or("<non-string panic payload>");
            println!("PANICKED: {message}");
            std::process::exit(3);
        }
    }
}
