use egui::theme::Themes;

fn main() {
    let themes = Themes::default();
    let _ = themes.get::<egui::widget_style::SeparatorStyle>();
}
