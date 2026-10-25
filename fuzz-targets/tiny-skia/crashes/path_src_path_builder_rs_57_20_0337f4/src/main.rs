use tiny_skia_path::PathBuilder;

fn main() {
    let _ = PathBuilder::with_capacity(usize::MAX, 0usize);
}
