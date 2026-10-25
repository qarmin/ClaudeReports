use object::Architecture;
use object::BinaryFormat;
use object::ComdatKind;
use object::endian::Endianness;
use object::write::Comdat;
use object::write::Object;

fn main() {
    let comdat = Comdat { kind: ComdatKind::Unknown, symbol: Object::new(BinaryFormat::Wasm, Architecture::Bpf, Endianness::Big).add_file_symbol(vec![]), sections: vec![] };
    let mut object = Object::new(BinaryFormat::Xcoff, Architecture::S390x, <object::endian::Endianness as object::endian::Endian>::from_big_endian(false).unwrap());
    let _ = object.comdat_mut(Object::new(BinaryFormat::MachO, Architecture::X86_64, <object::endian::Endianness as object::endian::Endian>::from_little_endian(false).unwrap()).add_comdat(comdat));
}
