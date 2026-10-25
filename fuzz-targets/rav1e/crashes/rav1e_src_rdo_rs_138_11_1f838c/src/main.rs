use rav1e::bench::transform::TxSize;

fn main() {
    let _ = rav1e::bench::rdo::estimate_rate(0u8, TxSize::TX_16X64, 9223372036854775810u64);
}
