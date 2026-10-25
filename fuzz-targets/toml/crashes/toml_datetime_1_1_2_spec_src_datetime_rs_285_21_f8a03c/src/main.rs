use toml_datetime::Offset;

fn main() {
    let offset = Offset::Custom { minutes: i16::MIN };
    let _ = offset.to_string();
}
