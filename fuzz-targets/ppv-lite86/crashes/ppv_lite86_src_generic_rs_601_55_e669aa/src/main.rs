use ppv_lite86::generic::GenericMachine;

fn main() {
    let _ = <ppv_lite86::generic::GenericMachine as ppv_lite86::Machine>::read_le::<ppv_lite86::generic::u64x2_generic>(GenericMachine, &vec![]);
}
