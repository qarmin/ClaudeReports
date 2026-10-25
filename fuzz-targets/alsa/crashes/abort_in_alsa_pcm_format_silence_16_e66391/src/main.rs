use alsa::pcm::Format;

fn main() {
    let format = Format::dsd_u32();
    let _ = format.silence_16();
}
