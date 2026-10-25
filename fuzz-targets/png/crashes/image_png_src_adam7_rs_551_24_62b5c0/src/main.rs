use png::Adam7Info;

fn main() {
    png::splat_interlaced_row(&mut include_bytes!("data_0.bin").to_vec(), 1usize, &vec![0u8, 80u8, 78u8, 71u8, 0u8, 0u8, 0u8], &Adam7Info::new(2u8, 19u32, 10000u32), 8u8);
}
