use ppv_lite86::generic::GenericMachine;

fn main() {
    <ppv_lite86::generic::u32x4_generic as ppv_lite86::StoreBytes>::write_le(<ppv_lite86::generic::GenericMachine as ppv_lite86::Machine>::read_le::<ppv_lite86::generic::u32x4_generic>(GenericMachine, &vec![0u8, 34u8, 0u8, 67u8, 0u8, 86u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 113u8, 94u8, 0u8, 0u8]), &mut vec![]);
}
