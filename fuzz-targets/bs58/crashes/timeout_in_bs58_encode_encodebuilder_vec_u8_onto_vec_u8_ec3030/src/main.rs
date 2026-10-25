use bs58::Alphabet;

fn main() {
    let alphabet = Alphabet::FLICKR;
    let encode_builder = <bs58::encode::EncodeBuilder<Vec<u8>>>::new(include_bytes!("data_0.bin").to_vec(), alphabet);
    let _ = encode_builder.onto(include_bytes!("Cargo.toml").to_vec());
}
