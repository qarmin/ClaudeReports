use pic_scale::{
    ImageSize, ImageStore, ImageStoreMut, ResamplingFunction, Scaler, ThreadingPolicy,
    WorkloadStrategy,
};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

const CN: usize = 2;

fn make_src(width: usize, height: usize) -> Vec<u8> {
    let mut v = Vec::with_capacity(width * height * CN);
    let mut state: u32 = 0x1234_5678;
    for _ in 0..(width * height * CN) {
        // simple xorshift PRNG, deterministic across runs/builds
        state ^= state << 13;
        state ^= state >> 17;
        state ^= state << 5;
        v.push((state % 256) as u8);
    }
    if v.len() >= 2 {
        v[0] = 0;
        v[1] = 255;
        let last = v.len() - 1;
        v[last] = 0;
        v[last - 1] = 255;
    }
    v
}

fn run_case(
    width: usize,
    height: usize,
    dst_width: usize,
    dst_height: usize,
    strategy: WorkloadStrategy,
    hasher: &mut DefaultHasher,
) {
    let src_bytes = make_src(width, height);

    let store = ImageStore::<u8, CN>::from_slice(&src_bytes, width, height).unwrap();

    let scaler = Scaler::new(ResamplingFunction::Lanczos3)
        .set_threading_policy(ThreadingPolicy::Single)
        .set_workload_strategy(strategy);

    let resampler = scaler
        .plan_cbcr_resampling(store.size(), ImageSize::new(dst_width, dst_height))
        .unwrap();
    let mut scratch = resampler.alloc_scratch();
    let mut target = ImageStoreMut::alloc_with_depth(dst_width, dst_height, 8);

    resampler
        .resample_with_scratch(&store, &mut target, &mut scratch)
        .unwrap();

    let out_slice: &[u8] = target.buffer.borrow();
    out_slice.hash(hasher);
    println!(
        "  strategy={strategy:?} {width}x{height}->{dst_width}x{dst_height} len={} first8={:?}",
        out_slice.len(),
        &out_slice[..out_slice.len().min(8)]
    );
}

fn main() {
    let mut hasher = DefaultHasher::new();

    // Sweep tap counts (1, 2, 4, 8, 16, 17+) via a mix of up/down scale ratios and both
    // PreferSpeed/PreferQuality (different filter widths), plus widths not a multiple of 16
    // taps and heights not a multiple of 4 to exercise every remainder path.
    let cases: &[(usize, usize, usize, usize)] = &[
        (137, 59, 41, 17),
        (137, 59, 41, 1),
        (64, 64, 16, 16),
        (64, 64, 256, 256),
        (9, 9, 3, 3),
        (503, 211, 127, 53),
        (503, 211, 499, 209),
        (8, 8, 8, 8),
        (5, 5, 33, 33),
        (4928, 3279, 1232, 819),
        (200, 7, 50, 3),
        (1, 1, 1, 1),
        (2, 4, 6, 4),
    ];

    for &(w, h, dw, dh) in cases {
        for strategy in [WorkloadStrategy::PreferSpeed, WorkloadStrategy::PreferQuality] {
            run_case(w, h, dw, dh, strategy, &mut hasher);
        }
    }

    println!("FINAL_HASH={}", hasher.finish());
}
