// Real end-to-end reproduction via pic-scale's PUBLIC API only (Scaler / ImageStore /
// ImageStoreMut / ResamplingFunction) - no internal/private functions are called here.
//
// This binary and its sibling `repro_scalar` run the *identical* source against the *identical*
// random input, through the *identical* public resize call (16-bit RGBA, bit_depth = 16,
// premultiply_alpha = true, Bilinear downscale 512 -> 256 pixels wide). The only difference
// between the two is which Cargo features are enabled on the `pic-scale` dependency (see each
// crate's Cargo.toml) - which determines whether the AVX2/SSE runtime-detected fast path or the
// scalar fallback handles premultiply/unpremultiply internally. Any output difference between the
// two runs is therefore attributable purely to that internal dispatch choice, not to anything this
// repro does differently.
use pic_scale::{ImageSize, ImageStore, ImageStoreMut, ResamplingFunction, Scaler};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

fn main() {
    let width = 512usize;
    let height = 1usize;
    let dst_width = 256usize;
    let bit_depth = 16usize;
    let seed: u64 = std::env::args()
        .nth(1)
        .map(|s| s.parse().unwrap())
        .unwrap_or(1);
    let mut rng = StdRng::seed_from_u64(seed);

    let mut src = vec![0u16; width * height * 4];
    for px in src.chunks_mut(4) {
        px[0] = rng.gen_range(0..=65535u16);
        px[1] = rng.gen_range(0..=65535u16);
        px[2] = rng.gen_range(0..=65535u16);
        px[3] = rng.gen_range(1..=65535u16);
    }

    let src_size = ImageSize::new(width, height);
    let dst_size = ImageSize::new(dst_width, height);

    let store = ImageStore::<u16, 4>::from_slice(&src, width, height).unwrap();
    let mut dst = ImageStoreMut::<u16, 4>::alloc_with_depth(dst_width, height, bit_depth);

    let scaler = Scaler::new(ResamplingFunction::Bilinear);
    let plan = scaler
        .plan_rgba_resampling16(src_size, dst_size, true, bit_depth)
        .unwrap();
    plan.resample(&store, &mut dst).unwrap();

    for v in dst.as_bytes() {
        println!("{v}");
    }
}
