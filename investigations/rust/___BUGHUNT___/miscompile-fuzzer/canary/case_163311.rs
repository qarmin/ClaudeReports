static mut OUT: u8 = 1;

unsafe fn f() {
    let (mut w, mut z, mut y, mut n, mut o, mut q, mut c, mut d, mut a) =
        (false, false, true, false, false, false, false, false, false);

    for mut x in 0..2 {
        let mut branch = false;
        loop {
            'step: {
                if x < 1 {
                    y = false;
                    branch = OUT > 0;
                    if branch {
                        break 'step;
                    }
                }
                loop {
                    if !o {
                        o = !y;
                        z = x < 1;
                        if o {
                            x = 3;
                        } else {
                            q = true;
                        }
                    }
                    if !z || q {
                        if !z {
                            OUT = x;
                            n = w;
                            d = n;
                        }
                        if !d {
                            if !z {
                                c = !n;
                            }
                            if z || !c {
                                d = true;
                            }
                            if c || !d {
                                c = false;
                                OUT = x;
                            }
                        }

                        if d {
                            return;
                        }
                    }
                    z = false;
                    w = !o;
                    if o {
                        if x < 1 {
                            a = true;
                        }
                        break;
                    }
                }
            }

            if branch {
                break;
            }
            if a {
                break;
            }
            OUT = 2;
            break;
        }

        if !branch {
            break;
        }
    }
}

fn main() {
    print!("{}", unsafe {
        f();
        OUT
    });
}
