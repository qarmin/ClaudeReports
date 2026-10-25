use png::Adam7Info;

fn main() {
    png::splat_interlaced_row(&mut include_bytes!("data_0.bin").to_vec(), 1usize, &vec![0u8], &Adam7Info::new(1u8, 16u32, 1000000000u32), 1u8);
}
