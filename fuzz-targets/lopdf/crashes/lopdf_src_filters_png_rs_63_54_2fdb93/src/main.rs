use lopdf::filters::png::FilterType;

fn main() {
    lopdf::filters::png::decode_row(FilterType::Up, 0usize, &vec![], &mut vec![0u8]);
}
