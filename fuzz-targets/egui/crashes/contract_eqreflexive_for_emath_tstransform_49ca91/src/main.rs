use emath::TSTransform;
use epaint::WHITE_UV;

fn main() {
    let t_s_transform = TSTransform { scaling: f32::NAN, translation: WHITE_UV.to_vec2() };
    assert!(t_s_transform == t_s_transform, "Eq is not reflexive");
}
