use alsa::Mixer;

fn main() {
    let _ = Mixer::new("\0", false);
}
