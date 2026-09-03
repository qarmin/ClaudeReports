use rawler::RawImageData;
use rawler::analyze::raw_to_srgb;
use rawler::cfa::CFA;
use rawler::decoders::Camera;
use rawler::decoders::RawDecodeParams;
use rawler::dng::writer::DngWriter;
use rawler::dng::{CropMode, DNG_VERSION_V1_4, DngCompression, DngPhotometricConversion};
use rawler::rawimage::{CFAConfig, RawImage, RawPhotometricInterpretation};
use std::fs::File;
use std::io::BufWriter;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let width: usize = args.get(1).map(|s| s.parse().unwrap()).unwrap_or(5);
    let height: usize = args.get(2).map(|s| s.parse().unwrap()).unwrap_or(5);
    let cpp = 1usize;

    let mut cam = Camera::default();
    cam.make = "TESTCORP".into();
    cam.model = "TESTCAM".into();
    cam.clean_make = "TESTCORP".into();
    cam.clean_model = "TESTCAM".into();
    cam.cfa = CFA::new("RGGB");
    cam.whitelevel = Some(vec![4095; cpp]);

    let pixels: Vec<u16> = vec![100u16; width * height * cpp];
    let photometric = RawPhotometricInterpretation::Cfa(CFAConfig::new_from_camera(&cam));

    let rawimage = RawImage::new_with_data(
        cam,
        RawImageData::Integer(pixels),
        width * cpp,
        height,
        cpp,
        [1.0, 1.0, 1.0, f32::NAN],
        photometric,
        None,
        None,
        false,
    );

    let out_path = std::env::temp_dir().join("bughunt_tiny.dng");
    let buf = BufWriter::new(File::create(&out_path).expect("create dng file"));
    let mut dng = DngWriter::new(buf, DNG_VERSION_V1_4).expect("dng writer init");
    {
        let mut raw = dng.subframe(0);
        raw.raw_image(&rawimage, CropMode::None, DngCompression::Uncompressed, DngPhotometricConversion::Original, 1)
            .expect("write raw_image");
        raw.finalize().expect("finalize subframe");
    }
    dng.close().expect("close dng");
    println!("wrote dng to {:?}", out_path);

    // Now decode it through the real, public end-to-end entry point.
    let image = raw_to_srgb(&out_path, &RawDecodeParams::default()).expect("raw_to_srgb should not crash");
    println!("decoded ok: {}x{}", image.width(), image.height());
}
