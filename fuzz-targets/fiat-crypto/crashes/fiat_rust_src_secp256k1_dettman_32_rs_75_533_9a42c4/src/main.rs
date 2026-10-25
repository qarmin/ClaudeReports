fn main() {
    fiat_crypto::secp256k1_dettman_32::fiat_secp256k1_dettman_mul(&mut [0u32, 0u32, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32, 0u32], &[0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, u32::MAX, 0u32, 10000u32], &[0u32, 100000000u32, 0u32, u32::MAX, 0u32, 1u32, 0u32, 0u32, 0u32, 0u32]);
}
