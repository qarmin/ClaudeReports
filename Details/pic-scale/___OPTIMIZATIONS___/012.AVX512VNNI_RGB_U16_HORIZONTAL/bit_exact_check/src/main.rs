use image::{EncodableLayout, GenericImageView, ImageReader};
use pic_scale::{ImageSize, ImageStore, ImageStoreMut, ResamplingFunction, Scaler, ThreadingPolicy};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn main() {
    let img = ImageReader::open("../clone/assets/nasa-4928x3279-rgba.png")
        .unwrap()
        .decode()
        .unwrap();
    let dimensions = img.dimensions();
    let binding = img.to_rgb8();
    let src_bytes = binding.as_bytes();

    // Same "RGB10: Lanczos 3" transform as app/benches/resize_rgb/main.rs - bit_depth=10 is
    // <=12, so it reaches src/factory/rgb_u16.rs's `bit_depth <= 12` branch this candidate
    // touches (RGB u16, CN=3, both the row4 and row1/remainder kernels).
    let mut copied: Vec<u16> = src_bytes
        .iter()
        .map(|&x| ((x as u16) << 2) | ((x as u16) >> 6))
        .collect();

    let mut scaler = Scaler::new(ResamplingFunction::Lanczos3);
    scaler.set_threading_policy(ThreadingPolicy::Single);
    let store =
        ImageStore::<u16, 3>::from_slice(&mut copied, dimensions.0 as usize, dimensions.1 as usize)
            .unwrap();
    let resampler = scaler
        .plan_rgb_resampling16(
            store.size(),
            ImageSize::new(dimensions.0 as usize / 4, dimensions.1 as usize / 4),
            10,
        )
        .unwrap();
    let mut scratch = resampler.alloc_scratch();
    let mut target =
        ImageStoreMut::alloc_with_depth(dimensions.0 as usize / 4, dimensions.1 as usize / 4, 10);
    resampler
        .resample_with_scratch(&store, &mut target, &mut scratch)
        .unwrap();

    let out_bytes = target.buffer.borrow();
    let mut hasher = DefaultHasher::new();
    out_bytes.hash(&mut hasher);
    println!("len={} hash={:016x}", out_bytes.len(), hasher.finish());
}
