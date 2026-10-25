use egui::layers::PaintList;
use egui::layers::ShapeIdx;
use emath::TSTransform;
use emath::Vec2;

fn main() {
    let mut paint_list = PaintList::default();
    paint_list.transform_range(PaintList::default().next_idx(), ShapeIdx(1usize), TSTransform::new(Vec2::NAN, 0.0f32));
}
