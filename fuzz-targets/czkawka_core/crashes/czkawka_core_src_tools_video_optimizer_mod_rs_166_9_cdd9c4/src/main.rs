use czkawka_core::tools::similar_videos::DEFAULT_THUMBNAIL_GRID_TILES_PER_SIDE;
use czkawka_core::tools::video_optimizer::VideoCropParams;
use czkawka_core::tools::video_optimizer::VideoCroppingMechanism;

fn main() {
    let u8_v = DEFAULT_THUMBNAIL_GRID_TILES_PER_SIDE;
    let _ = VideoCropParams::with_custom_params(VideoCroppingMechanism::BlackBars, 255u8, 0u8, 0usize, 0u32, false, 0u8, false, u8_v);
}
