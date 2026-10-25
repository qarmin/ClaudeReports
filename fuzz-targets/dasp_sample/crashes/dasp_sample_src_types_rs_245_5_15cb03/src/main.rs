use dasp_sample::types::I20;
use dasp_sample::types::i20::MIN;

fn main() {
    let i20 = I20::from(0u16);
    let _ = <dasp_sample::types::I20 as ::core::ops::Shr<dasp_sample::types::I20>>::shr(i20, MIN);
}
