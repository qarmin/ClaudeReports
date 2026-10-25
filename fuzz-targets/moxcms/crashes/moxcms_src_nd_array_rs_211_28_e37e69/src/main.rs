use moxcms::Hypercube;
use moxcms::LutStore;

fn main() {
    let lut_store = LutStore::Store16(vec![]);
    let vec = lut_store.to_clut_f32();
    let hypercube = Hypercube::new(&vec, 0usize, 0usize).unwrap();
    let _ = hypercube.prism_vec4(0.0f32, 0.0f32, 0.0f32, 0.0f32);
}
