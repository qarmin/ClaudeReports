use dasp_sample::types::I11;

fn main() {
    let _ = <dasp_sample::types::I11 as ::core::ops::Neg>::neg(I11::new_unchecked(i16::MIN));
}
