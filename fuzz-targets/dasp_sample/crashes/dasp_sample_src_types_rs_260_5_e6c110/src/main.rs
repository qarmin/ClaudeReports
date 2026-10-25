fn main() {
    let _ = <dasp_sample::I48 as ::core::ops::Neg>::neg(<dasp_sample::I48 as ::core::ops::Not>::not(dasp_sample::conv::f64::to_i48(1e300f64)));
}
