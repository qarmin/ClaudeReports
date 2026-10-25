use ppv_lite86::generic::GenericMachine;

fn main() {
    <ppv_lite86::generic::u32x4_generic as ppv_lite86::StoreBytes>::write_be(<ppv_lite86::generic::GenericMachine as ppv_lite86::Machine>::read_le::<ppv_lite86::generic::u32x4_generic>(GenericMachine, &vec![57u8, 0u8, 0u8, 120u8, 0u8, 50u8, 0u8, 54u8, 116u8, 0u8, 71u8, 0u8, 0u8, 0u8, 0u8, 101u8]), &mut vec![]);
}
