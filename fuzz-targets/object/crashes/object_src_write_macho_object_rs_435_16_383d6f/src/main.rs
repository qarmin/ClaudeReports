use object::Architecture;
use object::BinaryFormat;
use object::RelocationFlags;
use object::endian::Endianness;
use object::macho::PPC_RELOC_HI16;
use object::write::Object;
use object::write::Relocation;
use object::write::StandardSection;

fn main() {
    let object = Object::new(BinaryFormat::MachO, Architecture::SuperH, Endianness::Little);
    let mut object2 = Object::new(object.format(), Architecture::Xtensa, <object::endian::Endianness as object::endian::Endian>::from_little_endian(false).unwrap());
    let _ = object2.add_relocation(Object::new(BinaryFormat::Elf, Architecture::SuperH, <object::endian::Endianness as object::endian::Endian>::from_little_endian(false).unwrap()).section_id(StandardSection::ReadOnlyDataWithRel), Relocation { offset: 0u64, symbol: Object::new(BinaryFormat::Pe, Architecture::Msp430, Endianness::default()).add_file_symbol(vec![]), addend: 1i64, flags: RelocationFlags::MachO { r_type: PPC_RELOC_HI16, r_pcrel: false, r_length: 10u8 } });
}
