use fearless_simd::Level;
use vello_common::viewport::ViewportState;

fn main() {
    let mut viewport_state = ViewportState::new(0u16, 0u16, Level::baseline());
    viewport_state.pop_clip();
}
