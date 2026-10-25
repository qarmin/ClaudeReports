use ppv_lite86::generic::GenericMachine;

fn main() {
    <ppv_lite86::generic::u64x2_generic as ppv_lite86::StoreBytes>::write_le(<ppv_lite86::generic::GenericMachine as ppv_lite86::Machine>::read_be::<ppv_lite86::generic::u64x2_generic>(GenericMachine, &vec![0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8]), &mut vec![]);
}
