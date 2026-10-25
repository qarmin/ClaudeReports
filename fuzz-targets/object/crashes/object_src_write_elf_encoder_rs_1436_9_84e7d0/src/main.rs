use object::write::elf::AttributesWriter;

fn main() {
    let mut attributes_writer = AttributesWriter::new(<object::endian::Endianness as object::endian::Endian>::from_big_endian(false).unwrap());
    attributes_writer.write_attribute_integer(0u64);
}
