use std::panic;

struct D(&'static str);
impl Drop for D {
    fn drop(&mut self) {
        println!("drop {}", self.0);
    }
}

fn level3() {
    let _a = D("level3-a");
    let _b = D("level3-b");
    panic!("boom");
}

fn level2() {
    let _c = D("level2-c");
    level3();
    let _unreached = D("level2-unreached");
}

fn level1() {
    let _d = D("level1-d");
    level2();
}

fn main() {
    panic::set_hook(Box::new(|_| {}));
    let result = panic::catch_unwind(|| {
        level1();
    });
    println!("caught: {}", result.is_err());

    // nested catch_unwind
    let result2 = panic::catch_unwind(|| {
        let _e = D("outer-e");
        let inner = panic::catch_unwind(|| {
            let _f = D("inner-f");
            panic!("inner boom");
        });
        println!("inner caught: {}", inner.is_err());
        panic!("outer boom");
    });
    println!("caught2: {}", result2.is_err());
}
