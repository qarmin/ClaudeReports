use dicom_core::smallvec::smallvec;
use dicom_encoding::adapters::{PixelDataObject, PixelDataReader, RawPixelData};
use dicom_transfer_syntax_registry::{TransferSyntaxIndex, TransferSyntaxRegistry};
use flate2::Compression;
use flate2::write::DeflateEncoder;
use std::borrow::Cow;
use std::io::Write;
use std::time::Instant;

struct FakePixelData {
    fragment: Vec<u8>,
}

impl PixelDataObject for FakePixelData {
    fn transfer_syntax_uid(&self) -> &str {
        "1.2.840.10008.1.2.8.1"
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
        Some(RawPixelData {
            fragments: smallvec![self.fragment.clone()],
            offset_table: smallvec![],
        })
    }
}

fn main() {
    // Declares an image that is 4x4x1 byte (16 bytes) - but the fragment actually decompresses
    // to 256 MiB of all-zero bytes, a ~16-million-to-one compression ratio, achievable from a
    // tiny amount of real compressed data.
    const TARGET_SIZE: usize = 256 * 1024 * 1024;
    let raw = vec![0u8; TARGET_SIZE];
    let mut encoder = DeflateEncoder::new(Vec::new(), Compression::best());
    encoder.write_all(&raw).unwrap();
    let compressed = encoder.finish().unwrap();
    drop(raw);

    println!(
        "compressed fragment: {} bytes, decompresses to {} bytes ({}x amplification)",
        compressed.len(),
        TARGET_SIZE,
        TARGET_SIZE / compressed.len().max(1),
    );

    let ts = TransferSyntaxRegistry
        .get("1.2.840.10008.1.2.8.1")
        .expect("Deflated Image Frame Compression should be registered");
    let reader = ts
        .pixel_data_reader()
        .expect("should have a pixel data reader (the `deflate` feature is enabled)");

    let src = FakePixelData { fragment: compressed };
    let mut dst = Vec::new();

    let start = Instant::now();
    let result = reader.decode(&src, &mut dst);
    let elapsed = start.elapsed();

    match &result {
        Ok(()) => println!(
            "decoded {} bytes for a declared 4x4x1-byte (16-byte) image in {:?}",
            dst.len(),
            elapsed
        ),
        Err(e) => println!("decode returned an error in {elapsed:?}: {e:?}"),
    }

    if result.is_ok() && dst.len() > 1024 * 1024 {
        eprintln!(
            "REPRODUCED - decode() applied no cap at all on decompressed output size: produced \
             {} bytes ({:.1} MiB) of output for an image whose own declared dimensions call for \
             only 16 bytes",
            dst.len(),
            dst.len() as f64 / (1024.0 * 1024.0)
        );
        std::process::exit(1);
    }

    // Sanity check: a legitimately-sized fragment (exactly 16 bytes once decompressed, matching
    // the declared 4x4x1-byte image) must still decode successfully - the fix must not reject
    // valid data along with oversized data.
    let legit_raw: Vec<u8> = (0..16).collect();
    let mut legit_encoder = DeflateEncoder::new(Vec::new(), Compression::default());
    legit_encoder.write_all(&legit_raw).unwrap();
    let legit_compressed = legit_encoder.finish().unwrap();
    let legit_src = FakePixelData {
        fragment: legit_compressed,
    };
    let mut legit_dst = Vec::new();
    reader
        .decode(&legit_src, &mut legit_dst)
        .expect("a correctly-sized legitimate fragment should still decode successfully");
    assert_eq!(legit_dst, legit_raw, "legitimate data should decode byte-for-byte correctly");
    println!("sanity check passed: a correctly-sized legitimate fragment still decodes correctly");

    println!("did not reproduce - decompressed output was reasonably bounded");
}
