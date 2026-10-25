fn main() {
    let tendril = <tendril::Tendril<tendril::fmt::Bytes, tendril::NonAtomic>>::with_capacity(2004863132u32).try_reinterpret::<tendril::fmt::UTF8>().unwrap();
    let _ = tendril.try_reinterpret_view::<tendril::fmt::ASCII>();
}
