use femtovg::Atlas;

fn main() {
    let mut atlas = Atlas::new(10000000000000000000usize, 0usize);
    let _ = atlas.add_rect(1000000000000000000usize, 0usize);
}
