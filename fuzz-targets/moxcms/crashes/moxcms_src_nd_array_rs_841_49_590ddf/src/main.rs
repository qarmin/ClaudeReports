use moxcms::Cube;

fn main() {
    let vec = vec![0.0f32];
    let cube = Cube::new(&vec, 65536usize, 0usize).unwrap();
    let _ = cube.prism_vec3(0.0f32, 0.0f32, 0.0f32);
}
