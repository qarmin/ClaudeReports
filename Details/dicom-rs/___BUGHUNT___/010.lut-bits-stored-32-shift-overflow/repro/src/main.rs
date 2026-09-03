use dicom_pixeldata::{Lut, Rescale};

fn main() {
    let result = std::panic::catch_unwind(|| {
        let lut: Lut<u16> = Lut::new_rescale(32, false, Rescale::new(1.0, 0.0))
            .expect("LUT construction with an identity rescale should not fail on value conversion");

        let low = lut.get(0u32);
        let mid = lut.get(12345u32);
        let high = lut.get(65535u32);
        (low, mid, high)
    });

    match result {
        Ok((low, mid, high)) => {
            println!("lut.get(0)     = {low}");
            println!("lut.get(12345) = {mid}");
            println!("lut.get(65535) = {high}");

            if low == mid && mid == high {
                eprintln!(
                    "REPRODUCED - BitsStored=32 collapsed the LUT to a single degenerate entry: \
                     every distinct sample value maps to the same output ({low}) instead of the \
                     identity mapping the Rescale(1.0, 0.0) parameters specify"
                );
                std::process::exit(1);
            }
            println!("did not reproduce - LUT correctly distinguishes different sample values");
        }
        Err(_) => {
            // A panic here means `Lut::new_with_fn`'s own precondition check rejected
            // bits_stored=32 outright (its documented, intentional "Panics if..." contract for
            // direct API misuse) - this is the fixed, correct behavior, not the bug. The bug was
            // BitsStored=32 being silently accepted and producing a degenerate, wrong LUT with no
            // error or panic at all.
            println!("did not reproduce - construction was rejected (panicked) instead of silently succeeding with a degenerate table");
        }
    }
}
