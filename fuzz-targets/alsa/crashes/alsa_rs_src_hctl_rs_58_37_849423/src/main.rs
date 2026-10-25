use alsa::HCtl;

fn main() {
    let _ = HCtl::new("\0", false);
}
