use object::elf::VersionIndex;

fn main() {
    let _ = <object::elf::VersionIndex as ::core::ops::Sub<object::elf::VersionIndex>>::sub(VersionIndex(0u16), VersionIndex::default().offset(1u16));
}
