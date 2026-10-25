use wgpu_types::SurfaceColorSpace;

fn main() {
    let _ = wgpu_hal::vulkan::conv::map_surface_color_space(SurfaceColorSpace::Auto);
}
