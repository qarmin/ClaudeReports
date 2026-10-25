use object::Architecture;
use object::BinaryFormat;
use object::endian::Endianness;
use object::write::Object;
use object::write::StandardSection;

fn main() {
    let mut object = Object::new(BinaryFormat::Elf, Architecture::Unknown, Endianness::Little);
    let _ = object.section_mut(Object::new(BinaryFormat::Coff, Architecture::Unknown, Endianness::Little).section_id(StandardSection::Text));
}
