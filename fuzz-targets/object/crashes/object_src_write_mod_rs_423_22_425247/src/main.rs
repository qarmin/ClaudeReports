use object::Architecture;
use object::BinaryFormat;
use object::endian::Endianness;
use object::write::Object;
use object::write::StandardSection;

fn main() {
    let mut object = Object::new(BinaryFormat::Elf, Architecture::Arm, Endianness::Big);
    object.set_symbol_data(Object::new(BinaryFormat::Coff, Architecture::Xtensa, Endianness::Little).add_file_symbol(vec![]), Object::new(BinaryFormat::MachO, Architecture::Sparc32Plus, Endianness::default()).section_id(StandardSection::Data), 0u64, 0u64);
}
