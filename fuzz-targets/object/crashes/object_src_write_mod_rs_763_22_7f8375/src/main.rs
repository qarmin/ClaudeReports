use object::Architecture;
use object::BinaryFormat;
use object::RelocationFlags;
use object::elf::R_TILEGX_DEST_IMM8_X1;
use object::endian::Endianness;
use object::write::Object;
use object::write::Relocation;
use object::write::StandardSection;

fn main() {
    let object = Object::new(BinaryFormat::Elf, Architecture::Wasm32, <object::endian::Endianness as object::endian::Endian>::from_big_endian(false).unwrap());
    let mut object2 = Object::new(object.format(), Architecture::E2K32, <object::endian::Endianness as object::endian::Endian>::from_big_endian(false).unwrap());
    let _ = object2.add_relocation(Object::new(BinaryFormat::native_object(), Architecture::Arm, <object::endian::Endianness as object::endian::Endian>::from_little_endian(false).unwrap()).section_id(StandardSection::GnuProperty), Relocation { offset: 0u64, symbol: Object::new(BinaryFormat::Wasm, Architecture::LoongArch32, Endianness::Big).add_file_symbol(vec![]), addend: 0i64, flags: RelocationFlags::Elf { r_type: R_TILEGX_DEST_IMM8_X1 } });
}
