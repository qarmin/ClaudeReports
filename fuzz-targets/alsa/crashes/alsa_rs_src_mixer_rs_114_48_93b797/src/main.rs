use alsa::mixer::MilliBel;

fn main() {
    let _ = <alsa::mixer::MilliBel as ::core::ops::Add<alsa::mixer::MilliBel>>::add(MilliBel(i64::MAX), MilliBel(1i64));
}
