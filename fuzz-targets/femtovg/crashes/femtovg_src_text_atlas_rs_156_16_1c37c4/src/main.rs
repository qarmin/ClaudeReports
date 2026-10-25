use femtovg::Atlas;

fn main() {
    let mut atlas = Atlas::new(10000usize, usize::MAX);
    let _ = atlas.add_rect(2usize, 2usize);
    let _ = atlas.add_rect(1usize, usize::MAX);
}
