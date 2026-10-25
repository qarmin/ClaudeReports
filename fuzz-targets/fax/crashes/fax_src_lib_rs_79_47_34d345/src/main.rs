use fax::Bits;
use fax::VecWriter;

fn main() {
    let bits = Bits { data: 0u16, len: 100u8 };
    let mut vec_writer = VecWriter::new();
    let _ = <fax::VecWriter as fax::BitWriter>::write(&mut vec_writer, bits);
}
