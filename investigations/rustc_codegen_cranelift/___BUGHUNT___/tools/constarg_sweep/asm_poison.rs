use std::arch::asm;
use std::arch::x86_64::*;

#[target_feature(enable = "sse4.2")]
unsafe fn go(a: __m128i, b: __m128i, x: u64) -> (i32, u64) {
    let mut y = x;
    unsafe { asm!("add {0}, 1", inout(reg) y) };
    (_mm_cmpestri::<0>(a, 3, b, 3), y)
}

fn main() {
    unsafe {
        let a = _mm_set1_epi8(1);
        let (i, y) = go(a, a, 41);
        println!("{i} {y}");
    }
}
