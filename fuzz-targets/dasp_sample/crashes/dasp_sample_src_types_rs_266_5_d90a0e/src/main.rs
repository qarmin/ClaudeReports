use dasp_sample::U48;
use dasp_sample::types::U11;

fn main() {
    let i16 = dasp_sample::conv::u48::to_i16(U48::new(0i64).unwrap());
    let _ = <dasp_sample::types::U11 as ::core::ops::Neg>::neg(U11::new_unchecked(i16));
}
