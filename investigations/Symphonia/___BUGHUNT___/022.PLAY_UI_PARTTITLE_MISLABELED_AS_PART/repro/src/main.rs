//! Standalone repro for candidate 022 (PLAY_UI_PARTTITLE_MISLABELED_AS_PART).
//!
//! Builds a real, well-formed Matroska byte stream containing a standard Matroska tag structure
//! (`Tag > Targets{TargetTypeValue=40 ("PART")} > SimpleTag{TagName="TITLE", TagString=...}`) -
//! exactly what a real muxer (e.g. mkvpropedit) would write to name a "part"/session inside a
//! multi-disc release, per the Matroska tagging spec. This is fed to the real, compiled
//! `symphonia-play` binary (built from `../clone`) via `--probe-only`, exercising the real,
//! genuine trigger path end-to-end:
//!
//!   real .mkv bytes -> real `symphonia::default::get_probe()` -> real `MkvReader` (which maps
//!   this exact tag structure to `StandardTag::PartTitle`, per
//!   `symphonia-format-mkv/src/tags.rs`) -> real `symphonia_play::ui::print_format` /
//!   `fmt_tag` (`symphonia-play/src/ui.rs`)
//!
//! No internal `StandardTag` value is hand-constructed and fed directly to any formatting
//! function - the enum variant is produced by the real Matroska tag parser from real file bytes,
//! and the real `symphonia-play` binary (not a stand-in) is what's actually run and whose actual
//! stdout is inspected.
//!
//! Expected (buggy) output: a line reading `Part                 My Part Title`.
//! Expected (fixed) output: a line reading `Part Title           My Part Title`.

use std::io::Write;
use std::path::Path;
use std::process::Command;

fn vint_size(n: u64) -> [u8; 8] {
    let mut b = [0u8; 8];
    b[0] = 0x01;
    let nb = n.to_be_bytes();
    b[1..8].copy_from_slice(&nb[1..8]);
    b
}

fn elem(id: &[u8], payload: &[u8]) -> Vec<u8> {
    let mut v = Vec::with_capacity(id.len() + 8 + payload.len());
    v.extend_from_slice(id);
    v.extend_from_slice(&vint_size(payload.len() as u64));
    v.extend_from_slice(payload);
    v
}

const EBML: &[u8] = &[0x1A, 0x45, 0xDF, 0xA3];
const DOC_TYPE: &[u8] = &[0x42, 0x82];
const SEGMENT: &[u8] = &[0x18, 0x53, 0x80, 0x67];
const INFO: &[u8] = &[0x15, 0x49, 0xA9, 0x66];
const MUXING_APP: &[u8] = &[0x4D, 0x80];
const WRITING_APP: &[u8] = &[0x57, 0x41];
const TRACKS: &[u8] = &[0x16, 0x54, 0xAE, 0x6B];
const TAGS: &[u8] = &[0x12, 0x54, 0xC3, 0x67];
const TAG: &[u8] = &[0x73, 0x73];
const SIMPLETAG: &[u8] = &[0x67, 0xC8];
const TARGETS: &[u8] = &[0x63, 0xC0];
const TARGETTYPEVALUE: &[u8] = &[0x68, 0xCA];
const TAGNAME: &[u8] = &[0x45, 0xA3];
const TAGSTRING: &[u8] = &[0x44, 0x87];

/// Builds a minimal, real Matroska byte stream with one PART-level TITLE tag.
///
/// Structure (standard Matroska tagging, see the Matroska tag spec's `TargetTypeValue`
/// table - 40 = "PART"/"SESSION"):
///
///   EBML{DocType: "matroska"}
///   Segment
///     Info{MuxingApp, WritingApp}
///     Tracks{}                                  (no tracks needed for --probe-only)
///     Tags
///       Tag
///         Targets{TargetTypeValue: 40}
///         SimpleTag{TagName: "TITLE", TagString: "My Part Title"}
fn build_mkv_with_part_title(title: &str) -> Vec<u8> {
    let mut out = Vec::new();

    out.extend(elem(EBML, &elem(DOC_TYPE, b"matroska")));

    let mut info_payload = Vec::new();
    info_payload.extend(elem(MUXING_APP, b"x"));
    info_payload.extend(elem(WRITING_APP, b"x"));

    let targets_payload = elem(TARGETTYPEVALUE, &[40]);
    let mut simpletag_payload = Vec::new();
    simpletag_payload.extend(elem(TAGNAME, b"TITLE"));
    simpletag_payload.extend(elem(TAGSTRING, title.as_bytes()));

    let mut tag_payload = Vec::new();
    tag_payload.extend(elem(TARGETS, &targets_payload));
    tag_payload.extend(elem(SIMPLETAG, &simpletag_payload));

    let tags_payload = elem(TAG, &tag_payload);

    let mut segment_payload = Vec::new();
    segment_payload.extend(elem(INFO, &info_payload));
    segment_payload.extend(elem(TRACKS, &[]));
    segment_payload.extend(elem(TAGS, &tags_payload));

    out.extend(elem(SEGMENT, &segment_payload));

    out
}

fn main() {
    let title = "My Part Title";
    let data = build_mkv_with_part_title(title);

    let repro_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let clone_dir = repro_dir.join("../clone");
    let mkv_path = repro_dir.join("part_title.mkv");

    std::fs::File::create(&mkv_path)
        .and_then(|mut f| f.write_all(&data))
        .expect("failed to write crafted .mkv file");

    eprintln!("wrote {} bytes to {}", data.len(), mkv_path.display());

    // Build the real symphonia-play binary from the pristine/patched clone.
    let build_status = Command::new("cargo")
        .args(["build", "--release", "-p", "symphonia-play"])
        .current_dir(&clone_dir)
        .status()
        .expect("failed to invoke cargo build");
    assert!(build_status.success(), "cargo build -p symphonia-play failed");

    let binary_path = clone_dir.join("target/release/symphonia-play");
    assert!(binary_path.exists(), "expected binary not found at {}", binary_path.display());

    // Run the real, compiled symphonia-play binary against the crafted file, exactly as a real
    // user inspecting a file's metadata would.
    let output = Command::new(&binary_path)
        .args(["--probe-only", mkv_path.to_str().unwrap()])
        .output()
        .expect("failed to run symphonia-play");

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("--- symphonia-play --probe-only stdout ---");
    println!("{stdout}");
    println!("--- end stdout ---");

    let tag_line = stdout
        .lines()
        .find(|l| l.contains(title))
        .unwrap_or_else(|| panic!("no output line contains the tag value {title:?}"));

    println!("tag line: {tag_line:?}");

    // The mislabeled (buggy) output prints the key "Part" (no "Title"); the fixed output prints
    // "Part Title". Distinguish precisely instead of just substring-matching "Part" (which is a
    // substring of "Part Title" too).
    let key_field = tag_line.trim_start_matches(|c: char| !c.is_ascii_alphabetic());
    let key_field = key_field.split("  ").next().unwrap_or(key_field).trim();

    println!("extracted key field: {key_field:?}");

    if key_field == "Part Title" {
        println!("RESULT: FIXED - StandardTag::PartTitle is labeled \"Part Title\" (correct).");
    }
    else if key_field == "Part" {
        println!(
            "RESULT: BUG REPRODUCED - StandardTag::PartTitle is mislabeled as \"Part\" \
             (indistinguishable from StandardTag::Part in the printed output)."
        );
    }
    else {
        panic!("unexpected key field {key_field:?}, neither \"Part\" nor \"Part Title\"");
    }
}
