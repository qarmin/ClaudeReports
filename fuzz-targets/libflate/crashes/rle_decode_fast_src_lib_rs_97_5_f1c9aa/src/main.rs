use libflate_lz77::Code;
use libflate_lz77::Lz77Decoder;

fn main() {
    let mut lz77_decoder = Lz77Decoder::default();
    let _ = lz77_decoder.decode(Code::Pointer { length: 0u16, backward_distance: 0u16 });
}
