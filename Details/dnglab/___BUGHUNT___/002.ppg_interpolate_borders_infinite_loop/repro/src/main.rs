// Standalone reproduction: PPG demosaic infinite loop (hang / DoS) for images 4-5px wide.
//
// Builds a tiny (4x8) but otherwise spec-valid uncompressed RGGB DNG file on disk using
// rawler's own public DngWriter, then decodes it through rawler's real, public end-to-end
// entry point (`rawler::analyze::raw_to_srgb`, the same function `dnglab analyze --srgb
// <file>` calls) on a background thread with a bounded watchdog timeout.
//
// Note: width 4-5 is *also* the trigger range for a separate, already-documented bug
// (candidate 001: `.take(h - 6)`/`.take(w - 6)` underflow in the interior demosaic passes,
// which run right after the code this candidate's patch touches). That means after applying
// ONLY this candidate's patch (without 001's), the decode legitimately proceeds past the fixed
// infinite loop and then panics from the *other*, unrelated bug - see RESULT.md. That still
// correctly demonstrates THIS candidate's fix: what this repro checks is "does the process hang
// forever", not "does the decode fully succeed" - a quick panic and a hang are trivially
// distinguishable and this repro reports which one happened.

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
use std::time::Duration;

const WATCHDOG_TIMEOUT: Duration = Duration::from_secs(10);

fn build_dng(width: usize, height: usize) -> std::path::PathBuf {
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

    let out_path = std::env::temp_dir().join("bughunt_002_ppg_hang.dng");
    let buf = BufWriter::new(File::create(&out_path).expect("create dng file"));
    let mut dng = DngWriter::new(buf, DNG_VERSION_V1_4).expect("dng writer init");
    {
        let mut raw = dng.subframe(0);
        raw.raw_image(&rawimage, CropMode::None, DngCompression::Uncompressed, DngPhotometricConversion::Original, 1)
            .expect("write raw_image");
        raw.finalize().expect("finalize subframe");
    }
    dng.close().expect("close dng");
    out_path
}

fn main() {
    let width = 4usize;
    let height = 8usize;

    let out_path = build_dng(width, height);
    println!("wrote {}x{} DNG to {:?}", width, height, out_path);
    println!("decoding with a {:?} watchdog...", WATCHDOG_TIMEOUT);

    // Real, public, end-to-end entry point - same one `dnglab analyze --srgb` uses - run on a
    // background thread so the main thread can enforce a bounded watchdog instead of hanging
    // this repro forever if the bug is present.
    let handle = std::thread::spawn(move || raw_to_srgb(&out_path, &RawDecodeParams::default()));

    let start = std::time::Instant::now();
    loop {
        if handle.is_finished() {
            match handle.join() {
                Ok(Ok(image)) => println!("COMPLETED (no hang) after {:?}: decoded {}x{}", start.elapsed(), image.width(), image.height()),
                Ok(Err(e)) => println!("COMPLETED (no hang) after {:?}: raw_to_srgb returned an error: {e}", start.elapsed()),
                Err(_) => println!("COMPLETED (no hang) after {:?}: decode thread panicked (see backtrace above - this is the separate, already-documented candidate 001 bug, not this one)", start.elapsed()),
            }
            return;
        }
        if start.elapsed() > WATCHDOG_TIMEOUT {
            println!("HANG DETECTED: decode did not finish within {:?} - infinite loop bug is present", WATCHDOG_TIMEOUT);
            std::process::exit(1);
        }
        std::thread::sleep(Duration::from_millis(50));
    }
}
