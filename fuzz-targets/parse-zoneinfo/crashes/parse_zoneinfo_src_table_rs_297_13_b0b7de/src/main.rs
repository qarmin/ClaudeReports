use parse_zoneinfo::table::Format;

fn main() {
    let format = Format::Placeholder("".to_string());
    let _ = format.format_constant();
}
