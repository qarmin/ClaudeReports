// Standalone reproduction for the out-of-bounds index in
// premultiply_luma_alpha_f32 / premultiplied_luma_alpha_f32 (src/alpha.rs).
//
// Both functions treat a 2-element (luma, alpha) chunk as if it had a third
// element at index 2, copy-pasted from the 4-channel RGBA sibling functions
// which legitimately index chunk[3]. Any non-empty input panics.
//
// Exit code convention: 0 = did not reproduce, non-zero = reproduced (panic).

fn main() {
    let mut in_place: Vec<f32> = vec![1.0, 0.5];
    let result_a = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        pic_scale_safe::premultiply_luma_alpha_f32(&mut in_place);
    }));

    let source: Vec<f32> = vec![1.0, 0.5];
    let result_b = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        pic_scale_safe::premultiplied_luma_alpha_f32(&source)
    }));

    let mut reproduced = false;

    match result_a {
        Ok(_) => println!("premultiply_luma_alpha_f32: did not panic"),
        Err(payload) => {
            eprintln!("premultiply_luma_alpha_f32: REPRODUCED - panicked: {:?}", payload);
            reproduced = true;
        }
    }

    match result_b {
        Ok(_) => println!("premultiplied_luma_alpha_f32: did not panic"),
        Err(payload) => {
            eprintln!("premultiplied_luma_alpha_f32: REPRODUCED - panicked: {:?}", payload);
            reproduced = true;
        }
    }

    if reproduced {
        std::process::exit(1);
    } else {
        println!("did not reproduce - both functions completed normally");
    }
}
