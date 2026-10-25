use dasp_sample::types::U20;

fn main() {
    let _ = <dasp_sample::types::U20 as ::core::ops::Sub<dasp_sample::types::U20>>::sub(U20::from(0u16), U20::new_unchecked(1i32));
}
