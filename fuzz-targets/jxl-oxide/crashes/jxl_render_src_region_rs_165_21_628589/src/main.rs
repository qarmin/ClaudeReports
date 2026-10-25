use jxl_render::Region;

fn main() {
    let region = Region { left: 0i32, top: 0i32, width: 0u32, height: 0u32 };
    let _ = region.downsample_separate(1000000000u32, 0u32);
}
