use dasp_sample::types::i24::MAX;

fn main() {
    let _ = <dasp_sample::U48 as ::core::ops::Add<dasp_sample::U48>>::add(dasp_sample::conv::i24::to_u48(MAX), dasp_sample::conv::u64::to_u48(1000000000000000000u64));
}
