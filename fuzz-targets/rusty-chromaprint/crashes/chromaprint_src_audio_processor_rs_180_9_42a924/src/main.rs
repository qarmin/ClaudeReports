use rusty_chromaprint::Configuration;
use rusty_chromaprint::Fingerprinter;

fn main() {
    let mut fingerprinter = Fingerprinter::new(&Configuration::default());
    let _ = fingerprinter.start(1000000000u32, 10000u32);
    fingerprinter.consume(&[0i16]);
}
