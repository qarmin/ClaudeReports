use png::Adam7Info;

fn main() {
    png::splat_interlaced_row(&mut vec![80u8], 1usize, &vec![], &Adam7Info::new(1u8, 0u32, 1u32), 10u8);
}
