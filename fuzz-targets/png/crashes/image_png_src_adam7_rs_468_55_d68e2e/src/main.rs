use png::Adam7Info;

fn main() {
    let adam7_info = Adam7Info::new(1u8, 0u32, 1000000000u32);
    png::splat_interlaced_row(&mut vec![0u8], 1usize, &vec![0u8], &adam7_info, 2u8);
}
