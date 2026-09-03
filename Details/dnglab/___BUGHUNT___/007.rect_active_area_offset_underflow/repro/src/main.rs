// Standalone reproduction: DNG ActiveArea larger than the image panics in Rect::as_ltrb_offsets.
//
// This bug was independently reported by a real user (via an automated fuzzer bot) against a
// real DNG file - GitHub issue #582 on dnglab/dnglab, still open as of this session's sync (see
// ___BUGHUNT___/IDEAS.md). This repro reconstructs the same class of trigger by hand: a spec-shaped
// DNG whose ActiveArea tag describes a rectangle larger than the image's own ImageWidth/ImageLength,
// built via rawler's own public low-level TIFF writer, then opened through rawler's real, public
// entry points `rawler::get_decoder` + `Decoder::raw_metadata` - the exact call chain the reported
// issue's own repro harness uses (get_decoder -> raw_metadata, before raw_image/develop even run).
// No internals of the suspect function are called directly, and no second participant needs to be
// faked - a single external artifact (a file) parsed by its real public entry point, per
// CONFIRMATION.md section 11.

use rawler::decoders::RawDecodeParams;
use rawler::formats::tiff::writer::TiffWriter;
use rawler::get_decoder;
use rawler::rawsource::RawSource;
use rawler::tags::{DngTag, TiffCommonTag};
use std::fs::File;
use std::io::BufWriter;

fn main() {
    let width: u32 = 100;
    let height: u32 = 100;
    let bps: u16 = 16;
    let cpp: u16 = 1;

    // ActiveArea in DNG's own [top, left, bottom, right] order, deliberately larger than the
    // image itself (bottom/right both exceed height/width).
    let active_area: [u32; 4] = [0, 0, height + 500, width + 500];

    let pixel_data: Vec<u16> = vec![100u16; (width * height * cpp as u32) as usize];
    let pixel_bytes: Vec<u8> = pixel_data.iter().flat_map(|v| v.to_le_bytes()).collect();

    let out_path = std::env::temp_dir().join("bughunt_007_activearea.dng");
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
    root.add_tag(DngTag::ActiveArea, active_area.as_slice());

    let data_offset = tiff.write_data(&pixel_bytes).expect("write strip data");
    root.add_tag(TiffCommonTag::StripOffsets, data_offset);
    root.add_tag(TiffCommonTag::StripByteCounts, pixel_bytes.len() as u32);

    tiff.build(root).expect("build tiff");
    println!("wrote DNG with oversized ActiveArea to {:?}", out_path);

    let source = RawSource::new(&out_path).expect("open raw source");
    let decoder = get_decoder(&source).expect("get_decoder should select the DNG decoder");
    match decoder.raw_metadata(&source, &RawDecodeParams::default()) {
        Ok(_) => println!("raw_metadata returned Ok (bug not present)"),
        Err(e) => println!("raw_metadata returned an error, no panic (bug not present): {e}"),
    }
}
