use alsa::mixer::SelemId;

fn main() {
    let _ = SelemId::new("\0", 0u32);
}
