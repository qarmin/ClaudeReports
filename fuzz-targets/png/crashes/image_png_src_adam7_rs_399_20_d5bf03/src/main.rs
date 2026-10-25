use png::Adam7Info;

fn main() {
    png::expand_interlaced_row(&mut vec![], 0usize, &vec![0u8], &Adam7Info::new(4u8, 0u32, 10000u32), 16u8);
}
