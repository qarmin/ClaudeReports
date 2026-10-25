use object::Architecture;
use object::BinaryFormat;
use object::endian::Endianness;
use object::write::Object;
use object::write::StandardSection;

fn main() {
    let object = Object::new(BinaryFormat::MachO, Architecture::LoongArch32, Endianness::Little);
    let _ = object.section(Object::new(BinaryFormat::native_object(), Architecture::Aarch64_Ilp32, <object::endian::Endianness as object::endian::Endian>::from_little_endian(false).unwrap()).section_id(StandardSection::UninitializedTls));
}
