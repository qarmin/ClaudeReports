fn main() {
    let tendril = <tendril::Tendril<tendril::fmt::ASCII, tendril::NonAtomic>>::with_capacity(1869478941u32);
    let _ = tendril.try_reinterpret_view::<tendril::fmt::WTF8>();
}
