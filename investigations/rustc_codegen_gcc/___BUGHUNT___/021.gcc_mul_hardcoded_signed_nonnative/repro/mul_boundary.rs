// Standalone repro program for candidate 021 (gcc_mul_hardcoded_signed_nonnative).
//
// Compiled directly with `y.sh rustc` (this project's own driver script), both against the
// backend as-is (native i128/u128 GCC types, `supports_128bit_integers = true` on this x86_64
// host) and again after forcing `CodegenCx::new`'s `supports_128bit_integers` parameter to
// `false` (see force_non_native.patch), which makes `is_native_int_type` treat i128/u128 as
// non-native and routes multiplication through the array-based `dyncast_array` branch in
// `multiplicative_operation` (src/int.rs) that `gcc_mul` reaches via a hardcoded `signed = true`,
// always calling `__multi3` regardless of whether the Rust-level operation is `i128 * i128` or
// `u128 * u128`.
//
// Boundary values chosen to catch a sign-selection bug in truncated 128-bit multiplication:
// i128::MIN (MSB set, most negative), values whose top bit is set when reinterpreted unsigned
// (which would misbehave if a signed multiply's overflow/extension logic differed from an
// unsigned one), u128::MAX and near-MAX, and mixed magnitude products that wrap around the
// 128-bit boundary.
fn main() {
    let a: i128 = i128::MIN;
    let b: i128 = 2;
    println!("i128::MIN.wrapping_mul(2) = {}", std::hint::black_box(a).wrapping_mul(std::hint::black_box(b)));

    let a2: i128 = i128::MIN;
    let b2: i128 = -1;
    println!("i128::MIN.wrapping_mul(-1) = {}", std::hint::black_box(a2).wrapping_mul(std::hint::black_box(b2)));

    let a3: i128 = -7;
    let b3: i128 = 6;
    println!("(-7i128).wrapping_mul(6) = {}", std::hint::black_box(a3).wrapping_mul(std::hint::black_box(b3)));

    let a4: u128 = u128::MAX;
    let b4: u128 = 3;
    println!("u128::MAX.wrapping_mul(3) = {}", std::hint::black_box(a4).wrapping_mul(std::hint::black_box(b4)));

    let a5: u128 = u128::MAX;
    let b5: u128 = u128::MAX;
    println!("u128::MAX.wrapping_mul(u128::MAX) = {}", std::hint::black_box(a5).wrapping_mul(std::hint::black_box(b5)));

    let a6: u128 = 1u128 << 127;
    let b6: u128 = 2;
    println!("(1u128<<127).wrapping_mul(2) = {}", std::hint::black_box(a6).wrapping_mul(std::hint::black_box(b6)));

    let a7: u128 = (1u128 << 127) + 5;
    let b7: u128 = (1u128 << 96) + 9;
    println!("((1<<127)+5).wrapping_mul((1<<96)+9) = {}", std::hint::black_box(a7).wrapping_mul(std::hint::black_box(b7)));

    let a8: i128 = i128::MAX;
    let b8: i128 = i128::MAX;
    println!("i128::MAX.wrapping_mul(i128::MAX) = {}", std::hint::black_box(a8).wrapping_mul(std::hint::black_box(b8)));

    let a9: i128 = i128::MIN;
    let b9: i128 = i128::MIN;
    println!("i128::MIN.wrapping_mul(i128::MIN) = {}", std::hint::black_box(a9).wrapping_mul(std::hint::black_box(b9)));

    let a10: u128 = 340282366920938463463374607431768211455u128; // u128::MAX
    let b10: u128 = 170141183460469231731687303715884105728u128; // 1 << 127
    println!("u128::MAX.wrapping_mul(1<<127) = {}", std::hint::black_box(a10).wrapping_mul(std::hint::black_box(b10)));
}
