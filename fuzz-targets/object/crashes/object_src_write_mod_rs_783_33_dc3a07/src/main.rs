use object::Architecture;
use object::BinaryFormat;
use object::RelocationEncoding;
use object::RelocationFlags;
use object::RelocationKind;
use object::endian::Endianness;
use object::write::Object;
use object::write::Relocation;
use object::write::StandardSection;

fn main() {
    let mut object = Object::new(BinaryFormat::Xcoff, Architecture::Sparc, Endianness::Little);
    let _ = object.add_relocation_with_subtractor(Object::new(BinaryFormat::Elf, Architecture::E2K32, Endianness::default()).section_id(StandardSection::Data), Relocation { offset: 0u64, symbol: Object::new(BinaryFormat::native_object(), Architecture::PowerPc64, Endianness::Little).add_file_symbol(vec![]), addend: 0i64, flags: RelocationFlags::Generic { kind: RelocationKind::Relative, encoding: RelocationEncoding::E2KDisp, size: 1u8 } }, None);
}
