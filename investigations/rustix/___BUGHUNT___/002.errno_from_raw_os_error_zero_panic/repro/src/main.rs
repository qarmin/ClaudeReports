use std::panic;

fn try_one(label: &str, raw: i32) {
    let result = panic::catch_unwind(|| rustix::io::Errno::from_raw_os_error(raw));
    match result {
        Ok(errno) => {
            println!(
                "{label} (raw={raw}): OK, no panic. Errno::from_raw_os_error({raw}).raw_os_error() = {}",
                errno.raw_os_error()
            );
        }
        Err(payload) => {
            let msg = if let Some(s) = payload.downcast_ref::<&str>() {
                (*s).to_string()
            } else if let Some(s) = payload.downcast_ref::<String>() {
                s.clone()
            } else {
                "<non-string panic payload>".to_string()
            };
            println!("{label} (raw={raw}): PANICKED! message = {msg:?}");
        }
    }
}

fn main() {
    panic::set_hook(Box::new(|_info| {}));

    println!("rustix::io::Errno::from_raw_os_error() probe");
    println!("==============================================");

    try_one("zero (the exact #1578 repro)", 0);
    try_one("negative", -1);
    try_one("negative (typical errno-style, should be fine)", 2);
    try_one(">= 4096 (out of Linux errno range)", 4096);
    try_one(">= 4096 (larger)", 100000);
    try_one("i32::MIN", i32::MIN);
    try_one("i32::MAX", i32::MAX);

    let enoent = rustix::io::Errno::from_raw_os_error(2);
    println!(
        "sanity check: from_raw_os_error(2).raw_os_error() == 2 -> {}",
        enoent.raw_os_error() == 2
    );
}
