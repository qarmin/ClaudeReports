fn main() {
    let tendril = <tendril::Tendril<tendril::fmt::UTF8, tendril::NonAtomic>>::with_capacity(1573871419u32);
    let _ = tendril.as_superset::<tendril::fmt::WTF8>();
}
