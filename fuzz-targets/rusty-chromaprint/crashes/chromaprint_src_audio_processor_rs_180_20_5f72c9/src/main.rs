use rusty_chromaprint::Configuration;
use rusty_chromaprint::Fingerprinter;

fn main() {
    let mut fingerprinter = Fingerprinter::new(&Configuration::preset_test1());
    fingerprinter.consume(&[]);
}
