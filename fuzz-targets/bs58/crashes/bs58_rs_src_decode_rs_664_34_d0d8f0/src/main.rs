use bs58::decode::Error;

fn main() {
    let error = Error::NoChecksum;
    let _ = error.unwrap_const();
}
