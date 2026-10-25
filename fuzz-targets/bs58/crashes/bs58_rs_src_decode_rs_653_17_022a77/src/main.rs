use bs58::decode::Error;

fn main() {
    let error = Error::BufferTooSmall;
    let _ = error.unwrap_const();
}
