fn main() {
    let tendril = <tendril::Tendril<tendril::fmt::WTF8, tendril::NonAtomic>>::with_capacity(1957461737u32);
    let _ = tendril.try_reinterpret_view::<tendril::fmt::ASCII>();
}
