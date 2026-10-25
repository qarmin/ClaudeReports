use clap_builder::error::ErrorKind;

fn main() {
    let error = <clap_builder::error::Error<clap_builder::error::KindFormatter>>::raw(ErrorKind::ValueValidation, "".to_string());
    let _ = error.exit();
}
