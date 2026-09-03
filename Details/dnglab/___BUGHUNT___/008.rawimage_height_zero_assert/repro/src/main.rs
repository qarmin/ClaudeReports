// Standalone reproduction: RawImage's constructor path panics on ImageLength=0.
//
// Builds a spec-shaped (but semantically degenerate: ImageLength=0, ImageWidth=5) uncompressed DNG
// file on disk using rawler's own public low-level TIFF writer (`formats::tiff::writer`), then
// decodes it through rawler's real, public, end-to-end entry point (`rawler::analyze::raw_to_srgb`,
// the same function `dnglab analyze --srgb <file>` calls). No internals of the suspect function are
// called directly, and no second participant needs to be faked - this is a single external artifact
// (a file) parsed by its real public entry point, per CONFIRMATION.md section 11.
//
// `raw_to_srgb` (rather than `rawler::decode_file`) is used deliberately: `decode_file` goes
// through `RawLoader::decode`, which wraps decoding in `panic::catch_unwind` and turns any panic
// into a returned `Err` - so the process itself never actually crashes there, it just returns an
// error. `raw_to_srgb` calls `decoder.raw_image(...)` directly, with no such wrapper, so it shows
// the real, uncaught-by-the-library panic a calling application (like `dnglab analyze --srgb`)
// would actually see.

use rawler::analyze::raw_to_srgb;
use rawler::decoders::RawDecodeParams;
use rawler::formats::tiff::writer::TiffWriter;
use rawler::tags::{DngTag, TiffCommonTag};
use std::fs::File;
use std::io::BufWriter;

fn main() {
    let width: u32 = 5;
    let height: u32 = 0;
    let bps: u16 = 16;
    let cpp: u16 = 1;

    let pixel_data: Vec<u16> = vec![100u16; (width * height * cpp as u32) as usize];
    let pixel_bytes: Vec<u8> = pixel_data.iter().flat_map(|v| v.to_le_bytes()).collect();

    let out_path = std::env::temp_dir().join("bughunt_008_zeroheight.dng");
    let buf = BufWriter::new(File::create(&out_path).expect("create dng file"));
    let mut tiff = TiffWriter::new(buf).expect("tiff writer init");

    let mut root = tiff.new_directory();
    root.add_tag(DngTag::DNGVersion, [1u8, 4, 0, 0].as_slice());
    root.add_tag(TiffCommonTag::Make, "TESTCORP");
    root.add_tag(TiffCommonTag::Model, "TESTCAM");
    root.add_tag(TiffCommonTag::ImageWidth, width);
    root.add_tag(TiffCommonTag::ImageLength, height);
    root.add_tag(TiffCommonTag::BitsPerSample, bps);
    root.add_tag(TiffCommonTag::SamplesPerPixel, cpp);
    root.add_tag(TiffCommonTag::Compression, 1u16); // uncompressed
    root.add_tag(TiffCommonTag::PhotometricInt, 32803u16); // CFA
    root.add_tag(TiffCommonTag::RowsPerStrip, height);
    root.add_tag(TiffCommonTag::CFAPattern, [0u8, 1, 1, 2].as_slice()); // RGGB

    let data_offset = tiff.write_data(&pixel_bytes).expect("write strip data");
    root.add_tag(TiffCommonTag::StripOffsets, data_offset);
    root.add_tag(TiffCommonTag::StripByteCounts, pixel_bytes.len() as u32);

    tiff.build(root).expect("build tiff");
    println!("wrote ImageWidth=5, ImageLength=0 DNG to {:?}", out_path);

    match raw_to_srgb(&out_path, &RawDecodeParams::default()) {
        Ok(image) => println!("decoded ok (bug not present): {}x{}", image.width(), image.height()),
        Err(e) => println!("returned an error, no panic (bug not present): {e}"),
    }
}
