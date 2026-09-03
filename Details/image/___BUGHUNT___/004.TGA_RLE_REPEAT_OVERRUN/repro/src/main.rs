// Tests the claim: a TGA RLE repeat packet whose count exceeds the remaining bytes in the
// decode buffer causes `index` in `read_encoded_data` to overshoot `buf.len()`, ending the
// `while index < buf.len()` loop early and leaving stale/unmodified bytes in the buffer tail.
//
// Uses the crate's public API (`image::load_from_memory`) to decode a hand-built minimal
// grayscale RLE-encoded TGA file end to end.

use image::GenericImageView;

fn build_tga(width: u16, height: u16, image_data: &[u8]) -> Vec<u8> {
    let mut out = vec![0u8; 18];
    out[2] = 11; // image_type = RunGrayScale (RLE)
    out[12] = (width & 0xff) as u8;
    out[13] = (width >> 8) as u8;
    out[14] = (height & 0xff) as u8;
    out[15] = (height >> 8) as u8;
    out[16] = 8; // pixel_depth = 8 bits (1 byte/pixel, grayscale)
    out[17] = 0x20; // top-left origin
    out.extend_from_slice(image_data);
    out
}

fn main() {
    // Case 1: single pixel, repeat packet claims 2 repeats but only 1 pixel of room exists.
    // run_packet = 0x81 -> high bit set, repeat_count = (0x81 & 0x7f) + 1 = 2
    let data1 = [0x81u8, 0xAB];
    let tga1 = build_tga(1, 1, &data1);
    let img1 = image::load_from_memory_with_format(&tga1, image::ImageFormat::Tga)
        .expect("decode case 1");
    let px1 = img1.get_pixel(0, 0);
    println!("case 1 (1x1, repeat_count=2 into 1 remaining pixel): pixel = {:?}", px1.0);
    let expected1 = 0xABu8;
    let ok1 = px1.0[0] == expected1;
    println!("  expected first channel = 0x{:02x}, got 0x{:02x}, match = {}", expected1, px1.0[0], ok1);

    // Case 2: 3 pixels wide. First a raw packet sets pixel 0 to 0x11, then a repeat packet
    // claims 5 repeats (0xCC) but only 2 pixels of room remain.
    // raw packet: run_packet = 0x00 -> num_raw_bytes = 1 raw byte follows (0x11)
    // repeat packet: run_packet = 0x84 -> repeat_count = (0x84 & 0x7f) + 1 = 5
    let data2 = [0x00u8, 0x11, 0x84, 0xCC];
    let tga2 = build_tga(3, 1, &data2);
    let img2 = image::load_from_memory_with_format(&tga2, image::ImageFormat::Tga)
        .expect("decode case 2");
    let got2: Vec<u8> = (0..3).map(|x| img2.get_pixel(x, 0).0[0]).collect();
    let expected2 = [0x11u8, 0xCC, 0xCC];
    println!("case 2 (3x1, raw+overrunning repeat): pixels = {:?}", got2);
    println!("  expected = {:?}, match = {}", expected2, got2 == expected2);

    // Case 3: random boundary sweep across widths/heights and randomized raw/repeat packet
    // sequences (grayscale, bpp=1) where at least one repeat packet's count is deliberately
    // inflated beyond the pixels actually remaining. Each trial encodes a byte stream by hand,
    // tracks the value each pixel *should* end up with (mirroring the packet semantics but
    // clamped to the buffer, i.e. the behavior a correct decoder should produce), decodes with
    // the real public API, and compares.
    let mut rng_state: u64 = 0x2545F4914F6CDD1D;
    let mut next_rand = move || {
        rng_state ^= rng_state << 13;
        rng_state ^= rng_state >> 7;
        rng_state ^= rng_state << 17;
        rng_state
    };

    let mut sweep_failures = 0usize;
    let trials = 500;
    for trial in 0..trials {
        let width = 1 + (next_rand() % 9) as u16; // 1..=9
        let total_pixels = width as usize;
        let mut expected = vec![0u8; total_pixels];
        let mut encoded = Vec::new();
        let mut produced = 0usize;
        while produced < total_pixels {
            let remaining = total_pixels - produced;
            let is_repeat = next_rand() % 2 == 0;
            if is_repeat {
                // Deliberately allow (and, half the time, force) the repeat count to exceed
                // what's left, to hit the overrun path under test.
                let nominal_count = 1 + (next_rand() % 128) as usize; // 1..=128
                let value = (next_rand() % 256) as u8;
                encoded.push(0x80 | ((nominal_count - 1) as u8));
                encoded.push(value);
                let actually_written = nominal_count.min(remaining);
                for i in 0..actually_written {
                    expected[produced + i] = value;
                }
                produced += actually_written;
            } else {
                let nominal_count = 1 + (next_rand() % remaining.max(1) as u64) as usize;
                let nominal_count = nominal_count.min(128);
                encoded.push((nominal_count - 1) as u8);
                let mut bytes = Vec::with_capacity(nominal_count);
                for _ in 0..nominal_count {
                    bytes.push((next_rand() % 256) as u8);
                }
                encoded.extend_from_slice(&bytes);
                let actually_written = nominal_count.min(remaining);
                for i in 0..actually_written {
                    expected[produced + i] = bytes[i];
                }
                produced += actually_written;
            }
        }

        let tga = build_tga(width, 1, &encoded);
        let img = match image::load_from_memory_with_format(&tga, image::ImageFormat::Tga) {
            Ok(i) => i,
            Err(e) => {
                println!("trial {trial}: decode error: {e}");
                sweep_failures += 1;
                continue;
            }
        };
        let got: Vec<u8> = (0..width as u32).map(|x| img.get_pixel(x, 0).0[0]).collect();
        if got != expected {
            println!("trial {trial}: MISMATCH width={width} expected={expected:?} got={got:?}");
            sweep_failures += 1;
        }
    }
    println!("boundary sweep: {trials} trials, {sweep_failures} mismatches");

    let bug_present = !ok1 || got2 != expected2 || sweep_failures > 0;
    if bug_present {
        eprintln!("REPRODUCED - stale/incorrect pixel data found in buffer tail");
        std::process::exit(1);
    } else {
        println!("did not reproduce - all pixels decoded correctly despite repeat-count overrun");
    }
}
