use rawler::analyze::raw_to_srgb;
use rawler::decoders::RawDecodeParams;
use rawler::formats::tiff::writer::{DirectoryWriter, TiffWriter};
use rawler::tags::{DngTag, TiffCommonTag};
use std::fs::File;
use std::io::BufWriter;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let width: u32 = args.get(1).map(|s| s.parse().unwrap()).unwrap_or(0);
    let height: u32 = args.get(2).map(|s| s.parse().unwrap()).unwrap_or(5);
    let bps: u16 = 16;
    let cpp: u16 = 1;

    let pixel_data: Vec<u16> = vec![100u16; (width * height * cpp as u32) as usize];
    let pixel_bytes: Vec<u8> = pixel_data.iter().flat_map(|v| v.to_le_bytes()).collect();

    let out_path = std::env::temp_dir().join("bughunt_width0.dng");
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
    root.add_tag(TiffCommonTag::Compression, 1u16);
    root.add_tag(TiffCommonTag::PhotometricInt, 32803u16); // CFA
    root.add_tag(TiffCommonTag::RowsPerStrip, height);
    root.add_tag(TiffCommonTag::CFAPattern, [0u8, 1, 1, 2].as_slice()); // RGGB

    let data_offset = tiff.write_data(&pixel_bytes).expect("write strip data");
    root.add_tag(TiffCommonTag::StripOffsets, data_offset);
    root.add_tag(TiffCommonTag::StripByteCounts, pixel_bytes.len() as u32);

    tiff.build(root).expect("build tiff");
    println!("wrote dng to {:?}", out_path);

    match raw_to_srgb(&out_path, &RawDecodeParams::default()) {
        Ok(image) => println!("decoded ok: {}x{}", image.width(), image.height()),
        Err(e) => println!("returned error, no panic: {e}"),
    }
}
