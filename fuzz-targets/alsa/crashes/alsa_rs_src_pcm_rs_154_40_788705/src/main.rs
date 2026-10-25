use alsa::Direction;
use alsa::PCM;

fn main() {
    let _ = PCM::new("\0", Direction::Capture, false);
}
