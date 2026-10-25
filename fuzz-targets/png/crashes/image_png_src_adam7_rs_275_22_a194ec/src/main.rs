use png::Adam7Info;

fn main() {
    png::expand_interlaced_row(&mut vec![], 10000000000000000000usize, &include_bytes!("image_0.png").to_vec(), &Adam7Info::new(4u8, 163954023u32, 3780522784u32), 1u8);
}
