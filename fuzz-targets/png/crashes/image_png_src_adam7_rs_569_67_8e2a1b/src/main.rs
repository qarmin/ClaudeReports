use png::Adam7Info;

fn main() {
    png::splat_interlaced_row(&mut vec![0u8], 1usize, &vec![0u8, 80u8, 78u8, 71u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 73u8, 72u8, 68u8, 82u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8], &Adam7Info::new(1u8, 0u32, 1u32), 224u8);
}
