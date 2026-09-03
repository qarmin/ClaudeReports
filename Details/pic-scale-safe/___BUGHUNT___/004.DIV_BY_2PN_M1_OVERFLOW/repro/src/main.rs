// Standalone reproduction: `div_by_2pn_m1` (src/alpha.rs) does `u32` arithmetic on
// `pixel * alpha` assuming the product fits the claimed `bit_depth`, but nothing validates
// that a caller's raw u16 pixel/alpha data actually stays within `bit_depth` bits.
//
// `premultiply_rgba16(&mut data, bit_depth)` with a small `bit_depth` (e.g. 1) but full-range
// u16 pixel/alpha values (65535) drives `v = pixel*alpha` up to ~4.29e9, and the subsequent
// `(v >> n) + v` addition overflows `u32` (max ~4.295e9).
//
// This must be run as a debug build (`cargo run`, NOT `--release`) - overflow-checks are what
// turn this into an observable panic; in release the same bug silently wraps to a wrong value
// instead (checked separately below via an overflow-free oracle so this repro also catches that
// case even if accidentally run in release mode).
//
// Exit code convention: 0 = did not reproduce, non-zero = reproduced.

fn oracle_div_by_2pn_m1(v: u32, n: u32) -> u16 {
    let round: u64 = 1u64 << (n - 1);
    let v = v as u64 + round;
    (((v >> n) + v) >> n).min(u16::MAX as u64) as u16
}

fn main() {
    let mut reproduced = false;

    // Boundary-value construction: full-range u16 pixel/alpha data under every small bit_depth
    // that takes the generic (non 10/12/16) path in premultiply_rgba16.
    for bit_depth in [1u32, 2, 3, 4, 5, 6, 7, 8, 9, 11, 13, 14, 15] {
        let mut data: Vec<u16> = vec![65535u16; 4];
        let expected = [
            oracle_div_by_2pn_m1(65535u32 * 65535u32, bit_depth),
            oracle_div_by_2pn_m1(65535u32 * 65535u32, bit_depth),
            oracle_div_by_2pn_m1(65535u32 * 65535u32, bit_depth),
            oracle_div_by_2pn_m1(((1u32 << bit_depth) - 1) * 65535u32, bit_depth),
        ];

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            pic_scale_safe::premultiply_rgba16(&mut data, bit_depth);
        }));

        match result {
            Err(_) => {
                println!("bit_depth={bit_depth}: PANICKED (arithmetic overflow)");
                reproduced = true;
            }
            Ok(()) => {
                if data != expected {
                    println!(
                        "bit_depth={bit_depth}: wrong result, got {data:?} expected {expected:?} (silent wraparound)"
                    );
                    reproduced = true;
                } else {
                    println!("bit_depth={bit_depth}: ok, got {data:?}");
                }
            }
        }
    }

    // Sanity check: a well-formed, bit_depth-conforming input must be unaffected by the fix.
    let mut conforming: Vec<u16> = vec![1000, 2000, 3000, 40000];
    pic_scale_safe::premultiply_rgba16(&mut conforming, 16);
    let expected_conforming = [610u16, 1221, 1831, 40000];
    if conforming != expected_conforming {
        println!(
            "conforming bit_depth=16 input changed: got {conforming:?} expected {expected_conforming:?}"
        );
        reproduced = true;
    } else {
        println!("conforming bit_depth=16 input unaffected: {conforming:?}");
    }

    if reproduced {
        eprintln!("REPRODUCED - div_by_2pn_m1 overflows/produces a wrong value on out-of-range 16-bit input");
        std::process::exit(1);
    } else {
        println!("did not reproduce - div_by_2pn_m1 stayed overflow-free and bounded on all boundary inputs");
    }
}
