use moxcms::Cube;

fn main() {
    let vec = vec![];
    let cube = Cube::new(&vec, 0usize, 0usize).unwrap();
    let _ = cube.trilinear_vec3(0.0f32, 0.0f32, 0.0f32);
}
