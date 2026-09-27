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
    let src_bytes = img.as_bytes();

    // Same "RGBA10 without alpha: Lanczos 3" transform as app/benches/resize_rgba/main.rs -
    // bit_depth=10 is <=12, so it reaches src/factory/rgba_u16.rs's `bit_depth <= 12` branch
    // this candidate touches (RGBA u16, CN=4, both the row4 and row1/remainder kernels).
    let mut copied_rgba: Vec<u16> = src_bytes
        .iter()
        .map(|&x| ((x as u16) << 2) | ((x as u16) >> 6))
        .collect();
    let mut scaler = Scaler::new(ResamplingFunction::Lanczos3);
    scaler.set_threading_policy(ThreadingPolicy::Single);
    let store_rgba =
        ImageStore::<u16, 4>::from_slice(&mut copied_rgba, dimensions.0 as usize, dimensions.1 as usize)
            .unwrap();
    let resampler_rgba = scaler
        .plan_rgba_resampling16(
            store_rgba.size(),
            ImageSize::new(dimensions.0 as usize / 4, dimensions.1 as usize / 4),
            false,
            10,
        )
        .unwrap();
    let mut scratch_rgba = resampler_rgba.alloc_scratch();
    let mut target_rgba =
        ImageStoreMut::alloc_with_depth(dimensions.0 as usize / 4, dimensions.1 as usize / 4, 10);
    resampler_rgba
        .resample_with_scratch(&store_rgba, &mut target_rgba, &mut scratch_rgba)
        .unwrap();
    let out_bytes_rgba = target_rgba.buffer.borrow();
    let mut hasher_rgba = DefaultHasher::new();
    out_bytes_rgba.hash(&mut hasher_rgba);
    println!(
        "rgba16: len={} hash={:016x}",
        out_bytes_rgba.len(),
        hasher_rgba.finish()
    );

    // Same "Plane10: Lanczos 3" transform as app/benches/resize_plane/main.rs - bit_depth=10,
    // Plane u16, CN=1, reaches src/factory/plane_u16.rs's row4/row1 kernels this candidate
    // touches.
    let binding16 = img.to_luma16();
    let copied_plane: Vec<u16> = binding16.as_raw().iter().map(|&x| x >> 6).collect();
    let store_plane =
        ImageStore::<u16, 1>::from_slice(&copied_plane, dimensions.0 as usize, dimensions.1 as usize)
            .unwrap();
    let resampler_plane = scaler
        .plan_planar_resampling16(
            store_plane.size(),
            ImageSize::new(dimensions.0 as usize / 4, dimensions.1 as usize / 4),
            10,
        )
        .unwrap();
    let mut scratch_plane = resampler_plane.alloc_scratch();
    let mut target_plane =
        ImageStoreMut::alloc_with_depth(dimensions.0 as usize / 4, dimensions.1 as usize / 4, 10);
    resampler_plane
        .resample_with_scratch(&store_plane, &mut target_plane, &mut scratch_plane)
        .unwrap();
    let out_bytes_plane = target_plane.buffer.borrow();
    let mut hasher_plane = DefaultHasher::new();
    out_bytes_plane.hash(&mut hasher_plane);
    println!(
        "plane16: len={} hash={:016x}",
        out_bytes_plane.len(),
        hasher_plane.finish()
    );
}
