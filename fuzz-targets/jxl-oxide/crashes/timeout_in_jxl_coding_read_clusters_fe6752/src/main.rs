use jxl_bitstream::Bitstream;

fn main() {
    let vec = vec![73u8, 73u8, 42u8, 0u8, 39u8, 0u8];
    let mut bitstream = Bitstream::new(&vec);
    let _ = jxl_coding::read_clusters(&mut bitstream, u32::MAX);
}
