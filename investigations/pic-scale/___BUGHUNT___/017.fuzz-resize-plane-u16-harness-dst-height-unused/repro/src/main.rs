// Reproduces the fuzz/resize_plane_u16/resize_plane_u16.rs harness bug: the fuzz_target! closure
// calls
//
//     resize_rgb(data.src_width as usize, data.src_height as usize,
//                data.dst_width as usize, data.dst_width as usize, ...)
//
// - the 3rd AND 4th positional arguments (dst_width, dst_height) are both `data.dst_width`. The
// `Arbitrary`-derived `SrcImage` struct DOES generate a `dst_height` field (the fuzzer decodes a
// real value for it from the input bytes), but that value is never read anywhere in the harness,
// so every execution of this fuzz target - no matter what bytes libFuzzer feeds it - can only ever
// resize to a SQUARE destination (dst_width == dst_height). This is byte-for-byte the same copy-
// paste mistake candidate 016 found and fixed in fuzz/resize_plane/resize_plane.rs, independently
// present in this sibling harness.
//
// `SrcImage` and the function below are verified byte-identical (argument-passing logic) to
// fuzz/resize_plane_u16/resize_plane_u16.rs at the pinned commit (see the candidate justfile's
// verify-source-before/-after recipes), other than (a) being a free function instead of an inline
// fuzz_target! closure body, since libfuzzer-sys's fuzz_target! macro requires a libFuzzer-linked
// driver binary and can't be invoked standalone here, and (b) additionally returning the actual
// `target.size()` reached from the first (u16, 10-bit) resample so it can be observed from a plain
// `fn main` - the argument-passing logic that IS the bug is untouched.
//
// This is a single-artifact finding (fuzzer-chosen dimension bytes -> a harness function), not one
// participant reacting to another's produced input across a process/protocol boundary, so a
// single-process repro is the correct shape per CONFIRMATION.md section 11's carve-out (directly
// analogous to "a malformed file handed to a parser's real entry point").
//
// `fuzzer_found_input.bin` is not hand-authored: it is a real corpus file libFuzzer itself kept
// (because it triggered new coverage) during a real 300s/241,522-exec run of the FIXED harness
// against this exact host/build this session (see ../output.txt and ../RESULT.md).

use arbitrary::{Arbitrary, Unstructured};
use pic_scale::{
    ImageStore, ImageStoreMut, ResamplingFunction, Scaler, ThreadingPolicy, WorkloadStrategy,
};

#[derive(Clone, Debug, Arbitrary)]
pub struct SrcImage {
    pub src_width: u16,
    pub src_height: u16,
    pub dst_width: u16,
    pub dst_height: u16,
    pub value: u16,
    pub use_quality: bool,
    pub premultiply_alpha: bool,
    pub threading: bool,
}

#[allow(clippy::too_many_arguments)]
fn resize_rgb(
    src_width: usize,
    src_height: usize,
    dst_width: usize,
    dst_height: usize,
    sampler: ResamplingFunction,
    use_quality: bool,
    value: u16,
    threading_policy: ThreadingPolicy,
) -> Option<(usize, usize)> {
    if src_width == 0
        || src_width > 2000
        || src_height == 0
        || src_height > 2000
        || dst_width == 0
        || dst_width > 512
        || dst_height == 0
        || dst_height > 512
    {
        return None;
    }

    let src_slice = vec![value; src_width * src_height];
    let store = ImageStore::<u16, 1>::borrow(&src_slice, src_width, src_height).unwrap();
    let mut target = ImageStoreMut::alloc_with_depth(dst_width, dst_height, 10);

    let scaler = Scaler::new(sampler)
        .set_threading_policy(threading_policy)
        .set_workload_strategy(if use_quality {
            WorkloadStrategy::PreferQuality
        } else {
            WorkloadStrategy::PreferSpeed
        });

    let planned = scaler
        .plan_planar_resampling16(store.size(), target.size(), 10)
        .unwrap();
    planned.resample(&store, &mut target).unwrap();

    let size = target.size();
    Some((size.width, size.height))
}

// The exact buggy call site: fuzz/resize_plane_u16/resize_plane_u16.rs lines 54-55 pass
// `data.dst_width` for BOTH the 3rd (dst_width) and 4th (dst_height) positional arguments.
fn call_as_written_before_fix(data: &SrcImage) -> Option<(usize, usize)> {
    resize_rgb(
        data.src_width as usize,
        data.src_height as usize,
        data.dst_width as usize,
        data.dst_width as usize,
        ResamplingFunction::Lanczos3,
        data.use_quality,
        data.value,
        if data.threading {
            ThreadingPolicy::Adaptive
        } else {
            ThreadingPolicy::Single
        },
    )
}

// patch.diff's only change: the 4th positional argument becomes `data.dst_height`.
fn call_as_written_after_fix(data: &SrcImage) -> Option<(usize, usize)> {
    resize_rgb(
        data.src_width as usize,
        data.src_height as usize,
        data.dst_width as usize,
        data.dst_height as usize,
        ResamplingFunction::Lanczos3,
        data.use_quality,
        data.value,
        if data.threading {
            ThreadingPolicy::Adaptive
        } else {
            ThreadingPolicy::Single
        },
    )
}

fn main() {
    let bytes = std::fs::read("fuzzer_found_input.bin").expect("read fuzzer_found_input.bin");
    let data = SrcImage::arbitrary_take_rest(Unstructured::new(&bytes))
        .expect("decode SrcImage via the real Arbitrary derive, same as libFuzzer would");

    println!("decoded from real libFuzzer-kept corpus bytes: {data:?}");
    println!(
        "src = {}x{}, requested dst = {}x{}",
        data.src_width, data.src_height, data.dst_width, data.dst_height
    );

    let before = call_as_written_before_fix(&data);
    let after = call_as_written_after_fix(&data);

    println!("call_as_written_before_fix -> target actually resized to: {before:?}");
    println!("call_as_written_after_fix  -> target actually resized to: {after:?}");

    let (bw, bh) = before.expect("valid dims (checked in corpus selection)");
    let (aw, ah) = after.expect("valid dims (checked in corpus selection)");

    assert_eq!(
        bw, data.dst_width as usize,
        "sanity: before-fix width should still be dst_width"
    );
    assert_eq!(
        bh, data.dst_width as usize,
        "REPRODUCED: before-fix height came from dst_width, not the fuzzer-supplied dst_height ({})",
        data.dst_height
    );
    assert_ne!(
        data.dst_width, data.dst_height,
        "sanity: this input must actually request a rectangular destination"
    );
    assert_eq!(
        (aw, ah),
        (data.dst_width as usize, data.dst_height as usize),
        "after-fix: target should honor the fuzzer-supplied dst_height"
    );

    println!(
        "REPRODUCED: before-fix harness call always ignores dst_height ({dh}) and produces a \
         square {bw}x{bh} target instead of the requested {dw}x{dh}; after-fix call correctly \
         produces {aw}x{ah}.",
        dh = data.dst_height,
        dw = data.dst_width,
    );
}
