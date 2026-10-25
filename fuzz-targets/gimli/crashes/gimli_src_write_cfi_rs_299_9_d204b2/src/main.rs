use gimli::RiscV;
use gimli::write::Address;
use gimli::write::CallFrameInstruction;
use gimli::write::FrameDescriptionEntry;

fn main() {
    let call_frame_instruction = CallFrameInstruction::Restore(RiscV::X28);
    let mut frame_description_entry = FrameDescriptionEntry::new(Address::Constant(0u64), 0u32);
    frame_description_entry.add_instruction(1u32, CallFrameInstruction::RestoreState);
    frame_description_entry.add_instruction(0u32, call_frame_instruction.clone());
}
