use zeno::Format;

fn main() {
    let format = Format::CustomSubpixel([0.0f32, 0.0f32, 0.0f32]);
    let _ = format.buffer_size(1u32, 1169106533u32);
}
