use emath::Rangef;

fn main() {
    let rangef = <emath::Rangef as ::core::ops::Mul<f32>>::mul(Rangef::new(0.0f32, 1.0f32), -0.00024414063f32);
    let _ = rangef.clamp(0.0f32);
}
