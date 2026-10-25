use object::Architecture;
use object::BinaryFormat;
use object::endian::Endianness;
use object::write::Object;

fn main() {
    let object = Object::new(BinaryFormat::MachO, Architecture::Unknown, Endianness::default());
    let _ = object.write();
}
