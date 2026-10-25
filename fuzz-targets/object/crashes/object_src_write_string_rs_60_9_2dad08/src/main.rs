use object::Architecture;
use object::BinaryFormat;
use object::write::Object;
use object::write::StandardSection;

fn main() {
    let mut object = Object::new(BinaryFormat::Coff, Architecture::M68k, <object::endian::Endianness as object::endian::Endian>::from_little_endian(false).unwrap());
    let _ = object.add_subsection(StandardSection::TlsVariables, &vec![80u8, 78u8, 71u8, 0u8, 0u8, 0u8, 0u8, 0u8]);
    let _ = object.write();
}
