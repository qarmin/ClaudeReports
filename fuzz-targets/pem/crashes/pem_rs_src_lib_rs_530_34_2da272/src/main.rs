use pem::EncodeConfig;
use pem::Pem;

fn main() {
    let _ = pem::encode_config(&Pem::new("".to_string(), vec![]), EncodeConfig::new().set_line_wrap(0usize));
}
