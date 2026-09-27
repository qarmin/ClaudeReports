use std::hint::black_box;

fn div_by_255(v: u16) -> u8 {
    ((((v + 0x80) >> 8) + v + 0x80) >> 8).min(255) as u8
}

fn div_by_2pn_m1(v: u32, n: u32) -> u16 {
    let round = 1 << (n - 1);
    let v = v + round;
    (((v >> n) + v) >> n) as u16
}

fn reference_premultiply_rgba8(buf: &[u8]) -> Vec<u8> {
    let mut out = buf.to_vec();
    for chunk in out.chunks_exact_mut(4) {
        let a = chunk[3] as u16;
        chunk[0] = div_by_255(chunk[0] as u16 * a);
        chunk[1] = div_by_255(chunk[1] as u16 * a);
        chunk[2] = div_by_255(chunk[2] as u16 * a);
        chunk[3] = div_by_255(255 * a);
    }
    out
}

fn reference_premultiply_la8(buf: &[u8]) -> Vec<u8> {
    let mut out = buf.to_vec();
    for chunk in out.chunks_exact_mut(2) {
        let a = chunk[1] as u16;
        chunk[0] = div_by_255(chunk[0] as u16 * a);
        chunk[1] = div_by_255(255 * a);
    }
    out
}

fn reference_premultiply_rgba16(buf: &[u16], bit_depth: u32) -> Vec<u16> {
    let max_colors = (1u32 << bit_depth) - 1;
    let mut out = buf.to_vec();
    for chunk in out.chunks_exact_mut(4) {
        let a = chunk[3] as u32;
        chunk[0] = div_by_2pn_m1(chunk[0] as u32 * a, bit_depth) as u16;
        chunk[1] = div_by_2pn_m1(chunk[1] as u32 * a, bit_depth) as u16;
        chunk[2] = div_by_2pn_m1(chunk[2] as u32 * a, bit_depth) as u16;
        chunk[3] = div_by_2pn_m1(max_colors * a, bit_depth) as u16;
    }
    out
}

fn reference_premultiply_la16(buf: &[u16], bit_depth: u32) -> Vec<u16> {
    let max_colors = (1u32 << bit_depth) - 1;
    let mut out = buf.to_vec();
    for chunk in out.chunks_exact_mut(2) {
        let a = chunk[1] as u32;
        chunk[0] = div_by_2pn_m1(chunk[0] as u32 * a, bit_depth) as u16;
        chunk[1] = div_by_2pn_m1(max_colors * a, bit_depth) as u16;
    }
    out
}

fn main() {
    let mut rng_state: u64 = 0x243F6A8885A308D3;
    let mut next = || {
        rng_state ^= rng_state << 13;
        rng_state ^= rng_state >> 7;
        rng_state ^= rng_state << 17;
        rng_state
    };

    let mut mismatches = 0usize;

    // rgba8 / la8: exhaustive over all 256 alpha values, randomized channel values, many trials.
    for _trial in 0..200 {
        let n_pixels = 2048;
        let mut buf8: Vec<u8> = (0..n_pixels * 4).map(|_| (next() % 256) as u8).collect();
        let reference = reference_premultiply_rgba8(&buf8);
        pic_scale_safe::premultiply_rgba8(black_box(&mut buf8));
        if buf8 != reference {
            mismatches += 1;
            println!("MISMATCH premultiply_rgba8 trial {}", _trial);
        }

        let mut buf8b: Vec<u8> = (0..n_pixels * 2).map(|_| (next() % 256) as u8).collect();
        let reference_la = reference_premultiply_la8(&buf8b);
        pic_scale_safe::premultiply_la8(black_box(&mut buf8b));
        if buf8b != reference_la {
            mismatches += 1;
            println!("MISMATCH premultiply_la8 trial {}", _trial);
        }

        let src8: Vec<u8> = (0..n_pixels * 4).map(|_| (next() % 256) as u8).collect();
        let reference_copy = reference_premultiply_rgba8(&src8);
        let got_copy = pic_scale_safe::premultiplied_rgba8(black_box(&src8));
        if got_copy != reference_copy {
            mismatches += 1;
            println!("MISMATCH premultiplied_rgba8 trial {}", _trial);
        }

        let src8b: Vec<u8> = (0..n_pixels * 2).map(|_| (next() % 256) as u8).collect();
        let reference_copy_la = reference_premultiply_la8(&src8b);
        let got_copy_la = pic_scale_safe::premultiplied_la8(black_box(&src8b));
        if got_copy_la != reference_copy_la {
            mismatches += 1;
            println!("MISMATCH premultiplied_la8 trial {}", _trial);
        }
    }

    for &bit_depth in &[10u32, 12, 16, 9, 11] {
        let max_colors = (1u32 << bit_depth) - 1;
        for _trial in 0..50 {
            let n_pixels = 1024;
            let mut buf16: Vec<u16> = (0..n_pixels * 4)
                .map(|_| (next() % (max_colors as u64 + 1)) as u16)
                .collect();
            let reference = reference_premultiply_rgba16(&buf16, bit_depth);
            pic_scale_safe::premultiply_rgba16(black_box(&mut buf16), bit_depth);
            if buf16 != reference {
                mismatches += 1;
                println!("MISMATCH premultiply_rgba16 bd={} trial {}", bit_depth, _trial);
            }

            let mut buf16b: Vec<u16> = (0..n_pixels * 2)
                .map(|_| (next() % (max_colors as u64 + 1)) as u16)
                .collect();
            let reference_la = reference_premultiply_la16(&buf16b, bit_depth);
            pic_scale_safe::premultiply_la16(black_box(&mut buf16b), bit_depth);
            if buf16b != reference_la {
                mismatches += 1;
                println!("MISMATCH premultiply_la16 bd={} trial {}", bit_depth, _trial);
            }

            let src16: Vec<u16> = (0..n_pixels * 4)
                .map(|_| (next() % (max_colors as u64 + 1)) as u16)
                .collect();
            let reference_copy = reference_premultiply_rgba16(&src16, bit_depth);
            let got_copy = pic_scale_safe::premultiplied_rgba16(black_box(&src16), bit_depth);
            if got_copy != reference_copy {
                mismatches += 1;
                println!("MISMATCH premultiplied_rgba16 bd={} trial {}", bit_depth, _trial);
            }

            let src16b: Vec<u16> = (0..n_pixels * 2)
                .map(|_| (next() % (max_colors as u64 + 1)) as u16)
                .collect();
            let reference_copy_la = reference_premultiply_la16(&src16b, bit_depth);
            let got_copy_la = pic_scale_safe::premultiplied_la16(black_box(&src16b), bit_depth);
            if got_copy_la != reference_copy_la {
                mismatches += 1;
                println!("MISMATCH premultiplied_la16 bd={} trial {}", bit_depth, _trial);
            }
        }
    }

    println!("done, {} mismatches", mismatches);
    assert_eq!(mismatches, 0);
}
