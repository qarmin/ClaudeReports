use egui::DragValue;

fn main() {
    let drag_value = DragValue::from_get_set(|_| 0 as f64);
    let drag_value2 = drag_value.suffix(());
    let _ = drag_value2.octal(0usize, false);
}
