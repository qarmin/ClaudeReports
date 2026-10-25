use png::Adam7Info;

fn main() {
    png::splat_interlaced_row(&mut vec![0u8], 1usize, &vec![0u8, 80u8, 78u8, 71u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8], &Adam7Info::new(2u8, 0u32, 10u32), 88u8);
}
