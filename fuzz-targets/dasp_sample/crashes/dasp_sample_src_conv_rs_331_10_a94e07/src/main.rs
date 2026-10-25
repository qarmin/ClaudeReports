fn main() {
    let _ = dasp_sample::conv::i48::to_u64(dasp_sample::conv::f32::to_i48(16777216.0f32));
}
