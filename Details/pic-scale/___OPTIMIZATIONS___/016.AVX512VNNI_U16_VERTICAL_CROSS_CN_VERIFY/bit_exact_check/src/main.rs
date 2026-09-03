use image::{GenericImageView, ImageReader};
use pic_scale::{
    ImageSize, ImageStore, ImageStoreMut, ResamplingFunction, Scaler, ThreadingPolicy,
    WorkloadStrategy,
};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn hash_and_print(label: &str, values: &[u16]) {
    let mut hasher = DefaultHasher::new();
    values.hash(&mut hasher);
    println!("{label}: len={} hash={:016x}", values.len(), hasher.finish());
}

fn main() {
    let img = ImageReader::open("../clone/assets/nasa-4928x3279-rgba.png")
        .unwrap()
        .decode()
        .unwrap();
    let dimensions = img.dimensions();

    // "Pic scale CbCr10: Lanczos 3" - same transform as app/benches/resize_cbcr/main.rs, CN=2.
    {
        let binding = img.to_luma_alpha16();
        let src_bytes = binding.as_raw().to_vec();
        let copied: Vec<u16> = src_bytes.to_vec();
        let mut store = ImageStore::<u16, 2>::from_slice(
            &copied,
            dimensions.0 as usize,
            dimensions.1 as usize,
        )
        .unwrap();
        store.bit_depth = 10;
        let scaler = Scaler::new(ResamplingFunction::Lanczos3)
            .set_threading_policy(ThreadingPolicy::Single)
            .set_workload_strategy(WorkloadStrategy::PreferSpeed);
        let resampler = scaler
            .plan_cbcr_resampling16(
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
        hash_and_print("CbCr10", target.buffer.borrow());
    }

    // "Pic scale RGB10: Lanczos 3" - same transform as app/benches/resize_rgb/main.rs, CN=3.
    {
        let binding = img.to_rgb8();
        let src_bytes = binding.as_raw();
        let mut copied: Vec<u16> = src_bytes
            .iter()
            .map(|&x| ((x as u16) << 2) | ((x as u16) >> 6))
            .collect();
        let mut scaler = Scaler::new(ResamplingFunction::Lanczos3);
        scaler.set_threading_policy(ThreadingPolicy::Single);
        let store = ImageStore::<u16, 3>::from_slice(
            &mut copied,
            dimensions.0 as usize,
            dimensions.1 as usize,
        )
        .unwrap();
        let resampler = scaler
            .plan_rgb_resampling16(
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
        hash_and_print("RGB10", target.buffer.borrow());
    }

    // "Pic scale RGBA10 without alpha: Lanczos 3" - same transform as
    // app/benches/resize_rgba/main.rs, CN=4.
    {
        let src_bytes = img.as_bytes();
        let mut copied: Vec<u16> = src_bytes
            .iter()
            .map(|&x| ((x as u16) << 2) | ((x as u16) >> 6))
            .collect();
        let store = ImageStore::<u16, 4>::from_slice(
            &mut copied,
            dimensions.0 as usize,
            dimensions.1 as usize,
        )
        .unwrap();
        let scaler =
            Scaler::new(ResamplingFunction::Lanczos3).set_threading_policy(ThreadingPolicy::Single);
        let resampler = scaler
            .plan_rgba_resampling16(
                store.size(),
                ImageSize::new(dimensions.0 as usize / 4, dimensions.1 as usize / 4),
                false,
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
        hash_and_print("RGBA10_no_alpha", target.buffer.borrow());
    }
}
