use clap_builder::error::ErrorKind;

fn main() {
    let error = <clap_builder::error::Error<clap_builder::error::KindFormatter>>::raw(ErrorKind::InvalidUtf8, "".to_string());
    let error2 = error.apply::<clap_builder::error::RichFormatter>();
    let _ = error2.exit();
}
