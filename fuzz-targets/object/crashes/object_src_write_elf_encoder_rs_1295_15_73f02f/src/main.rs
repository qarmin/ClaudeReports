use object::elf::EM_COREA_2ND;

fn main() {
    let encoder = <object::write::elf::Encoder<object::endian::Endianness>>::new(<object::write::macho::Encoder<object::endian::Endianness>>::new(<object::endian::Endianness as object::endian::Endian>::from_big_endian(false).unwrap(), false).endian(), false, EM_COREA_2ND);
    let _ = encoder.gnu_verneed_size(0u16, 10000000000000000000usize);
}
