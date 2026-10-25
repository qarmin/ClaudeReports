fn main() {
    let tendril = <tendril::Tendril<tendril::fmt::UTF8, tendril::NonAtomic>>::with_capacity(1160800565u32);
    let _ = tendril.try_reinterpret_view::<tendril::fmt::UTF8>();
}
