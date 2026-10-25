use moxcms::Hypercube;

fn main() {
    let vec = vec![1.0f32];
    let _ = Hypercube::new(&vec, 1000000000usize, 0usize);
}
