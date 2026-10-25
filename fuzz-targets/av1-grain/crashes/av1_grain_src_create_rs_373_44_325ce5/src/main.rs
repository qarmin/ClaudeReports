use av1_grain::NoiseGenArgs;
use av1_grain::TransferFunction;

fn main() {
    let _ = av1_grain::generate_photon_noise_params(0u64, 0u64, NoiseGenArgs { iso_setting: 0u32, width: 1000u32, height: 100000000u32, transfer_function: TransferFunction::BT1886, full_range: false, chroma_grain: false, random_seed: None });
}
