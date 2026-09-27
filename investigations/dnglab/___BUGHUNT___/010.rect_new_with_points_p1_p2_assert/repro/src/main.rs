// Standalone reproduction: a DNG's ActiveArea tag with an inverted axis (top > bottom, or
// left > right) panics on the hard `assert!(p1.x <= p2.x)`/`assert!(p1.y <= p2.y)` inside
// `Rect::new_with_points` - before `Rect::as_ltrb_offsets` (candidate 007's own, separate
// underflow bug) ever runs.
//
// This is a real-user-reported bug, still open as of this session's sync: GitHub issue #573 on
// dnglab/dnglab ("Panic `assertion failed: p1.y <= p2.y` in `rawler/src/imgop/mod.rs`"), filed by
// an automated fuzzer bot with a real, attached, minimized crash file. `testdata/issue573_real_crash_file.dng`
// in this folder *is* that exact attached file (downloaded from the issue, unmodified) - not a
// hand-authored stand-in for it. This repro opens it through rawler's real, public entry points
// (`rawler::get_decoder` + `Decoder::raw_metadata`), the exact call chain the issue's own repro
// harness uses (`get_raw_file` -> `get_decoder` -> `raw_metadata`). No internals of the suspect
// function are called directly, and no second participant needs to be faked - a single external
// artifact (a file) parsed by its real public entry point, per CONFIRMATION.md section 11.
//
// A second, hand-built input (inverted on the X axis, left > right - the sibling
// `assert!(p1.x <= p2.x)`) is also exercised, to vary the input within the same bug class per
// CONFIRMATION.md section 7, and because it does NOT chain into candidate 007's separate,
// still-unfixed `as_ltrb_offsets` underflow (its ActiveArea fits within the image bounds once
// clamped), so it demonstrates a fully clean `Ok` result once this candidate's fix is applied.

use rawler::decoders::RawDecodeParams;
use rawler::formats::tiff::writer::TiffWriter;
use rawler::get_decoder;
use rawler::rawsource::RawSource;
use rawler::tags::{DngTag, TiffCommonTag};
use std::fs::File;
use std::io::{BufWriter, Write};

const ISSUE_573_REAL_CRASH_FILE: &[u8] = include_bytes!("../testdata/issue573_real_crash_file.dng");

// Returns true if this candidate's own bug (the `assert!` panic in `Rect::new_with_points`) was
// observed. Wrapped in `catch_unwind` so a *different*, already-separately-tracked panic (e.g.
// candidate 007's `as_ltrb_offsets` underflow, reachable one step later on the same real crash
// file once this candidate's own assert! stops blocking it) doesn't abort the whole process and
// prevent the second, independent variant below from running too.
fn run_on_path(label: &str, path: &std::path::Path) -> bool {
    println!("--- {label}: {path:?} ---");
    let result = std::panic::catch_unwind(|| {
        let source = RawSource::new(path).expect("open raw source");
        let decoder = get_decoder(&source).expect("get_decoder should select the DNG decoder");
        decoder.raw_metadata(&source, &RawDecodeParams::default()).map(|_| ())
    });
    match result {
        Ok(Ok(())) => {
            println!("raw_metadata returned Ok (this candidate's assert!-panic bug not present)");
            false
        }
        Ok(Err(e)) => {
            println!("raw_metadata returned an error, no panic (this candidate's assert!-panic bug not present): {e}");
            false
        }
        Err(payload) => {
            let msg = payload
                .downcast_ref::<String>()
                .cloned()
                .or_else(|| payload.downcast_ref::<&str>().map(|s| s.to_string()))
                .unwrap_or_default();
            let is_this_bug = msg.contains("p1.x <= p2.x") || msg.contains("p1.y <= p2.y");
            if is_this_bug {
                println!("PANICKED with this candidate's own assert! ({msg:?})");
            } else {
                println!(
                    "panicked, but with a DIFFERENT message (not this candidate's assert! bug - likely candidate 007's separate, still-unfixed downstream bug): {msg:?}"
                );
            }
            is_this_bug
        }
    }
}

fn write_hand_built_inverted_x_dng(out_path: &std::path::Path) {
    let width: u32 = 100;
    let height: u32 = 100;
    let bps: u16 = 16;
    let cpp: u16 = 1;

    // DNG ActiveArea order is [top, left, bottom, right]. left(80) > right(10): triggers
    // assert!(p1.x <= p2.x), the sibling of the p1.y <= p2.y assert issue 573's own file trips.
    let active_area: [u32; 4] = [0, 80, 90, 10];

    let pixel_data: Vec<u16> = vec![100u16; (width * height * cpp as u32) as usize];
    let pixel_bytes: Vec<u8> = pixel_data.iter().flat_map(|v| v.to_le_bytes()).collect();

    let buf = BufWriter::new(File::create(out_path).expect("create dng file"));
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
}

fn main() {
    // Silence the default panic hook's own backtrace-suggestion noise for the *expected*
    // (first, real-file) panic case; catch_unwind above still gets the panic payload either way.
    let tmp_dir = std::env::temp_dir();

    let real_file_path = tmp_dir.join("bughunt_010_issue573_real_crash_file.dng");
    File::create(&real_file_path)
        .and_then(|mut f| f.write_all(ISSUE_573_REAL_CRASH_FILE))
        .expect("write embedded real crash file to disk");
    let bug_a = run_on_path("real issue #573 crash file (top > bottom in ActiveArea)", &real_file_path);

    let hand_built_path = tmp_dir.join("bughunt_010_activearea_inverted_x.dng");
    write_hand_built_inverted_x_dng(&hand_built_path);
    let bug_b = run_on_path("hand-built boundary variant (left > right in ActiveArea)", &hand_built_path);

    println!();
    if bug_a || bug_b {
        println!("RESULT: candidate 010's assert! bug IS present (at least one variant panicked on it).");
        std::process::exit(1);
    } else {
        println!("RESULT: candidate 010's assert! bug is NOT present in either variant.");
    }
}
