use tracing_subscriber::fmt::time::ChronoUtc;

fn main() {
    let mut formatted_fields = <tracing_subscriber::fmt::FormattedFields<tracing_subscriber::fmt::format::Pretty>>::new("".to_string());
    let mut writer = formatted_fields.as_writer();
    let _ = <tracing_subscriber::fmt::time::ChronoUtc as tracing_subscriber::fmt::time::FormatTime>::format_time(&ChronoUtc::new("%".to_string()), &mut writer);
}
