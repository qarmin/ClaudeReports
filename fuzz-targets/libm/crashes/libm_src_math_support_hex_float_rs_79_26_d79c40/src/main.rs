use libm::support::Round;

fn main() {
    let _ = libm::support::hex_float::parse_any("", 6u32, 0u32, Round::Positive);
}
