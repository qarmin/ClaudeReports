use rusty_chromaprint::Configuration;
use rusty_chromaprint::Fingerprinter;

fn main() {
    let configuration = Configuration::preset_test3();
    let _ = Fingerprinter::new(&configuration.clone().with_frame_size(1000000000usize));
}
