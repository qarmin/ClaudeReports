use object::build::elf::Builder;
use object::endian::Endianness;

fn main() {
    let builder = Builder::new(Endianness::Little, false);
    let encoder = builder.encoder();
    let _ = encoder.gnu_verdef_size(0u16, 10000000000000000000usize);
}
