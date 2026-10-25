use png::Adam7Info;

fn main() {
    png::expand_interlaced_row(&mut vec![], 0usize, &vec![], &Adam7Info::new(1u8, 0u32, 1u32), 100u8);
}
