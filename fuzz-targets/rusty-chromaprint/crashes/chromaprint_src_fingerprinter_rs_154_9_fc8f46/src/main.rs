use rusty_chromaprint::Configuration;

fn main() {
    let configuration = Configuration::preset_test2();
    let configuration2 = configuration.clone().with_frame_size(1000000000000000000usize);
    let _ = configuration2.delay();
}
