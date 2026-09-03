use pic_scale::{
    BufferStore, ImageStore, ImageStoreMut, ResamplingFunction, Scaler, ThreadingPolicy,
    WorkloadStrategy,
};
use rand::RngExt;
use std::time::Instant;

// Verbatim copy of fuzz/resize_rgba/resize_rgba.rs's resize_rgba() body.
fn resize_rgba(
    data: u8,
    src_width: usize,
    src_height: usize,
    dst_width: usize,
    dst_height: usize,
    sampler: ResamplingFunction,
    workload_strategy: WorkloadStrategy,
    threading_policy: ThreadingPolicy,
    supersampling: bool,
    multi_stage_upsampling: bool,
) {
    if src_width == 0
        || src_width > 2500
        || src_height == 0
        || src_height > 2500
        || dst_width == 0
        || dst_width > 512
        || dst_height == 0
        || dst_height > 512
    {
        return;
    }

    let src_stride = (src_width + rand::rng().random_range(0..100)) * 4;

    let mut src_data = vec![data; src_stride * src_height];
    src_data[0] = 255;
    src_data[3] = 17;

    let src_valid_size = src_stride * (src_height - 1) + src_width * 4;

    let store = ImageStore {
        buffer: std::borrow::Cow::Borrowed(&src_data[..src_valid_size]),
        channels: 4,
        width: src_width,
        height: src_height,
        stride: src_stride,
        bit_depth: 8,
    };

    let dst_stride = (dst_width + rand::rng().random_range(0..100)) * 4;
    let mut dst_data_full = vec![0u8; dst_stride * dst_height];

    let dst_valid_size = dst_stride * (dst_height - 1) + dst_width * 4;

    let mut target = ImageStoreMut {
        buffer: BufferStore::Borrowed(&mut dst_data_full[..dst_valid_size]),
        channels: 4,
        width: dst_width,
        height: dst_height,
        stride: dst_stride,
        bit_depth: 8,
    };

    let scaler = Scaler::new(sampler)
        .set_workload_strategy(workload_strategy)
        .set_threading_policy(threading_policy)
        .set_supersampling(supersampling)
        .set_multi_step_upsampling(multi_stage_upsampling);
    let planned = scaler
        .plan_rgba_resampling(store.size(), target.size(), false)
        .unwrap();
    planned.resample(&store, &mut target).unwrap();
    let store = ImageStore::<u8, 4>::borrow(&src_data, src_width, src_height).unwrap();
    let planned = scaler
        .plan_rgba_resampling(store.size(), target.size(), true)
        .unwrap();
    planned.resample(&store, &mut target).unwrap();
}

fn main() {
    let sizes = [1usize, 2, 3, 5, 7, 50, 500, 2500];
    let dst_sizes = [1usize, 2, 3, 5, 7, 50, 500];
    use std::io::Write;

    let mut n = 0usize;
    for &sw in &sizes {
        for &sh in &sizes {
            for &dw in &dst_sizes {
                for &dh in &dst_sizes {
                    for &quality in &[false, true] {
                        for &threading in &[false, true] {
                            for &ss in &[false, true] {
                                for &ms in &[false, true] {
                                    let strategy = if quality {
                                        WorkloadStrategy::PreferQuality
                                    } else {
                                        WorkloadStrategy::PreferSpeed
                                    };
                                    let tp = if threading {
                                        ThreadingPolicy::Adaptive
                                    } else {
                                        ThreadingPolicy::Single
                                    };
                                    n += 1;
                                    print!(
                                        "[{}] case src={}x{} dst={}x{} q={} th={} ss={} ms={} ... ",
                                        n, sw, sh, dw, dh, quality, threading, ss, ms
                                    );
                                    std::io::stdout().flush().unwrap();
                                    let start = Instant::now();
                                    resize_rgba(
                                        200,
                                        sw,
                                        sh,
                                        dw,
                                        dh,
                                        ResamplingFunction::Bilinear,
                                        strategy,
                                        tp,
                                        ss,
                                        ms,
                                    );
                                    println!("done in {:?}", start.elapsed());
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
