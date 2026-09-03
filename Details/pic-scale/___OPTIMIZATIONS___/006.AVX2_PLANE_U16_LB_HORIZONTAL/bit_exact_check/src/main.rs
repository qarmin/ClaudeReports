use pic_scale::{
    ImageSize, ImageStore, ImageStoreMut, ResamplingFunction, Scaler, ThreadingPolicy,
    WorkloadStrategy,
};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn make_src(width: usize, height: usize, max_val: u16) -> Vec<u16> {
    let mut v = Vec::with_capacity(width * height);
    let mut state: u32 = 0x1234_5678;
    for _ in 0..(width * height) {
        // simple xorshift PRNG, deterministic across runs/builds
        state ^= state << 13;
        state ^= state >> 17;
        state ^= state << 5;
        let val = (state % (max_val as u32 + 1)) as u16;
        v.push(val);
    }
    // force some exact boundary values in too (0 and max) at fixed offsets
    if !v.is_empty() {
        v[0] = 0;
        v[1] = max_val;
        let last = v.len() - 1;
        v[last] = 0;
        v[last - 1] = max_val;
    }
    v
}

fn run_case(
    width: usize,
    height: usize,
    dst_width: usize,
    dst_height: usize,
    bit_depth: usize,
    strategy: WorkloadStrategy,
    hasher: &mut DefaultHasher,
) {
    let max_val: u16 = ((1u32 << bit_depth as u32) - 1) as u16;
    let src_bytes = make_src(width, height, max_val);

    let mut store = ImageStore::<u16, 1>::from_slice(&src_bytes, width, height).unwrap();
    store.bit_depth = bit_depth;

    let scaler = Scaler::new(ResamplingFunction::Lanczos3)
        .set_threading_policy(ThreadingPolicy::Single)
        .set_workload_strategy(strategy);

    let resampler = scaler
        .plan_planar_resampling16(
            store.size(),
            ImageSize::new(dst_width, dst_height),
            bit_depth,
        )
        .unwrap();
    let mut scratch = resampler.alloc_scratch();
    let mut target = ImageStoreMut::alloc_with_depth(dst_width, dst_height, bit_depth);

    resampler
        .resample_with_scratch(&store, &mut target, &mut scratch)
        .unwrap();

    let out_slice: &[u16] = target.buffer.borrow();
    out_slice.hash(hasher);
    println!(
        "  bit_depth={bit_depth} strategy={strategy:?} {width}x{height}->{dst_width}x{dst_height} len={} first8={:?}",
        out_slice.len(),
        &out_slice[..out_slice.len().min(8)]
    );
}

fn main() {
    let mut hasher = DefaultHasher::new();

    // Cover: several bit depths (all <= 12, hitting the fixed-point path), up + down scale
    // ratios chosen so bounds_size (the per-destination-pixel tap count) sweeps through 1, 2,
    // 3, 4, 5, 7, 8, 9+ - exercising every batch size (1/2/4/8/16-tap) and every remainder
    // combination, plus PreferSpeed/PreferQuality (differing filter width), plus a height not a
    // multiple of 4 (exercises the single-row remainder path).
    let cases: &[(usize, usize, usize, usize, usize)] = &[
        (137, 59, 41, 17, 8),
        (137, 59, 41, 17, 10),
        (137, 59, 41, 17, 12),
        (137, 59, 41, 17, 1),
        (64, 64, 16, 16, 10),
        (64, 64, 256, 256, 10),
        (9, 9, 3, 3, 10),
        (503, 211, 127, 53, 10),
        (503, 211, 499, 209, 10),
        (8, 8, 8, 8, 10),
        (5, 5, 33, 33, 10),
        // Extreme downscale ratios (large Lanczos3 filter support) chosen specifically to push
        // bounds_size well past 16 taps/output pixel, exercising the new AVX2 16-tap main loop
        // and its jx==0/jx>0 fold-vs-no-fold branch (this candidate's avx2/plane_u16_lb.rs
        // change).
        (4001, 7, 50, 7, 10),
        (4001, 7, 17, 7, 12),
        (8009, 5, 21, 5, 8),
        (2000, 9, 31, 9, 10),
    ];

    for &(w, h, dw, dh, bd) in cases {
        for strategy in [WorkloadStrategy::PreferSpeed, WorkloadStrategy::PreferQuality] {
            run_case(w, h, dw, dh, bd, strategy, &mut hasher);
        }
    }

    println!("FINAL_HASH={}", hasher.finish());
}
