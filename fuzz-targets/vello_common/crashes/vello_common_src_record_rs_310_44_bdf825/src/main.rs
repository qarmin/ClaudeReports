fn main() {
    let mut command_recorder = <vello_common::record::CommandRecorder<u8> as ::core::default::Default>::default();
    let _ = command_recorder.pop_layer();
}
