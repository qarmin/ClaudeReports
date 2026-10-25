use zeno::Format;

fn main() {
    let format = Format::subpixel_bgra();
    let _ = format.buffer_size(1000000000u32, 1000000000u32);
}
