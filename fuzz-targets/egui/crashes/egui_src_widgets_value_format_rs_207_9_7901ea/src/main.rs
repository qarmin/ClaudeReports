use egui::DragValue;

fn main() {
    let drag_value = DragValue::from_get_set(|_| 0 as f64);
    let drag_value2 = drag_value.max_decimals(0usize);
    let _ = drag_value2.hexadecimal(0usize, false, false);
}
