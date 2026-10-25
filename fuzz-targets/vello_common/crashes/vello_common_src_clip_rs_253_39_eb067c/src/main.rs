use fearless_simd::Level;
use vello_common::clip::ClipState;
use vello_common::strip_generator::StripGenerator;

fn main() {
    let mut clip_state = ClipState::default();
    let mut strip_generator = StripGenerator::new(0u16, 0u16, Level::try_detect().unwrap());
    clip_state.pop_root_viewport(&mut strip_generator);
}
