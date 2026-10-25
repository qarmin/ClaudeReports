use png::Adam7Info;

fn main() {
    png::splat_interlaced_row(&mut vec![0u8, 80u8, 78u8, 71u8, 0u8, 0u8, 0u8, 0u8, 0u8], 1usize, &vec![0u8], &Adam7Info::new(1u8, 1u32, 1000000000u32), 8u8);
}
