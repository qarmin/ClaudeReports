use object::write::elf::AttributesWriter;

fn main() {
    let mut attributes_writer = AttributesWriter::new(<object::write::macho::Encoder<object::endian::Endianness>>::new(<object::endian::Endianness as object::endian::Endian>::from_big_endian(false).unwrap(), false).endian());
    attributes_writer.write_subsubsection_indices(&vec![]);
}
