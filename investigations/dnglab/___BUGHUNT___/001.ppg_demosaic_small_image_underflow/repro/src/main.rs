// Standalone reproduction: PPG demosaic integer underflow on tiny raw images.
//
// Builds a tiny (5x5, RGGB, uncompressed) but otherwise spec-valid DNG file on disk using
// rawler's own public DngWriter, then decodes it back through rawler's real, public
// end-to-end entry point (`rawler::analyze::raw_to_srgb`, the same function the `dnglab
// analyze --srgb <file>` CLI command calls) - the same path used to decode a genuine
// malformed/crafted raw file. No internals of the suspect function are called directly.
//
// Must be run WITHOUT --release: the bug is a `usize` subtraction underflow gated behind
// Rust's overflow checks, which dnglab's own [profile.release] (and vanilla `--release`)
// disables. In a debug/debug-assertions build this panics deterministically. In release it
// silently wraps instead of panicking, which removes the interior-region bound that keeps
// this code's raw-pointer pixel access in range - see RESULT.md for why that is not "fine".

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
    let width = 5usize;
    let height = 5usize;
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

    let out_path = std::env::temp_dir().join("bughunt_001_ppg_tiny.dng");
    let buf = BufWriter::new(File::create(&out_path).expect("create dng file"));
    let mut dng = DngWriter::new(buf, DNG_VERSION_V1_4).expect("dng writer init");
    {
        let mut raw = dng.subframe(0);
        raw.raw_image(&rawimage, CropMode::None, DngCompression::Uncompressed, DngPhotometricConversion::Original, 1)
            .expect("write raw_image");
        raw.finalize().expect("finalize subframe");
    }
    dng.close().expect("close dng");
    println!("wrote {}x{} DNG to {:?}", width, height, out_path);

    // Real, public, end-to-end entry point - same one `dnglab analyze --srgb` uses.
    let image = raw_to_srgb(&out_path, &RawDecodeParams::default()).expect("raw_to_srgb should not crash");
    println!("decoded ok: {}x{}", image.width(), image.height());
}
