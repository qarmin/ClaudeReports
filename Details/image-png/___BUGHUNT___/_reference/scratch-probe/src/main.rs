use std::io::{Cursor, Write};
use std::panic::{catch_unwind, AssertUnwindSafe};

fn run(name: &str, f: impl FnOnce()) {
    let r = catch_unwind(AssertUnwindSafe(f));
    match r {
        Ok(()) => println!("[{name}] finished without panic"),
        Err(_) => println!("[{name}] *** PANICKED ***"),
    }
}

// ---------------------------------------------------------------- test 1
// StreamWriter on an animated PNG: is the 4-byte fdAT sequence number
// wrongly prepended to the *first* frame which is written as IDAT?
fn stream_apng() {
    let mut out = Vec::new();
    {
        let mut enc = png::Encoder::new(&mut out, 4, 2);
        enc.set_color(png::ColorType::Grayscale);
        enc.set_depth(png::BitDepth::Eight);
        enc.set_animated(2, 0).unwrap();
        let mut w = enc.write_header().unwrap();
        let mut sw = w.stream_writer().unwrap();
        sw.write_all(&[1u8; 8]).unwrap();
        sw.write_all(&[2u8; 8]).unwrap();
        sw.finish().unwrap();
        w.finish().unwrap();
    }
    std::fs::write("/tmp/stream_apng.png", &out).unwrap();
    println!("  encoded {} bytes", out.len());
    let dec = png::Decoder::new(Cursor::new(&out));
    match dec.read_info() {
        Ok(mut r) => {
            let mut buf = vec![0; r.output_buffer_size().unwrap()];
            for i in 0..2 {
                match r.next_frame(&mut buf) {
                    Ok(_) => println!("  frame {i} ok: {:?}", &buf[..8.min(buf.len())]),
                    Err(e) => println!("  frame {i} ERR: {e}"),
                }
            }
        }
        Err(e) => println!("  read_info ERR: {e}"),
    }
}

// non-animated stream writer control
fn stream_plain() {
    let mut out = Vec::new();
    {
        let mut enc = png::Encoder::new(&mut out, 4, 2);
        enc.set_color(png::ColorType::Grayscale);
        enc.set_depth(png::BitDepth::Eight);
        let mut w = enc.write_header().unwrap();
        let mut sw = w.stream_writer().unwrap();
        sw.write_all(&[1u8; 8]).unwrap();
        sw.finish().unwrap();
        w.finish().unwrap();
    }
    let dec = png::Decoder::new(Cursor::new(&out));
    let mut r = dec.read_info().unwrap();
    let mut buf = vec![0; r.output_buffer_size().unwrap()];
    match r.next_frame(&mut buf) {
        Ok(_) => println!("  plain frame ok: {:?}", &buf[..8]),
        Err(e) => println!("  plain frame ERR: {e}"),
    }
}

// ---------------------------------------------------------------- test 2
// `Reader::finish()` then `Reader::next_frame_info()`.
fn finish_then_next_frame_info() {
    let data = std::fs::read("tests/pngsuite/basn0g01.png").unwrap();
    let dec = png::Decoder::new(Cursor::new(data));
    let mut r = dec.read_info().unwrap();
    r.finish().unwrap();
    let res = r.next_frame_info();
    println!("  next_frame_info -> {:?}", res.map(|f| f.sequence_number));
}

// ---------------------------------------------------------------- test 5
fn raw_bytes_overflow() {
    let mut info = png::Info::with_size(u32::MAX, u32::MAX);
    info.color_type = png::ColorType::Rgba;
    info.bit_depth = png::BitDepth::Sixteen;
    println!("  raw_bytes = {}", info.raw_bytes());
}

// ---------------------------------------------------------------- test 6
fn encode_huge_dims() {
    let mut out = Vec::new();
    let mut enc = png::Encoder::new(&mut out, u32::MAX, u32::MAX);
    enc.set_color(png::ColorType::Rgba);
    enc.set_depth(png::BitDepth::Sixteen);
    let mut w = enc.write_header().unwrap();
    let r = w.write_image_data(&[]);
    println!("  write_image_data -> {:?}", r.is_err());
}

// ---------------------------------------------------------------- test 7
// splat_interlaced_row with an img buffer shorter than the frame
fn splat_short_buffer() {
    let info = png::Adam7Info::new(7, 3, 8);
    // 8x8 1-byte-per-pixel image would need 64 bytes; give only 16 (2 rows).
    let mut img = vec![0u8; 16];
    png::splat_interlaced_row(&mut img, 8, &[1, 2, 3, 4, 5, 6, 7, 8], &info, 8);
}

fn splat_zero_stride() {
    let info = png::Adam7Info::new(1, 0, 8);
    let mut img = vec![0u8; 8];
    png::splat_interlaced_row(&mut img, 0, &[1], &info, 8);
}

fn main() {
    let which: Vec<String> = std::env::args().skip(1).collect();
    let all = which.is_empty();
    let want = |n: &str| all || which.iter().any(|w| w == n);

    if want("stream_plain") {
        run("stream_plain", stream_plain);
    }
    if want("stream_apng") {
        run("stream_apng", stream_apng);
    }
    if want("finish_then_next_frame_info") {
        run("finish_then_next_frame_info", finish_then_next_frame_info);
    }
    if want("raw_bytes_overflow") {
        run("raw_bytes_overflow", raw_bytes_overflow);
    }
    if want("encode_huge_dims") {
        run("encode_huge_dims", encode_huge_dims);
    }
    if want("splat_short_buffer") {
        run("splat_short_buffer", splat_short_buffer);
    }
    if want("splat_zero_stride") {
        run("splat_zero_stride", splat_zero_stride);
    }
}
