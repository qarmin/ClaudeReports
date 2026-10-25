use object::Architecture;
use object::BinaryFormat;
use object::endian::Endianness;
use object::write::Object;
use object::write::StandardSection;

fn main() {
    let mut object = Object::new(BinaryFormat::Coff, Architecture::Alpha, Endianness::Little);
    let section_id = object.section_id(StandardSection::ReadOnlyData);
    let _ = object.add_symbol_data(Object::new(BinaryFormat::Wasm, Architecture::Sharc, Endianness::Big).add_file_symbol(vec![]), section_id, &vec![], 10u64);
}
