use bs58::decode::Error;

fn main() {
    let error = Error::InvalidVersion { ver: 0u8, expected_ver: 0u8 };
    let _ = error.unwrap_const();
}
