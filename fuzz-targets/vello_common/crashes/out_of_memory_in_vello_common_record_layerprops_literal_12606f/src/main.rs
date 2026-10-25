use vello_common::mask::Mask;
use vello_common::pixmap::Pixmap;
use vello_common::record::LayerProps;
use vello_cpu::RenderContext;

fn main() {
    let _ = LayerProps { blend_mode: RenderContext::new(0u16, 0u16).blend_mode(), opacity: 0.0f32, mask: Some(Mask::new_alpha(&Pixmap::new(u16::MAX, 10000u16))), clip_path: None };
}
