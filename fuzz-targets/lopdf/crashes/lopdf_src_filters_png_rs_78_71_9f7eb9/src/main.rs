use lopdf::filters::png::FilterType;

fn main() {
    lopdf::filters::png::decode_row(FilterType::Paeth, 1usize, &vec![], &mut vec![0u8]);
}
