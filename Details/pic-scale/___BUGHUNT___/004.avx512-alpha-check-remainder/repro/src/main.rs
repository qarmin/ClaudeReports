use pic_scale::{
    BufferStore, ImageStore, ImageStoreMut, ResamplingFunction, Scaler, ThreadingPolicy,
};

fn run(premultiply_alpha: bool) -> Vec<u8> {
    // 8x1 RGBA image. Last pixel (index 7) is bright and translucent; everything else is
    // opaque dark gray. width*4 = 32 bytes < 64, so it fits entirely within the AVX-512
    // alpha-check's dropped "remainder" region (see check_alpha.rs `as_chunks::<64>().0`).
    let src_width = 8usize;
    let src_height = 1usize;
    let mut src_data = vec![10u8, 10, 10, 255].repeat(src_width);
    // last pixel: bright, translucent
    let last = (src_width - 1) * 4;
    src_data[last] = 250;
    src_data[last + 1] = 250;
    src_data[last + 2] = 250;
    src_data[last + 3] = 40;

    let store = ImageStore::<u8, 4> {
        buffer: std::borrow::Cow::Borrowed(&src_data),
        channels: 4,
        width: src_width,
        height: src_height,
        stride: src_width * 4,
        bit_depth: 8,
    };

    let dst_width = 4usize;
    let dst_height = 1usize;
    let mut dst_data = vec![0u8; dst_width * dst_height * 4];
    let mut target = ImageStoreMut::<u8, 4> {
        buffer: BufferStore::Borrowed(&mut dst_data),
        channels: 4,
        width: dst_width,
        height: dst_height,
        stride: dst_width * 4,
        bit_depth: 8,
    };

    let scaler = Scaler::new(ResamplingFunction::Bilinear)
        .set_threading_policy(ThreadingPolicy::Single);
    let planned = scaler
        .plan_rgba_resampling(store.size(), target.size(), premultiply_alpha)
        .unwrap();
    planned.resample(&store, &mut target).unwrap();
    dst_data
}

fn main() {
    println!(
        "avx512bw detected: {}",
        std::arch::is_x86_feature_detected!("avx512bw")
    );

    let with_premultiply = run(true);
    let without_premultiply = run(false);

    println!("resample(premultiply_alpha=true)  -> {:?}", with_premultiply);
    println!("resample(premultiply_alpha=false) -> {:?}", without_premultiply);

    if with_premultiply == without_premultiply {
        eprintln!(
            "REPRODUCED: premultiply_alpha=true produced byte-identical output to \
             premultiply_alpha=false - premultiplication was silently skipped even though \
             the image has non-constant (translucent) alpha."
        );
        std::process::exit(1);
    } else {
        println!("did not reproduce - premultiply_alpha=true changed the output as expected");
    }
}
