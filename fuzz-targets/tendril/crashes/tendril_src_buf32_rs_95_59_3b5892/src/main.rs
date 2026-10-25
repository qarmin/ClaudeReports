fn main() {
    let _ = <tendril::Tendril<tendril::fmt::Bytes, tendril::NonAtomic>>::with_capacity(u32::MAX);
}
