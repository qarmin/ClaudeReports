use v_frame::plane::PlaneGeometry;

fn main() {
    let plane_geometry = PlaneGeometry::new(1usize, 1usize, 0usize, 0usize, usize::MAX, 0usize, 1u8, 1u8).unwrap();
    let _ = plane_geometry.alloc_height();
}
