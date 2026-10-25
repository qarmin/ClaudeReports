use femtovg::Atlas;

fn main() {
    let mut atlas = Atlas::new(1usize, 1000000000000000000usize);
    let _ = atlas.add_rect(1usize, 1usize);
    let _ = atlas.add_rect(0usize, usize::MAX);
}
