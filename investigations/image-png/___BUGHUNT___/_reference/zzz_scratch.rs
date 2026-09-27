//! Temporary scratch test - DELETE.
use png::*;
use std::io::Write;

/// (c) ChunkWriter::write indexes buffer[0..4] unconditionally for animated PNGs.
#[test]
fn stream_writer_small_buffer() {
    let mut out = Vec::new();
    let mut enc = Encoder::new(&mut out, 4, 4);
    enc.set_color(ColorType::Grayscale);
    enc.set_depth(BitDepth::Eight);
    enc.set_animated(2, 0).unwrap();
    let mut w = enc.write_header().unwrap();
    let mut sw = w.stream_writer_with_size(2).unwrap();
    let _ = sw.write_all(&[0u8; 16]);
    let _ = sw.finish();
}

/// (d) write_image_data multiplies in_len * height without checking.
#[test]
fn write_image_data_overflow() {
    let mut out = Vec::new();
    let mut enc = Encoder::new(&mut out, u32::MAX, u32::MAX);
    enc.set_color(ColorType::Rgba);
    enc.set_depth(BitDepth::Sixteen);
    let mut w = enc.write_header().unwrap();
    let r = w.write_image_data(&[]);
    println!("result: {:?}", r.is_err());
}

/// (a) StreamWriter uses full-image-width buffers when the subframe is narrower.
#[test]
fn stream_writer_subframe_narrower() {
    let mut out = Vec::new();
    {
        let mut enc = Encoder::new(&mut out, 8, 2);
        enc.set_color(ColorType::Grayscale);
        enc.set_depth(BitDepth::Eight);
        enc.set_animated(2, 0).unwrap();
        let mut w = enc.write_header().unwrap();
        let mut sw = w.stream_writer().unwrap();
        // frame 1: full 8x2
        sw.write_all(&[1u8; 16]).unwrap();
        // frame 2: 4x2 subframe
        sw.set_frame_dimension(4, 2).unwrap();
        sw.write_all(&[2u8; 8]).unwrap();
        sw.finish().unwrap();
    }
    std::fs::write("/tmp/subframe.png", &out).unwrap();
    let dec = Decoder::new(std::io::Cursor::new(&out));
    let mut r = dec.read_info().unwrap();
    let mut buf = vec![0; r.output_buffer_size().unwrap()];
    let i1 = r.next_frame(&mut buf);
    println!("frame1: {:?} buf={:?}", i1.map(|i| (i.width, i.height, i.line_size)), &buf);
    let i2 = r.next_frame(&mut buf);
    println!("frame2: {:?} buf={:?}", i2.map(|i| (i.width, i.height, i.line_size)), &buf);
}

/// (b) Interlaced APNG: next_frame writes with the full-image stride but reports the
/// subframe line_size.
#[test]
fn interlaced_apng_subframe_stride() {
    // Build by hand since the encoder refuses interlaced output.
    println!("see manual construction below");
}
