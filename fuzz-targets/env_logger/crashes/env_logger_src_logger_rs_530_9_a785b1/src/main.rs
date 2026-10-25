use env_logger::Builder;

fn main() {
    let mut builder = Builder::new();
    let _ = builder.build();
    let _ = builder.try_init();
}
