use object::endian::Endianness;
use object::write::elf::AttributesWriter;

fn main() {
    let mut attributes_writer = AttributesWriter::new(Endianness::default());
    attributes_writer.start_subsection(&vec![]);
    attributes_writer.write_attribute_integer(0u64);
}
