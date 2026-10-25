use object::Architecture;
use object::BinaryFormat;
use object::endian::Endianness;
use object::write::Object;

fn main() {
    let binary_format = BinaryFormat::native_object();
    let mut object = Object::new(binary_format, Architecture::Unknown, Endianness::Big);
    object.add_elf_gnu_property_u32(<object::elf::GnuPropertyType as object::Wrap>::from_inner(0u32), 0u32);
}
