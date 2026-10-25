fn main() {
    let _ = dasp_sample::conv::i24::to_u32(dasp_sample::conv::f64::to_i24(65536.0f64));
}
