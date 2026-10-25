use moxcms::Cube;

fn main() {
    let vec = vec![];
    let cube = Cube::new_cube(&vec, [0u8, 0u8, 0u8], 0usize).unwrap();
    let _ = cube.trilinear_vec4(0.0f32, 0.0f32, 1.0f32);
}
