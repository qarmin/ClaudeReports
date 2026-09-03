// Verbatim (whitespace-normalized) reproduction from
// https://github.com/rust-lang/rustc_codegen_gcc/issues/691 - a rustlantis-minimized program
// where the reporter observed `!arg as usize` (bool -> usize cast after logical negation)
// print as `-1` in a GCC "release" build, and the correct `1` in a GCC "debug" build.
fn dump_var(var2: usize, val2: usize, var3: usize) {
    println!("val2:{val2}")
}
#[inline(never)]
fn dump_var2(var2: usize, val2: usize, var3: usize) {
    println!("val2:{val2}")
}
fn fn6(arg: bool, arg2: usize) -> bool {
    dump_var(0, arg2, 0);
    dump_var2(0, !arg as usize, 0);
    true
}
fn main() {
    let mut _1 = false;
    let mut _3 = 2;
    let mut _4 = (&1, 1);
    loop {
        if _4.1 == 0 {
            if _4.1 == 1 {
                _1 = true;
            } else {
                _1 = false;
            }
        } else if fn6(_1, _3) {
            loop {
                _3 = _4.1;
                let mut _6 = 0;
                _6 = 0;
                let tmp2 = _6;
                if tmp2 == 1 {
                    _4.0 = &_4.1
                } else if tmp2 == 0 {
                    return;
                } else {
                    break;
                }
            }
        }
    }
}
