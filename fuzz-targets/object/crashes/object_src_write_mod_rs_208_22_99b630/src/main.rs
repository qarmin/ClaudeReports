use object::Architecture;
use object::BinaryFormat;
use object::write::Object;
use object::write::StandardSection;

fn main() {
    let mut object = Object::new(BinaryFormat::Pe, Architecture::Aarch64, <object::endian::Endianness as object::endian::Endian>::from_little_endian(false).unwrap());
    let _ = object.append_section_data(Object::new(BinaryFormat::Xcoff, Architecture::X86_64, <object::endian::Endianness as object::endian::Endian>::from_big_endian(false).unwrap()).section_id(StandardSection::ReadOnlyData), &vec![], 0u64);
}
