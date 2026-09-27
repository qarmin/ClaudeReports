// Built with `-C target-feature=+avx2,+fma,+bmi2,+popcnt`, which is a documented rustc flag.
//
// `cfg(target_feature = "...")` is documented to reflect it, and real crates use exactly this
// pattern to pick between a SIMD and a scalar implementation (e.g. `rand`'s
// `src/distributions/utils.rs`, which is part of cg_clif's own extended-sysroot test set, selects
// `_mm256_mulhi_epu16`-based `WideningMultiply` impls under `#[cfg(target_feature = "avx2")]`).
//
// cg_clif's `target_config` hardcodes the enabled feature set to fxsr/sse/sse2/x87 and ignores
// `-C target-feature` entirely, so the generated program takes a different branch than the one
// LLVM builds from the same source and the same flags.
fn kernel_name() -> &'static str {
    if cfg!(target_feature = "avx512f") {
        "avx512"
    } else if cfg!(target_feature = "avx2") {
        "avx2"
    } else if cfg!(target_feature = "sse4.2") {
        "sse42"
    } else {
        "baseline"
    }
}

// A rounding difference that is observable in the program's output, standing in for the real
// crates' "same value, different instructions" case: with `fma` enabled a crate would fuse this
// into a single rounding step, without it there are two.
fn fused() -> bool {
    cfg!(target_feature = "fma")
}

fn main() {
    println!("kernel={}", kernel_name());
    println!("fused_multiply_add={}", fused());
    println!(
        "avx={} avx2={} fma={} bmi2={} popcnt={}",
        cfg!(target_feature = "avx"),
        cfg!(target_feature = "avx2"),
        cfg!(target_feature = "fma"),
        cfg!(target_feature = "bmi2"),
        cfg!(target_feature = "popcnt"),
    );
}
