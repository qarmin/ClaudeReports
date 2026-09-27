use std::arch::x86_64::_mm_clflush;

fn main() {
    let mut buf = [0u8; 256];
    for (i, b) in buf.iter_mut().enumerate() {
        *b = i as u8;
    }
    unsafe {
        _mm_clflush(buf.as_ptr());
        _mm_clflush(buf.as_ptr().add(64));
    }
    let s: u32 = buf.iter().map(|&b| b as u32).sum();
    println!("clflush ok sum={s}");
}
