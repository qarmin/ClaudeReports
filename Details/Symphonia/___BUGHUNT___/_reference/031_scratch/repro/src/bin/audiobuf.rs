use symphonia_core::audio::{AudioBuffer, AudioSpec, Channels};

fn main() {
    let spec = AudioSpec::new(44100u32, Channels::None);
    println!("channels.count() = {}", spec.channels().count());
    let _buf: AudioBuffer<f32> = AudioBuffer::new(spec, 1024);
    println!("constructed ok");
}
