fn main() {
    for decimals in [5, 6, 7, 8] {
        let r = std::panic::catch_unwind(|| emath::format_with_minimum_decimals(1.5, decimals));
        match r {
            Ok(s) => println!("min decimals {decimals}: {s:?} ({} decimals)", s.split('.').nth(1).map_or(0, str::len)),
            Err(_) => println!("min decimals {decimals}: PANIC"),
        }
    }
}
