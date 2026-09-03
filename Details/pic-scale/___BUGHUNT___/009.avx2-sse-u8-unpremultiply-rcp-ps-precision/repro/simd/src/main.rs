// Real reproduction of the candidate-009 bug: calls pic-scale's own internal AVX2 u8
// unpremultiply dispatch function (`avx_unpremultiply_alpha_rgba`, src/avx2/alpha_u8.rs) via a
// doc-hidden visibility shim added by expose.diff (a zero-behavior-change `pub` wrapper around
// the crate's real `pub(crate)` function - not a reimplementation, not hand-authored SIMD).
//
// Input: RGBA(200, 200, 200, 1) repeated 8 times (one full AVX2 32-byte chunk). This is a
// genuine, valid RGBA byte buffer - exactly the type of data the crate's own resize pipeline
// hands this function after premultiply+resample (see RESULT.md for why an end-to-end trigger
// through the public `Scaler` resize API specifically was attempted extensively but not found
// within the time available this session).
use pic_scale::{WorkloadStrategy, __bughunt_candidate_009_avx2_unpremultiply_u8};

fn scalar_reference(premult: u8, alpha: u8) -> u8 {
    if alpha == 0 {
        return 0;
    }
    let value = (premult as u32 * 255 + alpha as u32 / 2) / alpha as u32;
    value.min(255) as u8
}

fn main() {
    let premult = 200u8;
    let alpha = 1u8;

    let mut src = Vec::new();
    for _ in 0..8 {
        src.extend_from_slice(&[premult, premult, premult, alpha]);
    }

    let expected_color = scalar_reference(premult, alpha);
    println!("scalar reference expects: [{expected_color}, {expected_color}, {expected_color}, {alpha}] for every pixel");

    let mut avx_quality = src.clone();
    __bughunt_candidate_009_avx2_unpremultiply_u8(&mut avx_quality, WorkloadStrategy::PreferQuality);
    println!("avx2 PreferQuality output: {avx_quality:?}");

    let mut avx_speed = src.clone();
    __bughunt_candidate_009_avx2_unpremultiply_u8(&mut avx_speed, WorkloadStrategy::PreferSpeed);
    println!("avx2 PreferSpeed output:   {avx_speed:?}");

    let quality_ok = avx_quality.chunks(4).all(|p| p[0] == expected_color && p[1] == expected_color && p[2] == expected_color && p[3] == alpha);
    let speed_ok = avx_speed.chunks(4).all(|p| p[0] == expected_color && p[1] == expected_color && p[2] == expected_color && p[3] == alpha);

    if quality_ok && speed_ok {
        println!("RESULT: matches scalar reference (bug not present / fixed)");
        std::process::exit(0);
    } else {
        println!("RESULT: DIVERGES from scalar reference (bug present)");
        std::process::exit(1);
    }
}
