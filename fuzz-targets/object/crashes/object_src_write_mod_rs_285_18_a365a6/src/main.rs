use object::Architecture;
use object::BinaryFormat;
use object::write::Object;
use object::write::StandardSection;

fn main() {
    let _ = Object::new(BinaryFormat::Wasm, Architecture::Wasm32, <object::endian::Endianness as object::endian::Endian>::from_little_endian(false).unwrap()).section_id(StandardSection::ReadOnlyDataWithRel);
}
