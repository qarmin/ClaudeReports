use egui::Context;

fn main() {
    let context = Context::default();
    let _ = context.tessellate(vec![], 1.0f32);
}
