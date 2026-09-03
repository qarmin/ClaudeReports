// Standalone reproduction for the zero-width panic in
// has_non_constant_alpha_* / has_non_constant_alpha_f32_impl (src/alpha_check.rs).
//
// Both the generic integer path (has_non_constant_alpha) and the f32 path
// (has_non_constant_alpha_f32_impl) only guard against an empty `store`
// before calling `store.chunks_exact(width * CHANNELS)`. When `width == 0`
// and `store` is non-empty, the chunk size is 0, and `chunks_exact` panics
// unconditionally ("chunk size must be non-zero") instead of returning a
// sensible result or an error.
//
// Exit code convention: 0 = did not reproduce, non-zero = reproduced (panic).

use pic_scale_safe::{
    has_non_constant_alpha_la16, has_non_constant_alpha_la8, has_non_constant_alpha_luma_alpha_f32,
    has_non_constant_alpha_rgba16, has_non_constant_alpha_rgba8, has_non_constant_alpha_rgba_f32,
};

fn main() {
    let mut reproduced = false;

    macro_rules! try_call {
        ($name:expr, $call:expr) => {
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| $call));
            match result {
                Ok(_) => println!("{}: did not panic", $name),
                Err(payload) => {
                    eprintln!("{}: REPRODUCED - panicked: {:?}", $name, payload);
                    reproduced = true;
                }
            }
        };
    }

    let store_u8: Vec<u8> = vec![1, 2, 3, 4];
    try_call!(
        "has_non_constant_alpha_rgba8",
        has_non_constant_alpha_rgba8(&store_u8, 0)
    );
    try_call!(
        "has_non_constant_alpha_la8",
        has_non_constant_alpha_la8(&store_u8, 0)
    );

    let store_u16: Vec<u16> = vec![1, 2, 3, 4];
    try_call!(
        "has_non_constant_alpha_rgba16",
        has_non_constant_alpha_rgba16(&store_u16, 0)
    );
    try_call!(
        "has_non_constant_alpha_la16",
        has_non_constant_alpha_la16(&store_u16, 0)
    );

    let store_f32: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0];
    try_call!(
        "has_non_constant_alpha_rgba_f32",
        has_non_constant_alpha_rgba_f32(&store_f32, 0)
    );
    try_call!(
        "has_non_constant_alpha_luma_alpha_f32",
        has_non_constant_alpha_luma_alpha_f32(&store_f32, 0)
    );

    if reproduced {
        std::process::exit(1);
    } else {
        println!("did not reproduce - all functions completed normally");
    }
}
