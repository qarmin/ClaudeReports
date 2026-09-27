// Standalone reproduction for candidate 014.gated_row_batching_dryrun_overflow.
//
// Drives jpeg-decoder's real, public `Decoder::decode()` entry point with a real on-disk
// adversarial JPEG fixture (`max_size.jpg`, copied verbatim from the main project's own
// `tests/crashtest/images/max_size.jpg` - a SOF0 frame declaring width=height=0xFFFF with a
// single component and a truncated/garbage entropy-coded scan, already checked into the project
// specifically to exercise this kind of edge case). No second participant is faked: the input is
// a single external byte stream, exactly like a real caller handing an untrusted file to the
// decoder (CONFIRMATION.md section 11's "no second participant" case).
//
// This crate must be built against a tree that has the "under review" state applied first (see
// this candidate's justfile / RESULT.md) - i.e. base_commit plus
// ___OPTIMIZATIONS___/009.gated_row_batching/patch.diff (or equivalently
// 016.hybrid_gated_periodic_flush/patch.diff) - since the bug does not exist in the pristine
// tree at all; it is introduced by that not-yet-applied performance patch.
//
// Exit code convention: 0 = did not reproduce (decode returned normally, Ok or Err), non-zero =
// reproduced (panicked).
use std::io::Cursor;
use std::panic;

fn main() {
    let data: &[u8] = include_bytes!("../max_size.jpg");

    let result = panic::catch_unwind(|| {
        let mut decoder = jpeg_decoder::Decoder::new(Cursor::new(data));
        decoder.decode()
    });

    match result {
        Ok(Ok(pixels)) => {
            println!("did not reproduce - decode() succeeded ({} bytes)", pixels.len());
        }
        Ok(Err(e)) => {
            println!("did not reproduce - decode() returned a clean error: {}", e);
        }
        Err(payload) => {
            let msg = if let Some(s) = payload.downcast_ref::<&str>() {
                s.to_string()
            } else if let Some(s) = payload.downcast_ref::<String>() {
                s.clone()
            } else {
                "<non-string panic payload>".to_string()
            };
            eprintln!("REPRODUCED - decode() panicked: {}", msg);
            std::process::exit(1);
        }
    }
}
