use ppv_lite86::generic::GenericMachine;

fn main() {
    let generic_machine = GenericMachine;
    let _ = <ppv_lite86::generic::GenericMachine as ppv_lite86::Machine>::read_be::<ppv_lite86::generic::u32x4_generic>(generic_machine, &vec![]);
}
