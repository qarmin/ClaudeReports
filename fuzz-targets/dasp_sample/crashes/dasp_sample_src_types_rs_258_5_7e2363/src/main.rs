fn main() {
    let i48 = dasp_sample::conv::f64::to_i48(1.0f64);
    let _ = <dasp_sample::I48 as ::core::ops::Mul<dasp_sample::I48>>::mul(dasp_sample::conv::u64::to_i48(1u64), i48);
}
