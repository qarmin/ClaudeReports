use clap_builder::builder::ValueRange;

fn main() {
    let _ = ValueRange::from(usize::MAX..12usize);
}
