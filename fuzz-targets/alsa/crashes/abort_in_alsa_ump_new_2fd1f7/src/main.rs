use alsa::Direction;
use alsa::Output;
use alsa::Ump;

fn main() {
    let _ = Output::local_error_handler();
    let _ = Ump::new("\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}\u{feff}", Direction::Capture, false);
}
