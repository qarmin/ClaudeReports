use tracing_subscriber::EnvFilter;

fn main() {
    let _ = EnvFilter::try_new::<String>("\n攼=".to_string());
}
