//! Standalone reproduction for ___BUGHUNT___ candidate 014.
//!
//! Hand-builds a real, spec-valid animated WebP file (RIFF/VP8X/ANIM/ANMF container, each
//! sub-frame a real VP8L lossless bitstream produced once by `image_webp::WebPEncoder` and
//! embedded here as a byte literal, so this crate only needs to depend on `image` itself) and
//! decodes it through `image`'s real public `image::codecs::webp::WebPDecoder` entry point - the
//! same path any real caller reading an animated WebP file goes through. The file itself is the
//! external input (a malformed/adversarial WebP is exactly what a real file on disk looks like),
//! so no second participant needs to be faked here (see CONFIRMATION.md section 11).
//!
//! Animation (4x4 canvas, 3 frames, full-canvas each):
//!   Frame 0: opaque green,  overwrite (no blend), do NOT dispose after showing.
//!   Frame 1: opaque blue,   overwrite (no blend), DISPOSE TO BACKGROUND after showing.
//!   Frame 2: fully transparent, alpha-blended over canvas, do NOT dispose.
//!
//! Per the WebP spec, "dispose to background" means the canvas region occupied by frame 1 must
//! be cleared to the background color *before* frame 2 is composited. Since frame 2 is fully
//! transparent and alpha-blended, a correctly cleared canvas must show through as transparent
//! black. If the canvas was never actually cleared, frame 2 instead still shows frame 1's opaque
//! blue - i.e. "a later frame appears drawn on top of an earlier one instead of the disposal
//! method being respected", exactly the shape of image-rs/image#2913.
//!
//! Exit code convention: 0 = did not reproduce (bug absent), non-zero = reproduced (bug present).

use std::io::Cursor;

// Raw `VP8L` RIFF sub-chunks (fourcc + size + payload [+ pad byte]), each a real single-frame
// lossless bitstream produced once via `image_webp::WebPEncoder::encode()` for a solid 4x4 RGBA8
// image, captured verbatim so this standalone crate doesn't need image-webp as a direct
// dependency.
const VP8L_GREEN: &[u8] = &[
    86, 80, 56, 76, 139, 0, 0, 0, 47, 3, 192, 0, 16, 205, 85, 32,
    34, 2, 30, 136, 4, 0, 0, 0, 0, 128, 1, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 1, 1, 0, 120, 32, 1,
    0, 0, 0, 0, 0, 231, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 224, 129, 4, 0, 0, 0, 0, 0, 156, 255,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 64,
    228, 135, 0, 0,
];
const VP8L_BLUE: &[u8] = &[
    86, 80, 56, 76, 97, 0, 0, 0, 47, 3, 192, 0, 16, 205, 85, 32,
    34, 2, 30, 72, 0, 0, 0, 0, 0, 128, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 64, 0, 128, 240, 64, 2,
    0, 0, 0, 0, 0, 206, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 48, 82, 6, 0,
];
const VP8L_TRANSPARENT: &[u8] = &[
    86, 80, 56, 76, 97, 0, 0, 0, 47, 3, 192, 0, 16, 205, 85, 32,
    34, 2, 30, 72, 0, 0, 0, 0, 0, 128, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 64, 0, 128, 8, 15, 36,
    0, 0, 0, 0, 0, 224, 252, 7, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 82, 6, 0,
];

fn write_u24_le(out: &mut Vec<u8>, v: u32) {
    out.push((v & 0xff) as u8);
    out.push(((v >> 8) & 0xff) as u8);
    out.push(((v >> 16) & 0xff) as u8);
}

fn write_chunk(out: &mut Vec<u8>, fourcc: &[u8; 4], payload: &[u8]) {
    out.extend_from_slice(fourcc);
    out.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    out.extend_from_slice(payload);
    if payload.len() % 2 == 1 {
        out.push(0);
    }
}

struct FrameSpec {
    dispose_to_background: bool,
    use_alpha_blending: bool,
    vp8l_chunk: &'static [u8],
}

fn build_anim_webp(canvas_width: u32, canvas_height: u32, frames: &[FrameSpec]) -> Vec<u8> {
    let mut vp8x = Vec::new();
    vp8x.push(0b0001_0010); // bit4 = has alpha, bit1 = has animation
    vp8x.extend_from_slice(&[0, 0, 0]);
    write_u24_le(&mut vp8x, canvas_width - 1);
    write_u24_le(&mut vp8x, canvas_height - 1);

    // Background color left at [0,0,0,0]; the decoder never auto-applies this hint unless the
    // caller explicitly calls `set_background_color`, which this repro deliberately does not do
    // (the default/unconfigured path is exactly what most real callers hit).
    let mut anim = Vec::new();
    anim.extend_from_slice(&[0, 0, 0, 0]);
    anim.extend_from_slice(&0u16.to_le_bytes());

    let mut anmfs = Vec::new();
    for f in frames {
        let mut payload = Vec::new();
        write_u24_le(&mut payload, 0); // x offset / 2
        write_u24_le(&mut payload, 0); // y offset / 2
        write_u24_le(&mut payload, canvas_width - 1);
        write_u24_le(&mut payload, canvas_height - 1);
        write_u24_le(&mut payload, 100); // duration ms
        let mut flags = 0u8;
        if !f.use_alpha_blending {
            flags |= 0b0000_0010;
        }
        if f.dispose_to_background {
            flags |= 0b0000_0001;
        }
        payload.push(flags);
        payload.extend_from_slice(f.vp8l_chunk);

        write_chunk(&mut anmfs, b"ANMF", &payload);
    }

    let mut riff_payload = Vec::new();
    riff_payload.extend_from_slice(b"WEBP");
    write_chunk(&mut riff_payload, b"VP8X", &vp8x);
    write_chunk(&mut riff_payload, b"ANIM", &anim);
    riff_payload.extend_from_slice(&anmfs);

    let mut out = Vec::new();
    out.extend_from_slice(b"RIFF");
    out.extend_from_slice(&(riff_payload.len() as u32).to_le_bytes());
    out.extend_from_slice(&riff_payload);
    out
}

fn main() {
    let (w, h) = (4u32, 4u32);

    let frames = [
        FrameSpec {
            dispose_to_background: false,
            use_alpha_blending: false,
            vp8l_chunk: VP8L_GREEN,
        },
        FrameSpec {
            dispose_to_background: true,
            use_alpha_blending: false,
            vp8l_chunk: VP8L_BLUE,
        },
        FrameSpec {
            dispose_to_background: false,
            use_alpha_blending: true,
            vp8l_chunk: VP8L_TRANSPARENT,
        },
    ];

    let bytes = build_anim_webp(w, h, &frames);

    let mut decoder =
        image::codecs::webp::WebPDecoder::new(Cursor::new(bytes)).expect("valid animated webp");
    assert!(decoder.has_animation());

    use image::ImageDecoder;
    let mut results = Vec::new();
    for i in 0..3 {
        let layout = decoder.prepare_image().expect("prepare_image");
        let mut buf = vec![0u8; layout.total_bytes() as usize];
        decoder.read_image(&mut buf).expect("read_image");
        let px = [buf[0], buf[1], buf[2], buf[3]];
        println!("frame {i} pixel(0,0) = {px:?}");
        results.push(px);
    }

    let frame3 = results[2];
    println!();
    println!("frame 3 (after frame 2 disposed to background) = {frame3:?}");
    println!("expected (disposal respected):   [0, 0, 0, 0] (transparent)");
    println!("buggy    (disposal NOT applied): [0, 0, 255, 255] (frame 2's blue bleeds through)");

    if frame3 == [0, 0, 0, 0] {
        println!("RESULT: disposal handled correctly - bug NOT observed");
        std::process::exit(0);
    } else if frame3 == [0, 0, 255, 255] {
        println!("RESULT: BUG REPRODUCED - frame 3 shows frame 2's undisposed content");
        std::process::exit(1);
    } else {
        println!("RESULT: unexpected pixel value, inconclusive: {frame3:?}");
        std::process::exit(2);
    }
}
