fn main() {
    let mut tendril = <tendril::Tendril<tendril::fmt::Bytes, tendril::NonAtomic> as ::core::convert::From<tendril::SendTendril<tendril::fmt::Bytes>>>::from(<tendril::Tendril<tendril::fmt::Bytes, tendril::NonAtomic>>::new().into_send());
    tendril.reserve(1620049837u32);
}
