fn main() {
    let tendril = <tendril::Tendril<tendril::fmt::WTF8, tendril::NonAtomic>>::with_capacity(1170330663u32);
    let _ = tendril.try_reinterpret_view::<tendril::fmt::Bytes>();
}
