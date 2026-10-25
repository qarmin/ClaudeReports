use object::Architecture;
use object::BinaryFormat;
use object::endian::Endianness;
use object::write::Object;

fn main() {
    let object = Object::new(BinaryFormat::Pe, Architecture::LoongArch32, Endianness::Big);
    let _ = object.write();
}
