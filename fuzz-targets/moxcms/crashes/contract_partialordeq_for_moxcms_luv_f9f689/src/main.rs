use moxcms::Luv;
use moxcms::Matrix4f;

fn main() {
    let matrix4f = Matrix4f { v: [[0.0f32, 0.0f32, 0.0f32, 0.0f32], [f32::MAX, 0.0f32, 0.0f32, 1.0f32], [0.0f32, 1.0f32, 0.0f32, 57.0f32], [0.0f32, 1.0f32, 1.0f32, 1.0f32]] };
    let f32_v = matrix4f.determinant().unwrap();
    let luv = Luv { l: 0.0f32, u: f32_v, v: 0.0f32 };
    assert_eq!(luv == luv, luv.partial_cmp(&luv) == Some(std::cmp::Ordering::Equal), "PartialOrd inconsistent with PartialEq");
}
