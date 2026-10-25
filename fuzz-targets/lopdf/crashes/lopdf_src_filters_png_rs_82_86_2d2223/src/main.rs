use lopdf::filters::png::FilterType;

fn main() {
    lopdf::filters::png::decode_row(FilterType::Paeth, 1usize, &vec![0u8], &mut vec![0u8, 80u8]);
}
