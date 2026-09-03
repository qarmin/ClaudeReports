// Standalone reproduction: a BMP-backed ICO file whose embedded BITMAPINFOHEADER declares an odd
// "doubled" height (the ICO convention doubles the real height to account for the AND mask row)
// panics `image::load_from_memory` instead of returning an error.
//
// `BmpDecoder::read_metadata_in_ico_format` (src/codecs/bmp/decoder.rs) runs the normal BMP
// `read_metadata()` first - which validates the *doubled* height via `check_for_overflow` (any
// value > 0 passes) - and only *afterwards* halves `self.height` to undo the ICO doubling
// convention. An odd doubled height (e.g. 1) integer-divides to 0, which was never itself
// validated. The pixel readers (`read_full_byte_pixel_data` and friends) then compute
// `(self.height - 1).try_into::<u32>().unwrap()`, which panics with a `TryFromIntError` because
// `self.height - 1 == -1`.
//
// Driven entirely through the public `image::load_from_memory` entry point on a hand-crafted
// ~66-byte ICO file - this is a single external artifact handed to a parser's real public entry
// point (the same bytes a real attacker/user would supply), so per CONFIRMATION.md section 11
// there is no "second participant" to fake here; a single-process repro is the correct shape.
//
// Exit code convention: 0 = did not reproduce (bug absent), non-zero = reproduced (bug present).

use image::GenericImageView;

/// Build a minimal single-entry ICO file embedding a bare BITMAPINFOHEADER (no BITMAPFILEHEADER,
/// per the ICO convention) with the given (possibly-doubled) height field.
fn build_ico(doubled_height: i32) -> Vec<u8> {
    let width: i32 = 1;
    let bit_count: u16 = 24;

    let mut bmp_header = Vec::new();
    bmp_header.extend_from_slice(&40u32.to_le_bytes()); // BITMAPINFOHEADER size
    bmp_header.extend_from_slice(&width.to_le_bytes());
    bmp_header.extend_from_slice(&doubled_height.to_le_bytes());
    bmp_header.extend_from_slice(&1u16.to_le_bytes()); // planes
    bmp_header.extend_from_slice(&bit_count.to_le_bytes());
    bmp_header.extend_from_slice(&0u32.to_le_bytes()); // compression = BI_RGB
    bmp_header.extend_from_slice(&0u32.to_le_bytes()); // image size (unused for BI_RGB)
    bmp_header.extend_from_slice(&0i32.to_le_bytes()); // x pixels per meter
    bmp_header.extend_from_slice(&0i32.to_le_bytes()); // y pixels per meter
    bmp_header.extend_from_slice(&0u32.to_le_bytes()); // colors used
    bmp_header.extend_from_slice(&0u32.to_le_bytes()); // colors important
    assert_eq!(bmp_header.len(), 40);

    // One 24bpp pixel row, padded to a 4-byte boundary (3 bytes color + 1 byte padding), plus
    // one AND-mask row (4 bytes for a 1px-wide mask row, 32-bit aligned) - enough trailing bytes
    // that a *correctly* halved height (real height 1) would decode successfully too.
    let pixel_row = [0x11u8, 0x22, 0x33, 0x00];
    let mask_row = [0x00u8, 0x00, 0x00, 0x00];

    let mut bmp_data = bmp_header;
    bmp_data.extend_from_slice(&pixel_row);
    bmp_data.extend_from_slice(&mask_row);

    let mut ico = Vec::new();
    // ICONDIR: reserved(2)=0, type(2)=1 (icon), count(2)=1
    ico.extend_from_slice(&0u16.to_le_bytes());
    ico.extend_from_slice(&1u16.to_le_bytes());
    ico.extend_from_slice(&1u16.to_le_bytes());

    // ICONDIRENTRY (16 bytes): width, height, colorCount, reserved, planes, bpp, size, offset
    ico.push(1); // real width
    ico.push(1); // real height
    ico.push(0); // color count
    ico.push(0); // reserved
    ico.extend_from_slice(&1u16.to_le_bytes()); // planes
    ico.extend_from_slice(&24u16.to_le_bytes()); // bits per pixel
    ico.extend_from_slice(&(bmp_data.len() as u32).to_le_bytes()); // image data size
    ico.extend_from_slice(&22u32.to_le_bytes()); // image data offset (6 + 16)

    ico.extend_from_slice(&bmp_data);
    ico
}

fn run(label: &str, bytes: &[u8]) -> Result<Option<(u32, u32)>, String> {
    let result = std::panic::catch_unwind(|| image::load_from_memory(bytes));
    match result {
        Ok(Ok(img)) => Ok(Some(img.dimensions())),
        Ok(Err(e)) => {
            println!("{label}: returned error (no panic): {e}");
            Ok(None)
        }
        Err(payload) => Err(format!("{payload:?}")),
    }
}

fn main() {
    // The actual bug: an odd doubled height (1) halves to 0.
    let odd_height_ico = build_ico(1);
    println!(
        "odd-height ICO: {} bytes: {:02x?}",
        odd_height_ico.len(),
        odd_height_ico
    );

    // Sanity control: an even doubled height (2, real height 1) must still decode fine, both
    // before and after any fix, to make sure the fix doesn't overreject valid input.
    let valid_ico = build_ico(2);

    let mut reproduced = false;

    match run("odd-height ICO (doubled height=1)", &odd_height_ico) {
        Ok(Some(dims)) => println!("did not reproduce - decoded ok: {dims:?}"),
        Ok(None) => println!("did not reproduce - got a clean error instead of a panic"),
        Err(msg) => {
            eprintln!("REPRODUCED - panicked: {msg}");
            reproduced = true;
        }
    }

    match run("valid ICO (doubled height=2, real height=1)", &valid_ico) {
        Ok(Some(dims)) => println!("control case decoded ok, as expected: {dims:?}"),
        Ok(None) => {
            eprintln!("UNEXPECTED - control case (valid input) returned an error");
        }
        Err(msg) => {
            eprintln!("UNEXPECTED - control case (valid input) panicked: {msg}");
        }
    }

    if reproduced {
        std::process::exit(1);
    }
}
