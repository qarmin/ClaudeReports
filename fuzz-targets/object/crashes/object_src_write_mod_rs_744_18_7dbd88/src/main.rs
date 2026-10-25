use object::Architecture;
use object::BinaryFormat;
use object::RelocationFlags;
use object::endian::Endianness;
use object::write::Object;
use object::write::Relocation;
use object::write::StandardSection;

fn main() {
    let mut object = Object::new(BinaryFormat::Wasm, Architecture::LoongArch64, Endianness::default());
    let _ = object.add_relocation(Object::new(BinaryFormat::Coff, Architecture::LoongArch32, Endianness::Big).section_id(StandardSection::EhFrame), Relocation { offset: 0u64, symbol: Object::new(BinaryFormat::MachO, Architecture::Riscv32, <object::endian::Endianness as object::endian::Endian>::from_little_endian(false).unwrap()).add_file_symbol(vec![]), addend: 0i64, flags: RelocationFlags::Wasm { r_type: 0u8 } });
}
