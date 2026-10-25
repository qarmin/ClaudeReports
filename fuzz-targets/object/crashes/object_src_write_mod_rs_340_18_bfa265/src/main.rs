use object::Architecture;
use object::BinaryFormat;
use object::endian::Endianness;
use object::write::Object;
use object::write::StandardSection;

fn main() {
    let _ = Object::new(BinaryFormat::Xcoff, Architecture::Wasm32, Endianness::Little).add_subsection(StandardSection::UninitializedData, &vec![]);
}
