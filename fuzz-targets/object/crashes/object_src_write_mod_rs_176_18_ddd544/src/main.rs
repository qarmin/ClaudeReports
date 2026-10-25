use object::Architecture;
use object::BinaryFormat;
use object::write::Object;
use object::write::StandardSegment;

fn main() {
    let object = Object::new(BinaryFormat::Pe, Architecture::Ia64, <object::endian::Endianness as object::endian::Endian>::from_big_endian(false).unwrap());
    let _ = object.segment_name(StandardSegment::Debug);
}
