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
    let src_bytes = img.as_bytes();

    let copied: Vec<u8> = Vec::from(src_bytes);
    let store =
        ImageStore::<u8, 4>::from_slice(&copied, dimensions.0 as usize, dimensions.1 as usize)
            .unwrap();
    let scaler = Scaler::new(ResamplingFunction::Lanczos3)
        .set_threading_policy(ThreadingPolicy::Single)
        .set_workload_strategy(WorkloadStrategy::PreferQuality);
    let resampler = scaler
        .plan_rgba_resampling(
            store.size(),
            ImageSize::new(dimensions.0 as usize / 4, dimensions.1 as usize / 4),
            true,
        )
        .unwrap();
    let mut scratch = resampler.alloc_scratch();
    let mut target = ImageStoreMut::alloc(dimensions.0 as usize / 4, dimensions.1 as usize / 4);
    resampler
        .resample_with_scratch(&store, &mut target, &mut scratch)
        .unwrap();

    let out_bytes = target.buffer.borrow();
    let mut hasher = DefaultHasher::new();
    out_bytes.hash(&mut hasher);
    println!("len={} hash={:016x}", out_bytes.len(), hasher.finish());
}
