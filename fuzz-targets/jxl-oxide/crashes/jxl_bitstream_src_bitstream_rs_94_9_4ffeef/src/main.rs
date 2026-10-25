use jxl_bitstream::Bitstream;

fn main() {
    let vec = vec![];
    let mut bitstream = Bitstream::new(&vec);
    let _ = bitstream.peek_bits(10000000000000000000usize);
}
