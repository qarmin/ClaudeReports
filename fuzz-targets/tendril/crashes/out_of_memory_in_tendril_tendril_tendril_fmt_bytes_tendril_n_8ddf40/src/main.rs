fn main() {
    let _ = <tendril::Tendril<tendril::fmt::Bytes, tendril::NonAtomic> as ::core::convert::From<tendril::SendTendril<tendril::fmt::Bytes>>>::from(<tendril::Tendril<tendril::fmt::ASCII, tendril::NonAtomic>>::with_capacity(1537703420u32).into_bytes().into_send());
}
