use egui::Context;
use egui::Id;
use egui::LayerId;
use egui::Order;
use egui::layers::PaintList;
use egui::layers::ShapeIdx;
use emath::TSTransform;
use epaint::Margin;
use epaint::MarginF32;
use epaint::PathStroke;

fn main() {
    let context = Context::default();
    let f32_v = emath::easing::quadratic_out(0.0f32);
    let mut paint_list = PaintList::default();
    paint_list.transform_after_rounding_range(ShapeIdx(1usize), context.layer_painter(LayerId::new(Order::Background, Id::new(0u32))).line(vec![], PathStroke::NONE), TSTransform { scaling: f32_v, translation: MarginF32::from(Margin::symmetric(0i8, 0i8)).right_bottom() });
}
