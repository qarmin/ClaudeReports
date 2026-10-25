use czkawka_core::tools::video_optimizer::VideoCropParams;
use czkawka_core::tools::video_optimizer::VideoCroppingMechanism;

fn main() {
    let _ = VideoCropParams::with_custom_params(VideoCroppingMechanism::BlackBars, 0u8, 0u8, 0usize, 0u32, false, 0u8, false, 0u8);
}
