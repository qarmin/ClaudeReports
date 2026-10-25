# av1-grain - crashes

Source: https://github.com/rust-av/av1-grain @ 3af392b40ddb4973ee6a0ae916d3c7e21222c400

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = av1_grain::generate_photon_noise_params(0u64, 0u64,
    NoiseGenArgs { iso_setting: 0u32, width: 1000u32, height: 100000000u32,
    transfer_function: TransferFunction::BT1886, full_range: false,
    chroma_grain: false, random_seed: None });</pre></td><td valign="top"><pre>thread 'main' panicked at av1-grain/src/create.rs:373:44:
attempt to multiply with overflow
backtrace:
  av1_grain::create::generate_luma_noise_points (av1-grain/src/create.rs:373:44)
  av1_grain::create::generate_photon_noise_params (av1-grain/src/create.rs:140:27)
  repro_av1_grain_src_create_rs_373_44_325ce5::main (src/main.rs:5:13)</pre></td></tr>
</table>
