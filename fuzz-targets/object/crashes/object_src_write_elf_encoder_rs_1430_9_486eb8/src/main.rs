use object::endian::Endianness;
use object::write::elf::AttributesWriter;

fn main() {
    let mut attributes_writer = AttributesWriter::new(Endianness::Big);
    attributes_writer.start_subsection(&vec![]);
    attributes_writer.write_attribute_tag(0u64);
}
