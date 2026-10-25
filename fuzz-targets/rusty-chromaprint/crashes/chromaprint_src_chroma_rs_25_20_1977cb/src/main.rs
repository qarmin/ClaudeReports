use rusty_chromaprint::Configuration;
use rusty_chromaprint::Fingerprinter;

fn main() {
    let _ = Fingerprinter::new(&Configuration::preset_test3().with_frame_size(10000000000000000000usize));
}
