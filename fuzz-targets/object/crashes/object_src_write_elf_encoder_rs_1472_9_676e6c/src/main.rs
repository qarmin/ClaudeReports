use object::endian::Endianness;
use object::write::elf::AttributesWriter;

fn main() {
    let mut attributes_writer = AttributesWriter::new(<object::write::macho::Encoder<object::endian::Endianness>>::new(Endianness::default(), false).endian());
    attributes_writer.start_subsection(&vec![]);
    let _ = attributes_writer.data();
}
