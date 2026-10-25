fn main() {
    let _ = <tendril::Tendril<tendril::fmt::UTF8, tendril::NonAtomic> as ::core::convert::From<tendril::SendTendril<tendril::fmt::UTF8>>>::from(<tendril::Tendril<tendril::fmt::UTF8, tendril::NonAtomic>>::with_capacity(1776097792u32).into_send());
}
