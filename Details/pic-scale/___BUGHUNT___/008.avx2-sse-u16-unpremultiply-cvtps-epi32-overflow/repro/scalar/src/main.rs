// Real end-to-end reproduction via pic-scale's PUBLIC API only (Scaler / ImageStore /
// ImageStoreMut / ResamplingFunction) - no internal/private functions are called here.
//
// This binary and its siblings `repro_scalar`/`repro_sse` run the *identical* source against the
// *identical* deterministic input, through the *identical* public resize call (16-bit RGBA,
// bit_depth = 16, premultiply_alpha = true, Ginseng filter, 10000 -> 9999 pixels wide - a
// negative-lobe filter at a near-1:1 downscale ratio, needed so the convolution stage's ringing
// can drive some output pixel's alpha down to exactly 1 while its color channel convolves to a
// value whose unpremultiply product (color * (max_colors / alpha)) exceeds i32::MAX). The only
// difference between the three binaries is which Cargo features are enabled on the `pic-scale`
// dependency (see each crate's Cargo.toml), which determines whether AVX2, SSE4.1, or the scalar
// fallback handles unpremultiply internally. Any output difference is therefore attributable
// purely to that internal dispatch choice, not to anything this repro does differently.
use pic_scale::{ImageSize, ImageStore, ImageStoreMut, ResamplingFunction, Scaler};

struct Xorshift64(u64);
impl Xorshift64 {
    fn next_u16(&mut self) -> u16 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        (x >> 33) as u16
    }
}

fn main() {
    let width = 10000usize;
    let height = 1usize;
    let dst_width = 9999usize;
    let bit_depth = 16usize;
    let seed: u64 = std::env::args()
        .nth(1)
        .map(|s| s.parse().unwrap())
        .unwrap_or(765);

    let mut rng = Xorshift64(seed.wrapping_mul(2685821657736338717).wrapping_add(1));
    let mut src = vec![0u16; width * height * 4];
    for px in src.chunks_mut(4) {
        px[0] = rng.next_u16();
        px[1] = rng.next_u16();
        px[2] = rng.next_u16();
        px[3] = rng.next_u16();
    }

    let src_size = ImageSize::new(width, height);
    let dst_size = ImageSize::new(dst_width, height);

    let store = ImageStore::<u16, 4>::from_slice(&src, width, height).unwrap();
    let mut dst = ImageStoreMut::<u16, 4>::alloc_with_depth(dst_width, height, bit_depth);

    let scaler = Scaler::new(ResamplingFunction::Ginseng);
    let plan = scaler
        .plan_rgba_resampling16(src_size, dst_size, true, bit_depth)
        .unwrap();
    plan.resample(&store, &mut dst).unwrap();

    for v in dst.as_bytes() {
        println!("{v}");
    }
}
