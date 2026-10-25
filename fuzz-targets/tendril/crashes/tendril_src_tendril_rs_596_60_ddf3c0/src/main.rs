fn main() {
    let tendril = <tendril::Tendril<tendril::fmt::UTF8, tendril::NonAtomic>>::from_char('a');
    let mut tendril2 = tendril.try_reinterpret::<tendril::fmt::WTF8>().unwrap();
    tendril2.reserve(u32::MAX);
}
