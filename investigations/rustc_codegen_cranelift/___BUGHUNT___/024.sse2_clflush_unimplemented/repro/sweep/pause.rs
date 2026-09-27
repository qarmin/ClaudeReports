use std::arch::x86_64::_mm_pause;
fn main() { unsafe { _mm_pause(); } println!("pause ok"); }
