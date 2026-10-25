use rav1e::bench::transform::TxSize;

fn main() {
    let _ = rav1e::bench::rdo::estimate_rate(0u8, TxSize::TX_8X4, 10000000000000000000u64);
}
