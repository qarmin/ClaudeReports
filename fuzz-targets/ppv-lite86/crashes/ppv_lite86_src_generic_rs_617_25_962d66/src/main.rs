use ppv_lite86::generic::GenericMachine;

fn main() {
    let u64x2_generic = <ppv_lite86::generic::u64x2_generic as ppv_lite86::Swap64>::swap4(<ppv_lite86::generic::GenericMachine as ppv_lite86::Machine>::read_be::<ppv_lite86::generic::u64x2_generic>(GenericMachine, &vec![66u8, 77u8, 70u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 54u8, 0u8, 0u8, 0u8, 40u8, 0u8]));
    <ppv_lite86::generic::u64x2_generic as ppv_lite86::StoreBytes>::write_be(u64x2_generic, &mut vec![]);
}
