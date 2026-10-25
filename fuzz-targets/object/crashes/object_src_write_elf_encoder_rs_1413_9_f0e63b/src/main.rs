use object::write::elf::AttributesWriter;

fn main() {
    let mut attributes_writer = AttributesWriter::new(<object::endian::Endianness as object::endian::Endian>::from_little_endian(false).unwrap());
    attributes_writer.start_subsection(&vec![]);
    attributes_writer.write_subsubsection_index(0u32);
}
