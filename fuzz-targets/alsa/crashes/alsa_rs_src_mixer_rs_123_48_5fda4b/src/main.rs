use alsa::mixer::MilliBel;

fn main() {
    let _ = <alsa::mixer::MilliBel as ::core::ops::Sub<alsa::mixer::MilliBel>>::sub(MilliBel(i64::MIN), MilliBel::from_db(1.0f32));
}
