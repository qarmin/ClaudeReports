use alsa::Ctl;

fn main() {
    let _ = Ctl::new("\0", false);
}
