//! Standalone reproduction for candidate 022.
//!
//! Drives the real, public `Scaler::plan_ar30_resampling` entry point with a
//! `source_size` whose width is chosen so that `width * height * 3 * size_of::<u16>()`
//! overflows `usize`, while `destination_size` stays modest and a fixed-kernel filter
//! (`Spline16`) is used so the resize itself never approaches any of the already-fixed
//! `generate_weights` overflow paths (candidates 010/021) - isolating this candidate's
//! own, distinct unchecked arithmetic in `src/plan/ar30_plan.rs::ScratchLayout`.
//!
//! Then calls the real, documented `resample()` entry point on a tiny, correctly-sized
//! real source/destination buffer - `ImageStore`'s own width/height are independent
//! fields from the plan's `source_size`, so this demonstrates the panic is reachable
//! purely from the `ImageSize` metadata passed to `plan_ar30_resampling`, not from
//! needing to actually allocate an exabyte-scale image.

use pic_scale::{Ar30ByteOrder, ImageSize, ImageStore, ImageStoreMut, ResamplingFunction, Scaler};

fn main() {
    let scaler = Scaler::new(ResamplingFunction::Spline16);

    let source_size = ImageSize::new(10_000_000_000_000_000_000usize, 100);
    let destination_size = ImageSize::new(100, 100);

    println!(
        "Calling plan_ar30_resampling with source_size={:?} destination_size={:?}",
        source_size, destination_size
    );

    let plan = match scaler.plan_ar30_resampling(source_size, destination_size, Ar30ByteOrder::Host) {
        Ok(plan) => {
            println!("Plan construction returned Ok (did not reproduce here)");
            plan
        }
        Err(e) => {
            println!("Plan construction cleanly returned Err: {e}");
            println!("did not reproduce");
            return;
        }
    };

    let src_buf = vec![0u8; 100 * 100 * 4];
    let store = ImageStore::<u8, 4>::new(src_buf, 100, 100).expect("tiny store construction");
    let dst_buf = vec![0u8; 100 * 100 * 4];
    let mut dst_store =
        ImageStoreMut::<u8, 4>::new(dst_buf, 100, 100).expect("tiny dst store construction");

    println!("Plan constructed OK, now calling the real resample() entry point...");
    match plan.resample(&store, &mut dst_store) {
        Ok(()) => println!("resample() returned Ok, did not reproduce"),
        Err(e) => println!("resample() cleanly returned Err: {e}\ndid not reproduce"),
    }
}
