use std::hint::black_box;
use std::panic;
use std::sync::Mutex;

static DROP_LOG: Mutex<Vec<i32>> = Mutex::new(Vec::new());

struct Logged(i32);
impl Drop for Logged {
    fn drop(&mut self) { DROP_LOG.lock().unwrap().push(self.0); }
}

fn main() {
    panic::set_hook(Box::new(|_| {}));

    { let r = panic::catch_unwind(|| { panic!("boom"); }); println!("catch/explicit_panic: caught={}", r.is_err()); }
    { let r = panic::catch_unwind(|| { let x: Option<i32> = black_box(None); x.unwrap(); }); println!("catch/unwrap_none: caught={}", r.is_err()); }
    { let r = panic::catch_unwind(|| { let x: Result<i32, &str> = black_box(Err("e")); x.unwrap(); }); println!("catch/unwrap_err: caught={}", r.is_err()); }
    { let r = panic::catch_unwind(|| { let a = black_box([1, 2, 3]); let i = black_box(10usize); let _ = a[i]; }); println!("catch/array_oob: caught={}", r.is_err()); }
    { let r = panic::catch_unwind(|| { let a = black_box(1i32); let b = black_box(0i32); let _ = a / b; }); println!("catch/div_by_zero: caught={}", r.is_err()); }
    { let r = panic::catch_unwind(|| { let a = black_box(i32::MAX); let b = black_box(1i32); let _ = a + b; }); println!("catch/overflow_add: caught={}", r.is_err()); }
    {
        DROP_LOG.lock().unwrap().clear();
        let r = panic::catch_unwind(|| {
            let _a = Logged(1);
            let _b = Logged(2);
            {
                let _c = Logged(3);
                let _d = Logged(4);
                if black_box(true) { panic!("mid-unwind"); }
            }
        });
        println!("drop_order: caught={} log={:?}", r.is_err(), DROP_LOG.lock().unwrap().clone());
    }
    {
        DROP_LOG.lock().unwrap().clear();
        let r = panic::catch_unwind(|| {
            let mut v = Vec::new();
            for i in 0..5i32 {
                v.push(Logged(i));
                if i == black_box(3) { panic!("mid-vec"); }
            }
        });
        println!("drop_order_vec: caught={} log={:?}", r.is_err(), DROP_LOG.lock().unwrap().clone());
    }
}
