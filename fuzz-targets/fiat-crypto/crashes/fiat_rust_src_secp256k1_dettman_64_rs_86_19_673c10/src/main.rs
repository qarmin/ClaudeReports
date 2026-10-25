fn main() {
    fiat_crypto::secp256k1_dettman_64::fiat_secp256k1_dettman_mul(&mut [0u64, 0u64, 0u64, 0u64, 0u64], &[u64::MAX, 0u64, 0u64, 1u64, 0u64], &[0u64, 0u64, u64::MAX, 0u64, 1000000000000000000u64]);
}
