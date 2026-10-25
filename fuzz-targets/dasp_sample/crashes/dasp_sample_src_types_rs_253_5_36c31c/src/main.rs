fn main() {
    let _ = <dasp_sample::I24 as ::core::ops::Neg>::neg(dasp_sample::conv::f32::to_i24(-1e30f32));
}
