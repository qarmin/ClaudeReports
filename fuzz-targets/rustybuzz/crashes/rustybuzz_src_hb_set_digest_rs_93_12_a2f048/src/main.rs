use rustybuzz::Face;

fn main() {
    let vec = include_bytes!("data_0.bin").to_vec();
    let _ = Face::from_slice(&vec, 0u32);
}
