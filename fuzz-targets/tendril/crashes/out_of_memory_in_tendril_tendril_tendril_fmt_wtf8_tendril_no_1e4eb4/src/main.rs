fn main() {
    let tendril = <tendril::Tendril<tendril::fmt::WTF8, tendril::NonAtomic>>::with_capacity(1961106048u32);
    let _ = tendril.try_as_subset::<tendril::fmt::UTF8>();
}
