use femtovg::Atlas;

fn main() {
    let mut atlas = Atlas::new(1usize, 0usize);
    atlas.expand(usize::MAX, 255usize);
    let _ = atlas.add_rect(usize::MAX, usize::MAX);
}
