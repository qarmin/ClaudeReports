fn main() {
    let _ = alsa::poll::poll(&mut vec![], -1000000000i32);
}
