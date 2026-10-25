use femtovg::Atlas;

fn main() {
    let mut atlas = Atlas::new(1000000000000000000usize, 0usize);
    atlas.expand(10000000000000000000usize, 1usize);
    let _ = atlas.add_rect(10000000000000000000usize, 1usize);
}
