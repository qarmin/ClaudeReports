// Standalone repro program for candidate 003 (i128_div_rem_non_native_sign_check).
//
// Compiled directly with `y.sh rustc` (this project's own driver script), both against the
// backend as-is (native i128/u128 GCC types, `supports_128bit_integers = true` on this x86_64
// host) and again after forcing `CodegenCx::new`'s `supports_128bit_integers` parameter to
// `false` (see force_non_native.patch), which makes `is_native_int_type` treat i128/u128 as
// non-native and routes every operation below through the array-based `dyncast_array` branches
// in src/int.rs (division_operation/multiplicative_operation calling __divti3/__udivti3/
// __modti3/__umodti3 by name).
//
// Boundary values chosen to catch a sign-selection bug specifically: i128::MIN (only representable
// negative value whose absolute value overflows), divisor -1 (would overflow if actually computed,
// so only exercised via wrapping_div), mixed-sign operands, and values with the MSB set for the
// u128 case (which would misbehave under a signed division/remainder).
fn main() {
    let a: i128 = i128::MIN;
    let b: i128 = -1;
    println!("i128::MIN.wrapping_div(-1) = {}", std::hint::black_box(a).wrapping_div(std::hint::black_box(b)));

    let a2: i128 = i128::MIN;
    let b2: i128 = 3;
    println!("i128::MIN / 3 = {}", std::hint::black_box(a2) / std::hint::black_box(b2));
    println!("i128::MIN % 3 = {}", std::hint::black_box(a2) % std::hint::black_box(b2));

    let a3: i128 = -7;
    let b3: i128 = 2;
    println!("-7i128 / 2 = {}", std::hint::black_box(a3) / std::hint::black_box(b3));
    println!("-7i128 % 2 = {}", std::hint::black_box(a3) % std::hint::black_box(b3));

    let a4: u128 = u128::MAX;
    let b4: u128 = 3;
    println!("u128::MAX / 3 = {}", std::hint::black_box(a4) / std::hint::black_box(b4));
    println!("u128::MAX % 3 = {}", std::hint::black_box(a4) % std::hint::black_box(b4));

    let a5: u128 = u128::MAX - 1;
    let b5: u128 = 2;
    println!("(u128::MAX - 1) / 2 = {}", std::hint::black_box(a5) / std::hint::black_box(b5));
    println!("(u128::MAX - 1) % 2 = {}", std::hint::black_box(a5) % std::hint::black_box(b5));

    let a6: i128 = 100;
    let b6: i128 = -7;
    println!("100i128 / -7 = {}", std::hint::black_box(a6) / std::hint::black_box(b6));
    println!("100i128 % -7 = {}", std::hint::black_box(a6) % std::hint::black_box(b6));
}
