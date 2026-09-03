// Real end-to-end reproduction via pic-scale's PUBLIC API only (Scaler / ImageStore /
// ImageStoreMut / ResamplingFunction / WorkloadStrategy) - no internal/private functions are
// called here.
//
// This binary and its siblings `repro_sse`/`repro_scalar` run the *identical* source against the
// *identical* deterministic input, through the *identical* public resize call (8-bit RGBA,
// premultiply_alpha = true, Lanczos3 filter, 4000 -> 3800 pixels wide). The only difference
// between the three binaries is which Cargo features are enabled on the `pic-scale` dependency
// (see each crate's Cargo.toml), which determines whether AVX2, SSE4.1, or the scalar fallback
// handles unpremultiply internally. Any output difference is therefore attributable purely to
// that internal dispatch choice, not to anything this repro does differently.
//
// Random (translucent, uncorrelated-channel) RGBA content is enough to trigger this bug on a
// large fraction of output pixels (no adversarial construction needed, unlike candidate 009's own
// packus-signedness bug, which this repro's random content essentially never triggers - see this
// candidate's RESULT.md for the isolation argument).
use pic_scale::{ImageSize, ImageStore, ImageStoreMut, ResamplingFunction, Scaler, WorkloadStrategy};

struct Xorshift64(u64);
impl Xorshift64 {
    fn next_u8(&mut self) -> u8 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        (x >> 33) as u8
    }
}

fn main() {
    let width = 4000usize;
    let height = 1usize;
    let dst_width = 3800usize;

    let mut args = std::env::args().skip(1);
    let seed: u64 = args.next().map(|s| s.parse().unwrap()).unwrap_or(1);
    let strategy_arg = args.next().unwrap_or_else(|| "speed".to_string());
    let strategy = match strategy_arg.as_str() {
        "quality" => WorkloadStrategy::PreferQuality,
        _ => WorkloadStrategy::PreferSpeed,
    };

    let mut rng = Xorshift64(seed.wrapping_mul(2685821657736338717).wrapping_add(1));
    let mut src = vec![0u8; width * height * 4];
    for px in src.chunks_mut(4) {
        px[0] = rng.next_u8();
        px[1] = rng.next_u8();
        px[2] = rng.next_u8();
        px[3] = rng.next_u8();
    }

    let src_size = ImageSize::new(width, height);
    let dst_size = ImageSize::new(dst_width, height);

    let store = ImageStore::<u8, 4>::from_slice(&src, width, height).unwrap();
    let mut dst = ImageStoreMut::<u8, 4>::alloc(dst_width, height);

    let mut scaler = Scaler::new(ResamplingFunction::Lanczos3);
    scaler.set_workload_strategy(strategy);
    let plan = scaler
        .plan_rgba_resampling(src_size, dst_size, true)
        .unwrap();
    plan.resample(&store, &mut dst).unwrap();

    for v in dst.as_bytes() {
        println!("{v}");
    }
}
