use cc::Build;

fn main() {
    let build = Build::new();
    let _ = build.get_archiver();
}
