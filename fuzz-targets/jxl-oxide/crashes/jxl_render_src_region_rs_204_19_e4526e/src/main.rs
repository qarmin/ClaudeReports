use jxl_render::Region;

fn main() {
    let region = Region::with_size(0u32, 1u32);
    let region2 = region.translate(0i32, 0i32);
    let _ = region2.upsample(1000000000u32);
}
