use png::Adam7Info;

fn main() {
    png::expand_interlaced_row(&mut vec![], usize::MAX, &vec![], &Adam7Info::new(1u8, 4294967294u32, 2862318691u32), 232u8);
}
