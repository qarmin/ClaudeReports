// Real, public rustix::fs::fstat API, called on a real std::fs::File's real fd.
// Cross-interpreted for the illumos target to see whether Miri's OS shim models
// `fstat()` as leaving any part of `libc::stat` uninitialized.
fn main() {
    let f = std::fs::File::open("Cargo.toml").unwrap();
    use std::os::fd::AsFd;
    let stat = rustix::fs::fstat(f.as_fd());
    println!("fstat result: {:?}", stat.map(|s| s.st_size));
}
