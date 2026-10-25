use dasp_sample::types::I11;
use dasp_sample::types::i11::MAX;

fn main() {
    let _ = <dasp_sample::types::I11 as ::core::ops::Shl<dasp_sample::types::I11>>::shl(MAX, I11::from(69u8));
}
