extern "C" fn panics() {
    panic!("panic across extern C");
}

fn main() {
    println!("before");
    panics();
    println!("after");
}
