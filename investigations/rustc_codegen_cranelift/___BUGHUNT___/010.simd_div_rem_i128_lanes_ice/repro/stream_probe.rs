use std::arch::x86_64::*;

fn main() {
    let mut buf = [0.0f64; 8];
    unsafe {
        let v = _mm256_set_pd(1.0, 2.0, 3.0, 4.0);
        _mm256_stream_pd(buf.as_mut_ptr(), v);
    }
    println!("{:?}", buf);
}
