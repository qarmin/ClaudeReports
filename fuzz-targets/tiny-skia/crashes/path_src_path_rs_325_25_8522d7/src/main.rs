use tiny_skia_path::PathBuilder;

fn main() {
    let path = PathBuilder::from_circle(1.0f32, 1.0f32, 0.0f32).unwrap();
    let path_segments_iter = path.segments();
    let _ = path_segments_iter.curr_verb();
}
