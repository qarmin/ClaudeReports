use egui::Context;
use egui::RawInput;

fn main() {
    let context = Context::default();
    let _ = context.run_ui(RawInput::default(), |_| ());
}
