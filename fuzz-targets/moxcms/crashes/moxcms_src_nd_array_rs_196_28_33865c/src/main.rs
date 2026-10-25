use moxcms::Hypercube;

fn main() {
    let vec = vec![];
    let hypercube = Hypercube::new(&vec, 1usize, 0usize).unwrap();
    let _ = hypercube.prism_vec3(0.0f32, 0.0f32, 0.0f32, 0.0f32);
}
