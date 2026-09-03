// Standalone reproduction: a DNG CFAPattern tag of an unexpected byte length panics in
// rawler::cfa::CFA::new via CFA::new_from_tag.
//
// Real-user-reported, GitHub issue #586 on dnglab/dnglab, still open as of this session's sync
// (see ___BUGHUNT___/IDEAS.md). CFA::new only accepts pattern-string lengths of 0, 4, 16, 36, or
// 144 and panic!()s on any other length. CFA::new_from_tag builds that pattern string directly
// from a DNG/TIFF file's own CFAPattern tag (0x828E), whose element count is fully attacker-
// controlled TIFF tag data - any raw file whose CFAPattern tag has a count outside that fixed set
// (e.g. 2, the most natural "malformed" value) panics.
//
// This repro builds a spec-shaped DNG file (via rawler's own public low-level TIFF writer) with a
// CFAPattern tag of 2 elements, then opens it through rawler's real, public entry points
// `rawler::get_decoder` + `Decoder::raw_metadata` - the same call chain a real file opener uses,
// reached before raw_image/develop even run. No internals of the suspect function are called
// directly, and no second participant needs to be faked - a single external artifact (a file)
// parsed by its real public entry point, per CONFIRMATION.md section 11.

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

    let pixel_data: Vec<u16> = vec![100u16; (width * height * cpp as u32) as usize];
    let pixel_bytes: Vec<u8> = pixel_data.iter().flat_map(|v| v.to_le_bytes()).collect();

    let out_path = std::env::temp_dir().join("bughunt_012_cfapattern.dng");
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
    // Malformed: CFAPattern with 2 elements. CFA::new() only accepts lengths 0, 4, 16, 36, 144.
    root.add_tag(TiffCommonTag::CFAPattern, [0u8, 1].as_slice());

    let data_offset = tiff.write_data(&pixel_bytes).expect("write strip data");
    root.add_tag(TiffCommonTag::StripOffsets, data_offset);
    root.add_tag(TiffCommonTag::StripByteCounts, pixel_bytes.len() as u32);

    tiff.build(root).expect("build tiff");
    println!("wrote DNG with 2-element CFAPattern to {:?}", out_path);

    let source = RawSource::new(&out_path).expect("open raw source");
    let decoder = get_decoder(&source).expect("get_decoder should select the DNG decoder");
    match decoder.raw_metadata(&source, &RawDecodeParams::default()) {
        Ok(_) => println!("raw_metadata returned Ok (bug not present)"),
        Err(e) => println!("raw_metadata returned an error, no panic (bug not present): {e}"),
    }
}
