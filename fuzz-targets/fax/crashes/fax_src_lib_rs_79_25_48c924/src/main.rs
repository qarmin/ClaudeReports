use fax::Bits;
use fax::VecWriter;

fn main() {
    let mut vec_writer = VecWriter::new();
    let _ = <fax::VecWriter as fax::BitWriter>::write(&mut vec_writer, Bits { data: 0u16, len: 0u8 });
}
