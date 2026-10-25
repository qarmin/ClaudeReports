use toml::Value;
use toml_datetime::Date;
use toml_datetime::Datetime;

fn main() {
    let value = Value::Datetime(Datetime::from(Date { year: 0u16, month: 0u8, day: 0u8 }));
    let _ = value.to_string();
}
