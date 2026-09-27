// Standalone reproduction for GitHub issue #283: three separate
// "attempt to multiply with overflow" panics in Decoder::decode_scan, all caused by the same
// pattern - a u16 MCU-grid loop counter (mcu_x/mcu_y) multiplied by the literal 8 with no
// widening, while comparing against frame.image_size.{width,height} (also u16). With overflow
// checks enabled (the default in a debug build, or with `overflow-checks = true` as set in this
// crate's own [profile.release]), reaching an MCU count >= 8192 in that inner loop panics
// instead of returning a decode error.
//
// Each input file below is one of the three attached fuzzer-found inputs from the issue, driving
// the real, public `jpeg_decoder::Decoder::decode()` end to end - not hand-constructed decoder
// internal state.

use jpeg_decoder::Decoder;
use std::panic;

const CASES: &[(&str, &[u8])] = &[
    (
        "panic_1025_overflow.jpg (decoder.rs:1025, MCU-row-send bound check)",
        include_bytes!("../inputs/panic_1025_overflow.jpg"),
    ),
    (
        "panic_911_overflow.jpg (decoder.rs:911, mcu_y * 8 bound check)",
        include_bytes!("../inputs/panic_911_overflow.jpg"),
    ),
    (
        "panic_916_overflow.jpg (decoder.rs:916, mcu_x * 8 bound check)",
        include_bytes!("../inputs/panic_916_overflow.jpg"),
    ),
];

fn main() {
    let mut any_panicked = false;

    for (name, data) in CASES {
        print!("{name}: ");
        let result = panic::catch_unwind(|| {
            let mut decoder = Decoder::new(*data);
            decoder.decode()
        });

        match result {
            Ok(Ok(_)) => println!("decoded OK (no panic)"),
            Ok(Err(e)) => println!("decode error, no panic: {e:?}"),
            Err(_) => {
                println!("PANICKED (attempt to multiply with overflow)");
                any_panicked = true;
            }
        }
    }

    if any_panicked {
        // Non-zero exit lets `just test`'s reproduce-before/reproduce-after steps be eyeballed
        // easily: this process exits with an error status while the bug is present.
        std::process::exit(101);
    }
}
