struct D;
impl Drop for D {
    fn drop(&mut self) {
        println!("dropping, about to panic again");
        panic!("second panic");
    }
}

fn main() {
    let _d = D;
    panic!("first panic");
}
