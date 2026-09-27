use std::arch::x86_64::*;

fn main() {
    let buf = [7u8; 256];
    unsafe {
        _mm_prefetch::<_MM_HINT_T0>(buf.as_ptr() as *const i8);
        _mm_prefetch::<_MM_HINT_T1>(buf.as_ptr() as *const i8);
        _mm_prefetch::<_MM_HINT_T2>(buf.as_ptr() as *const i8);
        _mm_prefetch::<_MM_HINT_NTA>(buf.as_ptr() as *const i8);
        _mm_prefetch::<_MM_HINT_ET0>(buf.as_ptr() as *const i8);
        _mm_prefetch::<_MM_HINT_ET1>(buf.as_ptr() as *const i8);
    }
    println!("prefetch ok {}", buf[0]);
}
