use object::Architecture;
use object::BinaryFormat;
use object::write::Object;
use object::write::StandardSection;

fn main() {
    let mut object = Object::new(BinaryFormat::Pe, Architecture::M68k, <object::write::macho::Encoder<object::endian::Endianness>>::new(<object::endian::Endianness as object::endian::Endian>::from_big_endian(false).unwrap(), false).endian());
    let _ = object.section_symbol(Object::new(BinaryFormat::native_object(), Architecture::LoongArch64, <object::endian::Endianness as object::endian::Endian>::from_little_endian(false).unwrap()).section_id(StandardSection::UninitializedTls));
}
