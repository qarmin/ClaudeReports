use v_frame::plane::PlaneGeometry;

fn main() {
    let plane_geometry = PlaneGeometry::new(1usize, 1usize, 0usize, 9499363980073713412usize, 10usize, 0usize, 1u8, 1u8).unwrap();
    let _ = plane_geometry.data_origin();
}
