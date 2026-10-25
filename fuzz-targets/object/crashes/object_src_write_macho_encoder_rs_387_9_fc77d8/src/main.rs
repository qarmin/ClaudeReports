use object::endian::Endianness;

fn main() {
    let encoder = <object::write::macho::Encoder<object::endian::Endianness>>::new(Endianness::Big, false);
    let _ = encoder.load_command_size(u64::MAX);
}
