use object::Architecture;
use object::BinaryFormat;
use object::endian::Endianness;
use object::write::Object;

fn main() {
    let mut object = Object::new(BinaryFormat::Coff, Architecture::E2K32, Endianness::Little);
    let _ = object.symbol_mut(Object::new(BinaryFormat::Xcoff, Architecture::Aarch64, <object::endian::Endianness as object::endian::Endian>::from_little_endian(false).unwrap()).add_file_symbol(vec![]));
}
