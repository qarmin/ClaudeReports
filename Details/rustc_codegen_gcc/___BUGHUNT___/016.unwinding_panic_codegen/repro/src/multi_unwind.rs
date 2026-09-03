use std::panic;

struct D(&'static str);
impl Drop for D {
    fn drop(&mut self) {
        println!("drop {}", self.0);
    }
}

#[inline(never)]
fn maybe_panic(n: u32, trigger: u32) {
    if n == trigger {
        panic!("boom at {}", n);
    }
    println!("call {} ok", n);
}

#[inline(never)]
fn chain(trigger: u32) {
    let _a = D("a");
    maybe_panic(1, trigger);
    let _b = D("b");
    maybe_panic(2, trigger);
    let _c = D("c");
    maybe_panic(3, trigger);
    let _e = D("d");
    maybe_panic(4, trigger);
    println!("chain completed without panic");
}

fn main() {
    panic::set_hook(Box::new(|_| {}));
    for trigger in [1u32, 2, 3, 4, 99] {
        println!("=== trigger {} ===", trigger);
        let result = panic::catch_unwind(|| {
            chain(trigger);
        });
        println!("caught: {}", result.is_err());
    }
}
