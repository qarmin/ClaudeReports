use wgpu_types::PresentMode;

fn main() {
    let _ = wgpu_hal::vulkan::conv::map_present_mode(PresentMode::AutoVsync);
}
