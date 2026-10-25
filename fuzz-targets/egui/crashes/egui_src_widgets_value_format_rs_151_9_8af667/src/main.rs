use egui::DragValue;

fn main() {
    let drag_value = DragValue::from_get_set(|_| 1 as f64);
    let _ = drag_value.binary(0usize, false);
}
