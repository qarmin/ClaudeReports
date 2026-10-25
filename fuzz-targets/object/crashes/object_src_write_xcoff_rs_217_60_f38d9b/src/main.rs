use object::Architecture;
use object::BinaryFormat;
use object::write::Object;

fn main() {
    let object = Object::new(BinaryFormat::Xcoff, Architecture::Unknown, <object::endian::Endianness as object::endian::Endian>::from_big_endian(false).unwrap());
    let _ = object.write();
}
