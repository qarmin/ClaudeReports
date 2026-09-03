// Standalone reproduction: a 2bpp-palettized BMP whose pixel-data offset (from
// BITMAPFILEHEADER) points past the actual end of the file panics `image::load_from_memory`
// in a debug/overflow-checked build instead of returning a decode error.
//
// `read_scanline` (src/codecs/bmp/decoder.rs) special-cases the last scanline in
// `SpecCompliance::Lenient` mode (the default) to tolerate BMPs that are missing their very last
// trailing byte:
//
//     let current_pos = reader.stream_position()?;
//     let end_pos = reader.seek(SeekFrom::End(0))?;
//     reader.seek(SeekFrom::Start(current_pos))?;
//     ...
//     if Ok(head.len()) == usize::try_from(end_pos - current_pos) {
//
// `current_pos` is derived from the file's own (attacker-controlled) `data_offset` field plus
// however many whole rows have already been consumed - nothing validates that this offset is
// still within the file before `read_palettized_pixel_data` seeks there and calls
// `read_scanline`. When `data_offset` places the read cursor past the real end of the file,
// `current_pos > end_pos`, and the unsigned subtraction `end_pos - current_pos` underflows.
// In a debug (or any overflow-checked) build this panics with "attempt to subtract with
// overflow"; in a plain release build it silently wraps to a huge value, the `if` is false, and
// execution falls through to the ordinary `read_exact`, which then just returns a normal
// `UnexpectedEof` `io::Error` - so the panic is specifically a debug/overflow-checked-build
// hazard, not a release-mode memory-safety bug.
//
// This was found by a short coverage-guided libFuzzer burst (`fuzz/fuzzers/fuzzer_script_bmp.rs`,
// debug-assertions build, ASAN disabled - see RESULT.md for why ASAN itself hangs in this
// environment) seeded with hand-crafted 2bpp BMPs, then minimized with `-minimize_crash=1`.
//
// Driven entirely through the public `image::load_from_memory` entry point on a single
// hand-crafted ~70-byte BMP file - a single external artifact handed to a parser's real public
// entry point (the same bytes a real attacker/user would supply), so per CONFIRMATION.md section
// 11 there is no "second participant" to fake here; a single-process repro is the correct shape.
//
// Exit code convention: 0 = did not reproduce (bug absent), non-zero = reproduced (bug present).

// The minimized crash input, exactly as produced by `cargo fuzz run fuzzer_script_bmp
// -minimize_crash=1 ... crash-<hash>`: a BITMAPFILEHEADER + BITMAPINFOHEADER (2bpp, BI_RGB,
// width=17, height=1) + a 4-color palette, with `data_offset` (pixel array offset) = 0x00000946
// = 2374, but the file itself is only 70 bytes long - no pixel data at all.
const CRASH_BMP: &[u8] = include_bytes!("../crash.bmp");

fn main() {
    let result = std::panic::catch_unwind(|| {
        image::load_from_memory_with_format(CRASH_BMP, image::ImageFormat::Bmp)
    });
    match result {
        Ok(Ok(img)) => {
            println!(
                "did NOT reproduce: decoded successfully as {}x{}",
                img.width(),
                img.height()
            );
            std::process::exit(0);
        }
        Ok(Err(e)) => {
            println!("did NOT reproduce: decode returned a clean error (as expected in a release/non-overflow-checked build): {e}");
            std::process::exit(0);
        }
        Err(payload) => {
            let msg = payload
                .downcast_ref::<String>()
                .cloned()
                .or_else(|| payload.downcast_ref::<&str>().map(|s| s.to_string()))
                .unwrap_or_default();
            println!("REPRODUCED: panicked: {msg}");
            std::process::exit(1);
        }
    }
}
