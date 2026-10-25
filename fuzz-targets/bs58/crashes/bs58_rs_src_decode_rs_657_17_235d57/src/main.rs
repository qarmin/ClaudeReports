use bs58::decode::Error;

fn main() {
    let error = Error::NonAsciiCharacter { index: 0usize };
    let _ = error.unwrap_const();
}
