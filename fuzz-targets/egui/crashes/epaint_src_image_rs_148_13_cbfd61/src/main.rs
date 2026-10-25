use epaint::ColorImage;

fn main() {
    let _ = ColorImage::from_gray([1000000000usize, usize::MAX], &vec![]).with_source_size(emath::vec2(f32::INFINITY, f32::NEG_INFINITY));
}
