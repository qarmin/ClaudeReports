use alsa::Direction;
use alsa::Ump;

fn main() {
    let _ = Ump::new("\0", Direction::Capture, false);
}
