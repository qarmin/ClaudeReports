fn main() {
    let tendril = <tendril::Tendril<tendril::fmt::Bytes, tendril::NonAtomic>>::with_capacity(1664946732u32);
    let _ = tendril.try_reinterpret_view::<tendril::fmt::Bytes>();
}
