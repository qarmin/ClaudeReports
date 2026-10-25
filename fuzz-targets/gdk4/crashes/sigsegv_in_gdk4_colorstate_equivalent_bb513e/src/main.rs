use gdk4::ColorState;

fn main() {
    let color_state = ColorState::oklab();
    let _ = color_state.equivalent(&ColorState::oklch());
}
