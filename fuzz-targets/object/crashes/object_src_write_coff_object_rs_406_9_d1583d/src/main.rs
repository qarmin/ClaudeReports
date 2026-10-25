use object::Architecture;
use object::BinaryFormat;
use object::write::CoffExportStyle;
use object::write::Object;

fn main() {
    let mut object = Object::new(BinaryFormat::Pe, Architecture::PowerPc, <object::endian::Endianness as object::endian::Endian>::from_little_endian(false).unwrap());
    object.add_coff_exports(CoffExportStyle::Gnu);
}
