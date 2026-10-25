use object::Architecture;
use object::BinaryFormat;
use object::write::Object;
use object::write::StandardSection;

fn main() {
    let mut object = Object::new(BinaryFormat::native_object(), Architecture::Xtensa, <object::endian::Endianness as object::endian::Endian>::from_little_endian(false).unwrap());
    let section_id = object.section_id(StandardSection::UninitializedData);
    let section = object.section(section_id);
    let _ = section.data();
}
