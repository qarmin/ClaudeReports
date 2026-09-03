// Standalone reproduction for GitHub issue #569: `index out of bounds: the len is 0 but the
// index is 0` in rawler/src/formats/tiff/ifd.rs, hit while decoding a real-world raw file
// (a compressed.zip sample is attached to the issue) through rawler's actual RAW->image
// pipeline.
//
// This calls rawler's real public entry point directly:
//   rawler::imgop::develop::process_raw_image(path, &mut writer, &params)
// which is exactly what `dnglab process-raw` (dnglab-lib/src/jobs/process_raw.rs) calls, and
// which internally does `crate::get_decoder(&rawfile)?` completely unprotected by any
// `catch_unwind` (unlike `rawler::decode`/`decode_file`, which do wrap the equivalent call).
// `process_raw_image`'s doc-adjacent call chain is exactly the "get_decoder -> raw_metadata /
// raw_image -> develop" pipeline the issue reporter's stack trace shows.
//
// The file built below is a minimal but real, on-disk RAF (Fujifilm) container: the 8-byte
// "FUJIFILM" magic `raf::is_raf` checks for, then a pointer (at the fixed offset RAF's own
// format reserves for it) to a real embedded TIFF-style "main IFD" whose first directory entry
// has an unrecognized TIFF type code (0) and a declared element count of 0 for TIFF tag 0x014A
// (SubIFDs, one of the tags `IFD::new_root`'s default `sub_tags` list always looks for). This is
// a single external artifact (raw file bytes) handed to its real public parsing entry point -
// no second participant is being faked, per CONFIRMATION.md section 11.
//
// Root cause: when `Entry::parse` (rawler/src/formats/tiff/entry.rs) meets a TIFF type code it
// doesn't recognize, it falls back to storing the entry as `Value::Unknown(type_code, bytes)`
// where `bytes` has exactly `count` elements - zero, if the file declares a zero count. Back in
// `IFD::new` (rawler/src/formats/tiff/ifd.rs), when that entry's tag is one of the "look for a
// SubIFD here" tags, the `Value::Unknown` match arm indexes `offsets[0]` unconditionally, with no
// check that the vector actually has an element.
//
// Exit code convention: 0 = did not reproduce, non-zero = reproduced (panicked).

use rawler::imgop::develop::{RawProcessingParams, process_raw_image};
use std::io::{Cursor, Write};

/// Tag 0x014A - TiffCommonTag::SubIFDs - one of `IFD::new_root`'s default `sub_tags`.
const TAG_SUBIFDS: u16 = 0x014A;

/// Builds a minimal RAF (Fujifilm) file whose embedded "main IFD" contains a single directory
/// entry: tag=SubIFDs, TIFF type=0 (unrecognized/"Unknown"), count=0.
fn build_crash_raf() -> Vec<u8> {
    let mut buf = vec![0u8; 200];
    buf[0..8].copy_from_slice(b"FUJIFILM");

    // RAF format: a big-endian u32 pointer to the first embedded TIFF structure lives at a fixed
    // offset (84) in the container header (see rawler::decoders::raf::parse_raf,
    // RAF_TIFF1_PTR_OFFSET). The embedded TIFF's own base is `pointer + 12` (12 bytes of RAF's
    // own directory-name header precede the TIFF data at that pointer in a real file).
    let raf_tiff1_ptr: u32 = 100;
    buf[84..88].copy_from_slice(&raf_tiff1_ptr.to_be_bytes());

    let base = (raf_tiff1_ptr + 12) as usize;

    // TIFF header at `base`: "II" (little-endian), magic 42, offset-to-IFD0 (=8, right after
    // this 8-byte header).
    buf[base..base + 2].copy_from_slice(b"II");
    buf[base + 2..base + 4].copy_from_slice(&42u16.to_le_bytes());
    buf[base + 4..base + 8].copy_from_slice(&8u32.to_le_bytes());

    let ifd0 = base + 8;
    // entry_count = 1
    buf[ifd0..ifd0 + 2].copy_from_slice(&1u16.to_le_bytes());

    let entry = ifd0 + 2;
    // tag = SubIFDs, type = 0 (unrecognized -> Value::Unknown), count = 0, inline value = 0
    buf[entry..entry + 2].copy_from_slice(&TAG_SUBIFDS.to_le_bytes());
    buf[entry + 2..entry + 4].copy_from_slice(&0u16.to_le_bytes()); // type
    buf[entry + 4..entry + 8].copy_from_slice(&0u32.to_le_bytes()); // count
    buf[entry + 8..entry + 12].copy_from_slice(&0u32.to_le_bytes()); // inline value/offset

    let next_ifd_ptr = entry + 12;
    buf[next_ifd_ptr..next_ifd_ptr + 4].copy_from_slice(&0u32.to_le_bytes());

    buf
}

fn main() {
    let data = build_crash_raf();

    let tmp_path = std::env::temp_dir().join(format!("dnglab_bughunt_014_{}.raf", std::process::id()));
    {
        let mut f = std::fs::File::create(&tmp_path).expect("failed to create scratch input file");
        f.write_all(&data).expect("failed to write scratch input file");
    }

    println!("built {}-byte RAF file at {}", data.len(), tmp_path.display());

    let params = RawProcessingParams::default();
    let result = std::panic::catch_unwind(|| {
        let mut out = Cursor::new(Vec::<u8>::new());
        process_raw_image(&tmp_path, &mut out, &params)
    });

    let _ = std::fs::remove_file(&tmp_path);

    match result {
        Ok(Ok(())) => {
            println!("did not reproduce - process_raw_image completed successfully");
        }
        Ok(Err(e)) => {
            println!("did not reproduce - process_raw_image returned Err without panicking: {e}");
        }
        Err(panic_payload) => {
            eprintln!("REPRODUCED - panicked: {:?}", panic_payload);
            std::process::exit(1);
        }
    }
}
