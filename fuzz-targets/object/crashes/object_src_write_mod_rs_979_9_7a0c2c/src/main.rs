use object::Architecture;
use object::BinaryFormat;
use object::endian::Endianness;
use object::write::Object;
use object::write::StandardSection;

fn main() {
    let mut object = Object::new(BinaryFormat::Xcoff, Architecture::Hexagon, <object::write::macho::Encoder<object::endian::Endianness>>::new(Endianness::default(), false).endian());
    let section_id = object.section_id(StandardSection::UninitializedTls);
    let _ = object.append_section_data(section_id, &vec![], 0u64);
}
