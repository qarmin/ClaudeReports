use lopdf::filters::png::FilterType;

fn main() {
    lopdf::filters::png::encode_row(FilterType::Paeth, usize::MAX, &include_bytes!("image_0.png").to_vec(), &mut include_bytes!("image_1.png").to_vec());
}
