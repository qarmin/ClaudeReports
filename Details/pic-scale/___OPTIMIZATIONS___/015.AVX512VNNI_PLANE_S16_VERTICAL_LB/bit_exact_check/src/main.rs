use image::{GenericImageView, ImageReader};
use pic_scale::{
    ImageSize, ImageStore, ImageStoreMut, ResamplingFunction, Scaler, ThreadingPolicy,
    WorkloadStrategy,
};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn main() {
    let img = ImageReader::open("../clone/assets/nasa-4928x3279-rgba.png")
        .unwrap()
        .decode()
        .unwrap();
    let dimensions = img.dimensions();

    // Same "Plane S10: Lanczos 3" transform as app/benches/resize_plane/main.rs - bit_depth=10 is
    // <=12, so it reaches src/factory/plane_s16.rs's LB column-filter branch this candidate
    // touches (Plane i16, CN=1, both the horizontal and vertical/column passes; PreferSpeed
    // downscale exercises the vertical convolution pass at every output row).
    let binding16 = img.to_luma16();
    let copied: Vec<i16> = binding16
        .as_raw()
        .iter()
        .map(|&x| (x >> 6) as i16 - 512)
        .collect();
    let store =
        ImageStore::<i16, 1>::from_slice(&copied, dimensions.0 as usize, dimensions.1 as usize)
            .unwrap();
    let mut scaler = Scaler::new(ResamplingFunction::Lanczos3);
    scaler.set_threading_policy(ThreadingPolicy::Single);
    scaler.set_workload_strategy(WorkloadStrategy::PreferSpeed);
    let resampler = scaler
        .plan_planar_resampling_s16(
            store.size(),
            ImageSize::new(dimensions.0 as usize / 4, dimensions.1 as usize / 4),
            10,
        )
        .unwrap();
    let mut scratch = resampler.alloc_scratch();
    let mut target = ImageStoreMut::alloc_with_depth(
        dimensions.0 as usize / 4,
        dimensions.1 as usize / 4,
        10,
    );
    resampler
        .resample_with_scratch(&store, &mut target, &mut scratch)
        .unwrap();

    let out_bytes = target.buffer.borrow();
    let mut hasher = DefaultHasher::new();
    out_bytes.hash(&mut hasher);
    println!("len={} hash={:016x}", out_bytes.len(), hasher.finish());
}
