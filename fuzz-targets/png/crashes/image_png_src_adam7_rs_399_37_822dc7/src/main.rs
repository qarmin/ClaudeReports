use png::Adam7Info;

fn main() {
    png::expand_interlaced_row(&mut vec![0u8, 80u8], 0usize, &vec![68u8], &Adam7Info::new(1u8, 0u32, 1u32), 16u8);
}
