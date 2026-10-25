use dasp_sample::I24;

fn main() {
    let _ = <dasp_sample::I24 as ::core::ops::Rem<dasp_sample::I24>>::rem(dasp_sample::conv::u24::to_i24(dasp_sample::conv::i64::to_u24(0i64)), I24::from(0u16));
}
