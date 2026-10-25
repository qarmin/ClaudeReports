use object::Architecture;
use object::BinaryFormat;
use object::endian::Endianness;
use object::write::Object;
use object::write::StandardSection;

fn main() {
    let mut object = Object::new(BinaryFormat::native_object(), Architecture::X86_64_X32, Endianness::Big);
    let section_id = object.section_id(StandardSection::ReadOnlyString);
    let _ = object.append_section_bss(section_id, 0u64, 0u64);
}
