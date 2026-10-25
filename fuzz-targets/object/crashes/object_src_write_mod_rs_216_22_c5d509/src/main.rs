use object::Architecture;
use object::BinaryFormat;
use object::elf::Machine;
use object::endian::Endianness;
use object::write::Object;
use object::write::StandardSection;

fn main() {
    let mut object = Object::new(BinaryFormat::Coff, Architecture::Mips64, <object::write::elf::Encoder<object::endian::Endianness>>::new(Endianness::Big, false, Machine::default()).endian());
    let _ = object.append_section_bss(Object::new(BinaryFormat::native_object(), Architecture::Aarch64_Ilp32, <object::endian::Endianness as object::endian::Endian>::from_little_endian(false).unwrap()).section_id(StandardSection::TlsVariables), 0u64, 0u64);
}
