use dasp_sample::U24;

fn main() {
    let _ = <dasp_sample::U24 as ::core::ops::Rem<dasp_sample::U24>>::rem(U24::new(0i32).unwrap(), U24::from(0i32));
}
