fn main() {
    let _ = <tendril::Tendril<tendril::fmt::WTF8, tendril::NonAtomic> as ::core::convert::From<tendril::SendTendril<tendril::fmt::WTF8>>>::from(<tendril::Tendril<tendril::fmt::UTF8, tendril::NonAtomic>>::with_capacity(2006686492u32).into_superset::<tendril::fmt::WTF8>().into_send());
}
