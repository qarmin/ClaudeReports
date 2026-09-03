#![feature(movrs_target_feature)]
use std::arch::x86_64::_m_prefetchrs;

#[target_feature(enable = "movrs")]
unsafe fn pf(p: *const u8) {
    _m_prefetchrs(p);
}

fn main() {
    let buf = [3u8; 128];
    unsafe { pf(buf.as_ptr()); }
    println!("prefetchrs ok {}", buf[0]);
}
