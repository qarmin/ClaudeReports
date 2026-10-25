fn main() {
    let tendril = <tendril::Tendril<tendril::fmt::WTF8, tendril::NonAtomic>>::with_capacity(1254267382u32);
    let _ = tendril.as_bytes();
}
