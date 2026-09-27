use std::thread;
use std::panic;

struct D(&'static str);
impl Drop for D {
    fn drop(&mut self) {
        println!("drop {}", self.0);
    }
}

fn main() {
    panic::set_hook(Box::new(|_| {}));

    let handle = thread::spawn(|| {
        let _g = D("thread-guard");
        panic!("panic in thread");
    });
    let result = handle.join();
    println!("thread panicked: {}", result.is_err());
    if let Err(payload) = result {
        if let Some(s) = payload.downcast_ref::<&str>() {
            println!("payload: {}", s);
        } else {
            println!("payload: <unknown>");
        }
    }

    // resume_unwind test
    let outer = panic::catch_unwind(|| {
        let inner_result = panic::catch_unwind(|| {
            panic!("resumable panic");
        });
        if let Err(e) = inner_result {
            let _g = D("resume-guard");
            panic::resume_unwind(e);
        }
    });
    println!("outer caught: {}", outer.is_err());
    if let Err(payload) = outer {
        if let Some(s) = payload.downcast_ref::<&str>() {
            println!("outer payload: {}", s);
        }
    }

    println!("main done");
}
