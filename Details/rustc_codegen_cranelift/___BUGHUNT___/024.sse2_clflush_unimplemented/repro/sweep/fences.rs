use std::arch::x86_64::{_mm_lfence, _mm_mfence, _mm_sfence};
fn main() {
    unsafe { _mm_sfence(); _mm_lfence(); _mm_mfence(); }
    println!("fences ok");
}
