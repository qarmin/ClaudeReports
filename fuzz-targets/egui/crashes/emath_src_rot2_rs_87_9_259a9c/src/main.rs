use egui::Context;
use emath::Rot2;

fn main() {
    let context = Context::default();
    let painter = context.debug_painter();
    let f32_v = painter.opacity();
    let rot2 = <emath::Rot2 as ::core::ops::Mul<f32>>::mul(<emath::Rot2 as ::core::ops::Div<f32>>::div(Rot2::IDENTITY, 0.0f32), f32_v);
    let rot22 = rot2.inverse();
    let rot23 = rot22.inverse();
    let _ = rot23.normalized();
}
