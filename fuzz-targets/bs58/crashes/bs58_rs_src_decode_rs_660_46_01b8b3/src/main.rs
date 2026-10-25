use bs58::decode::Error;

fn main() {
    let error = Error::InvalidChecksum { checksum: [0u8, 0u8, 0u8, 0u8], expected_checksum: [0u8, 0u8, 0u8, 0u8] };
    let _ = error.unwrap_const();
}
