fn main() {
    let tendril = <tendril::Tendril<tendril::fmt::Bytes, tendril::NonAtomic>>::with_capacity(0u32);
    let tendril2 = tendril.try_reinterpret::<tendril::fmt::UTF8>().unwrap();
    let mut tendril3 = tendril2.into_superset::<tendril::fmt::WTF8>();
    tendril3.reserve(2071774921u32);
}
