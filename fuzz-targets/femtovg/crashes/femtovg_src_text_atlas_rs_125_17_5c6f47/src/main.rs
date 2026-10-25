use femtovg::Atlas;

fn main() {
    let mut atlas = Atlas::new(0usize, 0usize);
    atlas.expand(usize::MAX, 10000000000000000000usize);
    let _ = atlas.add_rect(17390261748156048065usize, 65535usize);
}
