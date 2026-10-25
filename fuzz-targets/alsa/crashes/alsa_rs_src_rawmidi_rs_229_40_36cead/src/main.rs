use alsa::Direction;
use alsa::Rawmidi;

fn main() {
    let _ = Rawmidi::new("\0", Direction::output(), false);
}
